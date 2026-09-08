# Requirement-Quality Ruleset (Q1–Q31)

Grounded in INCOSE *Guide to Writing Requirements* v4 (characteristics + R1–R42)
and EARS (Mavin et al.). Each rule is a binary pass/fail check on a single
requirement, except Q28–Q31 which run across a set. Cite the rule ID in every
finding.

## Group 1 — Structure & language (Accuracy / Uniformity)
- **Q1** Exactly one binding **"shall"**. FAIL on missing "shall" or use of
  will/must/should/may for a binding requirement.
- **Q2** Active voice with a named, definite responsible subject. FAIL on passive
  ("data shall be encrypted") or no identifiable actor.
- **Q3** Definite, specific subject ("the Brake_Controller"), not "a/the system"
  generically.
- **Q4** No pronouns referring outside the statement (it, they, this, these).
- **Q5** Uses the project's defined term per entity (in glossary). FAIL on
  synonyms / undefined acronyms.
- **Q6** States *what*, not *how* (solution-free) — UNLESS the item is a
  `Constraint`.

## Group 2 — Singularity
- **Q7** A single capability/characteristic/constraint per statement.
- **Q8** No conjunctions joining independent requirements (and, or, then,
  unless, as well as).
- **Q9** No literal "and/or" token.
- **Q10** No oblique "/" used as a connective ("input/output", "pass/fail").
- **Q11** No purpose/parenthetical clause adding a second obligation ("in order
  to…", "so that…").

## Group 3 — Non-ambiguity
- **Q12** No vague adjectives/adverbs: fast, slow, easy, user-friendly,
  flexible, robust, efficient, adequate, reasonable, sufficient, quickly,
  approximately, normally, state-of-the-art.
- **Q13** No vague quantifiers: some, several, many, few, most, various, a
  number of.
- **Q14** No open-ended clauses: "etc.", "and so on", "including but not limited
  to", "such as", "and the like".
- **Q15** No escape clauses: "if possible", "where possible", "as appropriate",
  "as required", "if necessary", "to the extent practical".
- **Q16** No superfluous infinitives: "be able to", "capable of", "be designed
  to". Replace with direct "shall <verb>".
- **Q17** Not stated as a negative obligation where a positive measurable form
  exists ("shall not fail").

## Group 4 — Quantification
- **Q18** Every performance/quality claim has a measurable value.
- **Q19** Every numeric value carries an explicit unit; units consistent (no
  metric/imperial mixing).
- **Q20** Numeric targets specify tolerance/range where needed ("2.0 ± 0.3 s").
- **Q21** Universal quantification is explicit and bounded ("each", "every" with
  defined scope), never implied.

## Group 5 — Conditions, completeness, feasibility
- **Q22** Conditions/triggers/states stated in an EARS preamble BEFORE the
  "shall", not buried mid-clause.
- **Q23** Complete: no TBD/TBR/TBC without a tracked closure attribute.
- **Q24** Verifiable: an Inspection/Analysis/Demonstration/Test could objectively
  confirm it.
- **Q25** Feasible/realistic — no physically or technically impossible target.
- **Q26** Necessary — traces to a parent need/higher requirement (not
  gold-plating); no orphan.
- **Q27** Appropriate to its level (system vs subsystem vs component); not over-
  or under-specified.

## Group 6 — Set-level (run across the collection)
- **Q28** Consistent terminology (same concept = same term).
- **Q29** No conflicts/contradictions between requirements.
- **Q30** No duplicates; each requirement stated once.
- **Q31** Set complete — covers all parent needs (no coverage gaps in the trace).

## Sources
- INCOSE GtWR v4 summary: https://www.incose.org/docs/default-source/working-groups/requirements-wg/guidetowritingrequirements/incose_rwg_gtwr_v4_summary_sheet.pdf
- reqi.io 42-rule guide: https://reqi.io/articles/incose-requirements-quality-42-rule-guide
- EARS: https://alistairmavin.com/ears/
- SEBoK Requirements Management: https://sebokwiki.org/wiki/Requirements_Management
