//! zstd compression of log batches.
//!
//! Log text is highly compressible; the architecture targets 90–97% reduction.
//! Backed by the `zstd` crate.

use crate::error::Result;

/// Default zstd compression level (balanced ratio/speed for log text).
pub const DEFAULT_LEVEL: i32 = 19;

/// Compress a raw log batch with zstd.
///
/// TODO(impl): zstd-compress `input` at `level`.
pub fn compress(input: &[u8], level: i32) -> Result<Vec<u8>> {
    let _ = (input, level);
    todo!("zstd-compress the log batch")
}

/// Decompress a zstd-compressed batch.
///
/// TODO(impl): zstd-decompress `input`.
pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    let _ = input;
    todo!("zstd-decompress the log batch")
}
