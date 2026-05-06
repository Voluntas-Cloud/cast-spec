//! `discretionary_access_control` — resource owner controls permissions.

/// Sentinel for `discretionary_access_control`.
pub struct DiscretionaryAccessControl;

cast::concept! {
    name: "discretionary_access_control",
    summary: "resource owner controls permissions.",
    anchors: [cast_os_stdlib::security::discretionary_access_control::DiscretionaryAccessControl],
    tags: ["cast_os_stdlib", "security"],
}
