//! Filesystem-backed [`StorageBackend`] — the default, Sia-free backend.
//!
//! `LocalBackend` stores objects as files under a root directory, mirroring the
//! on-Sia layout (`<root>/chunks/...`, `<root>/index/...`, `<root>/manifest/...`).
//! It exists so the entire pipeline — compression, encryption, hash chaining,
//! ingest, query, verify — can be developed and tested with **no Sia node**.
//! This is the mock-first invariant (see CLAUDE.md / ADR-0004); implementing
//! this module is the natural first step that makes the `#[ignore]`d pipeline
//! tests runnable.

use std::path::PathBuf;

use async_trait::async_trait;
use obsidianlog_core::backend::{StorageBackend, TimeRange};
use obsidianlog_core::chunk::{Chunk, ChunkRef};
use obsidianlog_core::error::Result;
use obsidianlog_core::index::ServiceWindowIndex;
use obsidianlog_core::manifest::Manifest;

/// A [`StorageBackend`] backed by a local directory tree.
#[derive(Debug, Clone)]
pub struct LocalBackend {
    root: PathBuf,
}

impl LocalBackend {
    /// Create a backend rooted at `root` (objects are stored beneath it).
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The directory this backend stores objects under.
    pub fn root(&self) -> &std::path::Path {
        &self.root
    }
}

#[async_trait]
impl StorageBackend for LocalBackend {
    async fn put_chunk(&self, chunk: &Chunk) -> Result<()> {
        let _ = chunk;
        // TODO(impl): serialize the chunk to `<root>/chunks/<service>/<window>.bin`,
        // writing to a temp file + rename, and fsync before returning. Refuse to
        // overwrite an existing chunk — the store is append-only.
        todo!("durably write a chunk to the local filesystem")
    }

    async fn get_chunk(&self, service: &str, window: &str) -> Result<Chunk> {
        let _ = (service, window);
        // TODO(impl): read and deserialize `<root>/chunks/<service>/<window>.bin`.
        todo!("read a chunk from the local filesystem")
    }

    async fn put_index(&self, index: &ServiceWindowIndex) -> Result<()> {
        let _ = index;
        // TODO(impl): durably write to `<root>/index/<service>/<window>.idx`.
        todo!("durably write a service-window index to the local filesystem")
    }

    async fn get_index(&self, service: &str, window: &str) -> Result<ServiceWindowIndex> {
        let _ = (service, window);
        // TODO(impl): read and deserialize `<root>/index/<service>/<window>.idx`.
        todo!("read a service-window index from the local filesystem")
    }

    async fn list_chunks(&self, service: &str, range: Option<TimeRange>) -> Result<Vec<ChunkRef>> {
        let _ = (service, range);
        // TODO(impl): enumerate the service's chunks, filtered by the optional range.
        todo!("list chunk references on the local filesystem")
    }

    async fn read_manifest(&self) -> Result<Manifest> {
        // TODO(impl): read and deserialize `<root>/manifest/manifest.json`.
        todo!("read the manifest from the local filesystem")
    }

    async fn write_manifest(&self, manifest: &Manifest) -> Result<()> {
        let _ = manifest;
        // TODO(impl): atomically write the manifest (temp + rename) and fsync.
        todo!("durably write the manifest to the local filesystem")
    }
}
