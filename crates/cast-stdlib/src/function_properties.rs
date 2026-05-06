//! Predicates over functions — properties a function may or may not have.
//!
//! Where `algorithms/` names *what computation* a function performs,
//! this module names *how* it behaves: pure, deterministic, idempotent,
//! side-effect-free, … Each property is a sentinel anchored at
//! `cast_stdlib::function_properties::<property>::<Sentinel>` so a
//! concrete function in downstream code can declare its properties via
//! `cast::continues_in!`. Properties compose: a pure function is
//! always deterministic; a deterministic function is not necessarily
//! pure.

pub mod deterministic;
pub mod non_deterministic;
pub mod pure_function;
