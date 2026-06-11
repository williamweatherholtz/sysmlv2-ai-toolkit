"""Spike: validate the native-metaclass schema pattern before rewriting frozen
schema/core. Confirms a per-metaclass 'tracked base' (requirement def :> requirement
def, etc.) parses, plus satisfy/verify edges and native elementId."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import _kernel

SRC = """
package NativeSpike {
    private import ScalarValues::*;

    // Per-metaclass tracked base — Element (part def) can't be shared across
    // metaclasses, so each native metaclass gets its own abstract tracked base.
    abstract requirement def TrackedRequirement {
        attribute createdAt : String;
        attribute authoredBy : String;
    }
    requirement def Need :> TrackedRequirement {
        attribute source : String;
        attribute statement : String;
    }
    requirement def SystemRequirement :> TrackedRequirement {
        attribute statement : String;
        attribute priority : String;
    }

    abstract use case def TrackedUseCase {
        attribute createdAt : String;
        attribute authoredBy : String;
    }
    use case def Login :> TrackedUseCase {
        attribute mainSuccess : String;
    }

    abstract verification def TrackedVerification {
        attribute method : String;
        attribute outcome : String;
    }
    verification def CheckLogin :> TrackedVerification;

    // Edges across the native types.
    part def SystemA;
    part systemA1 : SystemA;
    requirement myReq : Need;
    satisfy myReq by systemA1;
}
"""

km, kc = _kernel.start()
status, text = _kernel.run_cell(kc, SRC)
print(f"STATUS: {status}")
print(text[:1500])
print("--- %show NativeSpike::Need (read structure + native elementId) ---")
_, t2 = _kernel.run_cell(kc, "%show NativeSpike::Need")
print(t2[:900])
_kernel.teardown_and_exit(km, 0)
