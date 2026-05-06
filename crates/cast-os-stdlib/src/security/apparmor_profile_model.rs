//! `apparmor_profile_model` — path/profile-based confinement.

/// Sentinel for `apparmor_profile_model`.
pub struct ApparmorProfileModel;

cast::concept! {
    name: "apparmor_profile_model",
    summary: "path/profile-based confinement.",
    anchors: [cast_os_stdlib::security::apparmor_profile_model::ApparmorProfileModel],
    tags: ["cast_os_stdlib", "security"],
}
