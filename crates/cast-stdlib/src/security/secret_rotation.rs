//! `secret_rotation` — replace secrets periodically or after exposure.

/// Sentinel for `secret_rotation`.
pub struct SecretRotation;

cast::concept! {
    name: "secret_rotation",
    summary: "Replace secrets periodically or after exposure. The \
              system supports two valid versions during overlap so \
              rotation is a normal operation, not a planned outage; \
              \"rotation is hard\" is how leaks get keys older than the \
              engineers who issued them.",
    anchors: [cast_stdlib::security::secret_rotation::SecretRotation],
    tags: ["cast_stdlib", "security"],
}
