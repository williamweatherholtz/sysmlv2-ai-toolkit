"""Lock the package/file structure before rewriting the schema."""
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
    ("qualified-package-name",
     "package Engine::Core::ElementPkg {\n"
     "    private import ScalarValues::*;\n"
     "    part def Element { attribute id : String; }\n"
     "}"),
    ("reopen-within-one-submission",
     "package E { package C { part def A; } }\n"
     "package E { package C { part def B; } }"),
    ("cross-package-private-import-1-submission",
     "package A1 { part def Base; }\n"
     "package B1 { private import A1::*; part def D :> Base; }"),
    ("qualified-cross-import",
     "package X::Core { part def Base; }\n"
     "package X::App { private import X::Core::*; part def D :> Base; }"),
    ("nested-reopen-with-imports",
     "package Eng { package Core {\n"
     "    private import ScalarValues::*;\n"
     "    part def Element { attribute id : String; }\n"
     "} }\n"
     "package Eng { package Core {\n"
     "    part def WorkItem :> Element { attribute title : String; }\n"
     "} }"),
    ("metadata-marker-on-dependency",
     "package M {\n"
     "    metadata def Supersede;\n"
     "    part def D1; part def D2;\n"
     "    part d1 : D1; part d2 : D2;\n"
     "    #Supersede dependency from d2 to d1;\n"
     "}"),
    ("ref-supersedes-feature",
     "package R {\n"
     "    part def Decision { ref supersedes : Decision[0..1]; }\n"
     "}"),
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
            print("    " + text.replace("\n", "\n    ")[:600])
    print(f"\n{n}/{len(PROBES)} passed")
    kc.stop_channels(); km.shutdown_kernel(now=True)


if __name__ == "__main__":
    main()
