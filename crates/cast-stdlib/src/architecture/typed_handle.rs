//! `typed_handle` — opaque newtype wrapping an internal ID, used as a value handle.

/// Sentinel for `typed_handle`.
pub struct TypedHandle;

cast::concept! {
    name: "typed_handle",
    summary: "Wrap a domain-internal ID (u64, UUID, index into an \
              arena) in a newtype distinct per concept. `UserId(u64)` \
              and `OrderId(u64)` no longer compare or substitute by \
              accident; the compiler enforces that a function asking \
              for an OrderId never silently accepts a UserId. \
              Distinct from `opaque_identifier` (which is about IDs \
              opaque to *external* callers); typed_handle is about \
              type-safety inside the program where the underlying \
              representation is known but the types are kept apart.",
    anchors: [cast_stdlib::architecture::typed_handle::TypedHandle],
    tags: ["cast_stdlib", "architecture"],
}
