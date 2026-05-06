//! `fiber_execution_model` — cooperative lightweight execution context.

/// Sentinel for `fiber_execution_model`.
pub struct FiberExecutionModel;

cast::concept! {
    name: "fiber_execution_model",
    summary: "cooperative lightweight execution context.",
    anchors: [cast_os_stdlib::execution_model::fiber_execution_model::FiberExecutionModel],
    tags: ["cast_os_stdlib", "execution_model"],
}
