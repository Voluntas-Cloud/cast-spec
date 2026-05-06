//! `input_validation` — reject malformed/untrusted input.

/// Sentinel for `input_validation`.
pub struct InputValidation;

cast::concept! {
    name: "input_validation",
    summary: "Reject malformed or untrusted input. The check happens at \
              the trust boundary, against an explicit shape; downstream \
              code is allowed to assume the input is well-formed.",
    anchors: [cast_stdlib::security::input_validation::InputValidation],
    tags: ["cast_stdlib", "security"],
}
