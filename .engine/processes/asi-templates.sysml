// asiTemplates — the ASI document kit as ONE transferable unit: brand currency, the content
// contract, both output surfaces, and the gate that holds them.
//
// SUPERSEDES the standalone `exec-summary` unit (this project's D0238). That unit carried its
// process definition and skill and NOTHING ELSE — `keel process show exec-summary` listed three
// files, so the template it consumes, the contract checker that gates it and the browser suite that
// verifies it all stayed home. A unit whose teeth stay behind is the D0219 class exactly: the
// discipline travelled by name and the machinery travelled not at all. This unit carries the
// machinery.
//
// WHY ONE UNIT AND NOT THREE. The two surfaces are not independent: both render from the same
// brand tokens, both obey the same content contract (templates/CONTENT-FORMAT.md), and both are
// gated by scripts/check_templates.py. Split into per-surface units, that shared base is either
// duplicated in each unit's extras or declared in prose `requires` that nothing enforces — and a
// consumer who imports a surface without its base gets silent font substitution and an
// unenforced contract rather than a refusal. The human chose the single unit on that basis.
package ProcessAsiTemplates {
    private import EngineElement::*;
    private import EngineProcess::*;

    action asiTemplates : Process {
        :>> id = "e7384d91-64dd-4eed-8192-13884fe87cd9";
        :>> title = "ASI document templates";
        :>> purpose = "Produce ASI-branded deliverables on one of two declared surfaces - an interactive exec-summary HTML decision page, or a formal requirements-doc PDF - from committed source templates that carry the brand tokens, obey one content contract for literal vs marked-up prose, and refuse to ship unless their gate and suite pass.";
        :>> cadence = Cadence::perItem;
        // APPLIES-WHEN: a deliverable leaves this project as a DOCUMENT a human reads - findings or
        // a decision request (exec-summary), or a formal specification (requirements-doc). Terminal
        // prose and ad-hoc HTML are what this process exists to replace.
    }

    action atBrandCurrency : ProcessStep {
        :>> id = "e3b5c1c9-08e5-4c3a-b942-96aba85dcb77";
        :>> title = "Check the brand has not drifted before building against it";
        :>> triggerCondition = "Template work is about to start, or a brand recheck is requested.";
        :>> actionText = "The visual identity derives from sources marketing owns and changes without telling anyone, so drift must be COMPUTED rather than noticed. Snapshot the watched sources and diff against the committed baseline: `python .claude/skills/brand-watch/check_manifest.py brand/manifest.json <fresh>`. Folder size is a recursive change signal; a null on either side is UNOBSERVED and is skipped, never reported as drift and never given a fabricated hash. No drift: say so and touch nothing - do not refresh the baseline to bump its date. Drift: in ONE commit record an Issue naming exactly which sources moved, update the baseline to the fresh snapshot, and refresh the derived mirrors (style tokens, ingest). A change absorbed without its Issue is the silent staleness this step exists to prevent.";
        :>> producedArtifact = "A drift verdict; on drift, an Issue plus a reconciled baseline.";
        :>> owner = Owner::ai;
    }

    action atChooseSurface : ProcessStep {
        :>> id = "bbb8a136-ca76-4954-b147-418c3b7087b5";
        :>> title = "Choose the surface from what the reader must DO";
        :>> triggerCondition = "Brand currency is established and a deliverable is due.";
        :>> actionText = "Two surfaces, chosen by the reader's action and never by preference. EXEC-SUMMARY (templates/exec-summary/exec-summary.html) when the reader must DECIDE: findings, options, a sign-off request - anything terminal text would bury. REQUIREMENTS-DOC (templates/requirements-doc/requirements-doc.typ) when the reader must be able to CITE a stable statement: a formal specification, an interface document, anything that will be baselined or reviewed clause by clause. A decision request rendered as a formal PDF buries the ask; a specification rendered as tabs cannot be cited. Both draw metadata, header, footer and every colour from brand/style-tokens.json - no value is hardcoded in either template.";
        :>> producedArtifact = "The named surface, with the reader-action that chose it.";
        :>> owner = Owner::ai;
    }

    action atHoldContentContract : ProcessStep {
        :>> id = "1bbe3765-cd51-4899-850f-76f927a2835f";
        :>> title = "Fill from the source template; hold the two field classes";
        :>> triggerCondition = "A surface is chosen.";
        :>> actionText = "Start from the SOURCE template and keep its tested machinery, replacing only content - improvements go to the source template, never into a copy, and every rendered artifact's footer links back to that source rather than to itself. Then hold templates/CONTENT-FORMAT.md, which admits exactly two field classes and nothing else. LITERAL fields (meta.*, requirement id and statement) render character for character with no markup interpretation in any renderer: a statement containing a dollar amount, a hash, an at-sign or angle brackets renders exactly those characters, inserted as content and NEVER through eval() - that route was issue001, which silently deleted a performance bound from a must-requirement and executed repo reads embedded in instance data. CONTENT fields (content, rationale, notes) parse as the declared CommonMark subset only. A construct outside the subset is refused loudly at the renderer, never silently dropped.";
        :>> producedArtifact = "A filled instance whose every field obeys its declared class.";
        :>> owner = Owner::ai;
    }

    action atGate : ProcessStep {
        :>> id = "672ce54b-7ec6-4f83-9984-588473d02415";
        :>> title = "Run the surface's gate; an untested document is not delivered";
        :>> triggerCondition = "The instance is filled.";
        :>> actionText = "The contract checker runs on every commit that touches templates (D0237, .githooks/pre-commit): `python scripts/check_templates.py` DERIVES its field inventory from the source template rather than a hand-list, so every data-d and data-digest key must exist in each instance and no prose field may still read as the template's own wording - an unreplaced placeholder is caught, not shipped. Then the surface's own suite. EXEC-SUMMARY: `EXEC_SUMMARY_FILE=<page> .venv/Scripts/python -m pytest tests/exec_summary -q` - real browsers, because one-click copy, tab wiring, 44px targets and one-tap-at-iPhone-scale are BEHAVIORAL claims that must be verified rather than asserted. REQUIREMENTS-DOC: `.venv/Scripts/python scripts/render.py <instance> --check` - schema validation refusing to render on failure, a zero-warning compile, field-map read-back by PDF text extraction asserting statements survive VERBATIM, and the golden pixel diff. Re-baselining a golden requires the author's approval, never the AI's.";
        :>> producedArtifact = "A passing gate run for the chosen surface, cited by command.";
        :>> owner = Owner::ai;
    }

    action atDeliver : ProcessStep {
        :>> id = "7f8ed755-777d-466f-a2f0-6143461be412";
        :>> title = "Deliver with provenance; a returned digest is a decision record";
        :>> triggerCondition = "The gate passes.";
        :>> actionText = "Deliver, with provenance on the artifact itself: WHICH PROJECT it asks about, who generated it, when, and from which source template. The source project is named in three places and the contract checker enforces all three - a header badge, the page title so the browser tab carries it, and the FIRST line of the Copy-for-AI digest. The reason is not tidiness: the author reads these with many pages open at once, and a paste arrives in a session that cannot see which tree produced it, so a signature given against the wrong project's claims is the failure this prevents (st010). EXEC-SUMMARY: the Copy-for-AI digest the reader pastes back carries their selections as checkbox markdown plus their own notes - a STATEMENT about specific claims, which is what an acceptance gate can honestly record, and what the intake discipline wants (their words verbatim, then the reading). It is never itself a human sign-off: recording method=confirmation still needs their explicit word on that specific claim. REQUIREMENTS-DOC: the rendered PDF plus the declared output set; a field declared in outputs.json that the harness cannot actually assert fails the run rather than passing on a promise.";
        :>> producedArtifact = "A delivered document; on return, a recordable statement.";
        :>> owner = Owner::ai;
    }

    first atBrandCurrency then atChooseSurface;
    first atChooseSurface then atHoldContentContract;
    first atHoldContentContract then atGate;
    first atGate then atDeliver;
}
