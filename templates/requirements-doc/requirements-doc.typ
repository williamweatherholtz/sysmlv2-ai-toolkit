// Requirements Document template — a discrete set of requirements sent to a recipient (D0235).
// Data in, PDF out; the template holds NO content. Render via scripts/render.py.
// CONTENT CONTRACT (templates/CONTENT-FORMAT.md, resolves issue001):
//   - requirements[].statement, ids, meta: LITERAL strings, inserted directly. NEVER eval().
//   - content / rationale / notes: CommonMark subset + $-math via the shared md-content (D0236).
#import "/templates/base/asi-base.typ": asi-doc, asi-blue, asi-navy, asi-light, asi-must, md-content

#let data-path = sys.inputs.at("data", default: "/templates/requirements-doc/example-data.json")
#let data = json(data-path)

// Closed priority enum — an out-of-enum value stops the render with a named error (D0235 A2).
// The panic names the defect class, not the data (log hygiene).
#let prio-badge(p) = {
  let colors = ("must": asi-must, "should": asi-blue, "may": asi-navy)
  if p not in colors { panic("priority not in closed enum (must/should/may)") }
  text(fill: colors.at(p), weight: "bold", upper(p))
}

// A prose field counts as present only when non-blank (dangling-label critique finding).
#let has(r, k) = k in r and r.at(k).trim() != ""

#show: asi-doc.with(meta: data.meta)

#if "content" in data [
  = Introduction
  #md-content(data.content)
]

= Requirements

#let n = data.requirements.len()
#if n == 1 [
  The following requirement constitutes the complete set conveyed by this document. It carries
  a stable identifier, a priority, and its verification method. Requirements are normative;
  supporting notes are informative.
] else [
  The following #n requirements constitute the complete set conveyed by this document. Each
  requirement carries a stable identifier, a priority, and its verification method.
  Requirements are normative; supporting notes are informative.
]

#table(
  columns: (auto, 1fr, auto, auto),
  stroke: 0.4pt + asi-light.darken(25%),
  inset: 6pt,
  fill: (_, row) => if row == 0 { asi-light } else { none },
  table.header(
    text(fill: asi-navy, weight: "medium")[ID],
    text(fill: asi-navy, weight: "medium")[Requirement],
    text(fill: asi-navy, weight: "medium")[Priority],
    text(fill: asi-navy, weight: "medium")[Verification],
  ),
  ..for r in data.requirements {
    (text(weight: "medium", r.id), r.statement, prio-badge(r.priority), r.verification)
  }
)

#if data.requirements.any(r => has(r, "notes") or has(r, "rationale")) [
  = Requirement Detail

  #for r in data.requirements.filter(r => has(r, "notes") or has(r, "rationale")) [
    == #r.id
    #r.statement

    #if has(r, "rationale") [ / Rationale: #md-content(r.rationale) ]
    #if has(r, "notes") [ / Notes: #md-content(r.notes) ]
  ]
]
