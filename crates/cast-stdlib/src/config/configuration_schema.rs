//! `configuration_schema` — config has formal shape.

/// Sentinel for `configuration_schema`.
pub struct ConfigurationSchema;

cast::concept! {
    name: "configuration_schema",
    summary: "Config has formal shape. Field names, types, defaults, \
              and constraints captured machine-readably; loader \
              rejects malformed input with structured errors.",
    anchors: [cast_stdlib::config::configuration_schema::ConfigurationSchema],
    tags: ["cast_stdlib", "config"],
}
