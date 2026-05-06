//! `fluent_builder` — chained Self-returning methods for incremental construction.

/// Sentinel for `fluent_builder`.
pub struct FluentBuilder;

cast::concept! {
    name: "fluent_builder",
    summary: "Builder methods take `self`, mutate or replace one \
              field, and return `Self` so calls chain: \
              `Foo::new().with_a(1).with_b(2).build()`. The chain \
              reads top-down, the type system tracks which fields \
              were set (via type-state in the more strict variant), \
              and the terminal `.build()` consumes the builder into \
              the target type. Most useful when constructors would \
              otherwise have many positional arguments or many \
              optional arguments.",
    anchors: [cast_stdlib::api::fluent_builder::FluentBuilder],
    tags: ["cast_stdlib", "api"],
}
