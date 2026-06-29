//! The canonical error shared across the workspace.
//!
//! One error type is used by the data model, the storage pipeline, and the
//! backends, so failures compose cleanly without per-crate conversion
//! boilerplate. `obsidianlog-store` re-exports these as
//! `obsidianlog_store::{Error, Result}`.

use thiserror::Error;

/// Convenience alias for results returned across ObsidianLog.
pub type Result<T> = std::result::Result<T, Error>;

/// Failures the data model, pipeline, and backends can surface to callers.
#[derive(Debug, Error)]
pub enum Error {
    /// Serialization or deserialization failed (serde, JSON, or the canonical
    /// chunk encoding).
    #[error("serialization error: {0}")]
    Serialization(String),

    /// A cryptographic operation failed — encryption, decryption, or GCM tag
    /// authentication (a tampered or corrupt ciphertext).
    #[error("crypto error: {0}")]
    Crypto(String),

    /// The hash chain is broken: a chunk was altered, reordered, or deleted.
    #[error("chain integrity violation: {0}")]
    ChainIntegrity(String),

    /// A [`crate::backend::StorageBackend`] operation failed (local or Sia).
    #[error("backend error: {0}")]
    Backend(String),

    /// A requested object (chunk, index, or manifest) does not exist.
    #[error("not found: {0}")]
    NotFound(String),

    /// An underlying I/O operation failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}
