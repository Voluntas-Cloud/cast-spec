//! `secret_injection` — provide secrets at runtime safely.

/// Sentinel for `secret_injection`.
pub struct SecretInjection;

cast::concept! {
    name: "secret_injection",
    summary: "Provide secrets at runtime safely. Secrets reach the \
              process via mounted file, env from a trusted broker, or \
              attested fetch — never baked into images, logs, or git \
              history.",
    anchors: [cast_stdlib::security::secret_injection::SecretInjection],
    tags: ["cast_stdlib", "security"],
}
