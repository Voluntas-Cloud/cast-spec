//! `green_thread` — lightweight user-space thread/fiber.

/// Sentinel for `green_thread`.
pub struct GreenThread;

cast::concept! {
    name: "green_thread",
    summary: "lightweight user-space thread/fiber.",
    anchors: [cast_os_stdlib::execution_model::green_thread::GreenThread],
    tags: ["cast_os_stdlib", "execution_model"],
}
