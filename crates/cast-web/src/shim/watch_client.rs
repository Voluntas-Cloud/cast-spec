//! Unix-socket client for the cast-watch JSON-line protocol.
//!
//! One [`WatchClient`] = one connection. Each WebSocket client gets
//! its own; the watcher's protocol is line-delimited request /
//! response, which means a shared client would need request-id
//! demultiplexing, and demultiplexing is the kind of cleverness
//! cast-web is meant to avoid.

use std::path::Path;

use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

pub struct WatchClient {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
}

impl WatchClient {
    pub async fn connect(socket: &Path) -> Result<Self> {
        let stream = UnixStream::connect(socket).await?;
        let (r, w) = stream.into_split();
        Ok(Self {
            reader: BufReader::new(r),
            writer: w,
        })
    }

    pub async fn send_line(&mut self, line: &str) -> Result<()> {
        self.writer.write_all(line.as_bytes()).await?;
        if !line.ends_with('\n') {
            self.writer.write_all(b"\n").await?;
        }
        self.writer.flush().await?;
        Ok(())
    }

    /// Returns `Ok(None)` on clean EOF.
    pub async fn read_line(&mut self) -> Result<Option<String>> {
        let mut buf = String::new();
        let n = self.reader.read_line(&mut buf).await?;
        if n == 0 {
            return Ok(None);
        }
        Ok(Some(buf))
    }
}
