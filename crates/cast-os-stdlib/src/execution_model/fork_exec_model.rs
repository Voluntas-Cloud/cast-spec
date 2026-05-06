//! `fork_exec_model` — Unix process creation and replacement model.

/// Sentinel for `fork_exec_model`.
pub struct ForkExecModel;

cast::concept! {
    name: "fork_exec_model",
    summary: "Unix process creation and replacement model.",
    anchors: [cast_os_stdlib::execution_model::fork_exec_model::ForkExecModel],
    tags: ["cast_os_stdlib", "execution_model"],
}
