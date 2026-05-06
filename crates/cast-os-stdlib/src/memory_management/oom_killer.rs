//! `oom_killer` — terminate processes under unrecoverable memory pressure.

/// Sentinel for `oom_killer`.
pub struct OomKiller;

cast::concept! {
    name: "oom_killer",
    summary: "terminate processes under unrecoverable memory pressure.",
    anchors: [cast_os_stdlib::memory_management::oom_killer::OomKiller],
    tags: ["cast_os_stdlib", "memory_management"],
}
