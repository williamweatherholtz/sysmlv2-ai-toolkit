# Validation tooling

Drives the SysML v2 pilot-implementation Jupyter kernel to parse/validate the
engine's `.sysml` files. Confirmed findings live in
`.engine/docs/sysmlv2-syntax-notes.md`.

## Prerequisites (already set up on this machine)

- Miniforge at `C:\Users\WilliamWeatherholtz\miniforge3`.
- Conda env `sysml` with `jupyter-sysml-kernel` 0.59.0 + OpenJDK 25 +
  the Jupyter stack. Recreate with:
  `conda create -n sysml -c conda-forge jupyter-sysml-kernel jupyter nbclient -y`

## Run

```
& "C:\Users\WilliamWeatherholtz\miniforge3\Scripts\conda.exe" run -n sysml \
    --no-capture-output python <this-dir>\validate_schema.py
```

Must run through `conda run -n sysml` (the kernelspec invokes bare `java`, which
is only on PATH inside the activated env). Disable the sandbox (subprocess +
kernel). Classifier: a cell FAILS iff kernel output contains `ERROR:`.

## Files (the four validators — one per layer)

- `validate_schema.py` — `schema/core/*` + `schema/safety/*` (13 files), in
  dependency order on one shared kernel.
- `validate_workflows.py` — `workflows/*.sysml` + `_meta.sysml`.
- `validate_instances.py` — the `.engine` instance content (decisions / processes /
  skills-registry), loaded after the schema.
- `validate_tracking.py` — `.tracking/*.sysml` (backlog, actors), loaded after `_meta`.
- `_kernel.py` — shared kernel driver (DEVNULL + clean teardown; see its header).
- `_spike_*.py` — pilot grammar spikes (evidence for syntax findings); kept as templates.
- `validate_probes.py`, `import_probe.py`, `structure_probe.py` — exploratory probes.

(The legacy `validate_sysml.py` was retired 2026-06-11 — it predated the flat-package split.)
