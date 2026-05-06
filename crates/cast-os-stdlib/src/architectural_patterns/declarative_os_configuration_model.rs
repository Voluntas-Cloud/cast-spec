//! `declarative_os_configuration_model` — desired system state declared in config.

/// Sentinel for `declarative_os_configuration_model`.
pub struct DeclarativeOsConfigurationModel;

cast::concept! {
    name: "declarative_os_configuration_model",
    summary: "desired system state declared in config.",
    anchors: [cast_os_stdlib::architectural_patterns::declarative_os_configuration_model::DeclarativeOsConfigurationModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
