//! `windows_etw` — Windows Event Tracing framework.

/// Sentinel for `windows_etw`.
pub struct WindowsEtw;

cast::concept! {
    name: "windows_etw",
    summary: "Windows Event Tracing framework.",
    anchors: [cast_os_stdlib::observability::windows_etw::WindowsEtw],
    tags: ["cast_os_stdlib", "observability"],
}
