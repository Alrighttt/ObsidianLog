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

use obsidianlog_core::backend::StorageBackend;
use obsidianlog_core::error::Result;

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

impl StorageBackend for SiaBackend {
    fn put(&self, path: &str, bytes: &[u8]) -> Result<()> {
        let _ = (path, bytes);
        todo!("write an append-only object to Sia via indexd")
    }

    fn get(&self, path: &str) -> Result<Vec<u8>> {
        let _ = path;
        todo!("read an object from Sia via indexd")
    }

    fn list(&self, prefix: &str) -> Result<Vec<String>> {
        let _ = prefix;
        todo!("list objects under a prefix via indexd")
    }

    fn exists(&self, path: &str) -> Result<bool> {
        let _ = path;
        todo!("check object existence via indexd")
    }
}
