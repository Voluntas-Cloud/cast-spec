//! `fire_and_forget_message` — sender does not wait for result.

/// Sentinel for `fire_and_forget_message`.
pub struct FireAndForgetMessage;

cast::concept! {
    name: "fire_and_forget_message",
    summary: "Sender does not wait for result. Asynchronous, no \
              acknowledgement; cheap but the sender does not know \
              whether work happened.",
    anchors: [cast_stdlib::messaging::fire_and_forget_message::FireAndForgetMessage],
    tags: ["cast_stdlib", "messaging"],
}
