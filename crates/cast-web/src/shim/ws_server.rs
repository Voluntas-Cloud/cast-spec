//! WebSocket → cast-watch unix socket bridge.
//!
//! On each `/ws` upgrade we open a fresh [`WatchClient`] and wire
//! two tasks: browser→watcher (forward text frames as JSON lines)
//! and watcher→browser (forward each watcher line as a text frame).
//! Either side closing tears the other down.

use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use tracing::{info, warn};

use super::watch_client::WatchClient;

#[derive(Clone)]
pub struct WsState {
    pub watch_socket: Arc<PathBuf>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<WsState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: WsState) {
    let mut client = match WatchClient::connect(state.watch_socket.as_path()).await {
        Ok(c) => c,
        Err(e) => {
            warn!(err = %e, "cast-watch connect failed");
            return;
        }
    };
    info!(socket = %state.watch_socket.display(), "ws ↔ cast-watch open");

    let (mut ws_tx, mut ws_rx) = socket.split();

    loop {
        tokio::select! {
            // browser → watcher
            msg = ws_rx.next() => {
                match msg {
                    Some(Ok(Message::Text(t))) => {
                        if let Err(e) = client.send_line(&t).await {
                            warn!(err = %e, "watcher write failed");
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {} // ignore Binary/Ping/Pong
                    Some(Err(e)) => {
                        warn!(err = %e, "ws read failed");
                        break;
                    }
                }
            }
            // watcher → browser
            line = client.read_line() => {
                match line {
                    Ok(Some(s)) => {
                        if let Err(e) = ws_tx.send(Message::Text(s)).await {
                            warn!(err = %e, "ws write failed");
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        warn!(err = %e, "watcher read failed");
                        break;
                    }
                }
            }
        }
    }

    let _ = ws_tx.close().await;
    info!("ws ↔ cast-watch closed");
}
