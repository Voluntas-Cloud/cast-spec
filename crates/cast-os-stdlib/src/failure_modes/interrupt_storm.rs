//! `interrupt_storm` — excessive interrupts degrade system.

/// Sentinel for `interrupt_storm`.
pub struct InterruptStorm;

cast::concept! {
    name: "interrupt_storm",
    summary: "excessive interrupts degrade system.",
    anchors: [cast_os_stdlib::failure_modes::interrupt_storm::InterruptStorm],
    tags: ["cast_os_stdlib", "failure_modes"],
}
