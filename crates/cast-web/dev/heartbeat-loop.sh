#!/usr/bin/env bash
# Continuously bump /tmp/cast-web-mailbox/meta.json so the cast-web
# UI keeps showing "claude session attached." Default cadence 30s
# (well under the 90s stale threshold the binary uses).
#
# Run in the background while you (the assistant) are present and
# willing to read mail. Stop by killing the bash task.
set -euo pipefail

INTERVAL="${CAST_WEB_HEARTBEAT_INTERVAL:-30}"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

while true; do
    "$HERE/bump-heartbeat.sh" >/dev/null
    sleep "$INTERVAL"
done
