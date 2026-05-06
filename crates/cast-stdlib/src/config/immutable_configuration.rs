//! `immutable_configuration` — config fixed for a deployment.

/// Sentinel for `immutable_configuration`.
pub struct ImmutableConfiguration;

cast::concept! {
    name: "immutable_configuration",
    summary: "Config fixed for a deployment. Changing config means \
              shipping a new artifact; eliminates the 'what was the \
              config when this broke?' question.",
    anchors: [cast_stdlib::config::immutable_configuration::ImmutableConfiguration],
    tags: ["cast_stdlib", "config"],
}
