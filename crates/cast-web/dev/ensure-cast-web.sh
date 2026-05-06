#!/usr/bin/env bash
# Idempotent "is the dogfood loop up?" check + boot, parameterised on
# the target workspace.
#
#   1. If cast-web responds at $CAST_WEB_URL AND `{"kind":"status"}`
#      relays through to the watcher socket → exit 0 with a status line.
#   2. If cast-web is up but the watcher behind it is unreachable →
#      exit 2 with a diagnostic (don't silently restart someone else's
#      cast-web; suggest --force-restart to reset).
#   3. If cast-web is down → make sure cast-watch is running on the
#      target workspace (under a systemd-run --user cgroup so an OOM
#      stays inside its boundary), then start cast-web in front of it.
#
# Never kills a running cast-web or cast-watch. Designed to be safe to
# run at the start of every assistant session.
#
# Usage:
#   dev/ensure-cast-web.sh                          # default workspace = parent cast workspace
#   dev/ensure-cast-web.sh /path/to/workspace       # explicit
#   dev/ensure-cast-web.sh --force-restart          # kill cast-web + cast-watch unit + socket, then boot fresh
#   dev/ensure-cast-web.sh --force-restart /path    # ditto, with explicit workspace
#   dev/ensure-cast-web.sh --continue [/path]       # attach to an existing watcher (succeed if up)
#
# Default policy when something is already running: REFUSE — the
# caller probably believes they're booting fresh. They have to opt
# into one of:
#   --force-restart   rebuild and replace what's there
#   --continue        attach to what's there as-is (no rebuild)
#
# `--force-restart` is the "I edited cast-spec / cast-watch / cast-web
# source and need the daemons to pick it up" path. It does NOT touch
# the mailbox or watcher-log directories — those survive.
#
# `--continue` is the "I just want to talk to whatever is already up
# for this workspace" path. Useful at session start when the daemon
# has been running across editor restarts.
#
# Per-workspace naming. Everything is keyed off `basename($WORKSPACE)`
# (collisions are intentional — they surface a stale watcher rooted on
# a different checkout with the same basename):
#
#   socket:      /tmp/cast-watch-<NAME>.sock
#   unit:        cast-watch-<NAME>.service       (systemd-run --user)
#   mailbox:     /tmp/cast-web-mailbox-<NAME>
#   watcher-log: /tmp/cast-web-watcher-log-<NAME>
#
# Each can be overridden via the matching env var if you want to
# co-locate two workspaces under one set of directories.
#
# Env:
#   CAST_WEB_URL          - http URL to check / curl  (default http://127.0.0.1:8765)
#   CAST_WEB_HTTP         - bind address for cast-web (default 127.0.0.1:8765)
#   CAST_WEB_SOCKET       - cast-watch socket path    (default /tmp/cast-watch-<NAME>.sock)
#   CAST_WEB_MAILBOX      - cast-web mailbox dir      (default /tmp/cast-web-mailbox-<NAME>)
#   CAST_WEB_WATCHER_LOG  - cast-web watcher-log dir  (default /tmp/cast-web-watcher-log-<NAME>)
#   CAST_WATCH_UNIT       - systemd --user unit name  (default cast-watch-<NAME>)
#   CAST_WATCH_MEMORY_MAX - cgroup MemoryMax for the watcher unit (default 8G)
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)" # crates/cast-web
WORKSPACE_ROOT="$(cd "$HERE/../.." && pwd)"             # parent cast workspace

URL="${CAST_WEB_URL:-http://127.0.0.1:8765}"
HTTP_BIND="${CAST_WEB_HTTP:-127.0.0.1:8765}"

FORCE_RESTART=0
CONTINUE=0
POSITIONAL=()
while [[ $# -gt 0 ]]; do
  case "$1" in
  --force-restart)
    FORCE_RESTART=1
    shift
    ;;
  --continue)
    CONTINUE=1
    shift
    ;;
  --)
    shift
    while [[ $# -gt 0 ]]; do
      POSITIONAL+=("$1")
      shift
    done
    ;;
  -*)
    echo "unknown option: $1" >&2
    exit 1
    ;;
  *)
    POSITIONAL+=("$1")
    shift
    ;;
  esac
done

if ((FORCE_RESTART && CONTINUE)); then
  echo "--force-restart and --continue are mutually exclusive" >&2
  exit 1
fi

# Workspace = first positional, or default to the parent cast workspace
# (the dogfood target). Absolutise so basename works on a clean string.
WORKSPACE_RAW="${POSITIONAL[0]:-$WORKSPACE_ROOT}"
if [[ ! -d "$WORKSPACE_RAW" ]]; then
  echo "workspace not found: $WORKSPACE_RAW" >&2
  exit 1
fi
WORKSPACE="$(cd "$WORKSPACE_RAW" && pwd)"
NAME="$(basename "$WORKSPACE")"

# Per-workspace defaults. Each can be overridden by env.
SOCKET="${CAST_WEB_SOCKET:-/tmp/cast-watch-${NAME}.sock}"
MAILBOX="${CAST_WEB_MAILBOX:-/tmp/cast-web-mailbox-${NAME}}"
WATCHER_LOG="${CAST_WEB_WATCHER_LOG:-/tmp/cast-web-watcher-log-${NAME}}"
UNIT="${CAST_WATCH_UNIT:-cast-watch-${NAME}}"
MEM_MAX="${CAST_WATCH_MEMORY_MAX:-8G}"

stop_watch_unit() {
  # Stop the per-workspace systemd unit if it's around. `systemctl
  # stop` on an unknown unit is silent; reset-failed clears any
  # leftover failed-state bookkeeping so the next start can reuse
  # the unit name.
  systemctl --user stop "$UNIT" >/dev/null 2>&1 || true
  systemctl --user reset-failed "$UNIT" >/dev/null 2>&1 || true
}

if ((FORCE_RESTART)); then
  echo "force-restart: stopping $UNIT, killing cast-web, removing $SOCKET" >&2
  # Watcher: prefer systemctl stop so we kill only the per-workspace
  # unit, not every cast-watch on the host (avoids reaping siblings
  # when multiple workspaces are loaded).
  stop_watch_unit
  # Cast-web: pkill -x because it's not under systemd-run today —
  # `-x` matches `comm` exactly so dev shells / editors that mention
  # cast-web in their argv are not affected.
  pkill -x cast-web || true
  rm -f "$SOCKET"
  # Give the kernel a moment to release the listening port + socket
  # path before the boot path tries to rebind them.
  sleep 1
fi

cast_web_alive() {
  curl -sS --max-time 2 -o /dev/null "$URL/" 2>/dev/null
}

# Returns 0 if cast-web's /watcher/query relays successfully to the
# cast-watch socket. Anything else (HTTP error, JSON `error` body) → 1.
watcher_reachable() {
  local body
  body="$(curl -sS --max-time 3 -X POST "$URL/watcher/query" \
    -H 'content-type: application/json' \
    --data-binary '{"kind":"status"}' 2>/dev/null)" || return 1
  [[ -n "$body" && "$body" != *'"error"'* ]]
}

# ---- Step 1: already up? ----------------------------------------------------
# When something's already running we refuse to silently no-op unless
# the caller explicitly opted in via --continue.
if cast_web_alive; then
  if watcher_reachable; then
    if ((CONTINUE)); then
      echo "cast-web up at $URL  (pid $(pgrep -x cast-web | head -n1))  socket: $SOCKET  unit: $UNIT"
      exit 0
    fi
    echo "cast-web is already running at $URL  (pid $(pgrep -x cast-web | head -n1))  socket: $SOCKET  unit: $UNIT" >&2
    echo "Pass --continue to attach to it, or --force-restart to rebuild and restart." >&2
    exit 1
  fi
  echo "cast-web is up at $URL but its watcher socket is unreachable." >&2
  echo "Pass --force-restart to reset, or set CAST_WEB_SOCKET and re-run." >&2
  exit 2
fi

# ---- Step 2: start cast-watch if its socket is missing ---------------------
need_start_watch=1
if [[ -S "$SOCKET" ]] && systemctl --user is-active "$UNIT" >/dev/null 2>&1; then
  need_start_watch=0
fi

if ((need_start_watch)); then
  echo "building cast-watch (release)…"
  (cd "$WORKSPACE_ROOT" && cargo build --release -p cast-watch >&2)

  BIN="$WORKSPACE_ROOT/target/release/cast-watch"
  if [[ ! -x "$BIN" ]]; then
    echo "cast-watch binary not found at $BIN" >&2
    exit 1
  fi

  # ROOTS handling. For the cast dogfood loop (workspace == cast
  # itself) load cast-web + cast-lsp as peer RA DBs — they're
  # standalone members the parent workspace excludes, so without
  # this their `cast::*!` blocks are invisible to the analyzer.
  # For any other workspace, single-root: pass it through and let
  # rust-analyzer's loader do its thing.
  ROOTS=("$WORKSPACE")
  if [[ "$WORKSPACE" == "$WORKSPACE_ROOT" ]]; then
    ROOTS+=("$WORKSPACE_ROOT/crates/cast-web" "$WORKSPACE_ROOT/crates/cast-lsp")
  fi

  echo "starting cast-watch on ${ROOTS[*]} → $SOCKET (unit $UNIT, MemoryMax=$MEM_MAX)"
  rm -f "$SOCKET"
  # Clear any leftover unit state before re-registering. systemd-run
  # refuses to reuse a unit name that's still in failed/listed state.
  stop_watch_unit
  systemd-run --user --unit="$UNIT" \
    --description="cast-watch on $WORKSPACE" \
    -p MemoryMax="$MEM_MAX" \
    -p MemorySwapMax=0 \
    -p MemoryAccounting=yes \
    "$BIN" --socket "$SOCKET" "${ROOTS[@]}" \
    >/dev/null

  for _ in $(seq 1 30); do
    [[ -S "$SOCKET" ]] && break
    sleep 0.5
  done
  if [[ ! -S "$SOCKET" ]]; then
    echo "cast-watch did not bind $SOCKET within 15s — see: journalctl --user -u $UNIT" >&2
    exit 1
  fi
fi

# ---- Step 3: start cast-web ------------------------------------------------
echo "building cast-web (release)…"
# cast-web is a member of the parent workspace ($WORKSPACE_ROOT/Cargo.toml),
# so cargo writes the binary to $WORKSPACE_ROOT/target/release/, not
# $HERE/target/release/. Build via -p from the workspace root and read
# the binary from the same place — otherwise we silently launch a
# stale per-crate binary if one happens to exist.
(cd "$WORKSPACE_ROOT" && cargo build --release -p cast-web >&2)

CAST_WEB_BIN="$WORKSPACE_ROOT/target/release/cast-web"
if [[ ! -x "$CAST_WEB_BIN" ]]; then
  echo "cast-web binary not found at $CAST_WEB_BIN" >&2
  exit 1
fi

echo "starting cast-web at $URL → $SOCKET (mailbox $MAILBOX)"
mkdir -p "$MAILBOX" "$WATCHER_LOG"
setsid -f "$CAST_WEB_BIN" \
  --socket "$SOCKET" \
  --http "$HTTP_BIND" \
  --mailbox-dir "$MAILBOX" \
  --watcher-log-dir "$WATCHER_LOG" \
  >/tmp/cast-web.out 2>&1

for _ in $(seq 1 20); do
  if cast_web_alive; then
    echo "cast-web up — pid $(pgrep -x cast-web | head -n1)  $URL  socket: $SOCKET  unit: $UNIT"
    exit 0
  fi
  sleep 0.5
done

echo "cast-web did not bind within 10s — see /tmp/cast-web.out" >&2
exit 1
