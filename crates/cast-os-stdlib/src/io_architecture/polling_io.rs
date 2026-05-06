//! `polling_io` — CPU repeatedly checks device status.

/// Sentinel for `polling_io`.
pub struct PollingIo;

cast::concept! {
    name: "polling_io",
    summary: "CPU repeatedly checks device status.",
    anchors: [cast_os_stdlib::io_architecture::polling_io::PollingIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
