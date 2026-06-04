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
    --no-capture-output python <this-dir>\validate_sysml.py
```

Must run through `conda run -n sysml` (the kernelspec invokes bare `java`, which
is only on PATH inside the activated env). Disable the sandbox (subprocess +
kernel). Classifier: a cell FAILS iff kernel output contains `ERROR:`.

## Files

- `validate_sysml.py` — main harness: probes + per-file checks. **NOTE:** its
  `ENGINE` path and the cell-by-cell approach predate the "distinct packages +
  imports, validate concatenated" decision — update it to concatenate
  dependency-ordered files into one submission during the schema rewrite.
- `validate_probes.py`, `import_probe.py`, `structure_probe.py` — the
  exploratory probes that established the syntax findings; kept as templates and
  evidence.
