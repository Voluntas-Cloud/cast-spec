//! `single_writer_principle` — one authority mutates given state.

/// Sentinel for `single_writer_principle`.
pub struct SingleWriterPrinciple;

cast::concept! {
    name: "single_writer_principle",
    summary: "Exactly one authority mutates a given piece of state. \
              Two writers means two opinions about the truth, and \
              you'll discover which one wins under exactly the load \
              that makes the bug irreproducible.",
    anchors: [cast_stdlib::state_data::single_writer_principle::SingleWriterPrinciple],
    tags: ["cast_stdlib", "state_data"],
}
