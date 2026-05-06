//! `audio_server_model` — OS/user-space audio routing.

/// Sentinel for `audio_server_model`.
pub struct AudioServerModel;

cast::concept! {
    name: "audio_server_model",
    summary: "OS/user-space audio routing.",
    anchors: [cast_os_stdlib::desktop_session::audio_server_model::AudioServerModel],
    tags: ["cast_os_stdlib", "desktop_session"],
}
