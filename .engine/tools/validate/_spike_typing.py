"""Spike: can we strongly-type engine fields instead of String everywhere?
Tests enumeration def + enum-typed attribute + value, a typed ref to a part
(Actor), and a timestamp value type. Reports what parses + what %show renders."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

CASES = {
    "enum_def_verdict": """
package T1 { enum def VerdictKind { pass; fail; inconclusive; error; } }
""",
    "enum_def_method": """
package T2 { enum def MethodKind { inspect; analyze; demo; test; confirmation; } }
""",
    "enum_typed_attr_value_on_part": """
package T3 {
    enum def VerdictKind { pass; fail; }
    part def TestResult { attribute outcome : VerdictKind; }
    part r1 : TestResult { :>> outcome = VerdictKind::pass; }
}
""",
    "enum_typed_on_requirement": """
package T4 {
    enum def Source { customer; operator; internal; }
    requirement def Need { attribute source : Source; }
    requirement n1 : Need { :>> source = Source::customer; }
}
""",
    "typed_ref_to_actor": """
package T5 {
    private import ScalarValues::*;
    part def Actor { attribute name : String; }
    part def Element { ref authoredBy : Actor; }
}
""",
    "timestamp_value_type": """
package T6 {
    private import ScalarValues::*;
    attribute def Timestamp :> String;
    part def Element { attribute createdAt : Timestamp; }
}
""",
}

km, kc = _kernel.start()
for name, src in CASES.items():
    status, text = _kernel.run_cell(kc, src)
    bad = any(w in (text or "").lower() for w in ("error", "couldn't", "cannot", "unexpected", "mismatched", "no viable", "unresolved", "extraneous"))
    print(f"[{'FAIL' if bad else 'ok  '}] {name}  (status={status})")
    if bad:
        print("    " + (text or "").strip().replace("\n", "\n    ")[:400])
# show how an enum-typed attribute value renders on a part
_, t = _kernel.run_cell(kc, "%show T3::r1")
print("\n--- %show T3::r1 (enum value on a part usage) ---")
print(t[:700])
_kernel.teardown_and_exit(km, 0)
