//! `allow_by_exception` — explicit grants only.

/// Sentinel for `allow_by_exception`.
pub struct AllowByException;

cast::concept! {
    name: "allow_by_exception",
    summary: "Explicit grants only. The complement of deny-by-default; \
              every permitted action is named, every other action is \
              implicitly forbidden.",
    anchors: [cast_stdlib::config::allow_by_exception::AllowByException],
    tags: ["cast_stdlib", "config"],
}
