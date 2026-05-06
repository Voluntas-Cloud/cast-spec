//! `polling_integration` — periodically check external system.

/// Sentinel for `polling_integration`.
pub struct PollingIntegration;

cast::concept! {
    name: "polling_integration",
    summary: "Periodically ask the external system for changes. \
              Simpler than webhooks, but the polling cadence is a \
              latency-vs-cost dial; pick it deliberately rather than \
              defaulting to one minute because that felt fine.",
    anchors: [cast_stdlib::integration::polling_integration::PollingIntegration],
    tags: ["cast_stdlib", "integration"],
}
