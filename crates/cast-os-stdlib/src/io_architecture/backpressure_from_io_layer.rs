//! `backpressure_from_io_layer` — storage/network layer slows producers.

/// Sentinel for `backpressure_from_io_layer`.
pub struct BackpressureFromIoLayer;

cast::concept! {
    name: "backpressure_from_io_layer",
    summary: "storage/network layer slows producers.",
    anchors: [cast_os_stdlib::io_architecture::backpressure_from_io_layer::BackpressureFromIoLayer],
    tags: ["cast_os_stdlib", "io_architecture"],
}
