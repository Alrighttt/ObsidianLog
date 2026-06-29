//! Append-only hash chaining for tamper-evidence.
//!
//! Each encrypted chunk gets a SHA-256 hash that is embedded in the next
//! chunk's header (`chunk_n.prev_hash = SHA-256(chunk_{n-1})`). Any deletion,
//! reorder, or modification breaks the chain at a detectable position. Backed
//! by `sha2`.
//!
//! Chains are maintained **per service** (see ADR-0003): each service has its
//! own independent chain so services can ingest in parallel, while writes within
//! a single service are serialized. [`ChunkHash`] and [`GENESIS`] are defined in
//! `obsidianlog-core` and re-exported here.

use obsidianlog_core::error::Result;

pub use obsidianlog_core::types::{ChunkHash, GENESIS};

/// Compute the SHA-256 hash of an encrypted chunk's bytes.
///
/// TODO(impl): SHA-256 over the canonical chunk encoding.
pub fn hash_chunk(chunk_bytes: &[u8]) -> ChunkHash {
    let _ = chunk_bytes;
    todo!("SHA-256 the encrypted chunk")
}

/// Verify that `chunk` correctly chains onto `prev_hash` (within one service's chain).
///
/// TODO(impl): recompute the expected link and compare; return
/// [`obsidianlog_core::Error::ChainIntegrity`] on mismatch.
pub fn verify_link(prev_hash: &ChunkHash, chunk: &[u8]) -> Result<()> {
    let _ = (prev_hash, chunk);
    todo!("verify a single hash-chain link")
}
