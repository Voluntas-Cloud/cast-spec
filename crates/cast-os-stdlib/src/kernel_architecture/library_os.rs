//! `library_os` — OS services provided as libraries inside application context.

/// Sentinel for `library_os`.
pub struct LibraryOs;

cast::concept! {
    name: "library_os",
    summary: "OS services provided as libraries inside application \
               context.",
    anchors: [cast_os_stdlib::kernel_architecture::library_os::LibraryOs],
    tags: ["cast_os_stdlib", "kernel_architecture"],
}
