//! `adaptive_readahead` — adjust file read-ahead.

/// Sentinel for `adaptive_readahead`.
pub struct AdaptiveReadahead;

cast::concept! {
    name: "adaptive_readahead",
    summary: "adjust file read-ahead.",
    anchors: [cast_os_stdlib::self_adaptive_os::adaptive_readahead::AdaptiveReadahead],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
