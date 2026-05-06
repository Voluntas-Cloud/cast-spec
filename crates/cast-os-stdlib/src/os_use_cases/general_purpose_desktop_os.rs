//! `general_purpose_desktop_os` — interactive multi-application human environment.

/// Sentinel for `general_purpose_desktop_os`.
pub struct GeneralPurposeDesktopOs;

cast::concept! {
    name: "general_purpose_desktop_os",
    summary: "interactive multi-application human environment.",
    anchors: [cast_os_stdlib::os_use_cases::general_purpose_desktop_os::GeneralPurposeDesktopOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
