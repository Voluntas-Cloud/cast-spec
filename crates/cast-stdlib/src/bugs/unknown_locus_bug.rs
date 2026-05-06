//! `unknown_locus_bug` — bug observed; locus undecided.

/// Sentinel for `unknown_locus_bug`.
pub struct UnknownLocusBug;

cast::concept! {
    name: "unknown_locus_bug",
    summary: "Bug observed; locus undecided. The symptom is real but \
              it's not yet established whether the primitive choice is \
              wrong (architectural_bug) or the realisation is wrong \
              (implementation_bug). This flavour is the OPEN form of \
              the diagnostic question — the edge says `something is \
              broken here, and we haven't decided where`. Re-classify \
              to architectural_bug or implementation_bug once \
              investigation lands, or remove the edge if the bug is \
              fixed. Resolving by deletion without classification \
              loses the question.",
    anchors: [cast_stdlib::bugs::unknown_locus_bug::UnknownLocusBug],
    tags: ["cast_stdlib", "bugs"],
}
