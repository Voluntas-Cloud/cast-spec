//! `rpc_local_call` — local procedure-call abstraction.

/// Sentinel for `rpc_local_call`.
pub struct RpcLocalCall;

cast::concept! {
    name: "rpc_local_call",
    summary: "local procedure-call abstraction.",
    anchors: [cast_os_stdlib::ipc::rpc_local_call::RpcLocalCall],
    tags: ["cast_os_stdlib", "ipc"],
}
