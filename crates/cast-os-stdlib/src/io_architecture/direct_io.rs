//! `direct_io` — bypass page cache.

/// Sentinel for `direct_io`.
pub struct DirectIo;

cast::concept! {
    name: "direct_io",
    summary: "bypass page cache.",
    anchors: [cast_os_stdlib::io_architecture::direct_io::DirectIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
