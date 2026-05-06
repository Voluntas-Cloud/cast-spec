//! `signed_request` — request signed for integrity and origin.

/// Sentinel for `signed_request`.
pub struct SignedRequest;

cast::concept! {
    name: "signed_request",
    summary: "Request integrity and origin proven by signature. \
              Signature must bind to the request body and salient \
              headers — an unbound signature is replayable.",
    anchors: [cast_stdlib::trust::signed_request::SignedRequest],
    tags: ["cast_stdlib", "trust"],
}
