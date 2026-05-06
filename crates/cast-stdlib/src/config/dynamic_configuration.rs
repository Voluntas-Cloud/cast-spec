//! `dynamic_configuration` — runtime-updatable config.

/// Sentinel for `dynamic_configuration`.
pub struct DynamicConfiguration;

cast::concept! {
    name: "dynamic_configuration",
    summary: "Runtime-updatable config. Values change without redeploy; \
              the system re-reads or is signalled, and applies the new \
              value safely.",
    anchors: [cast_stdlib::config::dynamic_configuration::DynamicConfiguration],
    tags: ["cast_stdlib", "config"],
}
