//! `implementation_bug` — right primitive, wrong realisation.

/// Sentinel for `implementation_bug`.
pub struct ImplementationBug;

cast::concept! {
    name: "implementation_bug",
    summary: "Right primitive, wrong realisation. The code is grounded \
              on the correct stdlib concept but violates one of its \
              rules — e.g. a signed_request whose signature doesn't \
              bind to the request body, breaking the replay guarantee \
              while the concept itself is the right fit. Diagnostic \
              edge: a `cast::continues_in!` to this sentinel records \
              that the realisation is the locus of the bug, not the \
              concept-choice.",
    anchors: [cast_stdlib::bugs::implementation_bug::ImplementationBug],
    tags: ["cast_stdlib", "bugs"],
}
