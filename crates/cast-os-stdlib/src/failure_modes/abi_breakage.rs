//! `abi_breakage` — interface compatibility breaks.

/// Sentinel for `abi_breakage`.
pub struct AbiBreakage;

cast::concept! {
    name: "abi_breakage",
    summary: "interface compatibility breaks.",
    anchors: [cast_os_stdlib::failure_modes::abi_breakage::AbiBreakage],
    tags: ["cast_os_stdlib", "failure_modes"],
}
