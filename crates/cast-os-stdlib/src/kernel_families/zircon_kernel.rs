//! `zircon_kernel` — capability-oriented kernel used by Fuchsia.

/// Sentinel for `zircon_kernel`.
pub struct ZirconKernel;

cast::concept! {
    name: "zircon_kernel",
    summary: "capability-oriented kernel used by Fuchsia.",
    anchors: [cast_os_stdlib::kernel_families::zircon_kernel::ZirconKernel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
