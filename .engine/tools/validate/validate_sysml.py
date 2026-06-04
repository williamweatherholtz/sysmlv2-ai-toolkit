"""
Headless validator for the .engine SysML v2 schema.
Drives the SysML v2 pilot-implementation Jupyter kernel via jupyter_client.

Phase 1: synthetic probes (does the tool allow package reopening across
         cells? do our key constructs parse?).
Phase 2: the real .engine/**/*.sysml files, in dependency order, each as
         its own cell (as-authored).

Run with the sysml env's python:
  C:\\Users\\WilliamWeatherholtz\\miniforge3\\envs\\sysml\\python.exe validate_sysml.py
"""
import os, sys, re
from queue import Empty

ENGINE = r"C:\Users\WilliamWeatherholtz\claude_code\sysmlv2-ai-toolkit\.engine"

# Heuristic markers that indicate a parse/validation problem in kernel output.
ERR_MARKERS = re.compile(
    r"(error|couldn't|could not|wasn't expected|mismatched|no viable|"
    r"unexpected|cannot|duplicate|not resolved|unresolved|missing|"
    r"extraneous|required)", re.IGNORECASE)


def run_cell(kc, code, timeout=180):
    msg_id = kc.execute(code, allow_stdin=False)
    status = "unknown"
    # shell reply -> final status
    while True:
        try:
            r = kc.get_shell_msg(timeout=timeout)
        except Empty:
            status = "timeout"
            break
        if r["parent_header"].get("msg_id") == msg_id:
            status = r["content"].get("status", "unknown")
            break
    # drain iopub until idle
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
            outs.append((c.get("name", "stdout"), c.get("text", "")))
        elif t == "error":
            outs.append(("error", c.get("evalue", "") + "\n" +
                         "\n".join(c.get("traceback", []))))
        elif t in ("execute_result", "display_data"):
            d = c.get("data", {}).get("text/plain", "")
            outs.append((t, str(d)))
    return status, outs


def classify(status, outs):
    text = "\n".join(t for _, t in outs)
    has_err_channel = any(ch == "error" for ch, _ in outs)
    looks_err = bool(ERR_MARKERS.search(text))
    ok = (status == "ok") and not has_err_channel and not looks_err
    return ok, text


def banner(s):
    print("\n" + "=" * 72)
    print(s)
    print("=" * 72)


def main():
    from jupyter_client.kernelspec import KernelSpecManager
    from jupyter_client.manager import start_new_kernel

    specs = KernelSpecManager().find_kernel_specs()
    print("Available kernelspecs:", list(specs.keys()))
    name = "sysml" if "sysml" in specs else None
    if not name:
        print("ERROR: no 'sysml' kernelspec found. Aborting.")
        sys.exit(2)

    print("Starting kernel (JVM startup can take ~20s)...")
    km, kc = start_new_kernel(kernel_name=name)
    print("Kernel up.")

    results = []

    # ---- Phase 1: probes ------------------------------------------------
    probes = [
        ("probe:simple-package", "package P1 { part def A; }"),
        ("probe:reopen-A", "package PR { part def X; }"),
        ("probe:reopen-B", "package PR { part def Y; }"),
        ("probe:import-ref",
         "package PU { private import PR::*; part x : X; }"),
        ("probe:metadata-def",
         "package PM { metadata def Tracked { attribute id : String; } }"),
        ("probe:dependency-def",
         "package PD { dependency def Supersede; }"),
        ("probe:requirement-spec",
         "package PRq { requirement def R; requirement def S :> R; }"),
        ("probe:redefine",
         "package PRd { part def Base { attribute k : String; } "
         "part i : Base { :>> k = \"v\"; } }"),
        ("probe:multiplicity",
         "package PMul { part def Z { attribute xs : String[*]; } }"),
        ("probe:numeric-bool",
         "package PNb { part def N { attribute a : Integer; "
         "attribute b : Real; attribute c : Boolean; } }"),
    ]
    banner("PHASE 1: PROBES")
    for nm, code in probes:
        st, outs = run_cell(kc, code)
        ok, text = classify(st, outs)
        results.append((nm, ok, st, text))
        print(f"[{'PASS' if ok else 'FAIL'}] {nm}  (status={st})")
        if text.strip():
            print("    " + text.strip().replace("\n", "\n    ")[:1500])

    # ---- Phase 2: real files -------------------------------------------
    order = [
        "schema/core/element.sysml",
        "schema/core/relationships.sysml",
        "schema/core/requirements.sysml",
        "schema/core/workflow.sysml",
        "schema/core/process.sysml",
        "schema/core/work.sysml",
        "schema/core/verification.sysml",
        "schema/core/risk.sysml",
        "schema/core/skills.sysml",
        "schema/safety/stpa.sysml",
        "processes/agile-workflow.sysml",
        "processes/definition-of-ready.sysml",
        "processes/definition-of-done.sysml",
        "skills/skills-registry.sysml",
    ] + [f"decisions/{f}" for f in sorted(os.listdir(os.path.join(ENGINE, "decisions")))]

    banner("PHASE 2: REAL FILES (as-authored, dependency order)")
    for rel in order:
        path = os.path.join(ENGINE, rel)
        with open(path, "r", encoding="utf-8") as fh:
            code = fh.read()
        st, outs = run_cell(kc, code)
        ok, text = classify(st, outs)
        results.append((rel, ok, st, text))
        print(f"[{'PASS' if ok else 'FAIL'}] {rel}  (status={st})")
        if text.strip():
            print("    " + text.strip().replace("\n", "\n    ")[:2000])

    # ---- Summary --------------------------------------------------------
    banner("SUMMARY")
    passed = sum(1 for _, ok, _, _ in results if ok)
    print(f"{passed}/{len(results)} cells passed")
    fails = [n for n, ok, _, _ in results if not ok]
    if fails:
        print("FAILURES:")
        for f in fails:
            print("  - " + f)

    kc.stop_channels()
    km.shutdown_kernel(now=True)


if __name__ == "__main__":
    main()
