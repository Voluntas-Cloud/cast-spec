//! `blue_green_deployment` — switch traffic between two full environments.

/// Sentinel for `blue_green_deployment`.
pub struct BlueGreenDeployment;

cast::concept! {
    name: "blue_green_deployment",
    summary: "Switch traffic between two full environments. Old (blue) \
              keeps serving while new (green) warms up; cutover is one \
              router change away from rollback.",
    anchors: [cast_stdlib::deployment::blue_green_deployment::BlueGreenDeployment],
    tags: ["cast_stdlib", "deployment"],
}
