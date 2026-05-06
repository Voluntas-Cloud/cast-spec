//! `hardware_backed_key` — private key protected by device hardware.

/// Sentinel for `hardware_backed_key`.
pub struct HardwareBackedKey;

cast::concept! {
    name: "hardware_backed_key",
    summary: "Private key protected by device hardware. Key cannot \
              be extracted by software compromise alone; signing \
              operations happen inside the hardware boundary.",
    anchors: [cast_stdlib::trust::hardware_backed_key::HardwareBackedKey],
    tags: ["cast_stdlib", "trust"],
}
