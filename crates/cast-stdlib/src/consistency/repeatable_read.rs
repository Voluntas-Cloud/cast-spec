//! `repeatable_read` — repeated reads see stable results.

/// Sentinel for `repeatable_read`.
pub struct RepeatableRead;

cast::concept! {
    name: "repeatable_read",
    summary: "Repeated reads see stable results within a transaction. \
              Avoids non-repeatable reads; phantoms still possible.",
    anchors: [cast_stdlib::consistency::repeatable_read::RepeatableRead],
    tags: ["cast_stdlib", "consistency"],
}
