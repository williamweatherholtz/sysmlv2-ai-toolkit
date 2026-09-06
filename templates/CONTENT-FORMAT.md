# Content format — the data-side prose contract (D0235 A2, resolves issue001)

Two field classes exist in every instance. Nothing else is legal.

## 1. Literal fields

`meta.*`, `requirements[].id`, `requirements[].statement` — rendered as **literal text,
character for character**. No markup interpretation of any kind, in any renderer. A statement
containing `$500`, `#4`, `@10Hz`, `<200ms>` or `*emphasis*` renders exactly those characters.
Mechanism (Typst): the string value is inserted directly as content — never `eval()`.
Regression: `templates/requirements-doc/example-awkward.json` carries every character class
issue001 demonstrated; the harness asserts each statement survives verbatim in the extracted
text of every declared output.

## 2. Content fields (CommonMark subset)

`content`, `requirements[].rationale`, `requirements[].notes` — parsed as this **CommonMark
subset**, nothing more:

| Construct | Syntax | Typst mapping (cmarker 0.1.6) | docx mapping (future) | xlsx mapping |
|---|---|---|---|---|
| paragraph | blank-line separated | parbreak | paragraph | newline (plain text) |
| emphasis | `*text*` | italic | italic run | plain text |
| strong | `**text**` | bold | bold run | plain text |
| inline code | `` `text` `` | raw | monospace run | plain text |
| — code spans are opaque: a `$` or `<` inside backticks is literal, needs no escape | | | | |
| link | `[label](url)` | link | hyperlink | `label (url)` |
| bullet list | `- item` | list | bullet paragraph | `- item` lines |
| numbered list | `1. item` | enum | numbered paragraph | `1. item` lines |

**Math (contract v2, D0236):** in prose fields only, a single `$...$` delimits an inline
**Typst-native math** segment (e.g. `$x^2 + sqrt(y_i) / 2$`, `$Delta v <= 0.5$`). Rules,
enforced by the harness before any compile:
- a literal dollar in a prose field is written `\$` (statements/meta are unaffected — they
  are literal fields where `$500` needs no escaping and no math exists);
- **`#` is forbidden inside a math segment** — math-mode `eval` executes `#`-code (proven:
  a probe embedded a repo file into a PDF), so the hash ban is what keeps math data-only;
- unbalanced `$` refuses the render (a forgotten escape must never misrender);
- math notation is verified by the golden diff, not by verbatim text extraction (its glyphs
  are mathematical alphanumerics). Display/block math and math in statements are out of
  scope until a further decision.

**Excluded** (rejected by the harness, not silently dropped): raw HTML, images, tables,
headings, block quotes, footnotes. Document structure belongs to the template; data carries
prose only. Escaping follows CommonMark: backslash-escape a literal `*`, `_`, `` ` ``, `[`,
and (in prose fields) `$`.

## Renderer rules

- The parser is ONE shared implementation per renderer family, pinned: Typst path uses
  `@preview/cmarker:0.1.6` (version pinned in the import and in toolchain-manifest.json).
  **`eval()` of any raw data string is forbidden** — the defect class recorded as issue001.
- The xlsx downgrade (plain text, no list/paragraph fidelity) is declared in that output's
  field map, never silent.
- Instance data is untrusted input at the render boundary (D0235 A2).
