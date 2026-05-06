//! `eventual_projection` — derived view catches up asynchronously.

/// Sentinel for `eventual_projection`.
pub struct EventualProjection;

cast::concept! {
    name: "eventual_projection",
    summary: "A derived view that catches up to the source \
              asynchronously. Read-after-write surprises are the \
              cost; design the UX to expect them rather than fight \
              them, or users will discover the gap on their own.",
    anchors: [cast_stdlib::state_data::eventual_projection::EventualProjection],
    tags: ["cast_stdlib", "state_data"],
}
