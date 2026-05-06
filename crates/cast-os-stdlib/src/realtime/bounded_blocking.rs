//! `bounded_blocking` — guarantee maximum wait on locks/resources.

/// Sentinel for `bounded_blocking`.
pub struct BoundedBlocking;

cast::concept! {
    name: "bounded_blocking",
    summary: "guarantee maximum wait on locks/resources.",
    anchors: [cast_os_stdlib::realtime::bounded_blocking::BoundedBlocking],
    tags: ["cast_os_stdlib", "realtime"],
}
