//! `secret_configuration_split` — secrets separate from normal config.

/// Sentinel for `secret_configuration_split`.
pub struct SecretConfigurationSplit;

cast::concept! {
    name: "secret_configuration_split",
    summary: "Secrets separate from normal config. Different storage, \
              different access controls, different audit trail; \
              prevents the leak where a config dump exposes credentials.",
    anchors: [cast_stdlib::config::secret_configuration_split::SecretConfigurationSplit],
    tags: ["cast_stdlib", "config"],
}
