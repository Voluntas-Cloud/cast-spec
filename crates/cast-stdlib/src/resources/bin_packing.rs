//! `bin_packing` — place work to use capacity efficiently.

/// Sentinel for `bin_packing`.
pub struct BinPacking;

cast::concept! {
    name: "bin_packing",
    summary: "Place work to use capacity efficiently. The scheduler \
              tries to fill nodes tightly so unused machines can shut \
              down; tighter packing trades headroom against a quicker \
              path to eviction.",
    anchors: [cast_stdlib::resources::bin_packing::BinPacking],
    tags: ["cast_stdlib", "resources"],
}
