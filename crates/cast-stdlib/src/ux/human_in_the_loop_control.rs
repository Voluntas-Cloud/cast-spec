//! `human_in_the_loop_control` — user approves important decisions.

/// Sentinel for `human_in_the_loop_control`.
pub struct HumanInTheLoopControl;

cast::concept! {
    name: "human_in_the_loop_control",
    summary: "Important decisions pause for a human to approve. The \
              system can prepare, propose, and roll back; the human \
              owns the moment the change goes live for the parts \
              where automation getting it wrong is unacceptable.",
    anchors: [cast_stdlib::ux::human_in_the_loop_control::HumanInTheLoopControl],
    tags: ["cast_stdlib", "ux"],
}
