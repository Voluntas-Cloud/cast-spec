//! `interrupt_driven_io` — device interrupts CPU on event.

/// Sentinel for `interrupt_driven_io`.
pub struct InterruptDrivenIo;

cast::concept! {
    name: "interrupt_driven_io",
    summary: "device interrupts CPU on event.",
    anchors: [cast_os_stdlib::io_architecture::interrupt_driven_io::InterruptDrivenIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
