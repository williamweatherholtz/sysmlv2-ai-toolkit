---
name: view
description: |
  Read, write, and execute lightweight TOML viewpoints (D0075). A view is a FILTER over the
  tracking model — select (type/attribute/has-missing-edge) + optional traverse (typed edges,
  direction, depth, far-endpoint target) + project — emitted as a subgraph (items+edges) JSON.
  Use when asked to define a new view/lens, run an existing one, or answer "show me items that …".
when_to_use: Asked to create/run a viewpoint, slice the model by a concern, or trace items along edges.
domain: [views, viewpoints, query, TOML, traceability, SysMLv2]
---

# view — lightweight query-driven viewpoints (D0075)

The deploying skill for D0075. A view is a concise, trivially-editable TOML declaration, executed
by the Rust authority (`keel view`), not per-view code (resolves issue018). Presentation is a
separate layer — the view's output is data (items + edges as JSON).

## Execute (x)
```
keel view <name> [ROOT]      # reads .engine/views/<name>.view.toml, prints the subgraph JSON
```
Fail-loud (D0074): an unknown TOML field or an unknown edge kind is a hard error — fix the view file.

## Read (r)
List `.engine/views/*.view.toml`. Each file IS the view's definition (name, concern, select,
traverse, project). The SysML viewpoint-registry (D0056/D0057) remains the concern-COVERAGE index.

## Write (w) — the model
```toml
name     = "traceability"
concern  = "Are needs → requirements → verification linked?"
audience = "reviewer"

[select]                      # the starting set (a filter over the enriched item-table)
type = "Requirement"          # by SysML type;  OR  item = "<name>"  for a single seed
[select.attrs]                # authored-attribute predicates (value OR set-membership)
# status = ["accepted", "superseded"]
# has_edge = "satisfy"  /  missing_edge = "deployedBySkill"   (edge presence/absence)

[traverse]                    # OPTIONAL — expand along typed edges
edges     = ["satisfy", "charteredBy"]   # known: satisfy allocate dependency ordering
                                         #        charteredBy prospectiveChange safetyChange dependsOn supersede
direction = "down"            # down | up | both
depth     = "closure"         # an integer (hops) OR "closure"
# [traverse.target]           # far-endpoint predicate — ICD-style boundary (keep edge iff target matches)
# allocatedTo = "subsystemB"

[project]                     # OPTIONAL — keep only these types in the result
# types = ["Need", "Requirement", "Test"]
```

## Rules (D0075)
- **Bounded vocabulary, no sledgehammer:** select / traverse(+target) / project only. NO joins
  beyond traverse, NO aggregation, NO arbitrary expressions.
- **Aggregation is NOT a view.** Velocity (points-per-sprint) etc. = an analysis ON a view's
  output, computed separately.
- **Two kinds (unified):** query viewpoints are these TOML files; "computed" lenses (orient,
  suspect, governing-version) are enrichment functions, not TOML — they populate computed columns.
- M1 scope: AUTHORED attributes + the edges the AST extracts. COMPUTED attrs (done/ready/
  governingVersion), `verify`/`:>` edges, and temporal predicates are M1b (tracked).
- New recurring view → add a `.view.toml` here (don't write code); run it with `keel view`.
