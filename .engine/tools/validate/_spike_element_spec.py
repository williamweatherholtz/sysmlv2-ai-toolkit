"""Spike: is the claim 'a requirement/use case/verification def CANNOT specialize a
part def (Element)' actually TRUE in the pilot kernel, or an untested assumption?
Also re-tests the metadata-with-values migration shape across metatypes (the real
trackedMetadata target). Drives the D0053 reassessment."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    # THE key test: can a requirement def specialize a (concrete) part def?
    "req_def_specializes_part_def": """
package T1 {
    private import ScalarValues::*;
    part def Element { attribute id : String; attribute title : String; }
    requirement def Need :> Element { attribute statement : String; }
}
""",
    # abstract part def base
    "req_def_specializes_abstract_part_def": """
package T2 {
    private import ScalarValues::*;
    abstract part def Element { attribute id : String; }
    requirement def Need :> Element;
}
""",
    "usecase_def_specializes_part_def": """
package T3 {
    private import ScalarValues::*;
    abstract part def Element { attribute id : String; }
    use case def Login :> Element;
}
""",
    "verification_def_specializes_part_def": """
package T4 {
    private import ScalarValues::*;
    abstract part def Element { attribute id : String; }
    verification def Check :> Element;
}
""",
    # The metadata alternative — same @Tracked WITH VALUES on different metatypes,
    # including the instance value-redefinition the migration would require.
    "metadata_values_across_metatypes": """
package T5 {
    private import ScalarValues::*;
    metadata def Tracked { attribute id : String; attribute title : String; }
    part def Widget;
    requirement def Need;
    verification def Check;
    part w1 : Widget { @Tracked { id = "p-1"; title = "w"; } }
    requirement r1 : Need { @Tracked { id = "r-1"; title = "n"; } }
    verification v1 : Check { @Tracked { id = "v-1"; title = "c"; } }
}
""",
}

def bad(text):
    t = (text or "").lower()
    return any(w in t for w in ("error", "couldn't", "cannot", "unexpected",
               "mismatched", "no viable", "unresolved", "extraneous",
               "wasn't expected", "not a valid", "is not", "shall"))

km, kc = _kernel.start()
for name, src in CASES.items():
    status, text = _kernel.run_cell(kc, src)
    flag = bad(text)
    print(f"[{'FAIL' if flag else 'ok  '}] {name}  (status={status})")
    if flag:
        print("    " + (text or "").strip().replace("\n", "\n    ")[:400])
_kernel.teardown_and_exit(km, 0)
