#!/usr/bin/env bash
# probe-voluntas.sh — measure peak RSS of cast-extract against a target.
#
# Wraps systemd-run --user so an OOM kill stays inside its own cgroup and
# can't take the parent shell with it. Uses systemd's MemoryPeak accounting
# to report peak RSS without requiring GNU /usr/bin/time. Output is one
# line per run, easy to paste into the curve we're building.
#
# Usage:
#   probe-voluntas.sh [TARGET]
#       TARGET defaults to /home/camilla/git/voluntas/voluntas
#
#   CAP=4G probe-voluntas.sh    # memory cap (default 8G; swap forbidden)

set -eu

TARGET="${1:-/home/camilla/git/voluntas/voluntas}"
CAP="${CAP:-8G}"

# cargo runs from the cast workspace root.
cast_root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$cast_root"

# Best-effort member count from the toplevel Cargo.toml — used as the N
# label for the curve. Counts quoted entries between `members = [` and `]`.
toplevel="$TARGET/Cargo.toml"
if [ -f "$toplevel" ]; then
    n=$(awk '/^members[[:space:]]*=/{f=1} f&&/^]/{f=0} f' "$toplevel" \
        | grep -cE '^[[:space:]]*"' || true)
else
    n="?"
fi

unit="probe-voluntas-$$-$(date +%s)"

start=$(date +%s)
status=0
systemd-run --user --pipe --wait --quiet \
    --unit="$unit" \
    --working-directory="$cast_root" \
    -p MemoryMax="$CAP" -p MemorySwapMax=0 -p MemoryAccounting=yes \
    -- cargo run --release -p cast-extract -- "$TARGET" \
    || status=$?
wall=$(( $(date +%s) - start ))

# Read the unit's MemoryPeak before it's garbage-collected. systemctl
# show prints `[not set]` if accounting was off; we already enabled it.
peak_bytes=$(systemctl --user show "$unit" --property=MemoryPeak --value 2>/dev/null || true)
systemctl --user reset-failed "$unit" 2>/dev/null || true
systemctl --user stop "$unit" 2>/dev/null || true

if [ -n "$peak_bytes" ] && [ "$peak_bytes" != "[not set]" ]; then
    rss_mb=$(( peak_bytes / 1024 / 1024 ))
else
    rss_mb="?"
fi

printf 'N=%s  RSS=%sMB  exit=%d  wall=%ds  cap=%s  target=%s\n' \
    "$n" "$rss_mb" "$status" "$wall" "$CAP" "$TARGET"
