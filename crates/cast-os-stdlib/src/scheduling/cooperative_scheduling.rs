//! `cooperative_scheduling` — tasks yield voluntarily.

/// Sentinel for `cooperative_scheduling`.
pub struct CooperativeScheduling;

cast::concept! {
    name: "cooperative_scheduling",
    summary: "tasks yield voluntarily.",
    anchors: [cast_os_stdlib::scheduling::cooperative_scheduling::CooperativeScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
