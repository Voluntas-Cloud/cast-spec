//! `priority_ceiling_protocol` — prevent priority inversion via ceiling rules.

/// Sentinel for `priority_ceiling_protocol`.
pub struct PriorityCeilingProtocol;

cast::concept! {
    name: "priority_ceiling_protocol",
    summary: "prevent priority inversion via ceiling rules.",
    anchors: [cast_os_stdlib::scheduling::priority_ceiling_protocol::PriorityCeilingProtocol],
    tags: ["cast_os_stdlib", "scheduling"],
}
