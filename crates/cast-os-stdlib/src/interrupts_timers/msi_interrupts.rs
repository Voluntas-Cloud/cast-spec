//! `msi_interrupts` — message-signaled device interrupts.

/// Sentinel for `msi_interrupts`.
pub struct MsiInterrupts;

cast::concept! {
    name: "msi_interrupts",
    summary: "message-signaled device interrupts.",
    anchors: [cast_os_stdlib::interrupts_timers::msi_interrupts::MsiInterrupts],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
