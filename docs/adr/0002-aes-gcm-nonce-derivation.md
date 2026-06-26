# 0002 — Deterministic AES-GCM nonce derivation

- Status: Accepted
- Date: 2026-06-26

## Context

Chunks are encrypted client-side with AES-256-GCM before they leave the user's
infrastructure. AES-GCM's security depends on an absolute invariant: a (key,
nonce) pair must never repeat. A single nonce reuse under one key is
catastrophic — it leaks the XOR of the two plaintexts and allows forgery of the
authentication tag.

The tempting default is to sample a random 96-bit nonce per chunk. At
archival scale that is unsafe: by the birthday bound, the probability of a
collision becomes non-negligible after roughly 2^32 chunks under one key, and
ObsidianLog is built to archive logs indefinitely. We cannot "random-and-hope".

## Decision

We derive nonces **deterministically from a per-service monotonic counter**
rather than sampling them randomly. Each (key, service) maintains a counter that
increments once per chunk; the counter is encoded into the 96-bit nonce
(`encryption::derive_nonce`). Because the counter is monotonic and scoped per
service, every nonce is unique by construction, with zero collision probability.

Chains — and therefore counters — are partitioned per service (see ADR-0003),
and writes within a service are serialized, so the counter has a single writer.

## Consequences

- Nonce uniqueness is guaranteed structurally, not probabilistically.
- The per-service counter must be persisted with the manifest and recovered on
  restart; resuming from a stale counter would reuse a nonce, so recovery reads
  the current head before issuing the next nonce.
- The nonce is still stored in the chunk framing (`nonce || ciphertext || tag`),
  so decryption needs no counter state.
- Counter state is one more thing the per-service write path must own; this is
  accepted as the cost of eliminating reuse risk.
