//! `model_routing` — select model based on task.

/// Sentinel for `model_routing`.
pub struct ModelRouting;

cast::concept! {
    name: "model_routing",
    summary: "Route a request to the model that fits the task: cheap \
              and fast for triage, large and slow for hard reasoning. \
              The router is itself a piece of the system, with its \
              own evals, not a config knob set once and forgotten.",
    anchors: [cast_stdlib::ai::model_routing::ModelRouting],
    tags: ["cast_stdlib", "ai"],
}
