# 0005 — Storage data model

- Status: Accepted
- Date: 2026-06-29

## Context

The storage pipeline needs a shared, serializable data model that every crate
agrees on: what a parsed log record is, what a chunk looks like on disk, what the
index and manifest contain, and — critically — exactly which bytes are hashed for
tamper-evidence. This ADR records those type definitions and the two decisions
that aren't obvious from the structs alone: the canonical hashing layout and the
per-service chain structure. The model lives in `obsidianlog-core` as types and
serialization only (no business logic), so it can be depended on everywhere
without dragging in crypto or I/O.

## Decision

### Types

- **`LogRecord` / `LogBatch`** — a parsed log line (raw JSON plus extracted
  `timestamp`, `service`, `level`, `host`, `trace_id`) and a batch of them as
  received from ingest, before chunking.
- **`ChunkHeader`** — `service`, `time_window` (`YYYY-MM-DD-HH`), `sequence`,
  `prev_hash: [u8;32]`, `nonce: [u8;12]`, `created_at`, `record_count`,
  `uncompressed_len`.
- **`Chunk`** — `{ header, ciphertext }`, the unit written to storage.
- **`ChunkRef`** — `(service, window, sequence)`, used by the index and manifest
  to point at a chunk.
- **`ServiceWindowIndex`** (alias `IndexEntry`) — per-`(service, window)`
  metadata: min/max timestamp, the sets of levels and hosts present, a keyword
  token set for prefiltering, and the `ChunkRef`. Targets <1% of raw log size.
- **`ManifestServiceChain`** — `{ service, head_hash, next_sequence, chunks }`.
- **`Manifest`** — `{ bucket, services: Map<service, ManifestServiceChain>,
  version }`.
- **`StorageBackend`** — the async trait for putting/getting chunks and indexes,
  listing chunk references, and reading/writing the manifest.

### Canonical hashing layout, and why it covers `prev_hash`

A chunk's hash is `SHA-256(ChunkHeader::canonical_bytes() ++ ciphertext)`.
`canonical_bytes` is a fixed, big-endian, length-prefixed encoding (see
`chunk.rs` for the field table) — deliberately hand-rolled rather than delegated
to a format like JSON, whose field ordering and whitespace are not guaranteed
stable. A unit test pins the exact byte layout so it cannot change accidentally.

The hashed bytes **include `prev_hash`** and **exclude the chunk's own hash**:

- Including `prev_hash` is what makes the structure a *chain*: each chunk commits
  to its predecessor's hash, so deleting, reordering, or modifying any earlier
  chunk changes every subsequent hash and is detected by `verify`.
- The chunk has no hash field of its own — the hash is derived from these bytes
  and stored *outside* the chunk (in the next chunk's `prev_hash` and in the
  manifest head). Hashing a value into itself is impossible, so this is both
  necessary and explicitly tested.

### Per-service chain structure

Chains are partitioned per service (see ADR-0003): the manifest holds one
`ManifestServiceChain` per service, each with its own `head_hash` and
`next_sequence`. This lets independent services ingest in parallel — writes are
serialized only within a service — while each service's chain remains
independently verifiable. The per-service partition is also the scope of the
monotonic nonce counter from ADR-0002.

## Consequences

- One serializable model is shared across the pipeline, ingest, and CLI; the
  `StorageBackend` trait is the only seam to durable storage (ADR-0004).
- The hashing layout is locked by a golden-bytes test; changing it is a
  deliberate, breaking act that the test forces you to acknowledge.
- The async `StorageBackend` (via `async-trait`) is object-safe and returns
  `Send` futures, so the ingest server can hold a backend across `.await` points
  on a multi-threaded runtime.
- `obsidianlog-store`'s existing pipeline stubs (`chunk`, `index`, `manifest`,
  `hashchain`) will be migrated onto these core types as their logic is
  implemented in subsequent Month 1 tasks.
