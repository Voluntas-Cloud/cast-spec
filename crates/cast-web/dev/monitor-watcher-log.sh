#!/usr/bin/env bash
# Tail cast-web's SSE watcher-log stream and emit one line per new
# response file. Designed as a Claude Code Monitor command — each
# emitted line becomes a chat notification when the watcher finishes
# a round-trip (the response file lands ~1ms after its paired query).
#
# Sister to monitor-mailbox.sh: same SSE-via-curl pattern, just
# pointed at the watcher-log directory's stream rather than the
# user/assistant mailbox stream. Resilient to cast-web restarts:
# when the SSE connection drops the loop reconnects after a brief
# back-off, so allowlisting once survives many force-restart cycles.
#
# Filter: only `*-response.json`. The query file lands a moment
# earlier; emitting on the response means one notification per
# round-trip rather than two.
set -uo pipefail

URL="${CAST_WEB_URL:-http://127.0.0.1:8765}"

while true; do
    curl -sN --connect-timeout 2 "$URL/watcher/log/stream" 2>/dev/null \
        | grep --line-buffered -E '^data: [0-9]+-response\.json$' || true
    # If we get here, the stream ended (likely cast-web restarted).
    # Back off briefly then reconnect.
    sleep 1
done
