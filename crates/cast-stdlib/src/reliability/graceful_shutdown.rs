//! `graceful_shutdown` — finish/stop work cleanly before exit.

/// Sentinel for `graceful_shutdown`.
pub struct GracefulShutdown;

cast::concept! {
    name: "graceful_shutdown",
    summary: "Finish/stop work cleanly before exit. Drain in-flight \
              requests, flush buffers, release leases — give the rest \
              of the system a chance to react before disappearing.",
    anchors: [cast_stdlib::reliability::graceful_shutdown::GracefulShutdown],
    tags: ["cast_stdlib", "reliability"],
}
