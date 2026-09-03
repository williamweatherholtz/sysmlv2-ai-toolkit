# stpa-diagram — draw a control structure so it can be read

Deploys `.engine/processes/stpa-diagram.sysml` (D0285). Use whenever a control structure is to be
shown to a human: this project's own (`keel show control-structure`) or any `EngineSafety`
Controller / ControlAction / Feedback set.

## The five properties (set by the human, 2026-09-02)

| # | Property | What it buys the reader |
|---|---|---|
| 1 | **Authority is the vertical axis**, highest at the top; controlled processes along the bottom; controllers in the gaps between process columns | who outranks whom, from position alone; no line ever passes through a box |
| 2 | **Control goes down on the left, feedback comes up on the right** — solid vs dashed | direction without reading an arrowhead |
| 3 | **Ortholinear**: vertical / horizontal / vertical, 90° only; every edge owns its horizontal channel and its trunk/drop slot, so **no two segments share a line** | any line can be followed end to end |
| 4 | **Every edge is labelled with what passes** along it — the record, the verdict JSON, the commands, the refusal — never "an action"; the label sits on the channel where no other vertical runs behind it | the diagram says what moves, which is the point |
| 5 | **A crossing is a semicircular hop**, never a junction | a false junction is a false authority |

## Procedure

```
keel show control-structure . | python .engine/tools/stpa_diagram.py --out diagram.svg
#   or, live:     python .engine/tools/stpa_diagram.py --root . --out diagram.svg
```

1. **Compute, never draw** (`sd0Compute`). The JSON is the input; the SVG is a view of it. If a box
   is missing, fix the view or the authored residue, not the picture.
2. The tool lays out rows by authority, assigns sides and directions, routes orthogonally with one
   channel per edge, places labels off other edges' verticals, and hops crossings (`sd1`–`sd5`) — by
   construction. There is nothing to do by hand here, and nothing may be done by hand here.
3. **Look at it** (`sd6Verify`): render in a browser at full size and at a zoomed crop of the densest
   region; check the five properties. The two defects construction cannot rule out are a label hiding
   a line it was placed over and a label floating past a short channel — both were found only by
   looking. Fix in the tool; never edit the SVG.
4. Embed the SVG where the human reads: the console or the published page. The SVG uses CSS
   variables (`--ctl --fb --proc --ctl-bg --proc-bg --panel --ink --muted`) so the host page owns the
   palette and dark mode.

## What this skill does not do

- It does not author the structure. That is `keel show control-structure` (D0284) and the residue in
  `.tracking/architecture/engine-control-structure.sysml`.
- It does not run UCA analysis. That is the STPA process (`dcStpaProcessForKeel`); this skill draws
  its step 2 input.
- It is Python under `.engine/tools/` as the sanctioned interim; the port into the binary as
  `keel render control-structure --mode graph` is chartered (`dcStpaDiagramInTheBinary`, D0221).

## Questions this skill answers

- "Draw the control structure" · "Make the STPA diagram readable" · "Which authority sends what to
  which process, and what comes back?"
