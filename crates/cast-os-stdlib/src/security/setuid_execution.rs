//! `setuid_execution` — execute with file owner privileges. Historic foot-gun, still somehow invited.

/// Sentinel for `setuid_execution`.
pub struct SetuidExecution;

cast::concept! {
    name: "setuid_execution",
    summary: "execute with file owner privileges. Historic foot-gun, \
               still somehow invited.",
    anchors: [cast_os_stdlib::security::setuid_execution::SetuidExecution],
    tags: ["cast_os_stdlib", "security"],
}
