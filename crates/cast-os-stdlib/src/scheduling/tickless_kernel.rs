//! `tickless_kernel` — reduce periodic timer interrupts.

/// Sentinel for `tickless_kernel`.
pub struct TicklessKernel;

cast::concept! {
    name: "tickless_kernel",
    summary: "reduce periodic timer interrupts.",
    anchors: [cast_os_stdlib::scheduling::tickless_kernel::TicklessKernel],
    tags: ["cast_os_stdlib", "scheduling"],
}
