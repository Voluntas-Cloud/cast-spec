//! `trust_calibration` — user understands system confidence/limits.

/// Sentinel for `trust_calibration`.
pub struct TrustCalibration;

cast::concept! {
    name: "trust_calibration",
    summary: "Help the user understand how confident the system is \
              and where it is guessing. Overstating confidence \
              produces overreliance; understating it produces \
              learned helplessness.",
    anchors: [cast_stdlib::ux::trust_calibration::TrustCalibration],
    tags: ["cast_stdlib", "ux"],
}
