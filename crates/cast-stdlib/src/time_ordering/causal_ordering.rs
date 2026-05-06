//! `causal_ordering` — preserve cause-before-effect relationship.

/// Sentinel for `causal_ordering`.
pub struct CausalOrdering;

cast::concept! {
    name: "causal_ordering",
    summary: "Preserve cause-before-effect even when events arrive \
              in different orders. \"Reply visible before original \
              post\" is the textbook violation; vector clocks and \
              causal metadata exist to prevent it.",
    anchors: [cast_stdlib::time_ordering::causal_ordering::CausalOrdering],
    tags: ["cast_stdlib", "time_ordering"],
}
