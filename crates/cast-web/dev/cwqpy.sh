#!/usr/bin/env bash
# cwq.sh + python3 post-processing in one stable command shape.
#
# Every call has the same shape — `cwqpy.sh <query> <program>` — so a
# permission grant against this script as a prefix lets every
# subsequent post-processed query through without a fresh prompt.
#
# Usage:
#   dev/cwqpy.sh '{"kind":"status"}' '
#       import json, sys
#       d = json.loads(sys.stdin.read())["body"]
#       print(json.dumps(d.get("summary"), indent=2))
#   '
#
# The first argument is sent verbatim to dev/cwq.sh. The second
# argument is fed to `python3 -c` and receives the watcher response on
# stdin. Anything the program prints is the script's stdout.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ $# -lt 2 ]]; then
    cat >&2 <<'USAGE'
usage: cwqpy.sh '<json-query>' '<python-program>'
       cwqpy.sh '<json-query>' @path/to/program.py     # @-prefix reads from a file

The python program receives the watcher's JSON response on stdin.
USAGE
    exit 2
fi

QUERY="$1"
PROG="$2"

if [[ "${PROG:0:1}" == "@" ]]; then
    PROG_FILE="${PROG:1}"
    if [[ ! -f "$PROG_FILE" ]]; then
        echo "cwqpy.sh: program file not found: $PROG_FILE" >&2
        exit 2
    fi
    "$HERE/cwq.sh" "$QUERY" | python3 "$PROG_FILE"
else
    "$HERE/cwq.sh" "$QUERY" | python3 -c "$PROG"
fi
