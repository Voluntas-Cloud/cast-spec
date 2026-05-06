//! `service_process_model` — managed OS service execution model.

/// Sentinel for `service_process_model`.
pub struct ServiceProcessModel;

cast::concept! {
    name: "service_process_model",
    summary: "managed OS service execution model.",
    anchors: [cast_os_stdlib::execution_model::service_process_model::ServiceProcessModel],
    tags: ["cast_os_stdlib", "execution_model"],
}
