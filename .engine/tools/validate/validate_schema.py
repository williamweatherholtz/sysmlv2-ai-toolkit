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
from queue import Empty

# .engine/tools/validate/validate_schema.py -> .engine
ENGINE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# (relative-path, ) in dependency order. EngineElement first; siblings import it.
ORDER = [
    "schema/core/element.sysml",
    "schema/core/needs.sysml",
    "schema/core/requirements.sysml",
    "schema/core/verification.sysml",
    "schema/core/work.sysml",
    "schema/core/architecture.sysml",
    "schema/core/relationships.sysml",
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
    km, kc = start_new_kernel(kernel_name="sysml")
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

    kc.stop_channels()
    km.shutdown_kernel(now=True)


if __name__ == "__main__":
    main()
