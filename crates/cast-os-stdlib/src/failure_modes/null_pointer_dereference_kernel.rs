//! `null_pointer_dereference_kernel` — invalid pointer fault in kernel.

/// Sentinel for `null_pointer_dereference_kernel`.
pub struct NullPointerDereferenceKernel;

cast::concept! {
    name: "null_pointer_dereference_kernel",
    summary: "invalid pointer fault in kernel.",
    anchors: [cast_os_stdlib::failure_modes::null_pointer_dereference_kernel::NullPointerDereferenceKernel],
    tags: ["cast_os_stdlib", "failure_modes"],
}
