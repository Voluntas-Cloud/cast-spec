//! `access_check_algorithm` — evaluate ACL/security policy.

/// Sentinel for `access_check_algorithm`.
pub struct AccessCheckAlgorithm;

cast::concept! {
    name: "access_check_algorithm",
    summary: "evaluate ACL/security policy.",
    anchors: [cast_os_stdlib::os_algorithms::access_check_algorithm::AccessCheckAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
