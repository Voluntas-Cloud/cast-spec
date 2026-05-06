# cast-stdlib

> **Highly experimental.** Concept names, anchor paths, and crate boundaries can change at any time. Pin exact versions if you depend on this. Feedback and breakage reports welcome at <https://github.com/Voluntas-Cloud/cast-spec/issues>.

A curated library of reusable architectural concepts expressed in [`cast`](https://crates.io/crates/cast-spec) vocabulary. Downstream projects pull this in and reference its concepts via `cast::continues_in!` rather than re-deriving the same patterns inline.

## Install

```toml
[dependencies]
cast = { package = "cast-spec", version = "0.1" }
cast-stdlib = "0.1"
```

## Use

```rust
cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "merge_reports",
    why: "Output is a function of the input Reports alone.",
}
```

The macros expand to nothing at compile time. Validation runs through [`cast-extract`](https://crates.io/crates/cast-extract), [`cast-watch`](https://crates.io/crates/cast-watch), or [`cast-lsp`](https://crates.io/crates/cast-lsp).

## License

MIT. See [`LICENSE`](LICENSE).
