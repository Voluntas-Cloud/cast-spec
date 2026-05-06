//! HTTP surface for the mailbox plane.
//!
//! - `POST /mailbox/send` — append the next-numbered message file
//! - `GET  /mailbox/messages` — list parsed messages
//! - `GET  /mailbox/meta` — heartbeat-derived liveness for the UI
//! - `GET  /mailbox/stream` — Server-Sent Events: filename of each
//!   new file the directory watcher sees
//!
//! Cast-web is a thin layer over the filesystem here. Validation
//! lives in `Mailbox::append` (atomic O_EXCL allocation) and in
//! `Message::parse` (frontmatter shape). Anything more ambitious
//! belongs in the attached Claude session, not in this shim.

use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use futures::stream::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;
use tracing::warn;

use crate::mailbox::watch_directory::{DirectoryEvent, DirectoryWatch};
use crate::mailbox::{AppendRequest, AppendResponse, Liveness, Mailbox, Message};

#[derive(Clone)]
pub struct MailboxState {
    pub mailbox: Arc<Mailbox>,
    pub watch: Arc<DirectoryWatch>,
}

pub async fn list_messages(State(state): State<MailboxState>) -> impl IntoResponse {
    match state.mailbox.list_messages().await {
        Ok(messages) => Json(MessagesResponse { messages }).into_response(),
        Err(e) => {
            warn!(err = %e, "list_messages failed");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn meta(State(state): State<MailboxState>) -> impl IntoResponse {
    match state.mailbox.liveness().await {
        Ok(liveness) => Json::<Liveness>(liveness).into_response(),
        Err(e) => {
            warn!(err = %e, "liveness failed");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn send(
    State(state): State<MailboxState>,
    Json(req): Json<AppendRequest>,
) -> impl IntoResponse {
    match state.mailbox.append(&req).await {
        Ok(resp) => (StatusCode::CREATED, Json::<AppendResponse>(resp)).into_response(),
        Err(e) => {
            warn!(err = %e, "append failed");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn clear(State(state): State<MailboxState>) -> impl IntoResponse {
    match state.mailbox.clear().await {
        Ok(removed) => (StatusCode::OK, Json(ClearResponse { removed })).into_response(),
        Err(e) => {
            warn!(err = %e, "clear failed");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

#[derive(serde::Serialize)]
struct ClearResponse {
    removed: usize,
}

pub async fn stream(
    State(state): State<MailboxState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.watch.events.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|item| async move {
        match item {
            Ok(DirectoryEvent { filename }) => Some(Ok(Event::default()
                .event("file")
                .data(filename))),
            // BroadcastStreamRecvError::Lagged — slow client, drop &
            // continue. The next event will resync them.
            Err(_) => None,
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}

#[derive(serde::Serialize)]
struct MessagesResponse {
    messages: Vec<Message>,
}
