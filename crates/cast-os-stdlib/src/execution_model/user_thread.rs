//! `user_thread` — thread scheduled in user space.

/// Sentinel for `user_thread`.
pub struct UserThread;

cast::concept! {
    name: "user_thread",
    summary: "thread scheduled in user space.",
    anchors: [cast_os_stdlib::execution_model::user_thread::UserThread],
    tags: ["cast_os_stdlib", "execution_model"],
}
