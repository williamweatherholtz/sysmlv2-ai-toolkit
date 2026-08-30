// =====================================================================
// ProprietyRound4 — the panel over VINTAGE RESOLUTION: should an item's authoring vintage be
// STAMPED on it (the Django `django_migrations` shape the human named) or DERIVED from git as it is
// today? Convened at the human's instruction (st048) BEFORE any implementation, to produce ranked
// proposals rather than a fait accompli.
//
// THE MEASURED GROUND, gathered before the panel sat, because two of the five verdicts turn on it:
//   * The capability EXISTS: `governing-version <item>` resolves, per item, the process definition
//     commit that governed it as-of its charter, and offers `git show <sha>:<def>` to read the
//     process AS IT WAS. Run against a June item it answered correctly, naming one later version
//     and two prospective process changes.
//   * `reprocess-candidates` — the lens that answers "what antiquated data needs revisiting" —
//     TIMED OUT at 240 seconds (exit 124, zero bytes). It walks 501 delivery stories with per-item
//     git subprocesses.
//   * 447 of 501 stories carry a `#CharteredBy` edge, so the derivation's INPUT is present for
//     stories. Nothing else in the corpus has one.
//   * A non-story item falls back UNCONDITIONALLY to Delivery (govern.rs:19,39) - so asking for a
//     Need's vintage returns a confident wrong answer rather than "unknown".
//
// INDEPENDENCE: assessor is claudeOpus5, who also wrote none of the code under assessment (govern.rs
// predates this session). That makes this the FIRST round in the series with genuine separation
// between author and critic - noted because every prior round had to discount its own `sound`
// verdicts as testimony (D0080/D0232).
// =====================================================================
package ProprietyRound4 {
    private import EngineElement::*;
    private import EnginePropriety::*;

    part pf51 : ProprietyFinding {
        :>> id = "c1a40004-0830-4c1e-9f10-000000000051";
        :>> title = "derive-vs-stamp x overfit";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:vintage-resolution";
        :>> lens = ProprietyLens::overfit;
        :>> verdict = ProprietyVerdict::sound;
        :>> rationale = "The DERIVED design is the general one and the STAMPED alternative is the overfit. migrate.rs states the case in its own header: a stored vintage stamp can drift from the tree it claims to describe, is ABSENT from every project inited before the stamp existed, and would let a migration step run twice. Derivation has none of those failure modes because the tree IS the evidence - a step whose pattern no longer matches plans zero edits, which delivers vintage detection, idempotency and dry-run fidelity from one property. Django needs its migrations table because a database cannot be inspected for which migrations shaped it; a git tree can.";
        :>> defeater = "TRIED: the strongest case for stamping is a tree whose history was rewritten or shallow-cloned, where ancestry is unavailable. It fails on this project's own terms - D0129/issue071 already forbid rebase and force-push precisely because a passing TestResult counts only while its judgedAgainst SHA resolves, so a tree with unusable ancestry has already lost more than vintage.";
    }

    part pf52 : ProprietyFinding {
        :>> id = "c1a40004-0830-4c1e-9f10-000000000052";
        :>> title = "derive-vs-stamp x classCoverage";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:vintage-resolution";
        :>> lens = ProprietyLens::classCoverage;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "The class is 'any authored item, what governed it when it was written'. The implementation covers ONE member: delivery Stories. `reprocess-candidates` iterates `all_delivery_stories` (govern.rs:362) and nothing else, and `governing_def_for` falls back UNCONDITIONALLY to the Delivery workflow (govern.rs:19,39) when no charter edge is found. So asking for the vintage of a Need, a SystemRequirement, an Issue, a Test or a Statement does not return 'unknown' - it returns DELIVERY, confidently and wrongly. A lens that answers the wrong question fluently is worse than one that refuses, because nobody checks a fluent answer.";
        :>> defeater = "DEMONSTRATED by reading the resolver, not merely possible: the fallback has no guard on item kind. The human's intuition - that antiquated data carries no indication what process authored it - is CORRECT for every item type except Story, and for Story it is wrong. Routed to work.";
    }

    part pf53 : ProprietyFinding {
        :>> id = "c1a40004-0830-4c1e-9f10-000000000053";
        :>> title = "derive-vs-stamp x changeRobustness";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:vintage-resolution";
        :>> lens = ProprietyLens::changeRobustness;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "Derivation is robust to SCHEMA change by construction - it reads git, not fields - but it is coupled to two incidentals. It greps for the literal `#CharteredBy dependency from X to`, so the marker's spelling is load-bearing; and it identifies the governing definition as the first `.sysml` file under `.engine/processes` or `.engine/workflows` touched by the charter Decision's commit, so a Decision that touches TWO process files resolves to whichever git lists first. Neither is announced anywhere a reader would look.";
        :>> defeater = "STANDS on the multi-file case, which is not exotic: several Decisions this week touched a process AND its skill registry. The single-file assumption is silent, and a wrong-but-plausible governing definition is the failure mode of pf52 in a second form.";
    }

    part pf54 : ProprietyFinding {
        :>> id = "c1a40004-0830-4c1e-9f10-000000000054";
        :>> title = "derive-vs-stamp x circumventability";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:vintage-resolution";
        :>> lens = ProprietyLens::circumventability;
        :>> verdict = ProprietyVerdict::sound;
        :>> rationale = "A derived vintage cannot be forged without forging git history, which the branch policy already forbids and CI already re-derives (audit-history over every pushed range, K15/D0179). A STAMPED vintage, by contrast, is a field like any other: an author can write last month's version into this week's item and nothing detects it, because the stamp is the only witness to itself. On this lens the two designs are not close.";
        :>> defeater = "TRIED: a stamp could be guard-checked against the commit that introduced the item - but that check IS derivation, so the stamp adds a second truth to maintain and settles nothing it did not already borrow.";
    }

    part pf55 : ProprietyFinding {
        :>> id = "c1a40004-0830-4c1e-9f10-000000000055";
        :>> title = "derive-vs-stamp x measurability";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "design:vintage-resolution";
        :>> lens = ProprietyLens::measurability;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "A measure nobody can take is not a measure. `reprocess-candidates` is the lens that answers the human's actual question - which items were authored under a process a SAFETY change later superseded - and it does not finish: 240 seconds, exit 124, zero bytes, because it runs several git subprocesses per story across 501 stories. So the corpus-wide answer has never been computed on this project, and the per-item answer is available only to someone who already suspects a particular item, which is the exact shape D0261's retro named as 'a metric only readable by someone who already knows the answer is not a control'.";
        :>> defeater = "DEMONSTRATED by measurement, twice (240s timeout; the per-item call also ran long enough to be backgrounded). The fix is not a design change - the derivation is right - but a batched traversal: one `git log --name-only` pass building a commit-to-definition map, then a lookup per item. Routed to work as the highest-value increment, because it converts an existing correct answer from unobtainable to instant.";
    }
}
