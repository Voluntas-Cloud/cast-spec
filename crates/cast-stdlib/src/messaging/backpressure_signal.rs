//! `backpressure_signal` — downstream signals 'slow down'.

/// Sentinel for `backpressure_signal`.
pub struct BackpressureSignal;

cast::concept! {
    name: "backpressure_signal",
    summary: "Downstream says 'slow down'. Producer responds by buffering, \
              dropping, or failing fast — anything but silently \
              overrunning the consumer.",
    anchors: [cast_stdlib::messaging::backpressure_signal::BackpressureSignal],
    tags: ["cast_stdlib", "messaging"],
}
