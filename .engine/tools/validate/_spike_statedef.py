"""Spike: test native SysML v2 state def / transition syntax (stateDefSpike task).
Verdict determines whether WorkflowDefinition.transitions : String[*] can be
replaced with native state defs. Run via:
  conda run -n sysml --no-capture-output python .engine/tools/validate/_spike_statedef.py
"""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

# Test 1: minimal state def declaration (no body)
SRC_MINIMAL = """
package StateDefSpike1 {
    private import ScalarValues::*;

    state def WorkflowPhase;
}
"""

# Test 2: state def with entry state and transition
SRC_TRANSITIONS = """
package StateDefSpike2 {
    private import ScalarValues::*;

    state def SimpleStateMachine {
        entry state s1;
        then s2;
        state s2;
    }
}
"""

# Test 3: state def modeling WorkflowDefinition transitions natively
SRC_WORKFLOW = """
package StateDefSpike3 {
    private import ScalarValues::*;

    state def DeliveryPhases {
        entry state refine;
        then standup;
        state standup;
        then implement;
        state implement;
        then review;
        state review;
        then closeOut;
        state closeOut;
    }
}
"""

km, kc = _kernel.start()

print("=== Test 1: minimal state def ===")
status1, text1 = _kernel.run_cell(kc, SRC_MINIMAL)
print(f"STATUS: {status1}")
if "ERROR:" in text1:
    print("RESULT: FAIL — state def does not parse")
    print(text1[:500])
else:
    print("RESULT: PASS — minimal state def parses")

print("\n=== Test 2: state def with transitions ===")
status2, text2 = _kernel.run_cell(kc, SRC_TRANSITIONS)
print(f"STATUS: {status2}")
if "ERROR:" in text2:
    print("RESULT: FAIL — state def transitions do not parse")
    print(text2[:500])
else:
    print("RESULT: PASS — state def with transitions parses")

print("\n=== Test 3: workflow state machine ===")
status3, text3 = _kernel.run_cell(kc, SRC_WORKFLOW)
print(f"STATUS: {status3}")
if "ERROR:" in text3:
    print("RESULT: FAIL — workflow state machine does not parse")
    print(text3[:500])
else:
    print("RESULT: PASS — workflow state machine parses")
    _, show = _kernel.run_cell(kc, "%show StateDefSpike3::DeliveryPhases")
    print(show[:600])

_kernel.teardown_and_exit(km, 0)
