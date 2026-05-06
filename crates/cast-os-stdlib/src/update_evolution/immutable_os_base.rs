//! `immutable_os_base` — base OS not mutated directly.

/// Sentinel for `immutable_os_base`.
pub struct ImmutableOsBase;

cast::concept! {
    name: "immutable_os_base",
    summary: "base OS not mutated directly.",
    anchors: [cast_os_stdlib::update_evolution::immutable_os_base::ImmutableOsBase],
    tags: ["cast_os_stdlib", "update_evolution"],
}
