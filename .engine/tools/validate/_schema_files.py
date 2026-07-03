"""Single source of truth for schema file dependency order.

When a new schema file is added, update SCHEMA_ORDER here FIRST.
validate_schema.py check_coverage() will fail hard at startup if any
schema/*.sysml is unregistered, catching omissions before the kernel runs.
"""
import os
import glob

# Dependency order: element.sysml first; each file may only import files
# that appear earlier in this list.
SCHEMA_ORDER = [
    "schema/core/element.sysml",
    "schema/core/rules.sysml",
    "schema/core/needs.sysml",
    "schema/core/requirements.sysml",
    "schema/core/verification.sysml",
    "schema/core/work.sysml",
    "schema/core/architecture.sysml",
    "schema/core/computed.sysml",
    "schema/core/relationships.sysml",
    "schema/core/workflow.sysml",
    "schema/core/process.sysml",
    "schema/core/skills.sysml",
    "schema/core/risk.sysml",
    "schema/core/indicator.sysml",
    "schema/core/baseline.sysml",
    "schema/safety/stpa.sysml",
]


def check_coverage(engine_root):
    """Return relative paths of schema/*.sysml files NOT in SCHEMA_ORDER.

    An empty list means full coverage. A non-empty list means a schema file
    was added without being registered — validate_schema.py treats this as a
    hard failure (exit 2) so the oversight cannot silently propagate.
    """
    registered = set(SCHEMA_ORDER)
    pattern = os.path.join(engine_root, "schema", "**", "*.sysml")
    missing = []
    for f in sorted(glob.glob(pattern, recursive=True)):
        rel = os.path.relpath(f, engine_root).replace("\\", "/")
        if rel not in registered:
            missing.append(rel)
    return missing
