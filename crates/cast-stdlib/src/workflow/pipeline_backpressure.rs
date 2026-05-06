//! `pipeline_backpressure` — slow producer when downstream overloaded.

/// Sentinel for `pipeline_backpressure`.
pub struct PipelineBackpressure;

cast::concept! {
    name: "pipeline_backpressure",
    summary: "Slow the producer when downstream is overloaded. Buffers \
              between stages are bounded; a full buffer pushes back \
              instead of growing without limit and turning RAM into \
              an outage.",
    anchors: [cast_stdlib::workflow::pipeline_backpressure::PipelineBackpressure],
    tags: ["cast_stdlib", "workflow"],
}
