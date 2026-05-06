//! `configuration_as_data` — config is structured and validated.

/// Sentinel for `configuration_as_data`.
pub struct ConfigurationAsData;

cast::concept! {
    name: "configuration_as_data",
    summary: "Config is structured and validated. Not a free-form \
              shell script that pretends to be config; a typed \
              document the loader can validate before applying.",
    anchors: [cast_stdlib::config::configuration_as_data::ConfigurationAsData],
    tags: ["cast_stdlib", "config"],
}
