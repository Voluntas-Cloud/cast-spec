//! `input_method_framework` — text/input composition service.

/// Sentinel for `input_method_framework`.
pub struct InputMethodFramework;

cast::concept! {
    name: "input_method_framework",
    summary: "text/input composition service.",
    anchors: [cast_os_stdlib::desktop_session::input_method_framework::InputMethodFramework],
    tags: ["cast_os_stdlib", "desktop_session"],
}
