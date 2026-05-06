//! `jail_isolation_model` — BSD-style filesystem/process isolation.

/// Sentinel for `jail_isolation_model`.
pub struct JailIsolationModel;

cast::concept! {
    name: "jail_isolation_model",
    summary: "BSD-style filesystem/process isolation.",
    anchors: [cast_os_stdlib::isolation::jail_isolation_model::JailIsolationModel],
    tags: ["cast_os_stdlib", "isolation"],
}
