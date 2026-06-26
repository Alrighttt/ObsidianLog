//! HTTP routing for the ingest endpoint.
//!
//! Vector's HTTP sink POSTs newline/JSON-encoded log batches to `/ingest`. This
//! module builds the `axum` router and the batch handler.

use crate::error::Result;

/// The path Vector's HTTP sink posts batches to (`uri = ".../ingest"`).
pub const INGEST_PATH: &str = "/ingest";

/// Build the ingest HTTP router.
///
/// TODO(impl): return an `axum::Router` with `POST /ingest` wired to the batch
/// handler and `GET /health` for readiness checks. The concrete return type is
/// left to implementation to avoid pinning the axum surface in the scaffold.
pub fn build_router() -> Result<()> {
    todo!("build the axum router for /ingest and /health")
}

/// Handle one log batch: parse, then archive via the storage pipeline.
///
/// TODO(impl): deserialize the batch, extract metadata, and push through
/// `obsidianlog_store` (compress → encrypt → chain → chunk → backend).
pub fn handle_batch(body: &[u8]) -> Result<()> {
    let _ = body;
    todo!("process a Vector log batch through the storage pipeline")
}
