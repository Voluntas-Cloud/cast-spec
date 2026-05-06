//! `memoization` — cache function result by input.

/// Sentinel for `memoization`.
pub struct Memoization;

cast::concept! {
    name: "memoization",
    summary: "Cache a function's result keyed on its input. Works \
              when the function is pure; turns subtle when it isn't, \
              because the cache will keep returning the answer that \
              was correct at first call.",
    anchors: [cast_stdlib::state_data::memoization::Memoization],
    tags: ["cast_stdlib", "state_data"],
}
