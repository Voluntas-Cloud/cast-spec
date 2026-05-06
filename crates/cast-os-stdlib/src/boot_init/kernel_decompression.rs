//! `kernel_decompression` — unpack compressed kernel image.

/// Sentinel for `kernel_decompression`.
pub struct KernelDecompression;

cast::concept! {
    name: "kernel_decompression",
    summary: "unpack compressed kernel image.",
    anchors: [cast_os_stdlib::boot_init::kernel_decompression::KernelDecompression],
    tags: ["cast_os_stdlib", "boot_init"],
}
