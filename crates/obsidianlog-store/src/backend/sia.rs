//! Sia-backed [`StorageBackend`] — the real network integration.
//!
//! Compiled only under the `sia` feature. This is the single place the pre-1.0
//! Sia SDK (`sia_storage`) is allowed to appear, so default builds and tests
//! stay Sia-free (mock-first invariant; see CLAUDE.md / ADR-0004). When wiring
//! the SDK, add it as an optional dependency enabled by the `sia` feature and
//! configure its HTTP client with **rustls** (never native-tls) to avoid
//! OpenSSL linking.
//!
//! Storage and retrieval are coordinated on the Sia network through the user's
//! `indexd` deployment; this backend abstracts away contract formation.

use async_trait::async_trait;
use obsidianlog_core::backend::{StorageBackend, TimeRange};
use obsidianlog_core::chunk::{Chunk, ChunkRef};
use obsidianlog_core::error::Result;
use obsidianlog_core::index::ServiceWindowIndex;
use obsidianlog_core::manifest::Manifest;

/// Connection settings for a Sia `indexd` deployment.
#[derive(Debug, Clone)]
pub struct SiaConfig {
    /// Base URL of the indexd HTTP API.
    pub url: String,
    /// Bucket / namespace logs are archived under.
    pub bucket: String,
}

/// Sia-backed implementation of [`StorageBackend`] (via the user's `indexd`).
#[derive(Debug, Clone)]
pub struct SiaBackend {
    config: SiaConfig,
}

impl SiaBackend {
    /// Construct a backend pointed at the user's indexd deployment.
    pub fn new(config: SiaConfig) -> Self {
        Self { config }
    }

    /// Borrow the backend's configuration.
    pub fn config(&self) -> &SiaConfig {
        &self.config
    }
}

#[async_trait]
impl StorageBackend for SiaBackend {
    async fn put_chunk(&self, chunk: &Chunk) -> Result<()> {
        let _ = chunk;
        todo!("durably write a chunk to Sia via indexd")
    }

    async fn get_chunk(&self, service: &str, window: &str) -> Result<Chunk> {
        let _ = (service, window);
        todo!("read a chunk from Sia via indexd")
    }

    async fn put_index(&self, index: &ServiceWindowIndex) -> Result<()> {
        let _ = index;
        todo!("durably write a service-window index to Sia via indexd")
    }

    async fn get_index(&self, service: &str, window: &str) -> Result<ServiceWindowIndex> {
        let _ = (service, window);
        todo!("read a service-window index from Sia via indexd")
    }

    async fn list_chunks(&self, service: &str, range: Option<TimeRange>) -> Result<Vec<ChunkRef>> {
        let _ = (service, range);
        todo!("list chunk references via indexd")
    }

    async fn read_manifest(&self) -> Result<Manifest> {
        todo!("read the manifest from Sia via indexd")
    }

    async fn write_manifest(&self, manifest: &Manifest) -> Result<()> {
        let _ = manifest;
        todo!("durably write the manifest to Sia via indexd")
    }
}
