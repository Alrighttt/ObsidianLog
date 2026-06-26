//! Client-side AES-256-GCM authenticated encryption.
//!
//! Encryption happens before any data leaves the user's infrastructure. The key
//! is user-generated and never transmitted; the GCM tag authenticates the
//! ciphertext so tampering is rejected at decryption time. Backed by `aes-gcm`.
//!
//! ## Nonce discipline (critical)
//!
//! AES-GCM nonces MUST be unique per key, forever — a single reuse under one key
//! is catastrophic (it leaks the XOR of plaintexts and breaks authentication).
//! ObsidianLog therefore **derives nonces deterministically from a per-service
//! monotonic counter** ([`derive_nonce`]) rather than sampling them randomly:
//! at archival scale, random 96-bit nonces hit the birthday bound and risk
//! collision. See ADR-0002. [`Key`] and the length constants live in
//! `obsidianlog-core` and are re-exported here.

use obsidianlog_core::error::Result;

pub use obsidianlog_core::types::{KEY_LEN, Key, NONCE_LEN};

/// Derive a unique 96-bit nonce from a per-service monotonic counter.
///
/// TODO(impl): encode `counter` into a [`NONCE_LEN`]-byte nonce (e.g.
/// big-endian in the low bytes). Because the counter is monotonic and scoped to
/// one (key, service), every nonce is unique without random-collision risk.
/// See ADR-0002.
pub fn derive_nonce(counter: u64) -> [u8; NONCE_LEN] {
    let _ = counter;
    todo!("derive a deterministic AES-GCM nonce from the per-service counter")
}

/// Encrypt a compressed batch with AES-256-GCM under `key` and `nonce`.
///
/// `nonce` MUST come from [`derive_nonce`] (a per-service monotonic counter) so
/// it is never reused under `key`.
///
/// TODO(impl): encrypt with `key`/`nonce` and return the framed
/// `nonce || ciphertext || tag` blob.
pub fn encrypt(plaintext: &[u8], key: &Key, nonce: &[u8; NONCE_LEN]) -> Result<Vec<u8>> {
    let _ = (plaintext, key, nonce);
    todo!("AES-256-GCM encrypt the compressed batch")
}

/// Authenticated-decrypt a framed chunk with AES-256-GCM.
///
/// TODO(impl): split the `nonce || ciphertext || tag` framing, verify the GCM
/// tag, and return plaintext (or [`obsidianlog_core::Error::Encryption`] on tag
/// mismatch).
pub fn decrypt(framed: &[u8], key: &Key) -> Result<Vec<u8>> {
    let _ = (framed, key);
    todo!("AES-256-GCM decrypt and authenticate the chunk")
}
