//! `resource_handle` — wraps a heavyweight external resource; identity is the resource.

/// Sentinel for `resource_handle`.
pub struct ResourceHandle;

cast::concept! {
    name: "resource_handle",
    summary: "Wraps an external resource (file, socket, db, GPU \
              context, analyzer database). Identity is the resource, \
              not the field shape. Cloning aliases or is forbidden; \
              equality is identity; not safely serialisable. \
              Contrast: value_type.",
    anchors: [cast_stdlib::type_properties::resource_handle::ResourceHandle],
    tags: ["cast_stdlib", "type_properties"],
}
