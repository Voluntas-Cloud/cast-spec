//! `secret_scoping` — secrets reach only the workload that needs them.

/// Sentinel for `secret_scoping`.
pub struct SecretScoping;

cast::concept! {
    name: "secret_scoping",
    summary: "Secrets available only where needed. A secret reaches \
              exactly the workload that needs it and nothing else; \
              minimizes the attack surface for exfiltration.",
    anchors: [cast_stdlib::trust::secret_scoping::SecretScoping],
    tags: ["cast_stdlib", "trust"],
}
