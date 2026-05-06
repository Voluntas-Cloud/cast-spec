//! `fan_out_fan_in` — parallel work then aggregate.

/// Sentinel for `fan_out_fan_in`.
pub struct FanOutFanIn;

cast::concept! {
    name: "fan_out_fan_in",
    summary: "Parallel work then aggregate. One step splits into N \
              independent tasks; a join step waits for all of them and \
              produces a single combined result.",
    anchors: [cast_stdlib::workflow::fan_out_fan_in::FanOutFanIn],
    tags: ["cast_stdlib", "workflow"],
}
