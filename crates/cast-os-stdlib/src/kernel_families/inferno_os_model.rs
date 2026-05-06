//! `inferno_os_model` — distributed OS based around virtual machine and namespace ideas.

/// Sentinel for `inferno_os_model`.
pub struct InfernoOsModel;

cast::concept! {
    name: "inferno_os_model",
    summary: "distributed OS based around virtual machine and namespace \
               ideas.",
    anchors: [cast_os_stdlib::kernel_families::inferno_os_model::InfernoOsModel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
