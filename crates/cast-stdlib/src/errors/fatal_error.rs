//! `fatal_error` — execution cannot continue safely.

/// Sentinel for `fatal_error`.
pub struct FatalError;

cast::concept! {
    name: "fatal_error",
    summary: "Execution cannot continue safely. Continuing would \
              corrupt state or violate invariants; the right response \
              is to abort the operation (or the process) and surface \
              loud diagnostics.",
    anchors: [cast_stdlib::errors::fatal_error::FatalError],
    tags: ["cast_stdlib", "errors"],
}
