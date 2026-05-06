//! `code_signing_enforcement` — executable authenticity policy.

/// Sentinel for `code_signing_enforcement`.
pub struct CodeSigningEnforcement;

cast::concept! {
    name: "code_signing_enforcement",
    summary: "executable authenticity policy.",
    anchors: [cast_os_stdlib::security::code_signing_enforcement::CodeSigningEnforcement],
    tags: ["cast_os_stdlib", "security"],
}
