"""Pin down the correct import / type-reference syntax for the kernel."""
from queue import Empty
from jupyter_client.manager import start_new_kernel


def run_cell(kc, code, timeout=120):
    mid = kc.execute(code, allow_stdin=False)
    while True:
        r = kc.get_shell_msg(timeout=timeout)
        if r["parent_header"].get("msg_id") == mid:
            break
    outs = []
    while True:
        try:
            m = kc.get_iopub_msg(timeout=timeout)
        except Empty:
            break
        if m["parent_header"].get("msg_id") != mid:
            continue
        t = m["header"]["msg_type"]; c = m["content"]
        if t == "status" and c.get("execution_state") == "idle":
            break
        if t == "stream":
            outs.append(c.get("text", ""))
        elif t == "error":
            outs.append(c.get("evalue", ""))
        elif t in ("execute_result", "display_data"):
            outs.append(str(c.get("data", {}).get("text/plain", "")))
    text = "\n".join(outs)
    return ("ERROR:" not in text), text.strip()


PROBES = [
    ("root-import-before-pkg",
     "import ScalarValues::*;\npackage Pa { part def X { attribute a : String; } }"),
    ("private-import-inside",
     "package Pb {\n    private import ScalarValues::*;\n    part def X { attribute a : String; }\n}"),
    ("public-import-inside",
     "package Pc {\n    public import ScalarValues::*;\n    part def X { attribute a : String; }\n}"),
    ("qualified-name-no-import",
     "package Pd {\n    part def X { attribute a : ScalarValues::String; }\n}"),
    ("specific-import",
     "package Pe {\n    import ScalarValues::Real;\n    part def X { attribute a : Real; }\n}"),
    ("no-import-bare-control",
     "package Pf {\n    part def X { attribute a : String; }\n}"),
    ("enum-as-attr-type",
     "package Pg {\n    enum def Method { test; analysis; }\n    part def T { attribute m : Method; }\n}"),
    ("import-recursive-doublestar",
     "package Ph {\n    import ScalarValues::**;\n    part def X { attribute a : String; }\n}"),
    ("private-import-recursive",
     "package Pi {\n    private import ScalarValues::**;\n    part def X { attribute a : String; }\n}"),
]


def main():
    km, kc = start_new_kernel(kernel_name="sysml")
    print("kernel up\n")
    n = 0
    for name, code in PROBES:
        ok, text = run_cell(kc, code)
        n += ok
        print(f"[{'PASS' if ok else 'FAIL'}] {name}")
        if not ok:
            print("    " + text.replace("\n", "\n    ")[:500])
    print(f"\n{n}/{len(PROBES)} passed")
    kc.stop_channels(); km.shutdown_kernel(now=True)


if __name__ == "__main__":
    main()
