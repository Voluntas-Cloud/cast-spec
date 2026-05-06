//! `kill_switch` — emergency disablement.

/// Sentinel for `kill_switch`.
pub struct KillSwitch;

cast::concept! {
    name: "kill_switch",
    summary: "Emergency disablement. A single, well-known knob that \
              turns a feature off immediately when it misbehaves; \
              shape it so flipping it is the easy thing to do under \
              stress.",
    anchors: [cast_stdlib::config::kill_switch::KillSwitch],
    tags: ["cast_stdlib", "config"],
}
