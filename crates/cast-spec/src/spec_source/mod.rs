//! Where cast annotations come *from*. A `SpecSource` is a notation
//! reader: given a project root, it produces a stream of parsed
//! [`crate::parser::CastAnnotation`] values for the analyzer to
//! validate.
//!
//! Two notations ship:
//!
//! - [`rust_macros::RustMacroSource`] — `cast::*!` invocations
//!   embedded in `.rs` source. Discovered via rust-analyzer's HIR.
//!   The original mode of cast.
//! - [`cast_file::CastFileSource`] — `*.cast` files. Clean Rust
//!   syntax constrained to `cast::*!` macro invocations only. Exists
//!   so foreign-language repos (Kotlin, TypeScript, YAML, …) can
//!   host cast annotations without embedding Rust syntax in their
//!   own source files.
//!
//! Both notations feed the *same* downstream pipeline. Parsing,
//! anchor resolution, validation, emit — none of those care which
//! notation produced an annotation.

cast::concept! {
    name: "spec_source",
    summary: "The notation through which a cast annotation enters \
              the analyzer. Two ship: Rust macros embedded in .rs \
              (inline_spec) and *.cast files (detached_spec). The \
              trait abstracts the discovery + parse step so the \
              validator and emit layers never have to branch on \
              notation. Adding a third notation later is a new \
              SpecSource impl, not a sweep through the whole \
              analyzer.",
    anchors: [
        crate::spec_source::SpecSource,
        crate::spec_source::rust_macros::RustMacroSource,
        crate::spec_source::cast_file::CastFileSource,
    ],
    tags: ["spec_source"],
}

cast::compare! {
    inline   @ crate::spec_source::rust_macros::RustMacroSource:
        "Annotation lives inside a .rs file; calling-module context \
         comes from HIR; one parse yields spec and anchor together. \
         Foreign-language code cannot host this notation because \
         cast::*! macros are Rust syntax.",
    detached @ crate::spec_source::cast_file::CastFileSource:
        "Annotation lives in a .cast file; no enclosing Rust module \
         so anchors must be absolute; spec and anchor sit in \
         different files and possibly different repos. Makes \
         foreign-language code annotatable.",
    tags: ["spec_source"],
    note: "Both notations share parsing (per-macro Parse impls in \
           crate::parser) and downstream validation. Only discovery \
           and the calling-module context differ.",
}

// The two SpecSource notations sit at different points on the
// discovery-cost axis. `compare!` above captures the choice between
// them; this `tier!` adds the cost ordering — `.cast` is cheap to
// discover (a filesystem walk for `*.cast`), inline macros are
// expensive (require a fully-loaded ra_ap_ide RootDatabase to walk
// HIR for `cast::*!` invocations).
cast::tier! {
    axis: discovery_cost,
    direction: increasing,
    stages: {
        detached @ crate::spec_source::cast_file::CastFileSource:
            "Filesystem walk under repo_root for `*.cast` files; no \
             compile, no HIR. Cheap enough to run on every snapshot. \
             Calling-module is None, so anchors must be absolute.",
        inline   @ crate::spec_source::rust_macros::RustMacroSource:
            "HIR walk over a loaded RootDatabase. Requires a \
             ProjectHandle and the cost of holding ra_ap_ide in \
             memory. Calling-module comes from HIR, so relative \
             anchors (`crate::*`, `self::*`, `super::*`) work.",
    },
    tags: ["spec_source"],
    note: "Cost ordering, not preference ordering — pick the notation \
           that fits the host language. `compare!` above governs the \
           choice; this tier governs the runtime cost class.",
}

pub mod cast_file;
pub mod rust_macros;
pub mod sidecar;

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "spec_source",
    why: "Discovery walks the filesystem or the HIR snapshot — both are \
          observable external state. Two calls separated by a file \
          edit or a HIR rebuild can return different invocation lists \
          without any change to the SpecSource impl itself.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "detached_spec",
    why: "CastFileSource walks the filesystem for *.cast files; result \
          depends on filesystem state at call time.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "inline_spec",
    why: "RustMacroSource walks the loaded RootDatabase HIR; result \
          depends on the salsa-tracked database state at call time.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "rs_cast_sidecar",
    why: "SidecarSource walks the filesystem for *.rs.cast files \
          alongside .rs siblings; result depends on filesystem state.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::adapter_pattern,
    concept: "detached_spec",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::adapter_pattern,
    concept: "inline_spec",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::adapter_pattern,
    concept: "rs_cast_sidecar",
    why: lazy,
}

use crate::finder::CastInvocation;

/// A notation for cast annotations.
///
/// Implementations carry whatever context they need (HIR handles for
/// inline macros, a discovered set of `.cast` files for detached
/// specs) and emit a flat stream of [`CastInvocation`] values. The
/// stream is the same shape regardless of notation, so downstream
/// stages — parser dispatch, anchor resolution, validation, emit —
/// never branch on where an annotation came from.
pub trait SpecSource {
    /// Discover and return every cast annotation reachable through
    /// this notation, in source order.
    fn invocations(&self) -> Vec<CastInvocation>;
}
