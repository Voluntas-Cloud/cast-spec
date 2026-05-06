//! `tool_call_contract` — typed contract for AI tool use.

/// Sentinel for `tool_call_contract`.
pub struct ToolCallContract;

cast::concept! {
    name: "tool_call_contract",
    summary: "Typed contract describing what a tool takes and \
              returns. The model gets a schema, the runtime gets a \
              validator; \"the LLM passed in something the tool \
              didn't expect\" should be a parser error, not a \
              production incident.",
    anchors: [cast_stdlib::ai::tool_call_contract::ToolCallContract],
    tags: ["cast_stdlib", "ai"],
}
