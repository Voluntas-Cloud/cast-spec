//! `license_compliance_scan` — detect license obligations.

/// Sentinel for `license_compliance_scan`.
pub struct LicenseComplianceScan;

cast::concept! {
    name: "license_compliance_scan",
    summary: "Detect license obligations. Pulls licenses from each \
              dependency's metadata and flags incompatibilities or \
              attribution requirements before release.",
    anchors: [cast_stdlib::supply_chain::license_compliance_scan::LicenseComplianceScan],
    tags: ["cast_stdlib", "supply_chain"],
}
