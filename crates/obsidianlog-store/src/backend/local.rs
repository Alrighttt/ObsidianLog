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

use obsidianlog_core::backend::StorageBackend;
use obsidianlog_core::error::Result;

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

impl StorageBackend for LocalBackend {
    fn put(&self, path: &str, bytes: &[u8]) -> Result<()> {
        let _ = (path, bytes);
        // TODO(impl): join `path` onto `root`, create parent dirs, and write the
        // file atomically (write temp + rename). Refuse to overwrite existing
        // objects — the store is append-only.
        todo!("write an append-only object to the local filesystem")
    }

    fn get(&self, path: &str) -> Result<Vec<u8>> {
        let _ = path;
        // TODO(impl): read `<root>/<path>` into a byte buffer.
        todo!("read an object from the local filesystem")
    }

    fn list(&self, prefix: &str) -> Result<Vec<String>> {
        let _ = prefix;
        // TODO(impl): walk `<root>/<prefix>` and return bucket-relative paths.
        todo!("list objects under a prefix on the local filesystem")
    }

    fn exists(&self, path: &str) -> Result<bool> {
        let _ = path;
        // TODO(impl): check whether `<root>/<path>` exists.
        todo!("check object existence on the local filesystem")
    }
}
