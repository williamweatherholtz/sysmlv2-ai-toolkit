"""
Validate the rewritten .engine/schema core (flat Engine<Concern> packages)
against the SysML v2 pilot kernel, in dependency order. Grows as concerns are
added. A file FAILS iff the kernel emits an error.

Run (sandbox disabled; kernel calls bare `java`, so go through conda run):
  conda run -n sysml --no-capture-output python .engine/tools/validate/validate_schema.py
"""
import os
import sys
import re
import subprocess
from queue import Empty
import _schema_files

# .engine/tools/validate/validate_schema.py -> .engine
ENGINE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# ORDER is the canonical dependency sequence — defined once in _schema_files.py.
ORDER = _schema_files.SCHEMA_ORDER

ERR = re.compile(
    r"(error|couldn't|could not|wasn't expected|mismatched|no viable|"
    r"unexpected|cannot|duplicate|not resolved|unresolved|missing|"
    r"extraneous|required)", re.IGNORECASE)


def run_cell(kc, code, timeout=180):
    msg_id = kc.execute(code, allow_stdin=False)
    status = "unknown"
    while True:
        try:
            r = kc.get_shell_msg(timeout=timeout)
        except Empty:
            status = "timeout"
            break
        if r["parent_header"].get("msg_id") == msg_id:
            status = r["content"].get("status", "unknown")
            break
    outs = []
    while True:
        try:
            m = kc.get_iopub_msg(timeout=timeout)
        except Empty:
            break
        if m["parent_header"].get("msg_id") != msg_id:
            continue
        t = m["header"]["msg_type"]
        c = m["content"]
        if t == "status" and c.get("execution_state") == "idle":
            break
        elif t == "stream":
            outs.append(c.get("text", ""))
        elif t == "error":
            outs.append(c.get("evalue", "") + "\n" + "\n".join(c.get("traceback", [])))
        elif t in ("execute_result", "display_data"):
            outs.append(str(c.get("data", {}).get("text/plain", "")))
    return status, "\n".join(outs)


def ok(status, text):
    return status == "ok" and not ERR.search(text or "")


def main():
    # Coverage check before the expensive kernel starts: fail hard if any
    # schema/*.sysml is unregistered in _schema_files.SCHEMA_ORDER.
    missing = _schema_files.check_coverage(ENGINE)
    if missing:
        for rel in missing:
            print(f"ERROR: schema file not registered in _schema_files.SCHEMA_ORDER: {rel}")
        print("Add the file to _schema_files.py SCHEMA_ORDER in dependency order, then re-run.")
        sys.exit(2)

    from jupyter_client.kernelspec import KernelSpecManager
    from jupyter_client.manager import start_new_kernel

    if "sysml" not in KernelSpecManager().find_kernel_specs():
        print("ERROR: no 'sysml' kernelspec found.")
        sys.exit(2)

    print("Starting kernel (JVM startup ~20s)...")
    # Send the kernel's stdout/stderr to DEVNULL: results come over ZMQ (iopub),
    # so we lose nothing — but the JVM no longer inherits our stdout pipe, which
    # is what made the shell hang after Python exited (and it kills the noise).
    km, kc = start_new_kernel(kernel_name="sysml",
                              stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    print("Kernel up.\n")

    results = []
    for rel in ORDER:
        with open(os.path.join(ENGINE, rel), "r", encoding="utf-8") as fh:
            code = fh.read()
        st, text = run_cell(kc, code)
        passed = ok(st, text)
        results.append((rel, passed))
        print(f"[{'PASS' if passed else 'FAIL'}] {rel}  (status={st})")
        if text.strip():
            print("    " + text.strip().replace("\n", "\n    ")[:2000])
        print()

    print("=" * 64)
    print("SCHEMA VALIDATION SUMMARY")
    for n, p in results:
        print(f"  {'PASS' if p else 'FAIL'}  {n}")
    print(f"  {sum(1 for _, p in results if p)}/{len(results)} files passed")

    sys.stdout.flush()
    code = 0 if all(p for _, p in results) else 1
    # Kill the kernel's JVM so it releases the stdout pipe — otherwise conda run
    # (and the shell) never sees EOF and hangs, even after Python exits. In
    # jupyter_client 8.x the process lives under km.provisioner (km.kernel was
    # removed); km.provisioner.pid is the JVM pid; os.kill is synchronous
    # (TerminateProcess on Windows). km.shutdown_kernel() is avoided — it BLOCKS.
    # Then hard-exit (bypasses lingering non-daemon threads).
    try:
        import signal
        os.kill(km.provisioner.pid, signal.SIGTERM)
    except Exception:
        pass
    os._exit(code)


if __name__ == "__main__":
    main()
