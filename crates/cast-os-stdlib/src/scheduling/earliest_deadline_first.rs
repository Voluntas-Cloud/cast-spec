//! `earliest_deadline_first` — choose task with nearest deadline.

/// Sentinel for `earliest_deadline_first`.
pub struct EarliestDeadlineFirst;

cast::concept! {
    name: "earliest_deadline_first",
    summary: "choose task with nearest deadline.",
    anchors: [cast_os_stdlib::scheduling::earliest_deadline_first::EarliestDeadlineFirst],
    tags: ["cast_os_stdlib", "scheduling"],
}
