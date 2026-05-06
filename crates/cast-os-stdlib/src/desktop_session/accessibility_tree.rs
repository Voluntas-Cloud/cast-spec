//! `accessibility_tree` — structured UI exposed to assistive tech.

/// Sentinel for `accessibility_tree`.
pub struct AccessibilityTree;

cast::concept! {
    name: "accessibility_tree",
    summary: "structured UI exposed to assistive tech.",
    anchors: [cast_os_stdlib::desktop_session::accessibility_tree::AccessibilityTree],
    tags: ["cast_os_stdlib", "desktop_session"],
}
