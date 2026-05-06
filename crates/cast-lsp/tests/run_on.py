#!/usr/bin/env python3
"""Spawn cast-lsp on a real workspace, dump every diagnostic it publishes.

Not a test — this is a thin LSP client that drives cast-lsp through one
analysis cycle and prints what fell out, so you can eyeball whether the
server is producing useful output on real code.
"""
import json
import subprocess
import sys
import threading
import time

BIN = "/home/camilla/git/voluntas/cast/crates/cast-lsp/target/release/cast-lsp"


def frame(msg):
    body = json.dumps(msg).encode()
    return f"Content-Length: {len(body)}\r\n\r\n".encode() + body


def read_message(stream):
    headers = {}
    while True:
        line = stream.readline()
        if not line:
            return None
        line = line.decode().rstrip("\r\n")
        if not line:
            break
        k, _, v = line.partition(":")
        headers[k.strip().lower()] = v.strip()
    n = int(headers["content-length"])
    body = stream.read(n)
    return json.loads(body)


def main():
    if len(sys.argv) < 2:
        sys.stderr.write("usage: run_on.py <workspace-root>\n")
        sys.exit(2)
    root = sys.argv[1]

    proc = subprocess.Popen(
        [BIN, "--root", root],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env={"RUST_LOG": "warn", "PATH": "/usr/bin:/bin"},
    )

    received = []
    done = threading.Event()

    def stderr_pump():
        for line in proc.stderr:
            sys.stderr.write(f"[lsp-stderr] {line.decode().rstrip()}\n")

    def reader():
        while not done.is_set():
            msg = read_message(proc.stdout)
            if msg is None:
                return
            received.append(msg)

    threading.Thread(target=stderr_pump, daemon=True).start()
    threading.Thread(target=reader, daemon=True).start()

    proc.stdin.write(frame({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {
            "processId": None,
            "rootUri": f"file://{root}",
            "capabilities": {},
        },
    }))
    proc.stdin.flush()

    init_deadline = time.time() + 30
    while time.time() < init_deadline:
        if any(m.get("id") == 1 and "result" in m for m in received):
            break
        time.sleep(0.05)
    else:
        sys.stderr.write("never saw initialize response\n")
        proc.kill()
        sys.exit(1)

    proc.stdin.write(frame({
        "jsonrpc": "2.0", "method": "initialized", "params": {},
    }))
    proc.stdin.flush()

    sys.stderr.write("waiting up to 600s for analysis to complete…\n")
    deadline = time.time() + 600
    while time.time() < deadline:
        if any(
            m.get("method") == "window/logMessage"
            and "complete" in m.get("params", {}).get("message", "")
            for m in received
        ):
            break
        time.sleep(0.5)

    proc.stdin.write(frame({"jsonrpc": "2.0", "id": 2, "method": "shutdown"}))
    proc.stdin.flush()
    time.sleep(0.5)
    proc.stdin.write(frame({"jsonrpc": "2.0", "method": "exit"}))
    proc.stdin.flush()
    proc.stdin.close()

    done.set()
    try:
        rc = proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        proc.kill()
        rc = proc.wait(timeout=5)

    diag_msgs = [m for m in received if m.get("method") == "textDocument/publishDiagnostics"]

    print(f"\n=== cast-lsp on {root} ===")
    print(f"exit code:          {rc}")
    print(f"diagnostic batches: {len(diag_msgs)}")
    total = 0
    for m in diag_msgs:
        ds = m.get("params", {}).get("diagnostics", [])
        if not ds:
            continue
        uri = m.get("params", {}).get("uri", "?")
        print(f"\n{uri} ({len(ds)} diagnostics)")
        for d in ds:
            sev = {1: "ERROR", 2: "WARN", 3: "INFO", 4: "HINT"}.get(d.get("severity"), "?")
            line = d.get("range", {}).get("start", {}).get("line", "?")
            msg = d.get("message", "").split("\n")[0]
            print(f"  L{line + 1 if isinstance(line, int) else line} [{sev}] {msg}")
            total += 1
    print(f"\ntotal diagnostics: {total}")


if __name__ == "__main__":
    main()
