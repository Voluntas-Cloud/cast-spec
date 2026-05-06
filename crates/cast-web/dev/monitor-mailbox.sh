#!/usr/bin/env bash
# Tail cast-web's own SSE mailbox stream and emit one line per new
# user-message file. Designed to be the `command:` of a Claude Code
# Monitor — each emitted line becomes a chat notification.
#
# Resilient to cast-web restarts: when the SSE connection drops
# (e.g. ensure-cast-web.sh --force-restart kills the binary),
# reconnects after a short back-off rather than letting the Monitor
# task die. So you can allowlist this once and forget about it across
# many rebuilds.
#
# Filter intentionally drops `*-assistant.md` (so my own replies
# never echo back) and any non-message keepalive frames.
set -uo pipefail

URL="${CAST_WEB_URL:-http://127.0.0.1:8765}"

while true; do
    curl -sN --connect-timeout 2 "$URL/mailbox/stream" 2>/dev/null \
        | grep --line-buffered -E '^data: [0-9]+-user\.md$' || true
    # If we get here, the stream ended (likely cast-web restarted).
    # Back off briefly then reconnect.
    sleep 1
done
