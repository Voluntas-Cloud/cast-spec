#!/usr/bin/env bash
# Stop and restart the three cast-watch instances we keep running for
# the dogfood loop:
#   - /tmp/cast-watch-cast-web.sock — rooted on crates/cast-web (the
#     thing under construction; what the browser at :8765 connects to)
#   - /tmp/cast-watch-cast.sock     — rooted on the whole cast
#     workspace; my own validation watcher for cast-watch / cast-spec
#     edits
#   - /tmp/cast-watch-voluntas.sock — rooted on the voluntas repo
#     (apps/voluntas-core, agents/voluntlet, mobile/rust-core); used
#     when cast-web points there for "show me a richer concept graph"
#
# Run after every `cargo build --release -p cast-watch` so the wire
# protocol the running watchers expose matches the source. Allowlist
# this exact path
# (Bash(/home/camilla/git/voluntas/cast/dev/restart-watchers.sh))
# so the rebuild loop is one command.
set -euo pipefail

BIN="/home/camilla/git/voluntas/cast/target/release/cast-watch"
CAST_REPO="/home/camilla/git/voluntas/cast"
VOLUNTAS_REPO="/home/camilla/git/voluntas/voluntas"

# `pkill -x cast-watch` matches the kernel-truncated comm only — kills
# every cast-watch daemon, never our shell or arbitrary processes that
# happen to mention "cast-watch" in their argline.
pkill -x cast-watch || true
sleep 1

rm -f /tmp/cast-watch-cast-web.sock /tmp/cast-watch-cast.sock /tmp/cast-watch-voluntas.sock

# Each watcher gets its own setsid-detached process so the script
# returns once they're all spawned. Logs go to /tmp/cast-watch-*.out
# for inspection.
setsid -f "$BIN" --no-walk-up \
    --socket /tmp/cast-watch-cast-web.sock \
    "$CAST_REPO/crates/cast-web" \
    >/tmp/cast-watch-cast-web.out 2>&1

setsid -f "$BIN" \
    --socket /tmp/cast-watch-cast.sock \
    "$CAST_REPO" \
    >/tmp/cast-watch-cast.out 2>&1

setsid -f "$BIN" \
    --socket /tmp/cast-watch-voluntas.sock \
    "$VOLUNTAS_REPO/apps/voluntas-core" \
    "$VOLUNTAS_REPO/agents/voluntlet" \
    "$VOLUNTAS_REPO/mobile/rust-core" \
    >/tmp/cast-watch-voluntas.out 2>&1

echo "watchers spawned:"
pgrep -x cast-watch | while read -r pid; do
    args=$(tr '\0' ' ' </proc/"$pid"/cmdline 2>/dev/null || true)
    echo "  pid=$pid  $args"
done
echo
echo "wait for ready: e.g. python3 /tmp/cast-claude/cwq.py <socket> <<< '{\"kind\":\"status\"}'"
