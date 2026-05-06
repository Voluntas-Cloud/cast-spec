//! `side_effect_tee` — passthrough that copies traffic to a side channel without coupling delivery.

/// Sentinel for `side_effect_tee`.
pub struct SideEffectTee;

cast::concept! {
    name: "side_effect_tee",
    summary: "On every passthrough, attempt to write a copy to a side \
              channel (log file, audit stream, observation socket) — \
              but the primary delivery never blocks on the side write \
              and a side-write failure is logged-and-swallowed, never \
              propagated as an error. The point is observability \
              without coupling: the side channel makes the traffic \
              inspectable, but it can disappear without breaking the \
              real flow.",
    anchors: [cast_stdlib::messaging::side_effect_tee::SideEffectTee],
    tags: ["cast_stdlib", "messaging"],
}
