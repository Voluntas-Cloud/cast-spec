//! `Draft` — a structured proposed edit to a cast data object,
//! attached to a mailbox message as a fenced `json draft` block.
//!
//! The shape is open: any JSON object is acceptable as `payload`,
//! plus an `intent` string the user provides for the assistant to
//! ground the edit in. Cast-web does no validation here — the
//! assistant on the other side of the mailbox is the validator.

use serde::{Deserialize, Serialize};
use serde_json::Value;

cast::concept! {
    name: "cast_web_draft",
    summary: "Structured proposed edit, encoded as a `json draft` \
              fenced block inside a mailbox message. Carries a \
              freeform `intent` string and an opaque JSON `payload` \
              describing the desired change. Cast-web emits drafts; \
              an attached Claude session reads them, makes the edit \
              in source, and replies with a `json applied` block \
              naming what changed.",
    anchors: [
        crate::draft::Draft,
        crate::draft::serialize,
    ],
    tags: ["cast_web"],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    pub intent: String,
    pub payload: Value,
}

/// Serialize a draft to the body fragment that goes inside a
/// markdown message's fenced `json draft` block.
pub fn serialize(draft: &Draft) -> String {
    serde_json::to_string_pretty(draft).expect("Draft is always serializable")
}
