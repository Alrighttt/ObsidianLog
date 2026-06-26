//! ObsidianLog core: the shared vocabulary every other crate is built on.
//!
//! This crate owns the three things that must be agreed on across the workspace
//! and must stay free of I/O so they can be depended on from anywhere:
//!
//! - [`error`] — the canonical [`Error`]/[`Result`] used throughout the pipeline.
//! - [`types`] — small, dependency-free domain types ([`types::ChunkId`],
//!   [`types::ChunkHash`], [`types::Key`], …) that travel across crate
//!   boundaries.
//! - [`backend`] — the [`backend::StorageBackend`] trait. Crucially, the trait
//!   lives here, **apart from any implementation**, so the pure pipeline crate
//!   (`obsidianlog-store`) never transitively depends on the pre-1.0 Sia SDK.
//!   That SDK is confined to one feature-gated backend impl (see the workspace
//!   `CLAUDE.md` and ADR-0004).
//!
//! # Status
//!
//! Scaffold. The surface is final; behavioral methods are `todo!()` with a
//! `TODO(impl)` note describing intended behavior.

pub mod backend;
pub mod error;
pub mod types;

pub use error::{Error, Result};
