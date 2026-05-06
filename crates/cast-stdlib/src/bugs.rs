//! Bug taxonomy — the meta-concepts of "this code has a flaw".
//!
//! These concepts are used as DIAGNOSTIC EDGES, not as documentation
//! tags. Attach a `cast::continues_in!` from the concept whose code
//! is misbehaving to one of `architectural_bug`, `implementation_bug`,
//! or `unknown_locus_bug`; the choice of flavour IS the answer to the
//! question "where does the bug live?".
//!
//! A flavour edge that points at `unknown_locus_bug` is the OPEN form
//! of the diagnostic — it records that investigation is still needed.
//! The action to close the question is to re-classify the edge
//! (architectural vs implementation) or remove it once the bug is
//! fixed. Removing without classifying erases the question instead of
//! answering it; the rule below pins that.

pub mod architectural_bug;
pub mod implementation_bug;
pub mod unknown_locus_bug;

cast::concept! {
    name: "bugs",
    summary: "Umbrella for the bugs stdlib category. Three diagnostic \
              flavours of `the code has a flaw`: architectural_bug \
              (wrong primitive used), implementation_bug (right \
              primitive, wrong realisation), unknown_locus_bug (bug \
              observed, locus undecided). The flavour you pick when \
              attaching a `continues_in` edge IS the diagnostic answer.",
    anchors: [
        crate::bugs::architectural_bug,
        crate::bugs::implementation_bug,
        crate::bugs::unknown_locus_bug,
    ],
    tags: ["cast_stdlib", "bugs"],
}

/// Sentinel for the bugs stdlib group.
pub struct BugsGroup;

cast::rule! {
    rule: "An `unknown_locus_bug` edge is closed by re-classifying \
           (to architectural_bug or implementation_bug) once the locus \
           is decided, or by removing the edge once the bug is fixed. \
           Do NOT close it by silently deleting the edge before \
           classification — that erases the diagnostic question \
           instead of answering it.",
    why:  "The whole point of the unknown-locus flavour is to make \
           an open architectural question searchable and reviewable. \
           If a developer deletes the edge as part of an unrelated \
           refactor without recording why, the team loses the record \
           that the question was ever asked, and the bug recurs the \
           next time someone touches the same surface.",
    governs: [cast_stdlib::bugs::BugsGroup],
    tags: ["cast_stdlib", "bugs"],
}
