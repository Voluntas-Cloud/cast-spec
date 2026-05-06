//! `poison_record_detection` — identify item that repeatedly fails.

/// Sentinel for `poison_record_detection`.
pub struct PoisonRecordDetection;

cast::concept! {
    name: "poison_record_detection",
    summary: "Identify items that repeatedly fail. After N attempts the \
              system stops retrying, sets the item aside, and lets \
              healthy work continue; without this, one bad message \
              becomes a queue-shaped outage.",
    anchors: [cast_stdlib::errors::poison_record_detection::PoisonRecordDetection],
    tags: ["cast_stdlib", "errors"],
}
