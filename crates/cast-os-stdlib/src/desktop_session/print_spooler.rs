//! `print_spooler` — queued printing service.

/// Sentinel for `print_spooler`.
pub struct PrintSpooler;

cast::concept! {
    name: "print_spooler",
    summary: "queued printing service.",
    anchors: [cast_os_stdlib::desktop_session::print_spooler::PrintSpooler],
    tags: ["cast_os_stdlib", "desktop_session"],
}
