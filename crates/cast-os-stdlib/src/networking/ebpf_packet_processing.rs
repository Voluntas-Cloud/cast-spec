//! `ebpf_packet_processing` — programmable packet handling.

/// Sentinel for `ebpf_packet_processing`.
pub struct EbpfPacketProcessing;

cast::concept! {
    name: "ebpf_packet_processing",
    summary: "programmable packet handling.",
    anchors: [cast_os_stdlib::networking::ebpf_packet_processing::EbpfPacketProcessing],
    tags: ["cast_os_stdlib", "networking"],
}
