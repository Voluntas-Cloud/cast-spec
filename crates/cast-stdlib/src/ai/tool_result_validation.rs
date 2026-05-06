//! `tool_result_validation` — verify tool output before relying on it.

/// Sentinel for `tool_result_validation`.
pub struct ToolResultValidation;

cast::concept! {
    name: "tool_result_validation",
    summary: "Verify a tool's output before letting the model build \
              on it. Tools fail, return partial data, or return data \
              the schema didn't promise; trusting them blindly turns \
              one bug into many.",
    anchors: [cast_stdlib::ai::tool_result_validation::ToolResultValidation],
    tags: ["cast_stdlib", "ai"],
}
