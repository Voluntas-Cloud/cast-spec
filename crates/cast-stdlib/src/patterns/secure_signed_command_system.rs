//! `secure_signed_command_system` — commands must be signed by a principal before execution.

/// Sentinel for `secure_signed_command_system`.
pub struct SecureSignedCommandSystem;

cast::concept! {
    name: "secure_signed_command_system",
    summary: "A system where commands must be signed by a principal \
              before execution. Composes principal_authentication, \
              signed_request, replay_guard, capability_token, \
              idempotency_key, authorization_policy, and \
              append_only_audit_trail. Used for mobile approval \
              flows, cluster admin commands, banking-style \
              transaction authorization, AI agents requiring human \
              approval, and Voluntas user-signed intent execution.",
    anchors: [cast_stdlib::patterns::secure_signed_command_system::SecureSignedCommandSystem],
    tags: ["cast_stdlib", "patterns"],
}
