"""instanceMigration (decisions): rewrite .engine/decisions/*.sysml from the old
nested Engine::Core to the new flat EngineWork package + rename decision->decisionText.
One package per file (Decision<NNNN>) to avoid cross-file package reopening."""
import glob
import os

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
for f in sorted(glob.glob(os.path.join(REPO, ".engine", "decisions", "*.sysml"))):
    num = os.path.basename(f).split("-")[0]          # 0001
    pkg = "Decision" + num
    t = open(f, encoding="utf-8").read()
    t = t.replace("package Engine { package Decisions {", "package %s {" % pkg)
    t = t.replace("private import Engine::Core::*;", "private import EngineWork::*;")
    t = t.replace(":>> decision =", ":>> decisionText =")
    t = t.rstrip()
    if t.endswith("}}"):                              # two package closes -> one
        t = t[:-2] + "}"
    with open(f, "w", encoding="utf-8") as fh:
        fh.write(t + "\n")
    print(f"  {os.path.basename(f)} -> package {pkg}")
print("decisions migrated.")
