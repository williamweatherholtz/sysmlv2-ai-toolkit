"""Spike: test native SysML v2 occurrence def (occurrenceRetype task).
Verdict determines whether TestResult (and Decision?) can be retyped from
`part def` to `occurrence def`. Run via:
  conda run -n sysml --no-capture-output python .engine/tools/validate/_spike_occurrence.py
"""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

# Test 1: minimal occurrence def (no specialization)
SRC_MINIMAL = """
package OccurrenceSpike1 {
    private import ScalarValues::*;

    occurrence def SimpleEvent;
}
"""

# Test 2: occurrence def with attributes
SRC_WITH_ATTRS = """
package OccurrenceSpike2 {
    private import ScalarValues::*;

    occurrence def TimestampedEvent {
        attribute timestamp : String;
        attribute actor : String;
    }
}
"""

# Test 3: occurrence def specializing a part def (the key test for TestResult retype)
SRC_SPECIALIZE_PART = """
package OccurrenceSpike3 {
    private import ScalarValues::*;

    part def BaseItem {
        attribute id : String;
        attribute createdAt : String;
    }
    occurrence def EventRecord :> BaseItem {
        attribute outcome : String;
        attribute judgedAt : String;
    }
}
"""

# Test 4: part def specializing an occurrence def (inverse direction)
SRC_PART_FROM_OCC = """
package OccurrenceSpike4 {
    private import ScalarValues::*;

    occurrence def OccurrenceBase {
        attribute timestamp : String;
    }
    part def StructuredEvent :> OccurrenceBase {
        attribute id : String;
    }
}
"""

# Test 5: occurrence def with non-reserved attributes + part instance usage
# This is the critical test: can we use `part x : OccDef { ... }` in instances?
SRC_PART_USAGE = """
package OccurrenceSpike5 {
    private import ScalarValues::*;

    abstract part def BaseElement {
        attribute id : String;
    }

    occurrence def TestResultOcc :> BaseElement {
        attribute outcome : String;
        attribute judgedAt : String;
        attribute judgedBy : String;
    }

    part myResult1 : TestResultOcc {
        :>> id = "abc123";
        :>> outcome = "pass";
        :>> judgedAt = "2026-06-15";
        :>> judgedBy = "ci";
    }
}
"""

km, kc = _kernel.start()

print("=== Test 1: minimal occurrence def ===")
status1, text1 = _kernel.run_cell(kc, SRC_MINIMAL)
print(f"STATUS: {status1}")
if "ERROR:" in text1:
    print("RESULT: FAIL — occurrence def does not parse")
    print(text1[:500])
else:
    print("RESULT: PASS — minimal occurrence def parses")

print("\n=== Test 2: occurrence def with attributes ===")
status2, text2 = _kernel.run_cell(kc, SRC_WITH_ATTRS)
print(f"STATUS: {status2}")
if "ERROR:" in text2:
    print("RESULT: FAIL — occurrence def with attributes does not parse")
    print(text2[:500])
else:
    print("RESULT: PASS — occurrence def with attributes parses")

print("\n=== Test 3: occurrence def :> part def (TestResult retype scenario) ===")
status3, text3 = _kernel.run_cell(kc, SRC_SPECIALIZE_PART)
print(f"STATUS: {status3}")
if "ERROR:" in text3:
    print("RESULT: FAIL — occurrence def cannot specialize part def")
    print(text3[:500])
else:
    print("RESULT: PASS — occurrence def :> part def parses")

print("\n=== Test 4: part def :> occurrence def (inverse) ===")
status4, text4 = _kernel.run_cell(kc, SRC_PART_FROM_OCC)
print(f"STATUS: {status4}")
if "ERROR:" in text4:
    print("RESULT: FAIL — part def cannot specialize occurrence def")
    print(text4[:500])
else:
    print("RESULT: PASS — part def :> occurrence def parses")

print("\n=== Test 5: occurrence def with non-reserved attrs + part instance usage ===")
status5, text5 = _kernel.run_cell(kc, SRC_PART_USAGE)
print(f"STATUS: {status5}")
if "ERROR:" in text5:
    print("RESULT: FAIL — part instance of occurrence def does not parse")
    print(text5[:600])
else:
    print("RESULT: PASS — part instance of occurrence def parses (retype viable)")
    _, show5 = _kernel.run_cell(kc, "%show OccurrenceSpike5::TestResultOcc")
    print(show5[:400])

_kernel.teardown_and_exit(km, 0)
