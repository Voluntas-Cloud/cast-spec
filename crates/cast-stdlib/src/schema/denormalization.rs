//! `denormalization` — duplicate data deliberately for read performance.

/// Sentinel for `denormalization`.
pub struct Denormalization;

cast::concept! {
    name: "denormalization",
    summary: "Duplicate data deliberately for read performance. Trades \
              update complexity for read simplicity; required when \
              joins are too expensive.",
    anchors: [cast_stdlib::schema::denormalization::Denormalization],
    tags: ["cast_stdlib", "schema"],
}
