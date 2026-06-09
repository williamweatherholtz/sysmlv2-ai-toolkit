"""
Focused parse spike (tier-2 foundation check): can SysML v2 (the pilot
implementation) actually EXPRESS the process-as-data meta-model
(Workflow / Phase / Gate, plus an instance)? Runs a few incremental cells so any
failure is localized. This de-risks the design's #1 critique (roof-before-foundation)
for the price of a few lines instead of months.

Run (sandbox disabled; kernel calls bare `java`, so must go through conda run):
  conda run -n sysml --no-capture-output python .engine/tools/validate/spike_metamodel.py
"""
import sys
import re
from queue import Empty

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


CELLS = [
    ("1. defs: Workflow / Phase / Gate", r'''
package MetaSpikeDefs {
    private import ScalarValues::*;
    part def Gate {
        attribute name : String;
        attribute kind : String;      // "entry" | "exit" (documented vocab; String avoids reserved-keyword enum literals)
    }
    part def Phase {
        attribute name : String;
        attribute order : Integer;
        ref entryGate : Gate[0..1];
        ref exitGate : Gate[0..1];
    }
    part def Workflow {
        attribute name : String;
        attribute purpose : String;
        attribute cadence : String;
        ref phases : Phase[*];
    }
}
'''),
    ("2. instance: a Business workflow wired to a phase + gate", r'''
package MetaSpikeInst {
    private import ScalarValues::*;
    part def Gate { attribute name : String; attribute kind : String; }
    part def Phase { attribute name : String; attribute order : Integer; ref exitGate : Gate[0..1]; }
    part def Workflow { attribute name : String; ref phases : Phase[*]; }

    part gateA : Gate { :>> name = "Needs baselined"; :>> kind = "exit"; }
    part personas : Phase { :>> name = "Personas"; :>> order = 2; :>> exitGate = gateA; }
    part business : Workflow { :>> name = "Business"; :>> phases = personas; }
}
'''),
    ("3. ordered multiplicity feature (phase sequence)", r'''
package MetaSpikeOrdered {
    private import ScalarValues::*;
    part def Phase { attribute name : String; }
    part def Workflow { ref phases : Phase[*] ordered; }
}
'''),
    ("4. ArtifactType with nature as String + Boolean", r'''
package MetaSpikeArtifact {
    private import ScalarValues::*;
    part def ArtifactType {
        attribute name : String;
        attribute nature : String;     // "real" | "view"
        attribute isView : Boolean;
        attribute derivation : String;
    }
}
'''),
]


def main():
    from jupyter_client.kernelspec import KernelSpecManager
    from jupyter_client.manager import start_new_kernel

    specs = KernelSpecManager().find_kernel_specs()
    if "sysml" not in specs:
        print("ERROR: no 'sysml' kernelspec found.")
        sys.exit(2)

    print("Starting kernel (JVM startup ~20s)...")
    km, kc = start_new_kernel(kernel_name="sysml")
    print("Kernel up.\n")

    results = []
    for name, code in CELLS:
        st, text = run_cell(kc, code)
        passed = ok(st, text)
        results.append((name, passed))
        print(f"[{'PASS' if passed else 'FAIL'}] {name}  (status={st})")
        if text.strip():
            print("    " + text.strip().replace("\n", "\n    ")[:1500])
        print()

    print("=" * 64)
    print("SPIKE SUMMARY")
    for n, p in results:
        print(f"  {'PASS' if p else 'FAIL'}  {n}")
    print(f"  {sum(1 for _, p in results if p)}/{len(results)} cells passed")

    kc.stop_channels()
    km.shutdown_kernel(now=True)


if __name__ == "__main__":
    main()
