//! Root manifest: the per-service chain heads and the registry of all chunks.
//!
//! `manifest.json` lives at the bucket root and links every chunk together with
//! its hash-chain values. Chains are **per service** (see ADR-0003): each
//! service has an independent append-only chain, so services ingest in parallel
//! while writes within a service stay serialized. The manifest records the
//! current head of each service's chain so `verify` knows where to start.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::chunk::ChunkId;
use crate::error::Result;
use crate::hashchain::ChunkHash;

/// One chunk's entry in a service's chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    /// Which chunk this entry describes.
    pub chunk: ChunkId,
    /// SHA-256 of this chunk's encrypted bytes.
    pub hash: ChunkHash,
    /// SHA-256 of the previous chunk in this service's chain
    /// (`hashchain::GENESIS` for the service's first chunk).
    pub prev_hash: ChunkHash,
}

/// The root manifest: one append-only chain per service.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Manifest {
    /// Manifest schema version, for forward compatibility.
    pub version: u32,
    /// Per-service chains, keyed by service name, each in write order.
    pub chains: BTreeMap<String, Vec<ManifestEntry>>,
}

impl Manifest {
    /// Current head (hash of the most recently written chunk) for `service`, if any.
    pub fn head(&self, service: &str) -> Option<&ChunkHash> {
        self.chains
            .get(service)
            .and_then(|c| c.last())
            .map(|e| &e.hash)
    }

    /// Append a new chunk entry, extending the chain for `entry.chunk.service`.
    ///
    /// TODO(impl): validate `entry.prev_hash` matches that service's current
    /// head (or `GENESIS` for the first chunk) before appending, then persist.
    pub fn append(&mut self, entry: ManifestEntry) -> Result<()> {
        let _ = entry;
        todo!("append a chunk entry to its service's chain")
    }
}
