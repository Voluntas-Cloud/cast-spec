//! `transactional_os_state_model` — OS changes applied atomically.

/// Sentinel for `transactional_os_state_model`.
pub struct TransactionalOsStateModel;

cast::concept! {
    name: "transactional_os_state_model",
    summary: "OS changes applied atomically.",
    anchors: [cast_os_stdlib::architectural_patterns::transactional_os_state_model::TransactionalOsStateModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
