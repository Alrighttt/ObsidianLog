//! ObsidianLog HTTP ingest service.
//!
//! Runs a lightweight local HTTP server that receives batched JSON log events
//! from Vector's built-in HTTP sink at `POST /ingest`, then drives each batch
//! through the [`obsidianlog_store`] pipeline (parse → compress → encrypt →
//! hash-chain → chunk) and on to Sia via indexd.
//!
//! Vector owns buffering, backpressure, and retry, so this service stays small:
//! accept a batch, process it, acknowledge. A native compiled Vector sink is
//! deferred to Phase 2.
//!
//! # Status
//!
//! Scaffold. [`serve`] / [`serve_blocking`] are wired but stubbed.

pub mod error;
pub mod server;

pub use error::{Error, Result};

/// Configuration for the ingest server.
#[derive(Debug, Clone)]
pub struct IngestConfig {
    /// Address to bind the HTTP endpoint to (e.g. `127.0.0.1:7080`).
    pub bind: String,
}

/// Run the ingest server to completion on the provided async runtime.
///
/// TODO(impl): build the [`server`] router, bind `config.bind`, and serve until
/// shutdown.
pub async fn serve(config: IngestConfig) -> Result<()> {
    let _ = config;
    todo!("run the Vector-compatible HTTP ingest server")
}

/// Synchronous entry point for callers without an async runtime (e.g. the CLI).
///
/// Builds a Tokio runtime and blocks on [`serve`].
///
/// TODO(impl): construct the multi-thread runtime and `block_on(serve(config))`.
pub fn serve_blocking(config: IngestConfig) -> Result<()> {
    let _ = config;
    todo!("build a Tokio runtime and block on `serve`")
}
