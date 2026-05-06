//! `remote_attestation` — prove system/software state remotely.

/// Sentinel for `remote_attestation`.
pub struct RemoteAttestation;

cast::concept! {
    name: "remote_attestation",
    summary: "prove system/software state remotely.",
    anchors: [cast_os_stdlib::security::remote_attestation::RemoteAttestation],
    tags: ["cast_os_stdlib", "security"],
}
