//! `microkernel_architecture` — small core, external features.

/// Sentinel for `microkernel_architecture`.
pub struct MicrokernelArchitecture;

cast::concept! {
    name: "microkernel_architecture",
    summary: "Small core with external features. Kernel handles only \
              the load-bearing primitives; everything else is a \
              service the kernel orchestrates.",
    anchors: [cast_stdlib::architecture::microkernel_architecture::MicrokernelArchitecture],
    tags: ["cast_stdlib", "architecture"],
}
