//! `sandboxing` — restrict execution environment.

/// Sentinel for `sandboxing`.
pub struct Sandboxing;

cast::concept! {
    name: "sandboxing",
    summary: "Restrict execution environment. The sandboxed process \
              cannot make syscalls, open files, or dial sockets that \
              its job doesn't require; escape costs the attacker an \
              extra step and a louder audit log.",
    anchors: [cast_stdlib::security::sandboxing::Sandboxing],
    tags: ["cast_stdlib", "security"],
}
