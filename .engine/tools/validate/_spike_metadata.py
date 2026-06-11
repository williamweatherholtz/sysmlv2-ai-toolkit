"""Spike (CR-12/U2): ATTRIBUTED metadata def + application with values — could one
@Tracked annotation replace the four duplicated Tracked* bases?"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    "metadata_def_with_attrs": """
package MD1 {
    private import ScalarValues::*;
    metadata def Tracked { attribute trackId : String; attribute createdAt : String; }
}
""",
    "metadata_application_with_values": """
package MD2 {
    private import ScalarValues::*;
    metadata def Tracked { attribute trackId : String; }
    part def Widget;
    part w1 : Widget { @Tracked { trackId = "abc-123"; } }
}
""",
    "metadata_on_requirement_def": """
package MD3 {
    private import ScalarValues::*;
    metadata def Tracked { attribute trackId : String; }
    #Tracked requirement def Need;
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
