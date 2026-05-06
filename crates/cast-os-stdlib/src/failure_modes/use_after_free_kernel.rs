//! `use_after_free_kernel` — freed memory used again.

/// Sentinel for `use_after_free_kernel`.
pub struct UseAfterFreeKernel;

cast::concept! {
    name: "use_after_free_kernel",
    summary: "freed memory used again.",
    anchors: [cast_os_stdlib::failure_modes::use_after_free_kernel::UseAfterFreeKernel],
    tags: ["cast_os_stdlib", "failure_modes"],
}
