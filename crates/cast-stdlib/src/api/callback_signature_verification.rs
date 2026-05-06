//! `callback_signature_verification` — prove webhook authenticity.

/// Sentinel for `callback_signature_verification`.
pub struct CallbackSignatureVerification;

cast::concept! {
    name: "callback_signature_verification",
    summary: "Prove webhook authenticity. Without it the callback URL \
              is a public endpoint that anyone with internet can spoof.",
    anchors: [cast_stdlib::api::callback_signature_verification::CallbackSignatureVerification],
    tags: ["cast_stdlib", "api"],
}
