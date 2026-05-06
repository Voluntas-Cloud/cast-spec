//! `configuration_precedence` — deterministic override order.

/// Sentinel for `configuration_precedence`.
pub struct ConfigurationPrecedence;

cast::concept! {
    name: "configuration_precedence",
    summary: "Deterministic override order. CLI > env > file > \
              default — or whatever order is chosen, written down, \
              and consistent across loaders.",
    anchors: [cast_stdlib::config::configuration_precedence::ConfigurationPrecedence],
    tags: ["cast_stdlib", "config"],
}
