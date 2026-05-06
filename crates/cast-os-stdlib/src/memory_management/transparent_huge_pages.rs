//! `transparent_huge_pages` — automatic huge page use.

/// Sentinel for `transparent_huge_pages`.
pub struct TransparentHugePages;

cast::concept! {
    name: "transparent_huge_pages",
    summary: "automatic huge page use.",
    anchors: [cast_os_stdlib::memory_management::transparent_huge_pages::TransparentHugePages],
    tags: ["cast_os_stdlib", "memory_management"],
}
