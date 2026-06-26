//! Chunking and chunk identity.
//!
//! Logs are grouped into configurable time windows (default 1 hour) and written
//! as discrete, write-once chunk files named by window. On-Sia layout:
//! `/<bucket>/chunks/<service>/<YYYY-MM-DD-HH>.bin`. All writes are append-only;
//! chunks are never modified or deleted post-write.
//!
//! [`ChunkId`] is defined in `obsidianlog-core` and re-exported here as its
//! semantic home (`obsidianlog_store::chunk::ChunkId`).

pub use obsidianlog_core::types::ChunkId;

/// Default chunk time-window length, in seconds (1 hour).
pub const DEFAULT_WINDOW_SECS: u64 = 3600;
