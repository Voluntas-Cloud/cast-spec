//! Error handling & recovery patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod authorization_error;
pub mod conflict_error;
pub mod error_redaction;
pub mod error_wrapping;
pub mod fatal_error;
pub mod not_found_error;
pub mod operator_facing_error;
pub mod partial_failure;
pub mod poison_record_detection;
pub mod recoverable_error;
pub mod retryable_error_marker;
pub mod safe_failure_mode;
pub mod timeout_error;
pub mod typed_error;
pub mod user_facing_error;
pub mod validation_error;

cast::concept! {
    name: "errors",
    summary: "Umbrella for the errors stdlib category. Error handling & \
              recovery patterns.",
    anchors: [
        crate::errors::authorization_error,
        crate::errors::conflict_error,
        crate::errors::error_redaction,
        crate::errors::error_wrapping,
        crate::errors::fatal_error,
        crate::errors::not_found_error,
        crate::errors::operator_facing_error,
        crate::errors::partial_failure,
        crate::errors::poison_record_detection,
        crate::errors::recoverable_error,
        crate::errors::retryable_error_marker,
        crate::errors::safe_failure_mode,
        crate::errors::timeout_error,
        crate::errors::typed_error,
        crate::errors::user_facing_error,
        crate::errors::validation_error,
    ],
    tags: ["cast_stdlib", "errors"],
}

/// Sentinel for the errors stdlib group.
pub struct ErrorsGroup;

cast::rule! {
    rule: "Errors are part of the API.",
    why:  "Treat them like contracts, not embarrassed exceptions \
           leaking out of the basement. Callers route on error shape; \
           changing it is a breaking change whether you noticed or not.",
    governs: [cast_stdlib::errors::ErrorsGroup],
    tags: ["cast_stdlib", "errors"],
}
