#!/usr/bin/env bash
# One-shot synchronous query to the cast-watch socket via cast-web's
# `POST /watcher/query` endpoint. Replaces /tmp/cast-claude/cwq.py
# with a repo-rooted command line.
#
# Usage:
#   dev/cwq.sh '{"kind":"manual"}'
#   dev/cwq.sh '{"kind":"query","from":"concepts","dim":"name_only"}'
#   echo '{"kind":"status"}' | dev/cwq.sh
#
# The cast-web process must be running (default :8765); set
# CAST_WEB_URL to point at a different host:port. Output is the
# watcher's response as JSON.
set -euo pipefail

URL="${CAST_WEB_URL:-http://127.0.0.1:8765}"

if [[ $# -ge 1 ]]; then
    BODY="$1"
else
    BODY="$(cat)"
fi

curl -sS --fail-with-body \
    -X POST "$URL/watcher/query" \
    -H 'content-type: application/json' \
    --data-binary "$BODY"
echo
