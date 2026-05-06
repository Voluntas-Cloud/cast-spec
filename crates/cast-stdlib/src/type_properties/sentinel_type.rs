//! `sentinel_type` — zero-sized marker / tag with no runtime data.

/// Sentinel for `sentinel_type`.
pub struct SentinelType;

cast::concept! {
    name: "sentinel_type",
    summary: "Zero-sized type carrying no runtime data — its name IS \
              its meaning. Used as a marker the analyzer or type \
              system anchors at. The cast_stdlib sentinels themselves \
              are the canonical example.",
    anchors: [cast_stdlib::type_properties::sentinel_type::SentinelType],
    tags: ["cast_stdlib", "type_properties"],
}
