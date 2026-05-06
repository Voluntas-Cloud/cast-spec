//! `dark_launch` — deploy without exposing behavior.

/// Sentinel for `dark_launch`.
pub struct DarkLaunch;

cast::concept! {
    name: "dark_launch",
    summary: "Deploy without exposing behavior. Code is in production \
              but disabled or invisible to users; lets you observe \
              real load and coverage before flipping the visibility \
              switch.",
    anchors: [cast_stdlib::deployment::dark_launch::DarkLaunch],
    tags: ["cast_stdlib", "deployment"],
}
