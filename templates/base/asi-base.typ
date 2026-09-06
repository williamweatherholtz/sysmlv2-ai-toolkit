// ASI base document template — the shared base every document type imports (D0235 A1/A2).
// Brand values come from /brand/style-tokens.json (single source of truth, watched by
// brand-watch/D0233) — a hardcoded color here is a defect the harness lint catches.
// Typography conforms to the recorded brand practice (tokens: observedWeightPractice —
// condensed/thin headings, light body), per the author's 2026-08-27 ruling (stCritiqueRulings).
// Render via scripts/render.py. CONTENT CONTRACT (issue001): meta fields are LITERAL strings,
// inserted directly — never eval() of data. Missing required fields FAIL LOUDLY; `recipient`
// is optional and its row is OMITTED when absent (author ruling: never a placeholder glyph).

#import "@preview/cmarker:0.1.6": render as cmarker-render

#let tokens = json("/brand/style-tokens.json")

// Primary blue: canonical manual-stated value; the three-way conflict is recorded in
// style-tokens.json conflicts[] and this binding is the single place adjudication lands
// (author 2026-08-27: adjudication deferred; the text-contrast finding rides it).
#let asi-blue = rgb(tokens.canonical.colors.primaryBlueTextStated.hex)
#let asi-navy = rgb(tokens.observed.asiColorsScheme.dk2) // adjudication-pending, see conflicts[]
#let asi-light = rgb(tokens.observed.asiColorsScheme.lt2)
#let asi-ink = rgb(tokens.documentDefaults.ink)
#let asi-must = rgb(tokens.documentDefaults.priorityMustAccent)

#let company = tokens.canonical.identity.company
#let tagline = tokens.canonical.identity.tagline
// Approved logo asset — none until its bytes are downloaded (see tokens.assets.$comment).
#let logo-path = tokens.at("assets", default: (:)).at("mainLogoNavy", default: none)

// Reference-renderer stack (D0235 A8 + D0236): Roboto first, then the vendored expressiveness
// fallbacks (math/symbols/emoji, all OFL). Rendered with --ignore-system-fonts over /fonts,
// so a glyph outside the vendored union is a coverage refusal, never a silent fallback.
#let asi-fonts = ("Roboto", "Noto Sans Math", "Noto Sans Symbols", "Noto Sans Symbols 2", "Noto Emoji")
// Typst normalizes the vendored RobotoCondensed cuts into family "Roboto" at stretch 75%
// (weights 300/400) and Roboto Thin at weight 250 — brand practice is selected via axes.
#let heading-style(body, size: 12pt) = text(font: asi-fonts, stretch: 75%, weight: 400,
  fill: asi-navy, size: size, body)

// THE shared content converter (D0235 A2, contract v2 per D0236): CommonMark subset via
// cmarker, plus $-delimited Typst-native inline math ("\$" = literal dollar). The hash ban
// closes the PROVEN injection vector: eval(mode: "math") executes #-code, so a math segment
// containing "#" refuses the render. The panic reports position data, not the data itself.
#let md-content(s) = cmarker-render(
  s,
  math: (raw, block: false) => {
    if raw.contains("#") {
      panic("content contract (D0236): # forbidden inside a math segment (segment of "
        + str(raw.len()) + " chars)")
    }
    math.equation(block: block, eval(raw, mode: "math"))
  },
)

// Loud accessor: a required meta field that is absent or blank stops the render with a
// named error (defense in depth behind the harness's schema gate — D0235 A2).
#let req(meta, key) = {
  if key not in meta { panic("required meta field missing: " + key) }
  let v = meta.at(key)
  if type(v) != str or v.trim() == "" { panic("required meta field blank: " + key) }
  v
}

#let asi-doc(meta: (:), body) = {
  let title = req(meta, "title")
  let doc-id = req(meta, "docId")
  let doc-type = req(meta, "docType")
  let version = req(meta, "version")
  let date = req(meta, "date")
  let author = req(meta, "author")
  let classification = req(meta, "classification")
  let recipient = meta.at("recipient", default: none)

  // PDF creation date comes from --creation-timestamp (reproducible builds), not from here.
  set document(title: title + " (" + doc-id + ")", author: author)

  // Brand practice: light body (hyphenate: false serves Q5 machine-parseability — the
  // harness asserts statements survive extraction character-for-character).
  set text(font: asi-fonts, weight: "light", size: 10pt, fill: asi-ink, hyphenate: false)
  set par(justify: true, leading: 0.65em)
  // Justification never applies inside tables or the page chrome (rivers in narrow cells).
  show table: set par(justify: false)
  set heading(numbering: "1.1  ")
  show heading.where(level: 1): it => {
    v(1.2em, weak: true)
    heading-style(it, size: 15pt)
    v(0.5em, weak: true)
  }
  show heading.where(level: 2): it => {
    v(1em, weak: true)
    heading-style(it, size: 12.5pt)
    v(0.4em, weak: true)
  }

  // Margins in inches — US-letter geometry stated in US units (critique NIT, recorded choice).
  set page(
    paper: "us-letter",
    margin: (top: 1.1in, bottom: 0.95in, x: 1in),
    header: {
      set par(justify: false)
      set text(size: 7.5pt, fill: asi-navy)
      grid(
        columns: (auto, 1fr, auto),
        column-gutter: 1em,
        align: (left + horizon, center + horizon, right + horizon),
        if logo-path != none { image(logo-path, height: 0.32in) } else { upper(company) },
        upper(classification),
        [#doc-id],
      )
      v(-0.5em)
      line(length: 100%, stroke: 0.75pt + asi-blue)
    },
    // Footer follows the manual's own recorded pattern (tokens footerPatternObserved):
    // COMPANY | tagline .... vX.Y – UPDATED <date> – PG. <n>
    footer: {
      line(length: 100%, stroke: 0.5pt + asi-blue)
      v(-0.35em)
      set par(justify: false)
      set text(size: 7pt, fill: asi-navy)
      grid(
        columns: (1fr, auto),
        column-gutter: 1.5em,
        align: (left + horizon, right + horizon),
        [#upper(company) #h(0.5em) _#tagline _],
        context [v#version – UPDATED #upper(date) – PG. #counter(page).display()],
      )
    },
  )

  // Title block — brand practice: thin cut at display size.
  v(0.5em)
  text(font: asi-fonts, size: 22pt, weight: 250, fill: asi-navy, title)
  v(0.2em)
  text(font: asi-fonts, stretch: 75%, weight: 400, size: 11pt, fill: asi-blue, upper(doc-type))
  v(0.8em)

  // Metadata table — every base field visible and machine-checkable on page 1.
  // The Recipient pair is omitted entirely when absent (author ruling).
  {
    set text(size: 8.5pt)
    let row(k, v) = (text(fill: asi-navy, weight: "medium", k), text(weight: "light", v))
    let cells = (
      ..row("Document ID", doc-id), ..row("Version", version),
      ..row("Date", date), ..row("Author", author),
      ..row("Classification", classification),
    )
    if recipient != none { cells += row("Recipient", recipient) }
    table(
      columns: (7em, 1fr, 7em, 1fr),
      stroke: 0.4pt + asi-light.darken(25%),
      inset: 5pt,
      ..cells,
    )
  }
  v(1em)

  body
}
