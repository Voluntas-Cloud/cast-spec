//! `architectural_bug` — wrong primitive used.

/// Sentinel for `architectural_bug`.
pub struct ArchitecturalBug;

cast::concept! {
    name: "architectural_bug",
    summary: "Wrong primitive used. The implementation may be \
              internally correct, but it's grounded on the wrong \
              stdlib concept — the symptom doesn't go away by fixing \
              realisation, it goes away by re-grounding on a different \
              primitive (e.g. uuid_identity instead of \
              monotonic_sequence_id when writers are uncoordinated). \
              Diagnostic edge: a `cast::continues_in!` to this sentinel \
              records that the concept-choice is the locus of the bug.",
    anchors: [cast_stdlib::bugs::architectural_bug::ArchitecturalBug],
    tags: ["cast_stdlib", "bugs"],
}
