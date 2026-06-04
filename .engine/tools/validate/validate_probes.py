"""
Focused SysML v2 probe: confirm the exact patterns before rewriting the schema.
Classifier: a cell FAILS iff the kernel emits a line containing 'ERROR:'.
Run: conda run -n sysml --no-capture-output python validate_probes.py
"""
import sys
from queue import Empty
from jupyter_client.manager import start_new_kernel


def run_cell(kc, code, timeout=120):
    msg_id = kc.execute(code, allow_stdin=False)
    while True:
        r = kc.get_shell_msg(timeout=timeout)
        if r["parent_header"].get("msg_id") == msg_id:
            break
    outs = []
    while True:
        try:
            m = kc.get_iopub_msg(timeout=timeout)
        except Empty:
            break
        if m["parent_header"].get("msg_id") != msg_id:
            continue
        t = m["header"]["msg_type"]; c = m["content"]
        if t == "status" and c.get("execution_state") == "idle":
            break
        if t == "stream":
            outs.append(c.get("text", ""))
        elif t == "error":
            outs.append(c.get("evalue", "") + "\n" + "\n".join(c.get("traceback", [])))
        elif t in ("execute_result", "display_data"):
            outs.append(str(c.get("data", {}).get("text/plain", "")))
    text = "\n".join(outs)
    ok = "ERROR:" not in text
    return ok, text.strip()


PROBES = [
    ("import-primitives",
     "package P1 { import ScalarValues::*; part def X { attribute a : String; "
     "attribute b : Integer; attribute c : Real; attribute d : Boolean; } }"),
    ("metadata-def-with-string",
     "package P2 { import ScalarValues::*; metadata def Tracked { attribute id : String; } }"),
    ("metadata-apply-prefix-hash",
     "package P3 { metadata def Mark; #Mark part def Q; }"),
    ("metadata-apply-at-member",
     "package P4 { metadata def Mark; part def Q { @Mark; } }"),
    ("ref-feature",
     "package P5 { part def D; part def E { ref s : D[0..1]; } }"),
    ("multiplicity-0-1-and-star",
     "package P6 { import ScalarValues::*; part def Z { attribute xs : String[*]; "
     "ref ys : Z[0..1]; } }"),
    ("name-Constraint",
     "package P7 { requirement def Constraint; }"),
    ("name-Decision-with-attr-decision",
     "package P8 { import ScalarValues::*; part def Decision { attribute decision : String; "
     "attribute context : String; } }"),
    ("requirement-subject-require",
     "package P9 { import ScalarValues::*; part def Sys { attribute m : Real; } "
     "requirement def R { subject s : Sys; require constraint { s.m >= 0 } } }"),
    ("enum-def",
     "package P10 { enum def Status { backlog; ready; done; } }"),
    ("cross-package-import-same-submission",
     "package A { import ScalarValues::*; part def Base { attribute n : String; } } "
     "package B { private import A::*; part def Derived :> Base; }"),
    ("redefine-with-import",
     "package P12 { import ScalarValues::*; part def Base { attribute k : String; } "
     "part i : Base { :>> k = \"v\"; } }"),
    ("doc-keyword-as-doc-clause",
     "package P13 { part def X { doc /* this is fine as a doc clause */ } }"),
    ("attr-named-description-actionText",
     "package P14 { import ScalarValues::*; part def Step { attribute description : String; "
     "attribute actionText : String; } }"),
]


def main():
    km, kc = start_new_kernel(kernel_name="sysml")
    print("kernel up\n")
    npass = 0
    for name, code in PROBES:
        ok, text = run_cell(kc, code)
        npass += ok
        print(f"[{'PASS' if ok else 'FAIL'}] {name}")
        if not ok:
            print("    " + text.replace("\n", "\n    ")[:900])
    print(f"\n{npass}/{len(PROBES)} probes passed")
    kc.stop_channels(); km.shutdown_kernel(now=True)


if __name__ == "__main__":
    main()
