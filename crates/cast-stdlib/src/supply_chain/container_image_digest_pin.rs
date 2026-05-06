//! `container_image_digest_pin` — deploy by digest, not mutable tag.

/// Sentinel for `container_image_digest_pin`.
pub struct ContainerImageDigestPin;

cast::concept! {
    name: "container_image_digest_pin",
    summary: "Deploy by digest, not mutable tag. `:latest` and other \
              floating tags can change underneath; pinning to the \
              content hash is the only way to guarantee what runs.",
    anchors: [cast_stdlib::supply_chain::container_image_digest_pin::ContainerImageDigestPin],
    tags: ["cast_stdlib", "supply_chain"],
}
