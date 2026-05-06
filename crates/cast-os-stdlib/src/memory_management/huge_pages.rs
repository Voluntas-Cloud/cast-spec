//! `huge_pages` — larger memory pages to reduce translation overhead.

/// Sentinel for `huge_pages`.
pub struct HugePages;

cast::concept! {
    name: "huge_pages",
    summary: "larger memory pages to reduce translation overhead.",
    anchors: [cast_os_stdlib::memory_management::huge_pages::HugePages],
    tags: ["cast_os_stdlib", "memory_management"],
}
