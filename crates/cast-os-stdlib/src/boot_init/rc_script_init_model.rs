//! `rc_script_init_model` — script-based Unix startup.

/// Sentinel for `rc_script_init_model`.
pub struct RcScriptInitModel;

cast::concept! {
    name: "rc_script_init_model",
    summary: "script-based Unix startup.",
    anchors: [cast_os_stdlib::boot_init::rc_script_init_model::RcScriptInitModel],
    tags: ["cast_os_stdlib", "boot_init"],
}
