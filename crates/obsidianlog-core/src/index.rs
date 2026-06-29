//! Lightweight per-`(service, window)` metadata index.
//!
//! Structured fields are extracted from each batch into a compact summary stored
//! separately from the log bodies. Queries scan these summaries first and fetch
//! full chunks only when a window can match — the index is designed to stay
//! under ~1% of the raw log size.

use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::chunk::ChunkRef;

/// Aggregate metadata for one `(service, time_window)` — the contents of an
/// `.idx` object. Used to prefilter before fetching and decrypting a chunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceWindowIndex {
    /// Service this window belongs to.
    pub service: String,
    /// Time-window label, `YYYY-MM-DD-HH`.
    pub window: String,
    /// Earliest record timestamp in the window.
    pub min_timestamp: DateTime<Utc>,
    /// Latest record timestamp in the window.
    pub max_timestamp: DateTime<Utc>,
    /// Distinct log levels present in the window.
    pub levels: BTreeSet<String>,
    /// Distinct hosts present in the window.
    pub hosts: BTreeSet<String>,
    /// Keyword tokens for cheap prefiltering before a full chunk scan.
    pub keywords: BTreeSet<String>,
    /// The chunk whose records this index summarizes.
    pub chunk: ChunkRef,
}

/// A single index entry is one [`ServiceWindowIndex`]; a service's index is the
/// set of these per-window entries.
pub type IndexEntry = ServiceWindowIndex;
