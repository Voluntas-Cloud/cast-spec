//! `security_test` — test attack resistance.

/// Sentinel for `security_test`.
pub struct SecurityTest;

cast::concept! {
    name: "security_test",
    summary: "Test attack resistance. Authentication bypass, injection, \
              authorization confusion — verify that the security model \
              holds against the threats it claims to handle.",
    anchors: [cast_stdlib::testing::security_test::SecurityTest],
    tags: ["cast_stdlib", "testing"],
}
