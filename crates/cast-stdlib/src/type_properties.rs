//! Predicates over types — properties a type may or may not have.
//!
//! Where `function_properties/` names *how a function behaves*
//! (pure, deterministic, side-effect-free), this module names *how a
//! type composes and what it stands for*: value vs handle, sum vs
//! product, sentinel marker, error carrier, serialisable. Each
//! property is a sentinel anchored at
//! `cast_stdlib::type_properties::<property>::<Sentinel>`. Concrete
//! types in downstream code declare their properties via
//! `cast::continues_in!`. Properties compose: a struct can be both
//! `product_type` and `value_type` and `serializable` at the same
//! time.

pub mod error_type;
pub mod product_type;
pub mod resource_handle;
pub mod sentinel_type;
pub mod serializable;
pub mod sum_type;
pub mod value_type;
