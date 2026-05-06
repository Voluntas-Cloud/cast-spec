//! `bugcheck_stop_error` — Windows fatal kernel stop.

/// Sentinel for `bugcheck_stop_error`.
pub struct BugcheckStopError;

cast::concept! {
    name: "bugcheck_stop_error",
    summary: "Windows fatal kernel stop.",
    anchors: [cast_os_stdlib::fault_recovery::bugcheck_stop_error::BugcheckStopError],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
