//! `signed_package_verification` — package authenticity check.

/// Sentinel for `signed_package_verification`.
pub struct SignedPackageVerification;

cast::concept! {
    name: "signed_package_verification",
    summary: "package authenticity check.",
    anchors: [cast_os_stdlib::update_evolution::signed_package_verification::SignedPackageVerification],
    tags: ["cast_os_stdlib", "update_evolution"],
}
