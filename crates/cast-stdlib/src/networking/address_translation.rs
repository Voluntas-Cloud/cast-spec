//! `address_translation` — map one address space to another.

/// Sentinel for `address_translation`.
pub struct AddressTranslation;

cast::concept! {
    name: "address_translation",
    summary: "Map one address space to another. NAT, port mapping, or \
              overlay-to-underlay rewriting let networks compose without \
              renumbering; the translation table is then a load-bearing \
              piece of the topology.",
    anchors: [cast_stdlib::networking::address_translation::AddressTranslation],
    tags: ["cast_stdlib", "networking"],
}
