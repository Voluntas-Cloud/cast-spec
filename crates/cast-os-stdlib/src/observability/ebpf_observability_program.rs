//! `ebpf_observability_program` — safe kernel instrumentation program.

/// Sentinel for `ebpf_observability_program`.
pub struct EbpfObservabilityProgram;

cast::concept! {
    name: "ebpf_observability_program",
    summary: "safe kernel instrumentation program.",
    anchors: [cast_os_stdlib::observability::ebpf_observability_program::EbpfObservabilityProgram],
    tags: ["cast_os_stdlib", "observability"],
}
