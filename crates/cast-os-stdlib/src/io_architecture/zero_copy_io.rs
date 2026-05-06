//! `zero_copy_io` — avoid copying between kernel/user buffers.

/// Sentinel for `zero_copy_io`.
pub struct ZeroCopyIo;

cast::concept! {
    name: "zero_copy_io",
    summary: "avoid copying between kernel/user buffers.",
    anchors: [cast_os_stdlib::io_architecture::zero_copy_io::ZeroCopyIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
