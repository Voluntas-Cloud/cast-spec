//! `debug_endpoint` — introspection endpoint for operators.

/// Sentinel for `debug_endpoint`.
pub struct DebugEndpoint;

cast::concept! {
    name: "debug_endpoint",
    summary: "Introspection endpoint for operators. Exposes runtime \
              state — config, queue depth, in-flight requests — \
              without a redeploy or attaching a debugger.",
    anchors: [cast_stdlib::observability::debug_endpoint::DebugEndpoint],
    tags: ["cast_stdlib", "observability"],
}
