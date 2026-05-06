//! `windows_net_buffer_list` — Windows network buffer concept.

/// Sentinel for `windows_net_buffer_list`.
pub struct WindowsNetBufferList;

cast::concept! {
    name: "windows_net_buffer_list",
    summary: "Windows network buffer concept.",
    anchors: [cast_os_stdlib::networking::windows_net_buffer_list::WindowsNetBufferList],
    tags: ["cast_os_stdlib", "networking"],
}
