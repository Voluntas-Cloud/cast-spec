//! `pledge_unveil_model` — OpenBSD-style process promise and filesystem exposure limits.

/// Sentinel for `pledge_unveil_model`.
pub struct PledgeUnveilModel;

cast::concept! {
    name: "pledge_unveil_model",
    summary: "OpenBSD-style process promise and filesystem exposure \
               limits.",
    anchors: [cast_os_stdlib::security::pledge_unveil_model::PledgeUnveilModel],
    tags: ["cast_os_stdlib", "security"],
}
