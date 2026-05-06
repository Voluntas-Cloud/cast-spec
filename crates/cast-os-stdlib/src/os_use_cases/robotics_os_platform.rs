//! `robotics_os_platform` — real-time, sensor, actuator, and safety-heavy OS environment.

/// Sentinel for `robotics_os_platform`.
pub struct RoboticsOsPlatform;

cast::concept! {
    name: "robotics_os_platform",
    summary: "real-time, sensor, actuator, and safety-heavy OS \
               environment.",
    anchors: [cast_os_stdlib::os_use_cases::robotics_os_platform::RoboticsOsPlatform],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
