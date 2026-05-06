# cast-os-stdlib

> **Highly experimental.** Concept names, anchor paths, and crate boundaries can change at any time. Pin exact versions if you depend on this. Feedback and breakage reports welcome at <https://github.com/Voluntas-Cloud/cast-spec/issues>.

A curated library of reusable OS-level architectural concepts expressed in [`cast`](https://crates.io/crates/cast-spec) vocabulary. Downstream OS / kernel / cluster projects pull this in and reference its concepts via `cast::continues_in!` rather than re-deriving the same patterns inline.

Companion to [`cast-stdlib`](https://crates.io/crates/cast-stdlib): `cast-stdlib` covers application-level patterns (pure functions, stateful workflows, format converters, …); `cast-os-stdlib` covers systems-level patterns (process boundaries, IPC shapes, syscall surfaces, …).

## Install

```toml
[dependencies]
cast = { package = "cast-spec", version = "0.1" }
cast-os-stdlib = "0.1"
```

## License

MIT. See [`LICENSE`](LICENSE).
