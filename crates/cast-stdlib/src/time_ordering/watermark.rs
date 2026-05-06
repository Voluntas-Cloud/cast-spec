//! `watermark` — marker for completeness in streams.

/// Sentinel for `watermark`.
pub struct Watermark;

cast::concept! {
    name: "watermark",
    summary: "A claim by the pipeline: \"I don't expect any more \
              events with a timestamp earlier than T\". Watermarks \
              let windows close and aggregates emit; without them, \
              every window stays open forever, just in case.",
    anchors: [cast_stdlib::time_ordering::watermark::Watermark],
    tags: ["cast_stdlib", "time_ordering"],
}
