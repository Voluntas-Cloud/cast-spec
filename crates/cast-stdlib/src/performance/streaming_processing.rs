//! `streaming_processing` — process incrementally as data arrives.

/// Sentinel for `streaming_processing`.
pub struct StreamingProcessing;

cast::concept! {
    name: "streaming_processing",
    summary: "Process incrementally as data arrives. Bounded memory, \
              continuous output; alternative to batch processing for \
              workloads where waiting for the whole input is wrong.",
    anchors: [cast_stdlib::performance::streaming_processing::StreamingProcessing],
    tags: ["cast_stdlib", "performance"],
}
