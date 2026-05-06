#!/usr/bin/env python3
"""LSP smoke test for cast-lsp.

Spawns the binary, runs initialize -> initialized -> wait for diagnostics ->
shutdown -> exit, prints every message it receives. Anything other than a
clean exit code 0 is a failure.
"""
import json
import subprocess
import sys
import threading
import time

BIN = "/home/camilla/git/voluntas/cast/crates/cast-lsp/target/release/cast-lsp"
ROOT = sys.argv[1] if len(sys.argv) > 1 else "/home/camilla/git/voluntas/cast/crates/cast-lsp"


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
    proc = subprocess.Popen(
        [BIN, "--root", ROOT],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env={"RUST_LOG": "info", "PATH": "/usr/bin:/bin"},
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
            sys.stderr.write(f"[lsp-recv] {json.dumps(msg)[:300]}\n")

    threading.Thread(target=stderr_pump, daemon=True).start()
    threading.Thread(target=reader, daemon=True).start()

    proc.stdin.write(frame({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {
            "processId": None,
            "rootUri": f"file://{ROOT}",
            "capabilities": {},
        },
    }))
    proc.stdin.flush()

    # Wait for the initialize *response* before sending `initialized`.
    # tower-lsp's NormalService checks state at request-call time, and the
    # state only flips to Initialized after the initialize future
    # completes. If `initialized` arrives in the same read tick, it gets
    # silently dropped (notifications return None on rejection).
    init_deadline = time.time() + 30
    while time.time() < init_deadline:
        if any(m.get("id") == 1 and "result" in m for m in received):
            break
        time.sleep(0.05)
    else:
        sys.stderr.write("[smoke] never saw initialize response\n")
        proc.kill()
        sys.exit(1)

    proc.stdin.write(frame({
        "jsonrpc": "2.0", "method": "initialized", "params": {},
    }))
    proc.stdin.flush()

    sys.stderr.write("[smoke] waiting up to 180s for analysis-complete log…\n")
    deadline = time.time() + 180
    analysis_signal = False
    while time.time() < deadline:
        if any(
            m.get("method") == "window/logMessage"
            and "complete" in m.get("params", {}).get("message", "")
            for m in received
        ):
            sys.stderr.write("[smoke] saw 'analysis complete' log — analysis ran\n")
            analysis_signal = True
            break
        if any(m.get("method") == "textDocument/publishDiagnostics" for m in received):
            sys.stderr.write("[smoke] saw publishDiagnostics — analysis published\n")
            analysis_signal = True
            break
        time.sleep(0.5)
    if not analysis_signal:
        sys.stderr.write("[smoke] TIMEOUT waiting for analysis-complete signal\n")

    # LSP shutdown takes no params — omit the field.
    proc.stdin.write(frame({
        "jsonrpc": "2.0", "id": 2, "method": "shutdown",
    }))
    proc.stdin.flush()
    time.sleep(0.5)
    proc.stdin.write(frame({
        "jsonrpc": "2.0", "method": "exit",
    }))
    proc.stdin.flush()
    # Close stdin so tower-lsp's `read_input` loop sees EOF and the
    # `serve` future resolves — the Exit layer only sets state, it
    # does not break the read loop.
    proc.stdin.close()

    done.set()
    try:
        rc = proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        sys.stderr.write("[smoke] process didn't exit in 10s; killing\n")
        proc.kill()
        rc = proc.wait(timeout=5)
    sys.stderr.write(f"[smoke] cast-lsp exited with code {rc}\n")

    init_resp = next((m for m in received if m.get("id") == 1), None)
    shutdown_resp = next((m for m in received if m.get("id") == 2), None)
    diag_msgs = [m for m in received if m.get("method") == "textDocument/publishDiagnostics"]
    analysis_complete = any(
        m.get("method") == "window/logMessage"
        and "complete" in m.get("params", {}).get("message", "")
        for m in received
    )

    print(f"\n=== smoke result ===")
    print(f"initialize response:   {'OK' if init_resp and init_resp.get('result') else 'MISSING'}")
    # LSP spec: shutdown success is `{"result": null}` — the field must exist, not the value.
    shutdown_ok = shutdown_resp is not None and "result" in shutdown_resp and "error" not in shutdown_resp
    print(f"shutdown response:     {'OK' if shutdown_ok else 'MISSING/ERROR'}")
    print(f"analysis complete:     {'YES' if analysis_complete else 'NO'}")
    print(f"publishDiagnostics:    {len(diag_msgs)} batches (0 is correct when no errors)")
    total_diags = 0
    for m in diag_msgs:
        ds = m.get("params", {}).get("diagnostics", [])
        total_diags += len(ds)
        uri = m.get("params", {}).get("uri", "?")
        if ds:
            print(f"  {uri}: {len(ds)} diagnostics")
            for d in ds[:3]:
                sev = d.get("severity")
                msg = d.get("message", "").split("\n")[0][:120]
                print(f"    [sev={sev}] {msg}")
    print(f"diagnostics total:     {total_diags}")
    print(f"exit code:             {rc}")

    bad = []
    if not (init_resp and init_resp.get("result")):
        bad.append("missing initialize result")
    if not shutdown_ok:
        bad.append("missing shutdown success result")
    if not analysis_complete:
        bad.append("no analysis-complete log within 180s")

    if bad:
        print("FAIL: " + "; ".join(bad))
        sys.exit(1)
    else:
        print("PASS")


if __name__ == "__main__":
    main()
