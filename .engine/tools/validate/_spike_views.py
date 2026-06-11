"""Spike: native view def / viewpoint def / expose / render in the pilot (finding A,
the materialized-view layer). What parses?"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    "view_def_empty": "package W1 { view def TasksView; }",
    "viewpoint_def": "package W2 { viewpoint def StakeholderVP; }",
    "view_def_with_expose": """
package W3 {
    part def Task; part t1 : Task; part t2 : Task;
    view def TasksView { expose Task; }
}
""",
    "view_usage_expose_wildcard": """
package W4 {
    part def Task; part t1 : Task;
    view def AllView { expose W4::*; }
    view v : AllView;
}
""",
    "view_with_render": """
package W5 {
    part def Task; part t1 : Task;
    view def TreeView { expose Task; render asTreeDiagram; }
}
""",
    "view_metadata_marker": """
package W6 {
    metadata def View;
    part def Task;
    #View view def TasksView { expose Task; }
}
""",
    "viewpoint_with_require": """
package W7 {
    viewpoint def OutstandingVP { }
    view def OutVerView { }
}
""",
}

km, kc = _kernel.start()
for name, src in CASES.items():
    status, text = _kernel.run_cell(kc, src)
    bad = any(w in (text or "").lower() for w in ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved", "extraneous", "wasn't expected"))
    print(f"[{'FAIL' if bad else 'ok  '}] {name}")
    if bad:
        print("    " + (text or "").strip().replace("\n", "\n    ")[:250])
_kernel.teardown_and_exit(km, 0)
