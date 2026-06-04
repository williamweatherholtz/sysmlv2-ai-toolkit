# EARS Patterns (Easy Approach to Requirements Syntax)

Mavin et al. Pick the pattern by the requirement's condition structure. The
preamble (When/While/Where/If) maps to a SysML v2 `assume constraint`; the
"shall <response>" maps to a `require constraint`.

| Pattern | Keyword | Template | Example |
|---|---|---|---|
| Ubiquitous | (none) | `The <system> shall <response>.` | The phone shall have a mass of less than 150 g. |
| State-driven | While | `While <precondition>, the <system> shall <response>.` | While no card is inserted, the ATM shall display "insert card". |
| Event-driven | When | `When <trigger>, the <system> shall <response>.` | When "mute" is selected, the laptop shall suppress audio output. |
| Optional-feature | Where | `Where <feature>, the <system> shall <response>.` | Where a sunroof is fitted, the car shall provide a sunroof control. |
| Unwanted behavior | If/Then | `If <condition>, then the <system> shall <response>.` | If an invalid card number is entered, then the site shall display "re-enter details". |
| Complex | While+When | `While <precondition>, when <trigger>, the <system> shall <response>.` | While on ground, when reverse thrust is commanded, the FADEC shall enable reverse thrust. |

## Rewrite procedure
1. Identify the condition(s): is there a trigger (event), a state (while), an
   optional feature (where), or an unwanted-event guard (if)?
2. Choose the pattern; if both a state and a trigger apply, use Complex.
3. Put all conditions in the preamble; keep exactly one "shall <response>".
4. Ensure the response is measurable (value + unit + tolerance where relevant).
5. Bind to SysML v2: preamble → `assume constraint`, response → `require
   constraint`, naming the `subject`.
