// =====================================================================
// ProprietyRound5 — the panel over the exec-summary SKILL, i.e. over the GENERATION of these pages,
// convened at the human's instruction after they rejected two of the three summaries produced this
// session.
//
// THE EVIDENCE IS UNUSUALLY GOOD, and it is not mine: three pages were generated from this skill in
// one session and the human rejected two of them in their own words. "I'm not seeing an executive
// summary" (produced no page at all when decisions were pending). "These executive summaries need to
// be improved. Insufficient context... WHAT DECIDES IT: picking which ones are good." A skill whose
// output is rejected twice by its only reader has a defect the skill text can be read for.
//
// INDEPENDENCE: the skill was authored in another project and NOT by this assessor; the failing
// pages WERE mine. So the verdicts below are testimony about my own output and a genuinely
// independent read of the instrument that produced it - which is the more useful half, because a
// bad page can be my fault while the skill still failed to prevent it.
// =====================================================================
package ProprietyRound5 {
    private import EngineElement::*;
    private import EnginePropriety::*;

    part pf51 : ProprietyFinding {
        :>> id = "c1a50005-0901-4c1e-9f10-000000000051";
        :>> title = "exec-summary skill x overfit";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> target = "skill:exec-summary";
        :>> lens = ProprietyLens::overfit;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "The skill is written almost entirely about the ARTEFACT - the template, the tab wiring, the word budgets, the copy control, the mobile targets - and barely at all about the JUDGMENT the page exists to obtain. Roughly forty lines govern markup and machinery; the steelman rule is the only substantive constraint on CONTENT, and it constrains how an option is phrased rather than what the reader needs to know to choose. That is a fit to the medium, not to the purpose.";
        :>> defeater = "STANDS, and it is what produced the failure: a page can satisfy every structural rule in this skill - tabs wired, budgets met, suite green - and still be unjudgeable, which is exactly what the human received and named. My pages passed the tests. The tests were about the artefact.";
    }

    part pf52 : ProprietyFinding {
        :>> id = "c1a50005-0901-4c1e-9f10-000000000052";
        :>> title = "exec-summary skill x classCoverage";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> target = "skill:exec-summary";
        :>> lens = ProprietyLens::classCoverage;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "The class is 'the reader can adjudicate this without leaving the page'. The skill covers the FRAME of a decision - question, options, costs, criterion - and never requires the SUBSTANCE: the item's own words, what accepting it binds, what is already true versus assumed, what happens on each answer. So a tab may name `nAutonomyBoundedByProof` and ask whether to accept it, which is a well-formed frame around nothing, and the reader must open the repository to know what they are deciding. The `driver` line makes it worse by permitting a restatement - nothing in the skill forbids 'what decides it: which one is better'.";
        :>> defeater = "DEMONSTRATED, twice, by the reader rather than by analysis. Routed to work: the sufficiency rule and the driver test added on 2026-09-01 close exactly this, and they were written into a DEPLOYED COPY rather than the unit - see pf55, which is why the fix has not travelled.";
    }

    part pf53 : ProprietyFinding {
        :>> id = "c1a50005-0901-4c1e-9f10-000000000053";
        :>> title = "exec-summary skill x changeRobustness";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> target = "skill:exec-summary";
        :>> lens = ProprietyLens::changeRobustness;
        :>> verdict = ProprietyVerdict::sound;
        :>> rationale = "The skill delegates its structural rules to a TEST SUITE rather than restating them: the budgets are enforced by test_prose_stays_within_executive_budgets, the contract by a page-contract check. So the skill's prose can drift from the template without the pages silently degrading - the suite catches it. Observed live three times this session: the budgets rejected drafts at 36, 21 and 15 words against their limits, and each rejection improved the line.";
        :>> defeater = "TRIED: the obvious refutation is that a skill delegating to a suite breaks when the suite moves. It does not fail silently, because the suite is what the author runs before publishing and its absence is loud. The delegation is the robustness, not a dependency.";
    }

    part pf54 : ProprietyFinding {
        :>> id = "c1a50005-0901-4c1e-9f10-000000000054";
        :>> title = "exec-summary skill x measurability";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> target = "skill:exec-summary";
        :>> lens = ProprietyLens::measurability;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "Everything the skill can MEASURE is a count: words per field, cards per tab, tabs per page. Everything that determines whether the page works is qualitative - is the steelman fair, is the criterion real, is the item's substance present. The skill knows this and offers one honest self-check ('would that option's advocate feel fairly represented?'), which is a judgment prompt rather than a measure. The gap is not a flaw in the skill so much as the nature of the artefact, but it means a GREEN SUITE CANNOT MEAN A GOOD PAGE, and nothing in the skill says so.";
        :>> defeater = "STANDS. Partially closeable: the sufficiency rule's items are structural enough to check mechanically - does the tab quote the item, does it name what accepting binds, does the driver line differ from the question. A check for 'the driver restates the legend' would have caught the exact failure the human reported. Not built here; named as the increment.";
    }

    part pf55 : ProprietyFinding {
        :>> id = "c1a50005-0901-4c1e-9f10-000000000055";
        :>> title = "exec-summary skill x circumventability";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> target = "skill:exec-summary";
        :>> lens = ProprietyLens::circumventability;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "The skill is circumvented not by ignoring it but by EDITING THE WRONG COPY, and I did exactly that hours ago. Three copies exist: the library unit (v1, authoritative, complete), a project's `.claude/skills/` DEPLOYED copy, and nothing in between - because the unit's own source project, se-templates, records exec-summary as installed in installed-units.toml and holds NONE of its files: no process file, no .engine skill, no deployed copy. I improved the deployed copy in a different project. That copy is regenerated output; `sync-claude` can overwrite it, and the library unit is unchanged at v1, so the fix reaches no other project and may not survive in the one it was made in.";
        :>> defeater = "DEMONSTRATED, not hypothetical: the library unit's SKILL.md is 121 lines with zero occurrences of the sufficiency rule; the deployed copy I edited is 161 lines with one. Routed to work - the unit needs a source project that actually holds it, the improvement re-applied there, and a v2 published. An install record naming files that are absent is itself a defect nothing currently detects.";
    }
}
