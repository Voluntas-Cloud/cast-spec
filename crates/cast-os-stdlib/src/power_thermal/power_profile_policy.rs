//! `power_profile_policy` — performance vs energy tradeoff.

/// Sentinel for `power_profile_policy`.
pub struct PowerProfilePolicy;

cast::concept! {
    name: "power_profile_policy",
    summary: "performance vs energy tradeoff.",
    anchors: [cast_os_stdlib::power_thermal::power_profile_policy::PowerProfilePolicy],
    tags: ["cast_os_stdlib", "power_thermal"],
}
