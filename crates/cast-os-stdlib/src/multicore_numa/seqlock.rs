//! `seqlock` — versioned lock optimized for readers.

/// Sentinel for `seqlock`.
pub struct Seqlock;

cast::concept! {
    name: "seqlock",
    summary: "versioned lock optimized for readers.",
    anchors: [cast_os_stdlib::multicore_numa::seqlock::Seqlock],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
