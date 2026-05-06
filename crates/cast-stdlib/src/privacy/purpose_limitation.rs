//! `purpose_limitation` — use data only for declared purpose.

/// Sentinel for `purpose_limitation`.
pub struct PurposeLimitation;

cast::concept! {
    name: "purpose_limitation",
    summary: "Use data only for the declared purpose. New uses require \
              new consent or a new lawful basis; \"we already had it \
              anyway\" is not a basis, however much product wants it \
              to be.",
    anchors: [cast_stdlib::privacy::purpose_limitation::PurposeLimitation],
    tags: ["cast_stdlib", "privacy"],
}
