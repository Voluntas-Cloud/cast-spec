//! `crash_only_service_design` — recover by restart.

/// Sentinel for `crash_only_service_design`.
pub struct CrashOnlyServiceDesign;

cast::concept! {
    name: "crash_only_service_design",
    summary: "recover by restart.",
    anchors: [cast_os_stdlib::fault_recovery::crash_only_service_design::CrashOnlyServiceDesign],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
