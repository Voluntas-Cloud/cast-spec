//! `ingestion_time` — when system received event.

/// Sentinel for `ingestion_time`.
pub struct IngestionTime;

cast::concept! {
    name: "ingestion_time",
    summary: "When the system first received the event. Sits between \
              event_time and processing_time and is sometimes the \
              best you can get when upstream timestamps aren't \
              trustworthy.",
    anchors: [cast_stdlib::time_ordering::ingestion_time::IngestionTime],
    tags: ["cast_stdlib", "time_ordering"],
}
