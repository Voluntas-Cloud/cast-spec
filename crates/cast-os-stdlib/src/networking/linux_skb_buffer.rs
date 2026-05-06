//! `linux_skb_buffer` — Linux socket buffer concept.

/// Sentinel for `linux_skb_buffer`.
pub struct LinuxSkbBuffer;

cast::concept! {
    name: "linux_skb_buffer",
    summary: "Linux socket buffer concept.",
    anchors: [cast_os_stdlib::networking::linux_skb_buffer::LinuxSkbBuffer],
    tags: ["cast_os_stdlib", "networking"],
}
