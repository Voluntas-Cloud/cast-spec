//! `worst_case_execution_time` — maximum execution duration estimate.

/// Sentinel for `worst_case_execution_time`.
pub struct WorstCaseExecutionTime;

cast::concept! {
    name: "worst_case_execution_time",
    summary: "maximum execution duration estimate.",
    anchors: [cast_os_stdlib::realtime::worst_case_execution_time::WorstCaseExecutionTime],
    tags: ["cast_os_stdlib", "realtime"],
}
