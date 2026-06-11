"""
Validate the .engine/workflows/*.sysml process-as-data files against the
SysML v2 pilot kernel, in dependency order (meta-model first, then each
concrete workflow). A file FAILS iff the kernel emits an error.

Run (sandbox disabled; kernel calls bare `java`, so go through conda run):
  conda run -n sysml --no-capture-output python .engine/tools/validate/validate_workflows.py
"""
import os
import sys
import re
import subprocess
from queue import Empty

# .engine/tools/validate/validate_workflows.py -> .engine
ENGINE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
WF = os.path.join(ENGINE, "workflows")

# Workflows are typed by schema/core (CR-2), so the schema is PRELOADED silently
# (validated by validate_schema; here it just makes imports resolve).
SCHEMA_PRELOAD = [os.path.join(ENGINE, *rel.split("/")) for rel in (
    "schema/core/element.sysml", "schema/core/needs.sysml", "schema/core/requirements.sysml",
    "schema/core/verification.sysml", "schema/core/work.sysml", "schema/core/architecture.sysml",
    "schema/core/computed.sysml", "schema/core/relationships.sysml", "schema/core/workflow.sysml",
    "schema/core/process.sysml", "schema/core/skills.sysml", "schema/core/risk.sysml",
    "schema/safety/stpa.sysml",
)]

# Dependency order: meta-model first, then workflows that import it.
ORDER = [
    "_meta.sysml",
    "business.sysml",
    "architecture.sysml",
    "delivery.sysml",
    "deploy.sysml",
    "operate.sysml",
    "change-request.sysml",
]

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

    for p in SCHEMA_PRELOAD:
        with open(p, "r", encoding="utf-8") as fh:
            run_cell(kc, fh.read())

    results = []
    for fn in ORDER:
        path = os.path.join(WF, fn)
        with open(path, "r", encoding="utf-8") as fh:
            code = fh.read()
        st, text = run_cell(kc, code)
        passed = ok(st, text)
        results.append((fn, passed))
        print(f"[{'PASS' if passed else 'FAIL'}] {fn}  (status={st})")
        if text.strip():
            print("    " + text.strip().replace("\n", "\n    ")[:2000])
        print()

    print("=" * 64)
    print("WORKFLOW VALIDATION SUMMARY")
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
