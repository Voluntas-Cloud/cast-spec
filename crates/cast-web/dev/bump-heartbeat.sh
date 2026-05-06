#!/usr/bin/env bash
# Write the current UTC timestamp to /tmp/cast-web-mailbox/meta.json's
# `last_heartbeat_at`. The mailbox protocol uses this to decide whether
# a Claude session is currently attached (stale > 90s ⇒ detached).
#
# Standalone — run once when you want the UI to flip to "attached"
# right now. For a continuous heartbeat use dev/heartbeat-loop.sh.
set -euo pipefail

DIR="${CAST_WEB_MAILBOX:-/tmp/cast-web-mailbox}"
mkdir -p "$DIR"

python3 - "$DIR/meta.json" <<'PY'
import json, sys
from datetime import datetime, timezone

path = sys.argv[1]
try:
    meta = json.load(open(path))
except Exception:
    meta = {"session_id": "claude-code-cast-web"}
meta["last_heartbeat_at"] = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
meta.setdefault("started_at", meta["last_heartbeat_at"])
json.dump(meta, open(path, "w"), indent=2)
print(meta["last_heartbeat_at"])
PY
