//! `name_cache_lookup` — accelerate path lookup.

/// Sentinel for `name_cache_lookup`.
pub struct NameCacheLookup;

cast::concept! {
    name: "name_cache_lookup",
    summary: "accelerate path lookup.",
    anchors: [cast_os_stdlib::os_algorithms::name_cache_lookup::NameCacheLookup],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
