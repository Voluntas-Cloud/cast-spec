//! `privilege_separation` — split powers across processes/roles.

/// Sentinel for `privilege_separation`.
pub struct PrivilegeSeparation;

cast::concept! {
    name: "privilege_separation",
    summary: "Split powers across processes or roles. Compromising one \
              component does not yield the keys to the rest; the \
              process that parses untrusted input is not the process \
              that holds private keys.",
    anchors: [cast_stdlib::security::privilege_separation::PrivilegeSeparation],
    tags: ["cast_stdlib", "security"],
}
