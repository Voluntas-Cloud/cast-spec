#!/usr/bin/env bash
# /cast-spec session bootstrap. ONE script the assistant runs at the
# start of every cast-spec session.
#
# Boots a per-workspace cast-watch + cast-web pair against the target
# directory (default: $PWD). All resources are namespaced by
# `basename($WORKSPACE)` so two checkouts with the same basename
# collide — that's intentional, surfaces a stale watcher.
#
#   1. ensure-cast-web.sh <workspace>  - boot or attach to the loop
#      (watcher under systemd-run --user with MemoryMax cap)
#   2. setsid heartbeat-loop.sh        - so the cast-web UI keeps
#      showing "Claude session attached" against this workspace's
#      mailbox (any prior heartbeat for a different workspace is
#      killed first to avoid two loops bumping different meta.json's)
#   3. cwq.sh status                   - poll until phase=ready
#   4. cwq.sh manual                   - emit the live grammar
#
# Usage:
#   crates/cast-web/dev/cast-spec.sh                          # workspace = $PWD
#   crates/cast-web/dev/cast-spec.sh /path/to/repo            # explicit
#   crates/cast-web/dev/cast-spec.sh --force-restart [/path]  # forwarded to ensure-cast-web.sh
#
# Output to stdout: status JSON line, a `----` separator, then the
# manual JSON line. stderr carries human-readable progress.
#
# The only piece the assistant still does separately is arm
# monitor-mailbox.sh as a Claude Code Monitor — Monitor is a tool,
# not a process, so it can't live in shell.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Forward unrecognised flags (e.g. --force-restart) verbatim to
# ensure-cast-web.sh; collect positional args separately so we can
# still take WORKSPACE as `$1` of the positional list.
FLAGS=()
POSITIONAL=()
while [[ $# -gt 0 ]]; do
    case "$1" in
    -*)
        FLAGS+=("$1")
        shift
        ;;
    *)
        POSITIONAL+=("$1")
        shift
        ;;
    esac
done

WORKSPACE_RAW="${POSITIONAL[0]:-$PWD}"
if [[ ! -d "$WORKSPACE_RAW" ]]; then
    echo "workspace not found: $WORKSPACE_RAW" >&2
    exit 1
fi
WORKSPACE="$(cd "$WORKSPACE_RAW" && pwd)"
NAME="$(basename "$WORKSPACE")"

# Per-workspace defaults — exported so heartbeat-loop.sh /
# bump-heartbeat.sh write to the right mailbox.
export CAST_WEB_SOCKET="${CAST_WEB_SOCKET:-/tmp/cast-watch-${NAME}.sock}"
export CAST_WEB_MAILBOX="${CAST_WEB_MAILBOX:-/tmp/cast-web-mailbox-${NAME}}"
export CAST_WEB_WATCHER_LOG="${CAST_WEB_WATCHER_LOG:-/tmp/cast-web-watcher-log-${NAME}}"
export CAST_WATCH_UNIT="${CAST_WATCH_UNIT:-cast-watch-${NAME}}"
export CAST_WATCH_MEMORY_MAX="${CAST_WATCH_MEMORY_MAX:-8G}"

echo "cast-spec workspace: $WORKSPACE  (name: $NAME)" >&2

"$HERE/ensure-cast-web.sh" "${FLAGS[@]}" "$WORKSPACE" >&2

# Heartbeat loop: kill any prior loop first so we don't end up with
# two competing bumpers writing to different mailboxes when the
# operator switches workspaces. Then re-arm under setsid -f so the
# cast-web UI keeps seeing us between sessions.
pkill -f 'heartbeat-loop\.sh' 2>/dev/null || true
sleep 0.2
setsid -f env CAST_WEB_MAILBOX="$CAST_WEB_MAILBOX" \
    "$HERE/heartbeat-loop.sh" </dev/null >/tmp/cast-web-heartbeat.out 2>&1
echo "heartbeat-loop spawned (mailbox: $CAST_WEB_MAILBOX)" >&2

# Poll status until ready. Cold RA load: 10-30s on a small workspace,
# minutes on voluntas-sized trees.
status=""
for _ in $(seq 1 240); do
    status="$("$HERE/cwq.sh" '{"kind":"status"}' 2>/dev/null || true)"
    [[ "$status" == *'"phase":"ready"'* ]] && break
    sleep 0.5
done

if [[ "$status" != *'"phase":"ready"'* ]]; then
    echo "watcher did not reach phase=ready within 120s" >&2
    echo "last status: $status" >&2
    exit 1
fi

echo "$status"
echo '----'
"$HERE/cwq.sh" '{"kind":"manual"}'
