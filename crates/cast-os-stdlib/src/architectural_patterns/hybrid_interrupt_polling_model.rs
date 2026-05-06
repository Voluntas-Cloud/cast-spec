//! `hybrid_interrupt_polling_model` — dynamic mix of both.

/// Sentinel for `hybrid_interrupt_polling_model`.
pub struct HybridInterruptPollingModel;

cast::concept! {
    name: "hybrid_interrupt_polling_model",
    summary: "dynamic mix of both.",
    anchors: [cast_os_stdlib::architectural_patterns::hybrid_interrupt_polling_model::HybridInterruptPollingModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
