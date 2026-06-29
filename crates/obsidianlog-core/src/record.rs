//! Parsed log records as they enter the pipeline, before chunking.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A single parsed log line: the original JSON event plus the fields extracted
/// for indexing and routing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRecord {
    /// The raw, unmodified log event as received.
    pub raw: Value,
    /// Event timestamp (UTC).
    pub timestamp: DateTime<Utc>,
    /// Originating service — the hash-chain and index partition key.
    pub service: String,
    /// Log level (e.g. `info`, `warn`, `error`), when present.
    pub level: Option<String>,
    /// Originating host, when present.
    pub host: Option<String>,
    /// Distributed-trace id, when present.
    pub trace_id: Option<String>,
}

/// A batch of [`LogRecord`]s as received from ingest, before chunking.
///
/// Serializes transparently as a bare array of records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogBatch(pub Vec<LogRecord>);
