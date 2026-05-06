//! `lsm_hook_model` — Linux Security Module hook framework.

/// Sentinel for `lsm_hook_model`.
pub struct LsmHookModel;

cast::concept! {
    name: "lsm_hook_model",
    summary: "Linux Security Module hook framework.",
    anchors: [cast_os_stdlib::security::lsm_hook_model::LsmHookModel],
    tags: ["cast_os_stdlib", "security"],
}
