// D0159 — PROPOSED. The unit machinery SHOULD generalize to viewpoints, and there is no demand for it
// today: `keel init` already ships all 32 to every project and no project has ever authored one. Records
// the design so whoever needs it inherits the analysis instead of rediscovering it. Blocks nothing.
// #ProspectiveChange (D0070): records a direction and declines scope; no requirement is derived.
package Decision0159 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0159 : Decision {
        :>> id = "6a35c81f-9d02-4b47-8e6a-70fb1c45d9e2";
        :>> title = "D0159: viewpoints are a second KIND of portable unit — the concept generalizes, the command waits for demand";
        :>> createdAt = "2026-08-17";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "issue138, from the human's question 'can viewpoint sharing not use the same mechanisms as process sharing? what's the issue here?' The answer is that it can, and MEASURED, the shapes are the same: `keel process export` writes .engine-relative files plus a `unit.toml` naming the skills, rules and guards a receiving project must have and activate. A viewpoint has 0..1 `.view.toml` to copy (7 of 32 have one), one declaration in a shared registry, and one renderer naming a keel subcommand (21 of 32) the receiving binary must possess. BOTH APPARENT OBSTACLES ALREADY HAVE ANSWERS INSIDE THE PROCESS MECHANISM: a declaration in a shared file is carried BY NAME in the manifest, exactly as process_cmd.rs already does for rules; and a dependency on binary capability is carried by name with the import reporting it, exactly as unit.toml already does for guards. So `unit.toml` is UNIT-shaped and merely NAMED after processes. BUT THE DEMAND IS PROVABLY ZERO, and that is what decides this: all 32 viewpoints are ENGINE-authored, `keel init` ships `.engine/views/` to every downstream project so every project starts holding the whole curated set — the same palette answer processes have — and no project has ever authored a viewpoint of its own. The only unserved case is a project-authored viewpoint being handed to another project, and that case has never occurred.";
        :>> decision = "PROPOSED: the CONCEPT generalizes and is recorded here; the COMMAND is not built. Nothing is added to `keel process export`, no `keel unit` command is introduced, and no requirement is derived. When it is built, this is the shape: viewpoints become a second unit KIND rather than a separate command — `unit.toml` gains a `kind`, a viewpoint bundle carries its `.view.toml` if it has one plus its declaration by name plus its renderer by name, and IMPORT REPORTS a renderer the receiving binary does not have rather than landing a lens that cannot render. That last clause is the load-bearing one, and it is the direct analogue of the warning unit.toml already carries about guards: importing the files alone would land the concern and leave the ANSWER behind. THE TRIGGER: a project authors its own Viewpoint and needs to hand it to another project. Until then this is a solution to a problem nobody has.";
        :>> rationale = "WHY NOT BUILD IT NOW, when the design is this clear and the code would be small: because D0157 was accepted five records ago on exactly this reasoning, and it would be incoherent to apply that discipline to the human's speculative feature and not to my own. A capability built for a demand that does not exist has to be maintained, documented, kept green by 34 guards, and reasoned about by every future reader — and the first real user would probably want it slightly differently anyway, because the requirement would be written against a real transfer instead of an imagined one. WHY RECORD THE DESIGN RATHER THAN JUST CLOSING THE ISSUE: the analysis cost real measurement — the file counts, the renderer counts, and finding that both obstacles already had precedents — and that work is what a future implementer would otherwise repeat. A Decision costs nothing to hold and no computation keys on it. WHY PROPOSED RATHER THAN ACCEPTED: declining to ADD scope is the default and needs no human word, but this Decision also asserts a design shape for future work, and that assertion is theirs to accept or redirect. It blocks nothing while it waits — no action depends on it.";
        :>> consequences = "issue138 is dispositioned acceptRisk and resolves: the gap is real, known, and knowingly not acted on — an AI may disposition a LOW finding (D0080), and the guard requires human authority only at or above Medium. THE ACCEPTED RISK, stated: if a project authors a viewpoint tomorrow, moving it to another project is a manual file edit into a shared registry with nothing checking that the receiving binary can render it. That is a small, visible, recoverable cost, and it is bounded by the fact that the receiving project's `viewpoint-renderer` guard would catch an unrenderable lens on its next commit — which is only true because sprint 322 made that guard see every viewpoint rather than one file's worth (issue139).";
    }

    #Resolves dependency from d0159 to issue138;

    // ── The disposition of the finding (D0092/D0080): AI-judged because issue138 is LOW. The authority
    // policy requires a human at or above Medium; the guard's own doc records the precedent that a guard
    // demanding a human on every disposition would fail a correct AI-dispositioned Low finding.
    verification issue138Disp1 : Test {
        :>> id = "2d9f6084-71c3-4e15-b8a7-05ce39b41f26";
        :>> title = "issue138 disposition — acceptRisk: the concept generalizes, the demand is zero";
        :>> createdAt = "2026-08-17";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::analyze;
        :>> disposition = DispositionKind::acceptRisk;
        :>> procedureText = "acceptRisk, not dismiss: the finding is REAL — a viewpoint is the same portable unit shape as a process and only processes can be exported — and it is knowingly not acted on. The measurement that decides it: all 32 viewpoints are engine-authored, `keel init` ships the whole set to every downstream project, and no project has ever authored one, so the unserved case (a project-authored viewpoint handed to another project) has never occurred. Dismissing it would assert the gap is not real, which would be false and would lose the analysis. The residual risk is bounded and named in D0159's consequences, and it is smaller than it was yesterday: a receiving project's viewpoint-renderer guard would catch an unrenderable imported lens, which became true only when sprint 322 made that guard see every viewpoint instead of one file's worth."; }
    part issue138Disp1R1 : TestResult { :>> id = "83b1e5c7-4a06-4d29-9f13-6b027ae1cd85"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "PENDING"; :>> judgedAt = "2026-08-17"; :>> judgedBy = "claudeOpus5"; }
    #Dispositions dependency from issue138Disp1 to issue138;
}
