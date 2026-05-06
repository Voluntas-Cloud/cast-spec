//! `pod_sandbox_model` — group of containers sharing namespaces.

/// Sentinel for `pod_sandbox_model`.
pub struct PodSandboxModel;

cast::concept! {
    name: "pod_sandbox_model",
    summary: "group of containers sharing namespaces.",
    anchors: [cast_os_stdlib::isolation::pod_sandbox_model::PodSandboxModel],
    tags: ["cast_os_stdlib", "isolation"],
}
