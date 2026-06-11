"""Spike (CR-2): can action pins/flows be typed by part defs and requirement defs
(schema/core types) instead of item defs? And does prefix #View on an item def parse?"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    "part_def_pins_and_flow": """
package P1 {
    part def Brief; part def Persona;
    action def Biz {
        action brief { out o : Brief; }
        action personas { in i : Brief; out o : Persona; }
        first brief then personas;
        flow from brief.o to personas.i;
    }
}
""",
    "requirement_def_pins_and_flow": """
package P2 {
    private import ScalarValues::*;
    requirement def Need { attribute statement : String; }
    part def Persona;
    action def Biz {
        action personas { out o : Persona; }
        action needs { in i : Persona; out o : Need; }
        first personas then needs;
        flow from personas.o to needs.i;
    }
}
""",
    "view_metadata_prefix_on_item": """
package P3 {
    metadata def View;
    #View item def ICD;
}
""",
}

km, kc = _kernel.start()
for name, src in CASES.items():
    status, text = _kernel.run_cell(kc, src)
    bad = any(w in (text or "").lower() for w in ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved", "extraneous", "wasn't expected"))
    print(f"[{'FAIL' if bad else 'ok  '}] {name}")
    if bad:
        print("    " + (text or "").strip().replace("\n", "\n    ")[:300])
_kernel.teardown_and_exit(km, 0)
