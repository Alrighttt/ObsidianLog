//! Lightweight metadata index.
//!
//! Structured fields (timestamp, service, level, host, trace_id) are extracted
//! from each batch into a compact index stored separately from the log bodies
//! (under 1% of full log size). Queries hit this index first; full chunks are
//! fetched only when a record actually matches.

use serde::{Deserialize, Serialize};

use crate::chunk::ChunkId;
use crate::error::Result;

/// A single metadata record pointing at where a log entry lives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    /// Event timestamp (epoch milliseconds).
    pub timestamp_ms: i64,
    /// Originating service.
    pub service: String,
    /// Log level (e.g. info, warn, error).
    pub level: String,
    /// Originating host.
    pub host: String,
    /// Distributed-trace id, when present.
    pub trace_id: Option<String>,
    /// Chunk this entry's full body lives in.
    pub chunk: ChunkId,
}

/// Filter predicates applied against the metadata index.
#[derive(Debug, Clone, Default)]
pub struct IndexQuery {
    /// Inclusive lower time bound (epoch milliseconds).
    pub since_ms: Option<i64>,
    /// Exclusive upper time bound (epoch milliseconds).
    pub until_ms: Option<i64>,
    /// Restrict to a service.
    pub service: Option<String>,
    /// Restrict to a log level.
    pub level: Option<String>,
    /// Restrict to a host.
    pub host: Option<String>,
}

/// Resolve the set of chunks that could contain matches for a filter.
///
/// TODO(impl): scan the relevant `.idx` files and return the deduplicated set of
/// chunks to fetch.
pub fn lookup(entries: &[IndexEntry], query: &IndexQuery) -> Result<Vec<ChunkId>> {
    let _ = (entries, query);
    todo!("resolve matching chunks from the index")
}
