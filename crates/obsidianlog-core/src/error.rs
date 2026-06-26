//! The canonical error shared across the workspace.
//!
//! One error type is used by the storage pipeline, the backends, and (wrapped)
//! the ingest service, so failures compose cleanly without per-crate conversion
//! boilerplate. `obsidianlog-store` re-exports these as `obsidianlog_store::{Error, Result}`.

use thiserror::Error;

/// Convenience alias for results returned across ObsidianLog.
pub type Result<T> = std::result::Result<T, Error>;

/// Failures the storage layer and backends can surface to their callers.
#[derive(Debug, Error)]
pub enum Error {
    /// Compression or decompression failed.
    #[error("compression error: {0}")]
    Compression(String),

    /// Encryption or authenticated decryption failed (e.g. GCM tag mismatch).
    #[error("encryption error: {0}")]
    Encryption(String),

    /// The hash chain is broken: a chunk was altered, reordered, or deleted.
    #[error("hash-chain integrity violation: {0}")]
    Integrity(String),

    /// The metadata index could not be read, written, or parsed.
    #[error("index error: {0}")]
    Index(String),

    /// A [`crate::backend::StorageBackend`] operation failed (local or Sia).
    #[error("backend error: {0}")]
    Backend(String),

    /// An underlying I/O operation failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// (De)serialization of manifest/index JSON failed.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
