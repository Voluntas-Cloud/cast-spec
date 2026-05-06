//! `declarative_system_configuration` — system state defined by config.

/// Sentinel for `declarative_system_configuration`.
pub struct DeclarativeSystemConfiguration;

cast::concept! {
    name: "declarative_system_configuration",
    summary: "system state defined by config.",
    anchors: [cast_os_stdlib::update_evolution::declarative_system_configuration::DeclarativeSystemConfiguration],
    tags: ["cast_os_stdlib", "update_evolution"],
}
