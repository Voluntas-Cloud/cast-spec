//! `plan_execute_observe_loop` — agent plans, acts, observes, revises.

/// Sentinel for `plan_execute_observe_loop`.
pub struct PlanExecuteObserveLoop;

cast::concept! {
    name: "plan_execute_observe_loop",
    summary: "The agent plans, executes a step, observes the result, \
              and revises the plan. Each phase is its own checkpoint, \
              which is how you debug an agent that has wandered \
              instead of reading a wall of tokens.",
    anchors: [cast_stdlib::ai::plan_execute_observe_loop::PlanExecuteObserveLoop],
    tags: ["cast_stdlib", "ai"],
}
