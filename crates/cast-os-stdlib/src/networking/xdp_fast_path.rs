//! `xdp_fast_path` — early high-speed packet processing path.

/// Sentinel for `xdp_fast_path`.
pub struct XdpFastPath;

cast::concept! {
    name: "xdp_fast_path",
    summary: "early high-speed packet processing path.",
    anchors: [cast_os_stdlib::networking::xdp_fast_path::XdpFastPath],
    tags: ["cast_os_stdlib", "networking"],
}
