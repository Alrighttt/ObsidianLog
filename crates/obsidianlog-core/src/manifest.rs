//! Root manifest: per-service chain heads and the chunk registry.
//!
//! `manifest.json` lives at the bucket root and tracks, for each service, its
//! hash-chain head, the next sequence number to assign, and the ordered list of
//! its chunks. Chains are per service (see ADR-0003), so the manifest holds one
//! [`ManifestServiceChain`] per service.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::chunk::ChunkRef;

/// Schema version written into newly created [`Manifest`]s.
pub const MANIFEST_VERSION: u32 = 1;

/// One service's append-only hash chain: its head, next sequence, and chunks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestServiceChain {
    /// Service this chain belongs to.
    pub service: String,
    /// SHA-256 of the chain's head (most recent) chunk; `[0u8; 32]` until the
    /// first chunk is written.
    pub head_hash: [u8; 32],
    /// Sequence number to assign to the next chunk appended to this chain.
    pub next_sequence: u64,
    /// All chunks in this service's chain, in write order.
    pub chunks: Vec<ChunkRef>,
}

impl ManifestServiceChain {
    /// A fresh, empty chain for `service` (genesis head, sequence 0).
    pub fn new(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            head_hash: [0u8; 32],
            next_sequence: 0,
            chunks: Vec::new(),
        }
    }
}

/// The root manifest: per-service chains plus the bucket and schema version.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    /// Storage bucket / namespace these chains live under.
    pub bucket: String,
    /// Per-service chains, keyed by service name.
    pub services: BTreeMap<String, ManifestServiceChain>,
    /// Manifest schema version (see [`MANIFEST_VERSION`]).
    pub version: u32,
}

impl Manifest {
    /// A new, empty manifest for `bucket` at the current schema version.
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            services: BTreeMap::new(),
            version: MANIFEST_VERSION,
        }
    }
}
