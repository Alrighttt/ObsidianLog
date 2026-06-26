//! The storage backend abstraction.
//!
//! The whole pipeline talks to durable storage only through [`StorageBackend`].
//! Keeping the trait here — in the pure `core` crate, away from every
//! implementation — is what lets the default build and test suite run with **no
//! Sia node**: `obsidianlog-store` ships an in-process `LocalBackend` by default
//! and confines the real Sia integration to a `sia`-feature-gated impl, so the
//! pre-1.0 Sia SDK never leaks into the pure pipeline (see ADR-0004).
//!
//! Backends are append-only: [`StorageBackend::put`] writes objects that are
//! never modified or deleted post-write.

use crate::error::Result;

/// An append-only object store keyed by bucket-relative path.
///
/// Implemented by `obsidianlog_store::backend::LocalBackend` (default) and, with
/// the `sia` feature, `obsidianlog_store::backend::SiaBackend`. Paths follow the
/// fixed layout (`chunks/...`, `index/...`, `manifest/...`).
pub trait StorageBackend {
    /// Write `bytes` at `path` (append-only; must not overwrite existing data).
    fn put(&self, path: &str, bytes: &[u8]) -> Result<()>;

    /// Read the object stored at `path`.
    fn get(&self, path: &str) -> Result<Vec<u8>>;

    /// List object paths under `prefix` (e.g. all chunks for a service).
    fn list(&self, prefix: &str) -> Result<Vec<String>>;

    /// Report whether an object exists at `path`.
    fn exists(&self, path: &str) -> Result<bool>;
}
