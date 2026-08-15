import os, sys
sys.path.insert(0, r"C:\Users\WilliamWeatherholtz\claude_code\sysmlv2-ai-toolkit\.engine\tools")
import _kernel
CASES = {
  "typed_action_usage_with_string_attr":
    'package B1 { private import ScalarValues::*; action def Proc { attribute title : String; } action deploy : Proc { :>> title = "x"; } }',
  "typed_action_usage_with_enum_attr":
    'package B2 { enum def K { a; b; } action def Proc { attribute k : K; } action deploy : Proc { :>> k = K::a; } }',
  "action_usage_with_assert_and_attr":
    'package B3 { private import ScalarValues::*; constraint def Ok; action def Proc { attribute title : String; } action deploy : Proc { :>> title = "x"; assert constraint c : Ok; } }',
}
km, kc = _kernel.start()
BAD = ("error","couldn't","cannot","unexpected","mismatched","no viable","unresolved","extraneous","must be","must have")
for n, s in CASES.items():
    _, t = _kernel.run_cell(kc, s)
    low = (t or "").lower()
    f = any(w in low for w in BAD)
    print(f"[{'FAIL' if f else 'ok  '}] {n}", flush=True)
    if f: print("   ", (t or "").strip().splitlines()[0][:130], flush=True)
_kernel.teardown_and_exit(km, 0)
