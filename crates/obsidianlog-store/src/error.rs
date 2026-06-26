//! Re-export of the canonical workspace error.
//!
//! The error type itself lives in `obsidianlog-core` so the backends, the
//! pipeline, and the ingest service all share one error without conversion
//! boilerplate. It is re-exported here as `obsidianlog_store::{Error, Result}`
//! (and via `crate::error::*`) so existing call sites are unaffected.

pub use obsidianlog_core::error::{Error, Result};
