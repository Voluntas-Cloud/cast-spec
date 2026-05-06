//! `explainable_status` — system state understandable to user.

/// Sentinel for `explainable_status`.
pub struct ExplainableStatus;

cast::concept! {
    name: "explainable_status",
    summary: "The user should be able to read the system state and \
              know what is happening. \"Loading\" forever, \"Failed\" \
              with no reason, and a quietly empty screen all teach \
              users that the UI is theatre.",
    anchors: [cast_stdlib::ux::explainable_status::ExplainableStatus],
    tags: ["cast_stdlib", "ux"],
}
