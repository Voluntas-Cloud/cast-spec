//! `qnx_microkernel` — production microkernel design with message-passing services.

/// Sentinel for `qnx_microkernel`.
pub struct QnxMicrokernel;

cast::concept! {
    name: "qnx_microkernel",
    summary: "production microkernel design with message-passing \
               services.",
    anchors: [cast_os_stdlib::kernel_families::qnx_microkernel::QnxMicrokernel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
