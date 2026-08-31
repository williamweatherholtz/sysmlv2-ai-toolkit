// =====================================================================
// ProprietyRound3 — the panel RE-CONVENED over the federation suite AS BUILT, per the human's
// instruction that the panel be reapproached with findings (st040).
//
// Round 2 assessed a design and returned four standing defeaters. This round asks the harder
// question: did building it DISCHARGE them, and what does the built thing get wrong that the design
// could not have shown? Two of the five verdicts below are WORSE than their round-2 counterparts,
// which is the answer to whether a second round is a formality.
//
// INDEPENDENCE: same assessor as the author (claudeOpus5), so `sound` here is testimony, not a
// receipt (D0232). The load-bearing evidence is not the verdicts but the RUN: the suite convicted
// the product twice (issue313, issue314) and convicted its own author's tests three times, which is
// a discrimination record no verdict of mine can supply.
// =====================================================================
package ProprietyRound3 {
    private import EngineElement::*;
    private import EnginePropriety::*;

    part pf46 : ProprietyFinding {
        :>> id = "c1a30003-0830-4c1e-9f10-000000000046";
        :>> title = "federation-suite as built x overfit";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "suite:federation";
        :>> lens = ProprietyLens::overfit;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "pf41 asked F1 to cover the path-SHAPE class rather than the found instance, and it now runs plain, spaced and non-ASCII names plus a deep path. But the deep case DOES NOT hold the original claim: at depth the project still cannot commit, so the assertion was narrowed to 'init NAMES the risk and its remedy'. That is the honest thing to assert - keel does not own the git repository and cannot set core.longpaths for it - but the claim F1 makes at depth is strictly weaker than the claim it makes elsewhere, and a reader skimming green would not know that.";
        :>> defeater = "STANDS as a narrowing, not as a hole: the residual is that a project at depth remains uncommittable and the suite records only that it was warned. Discharged fully only by removing the cause - shorter shipped reference filenames - which changes paths downstream readers may already cite, so it is a judgment call and not an oversight. Named here so the narrowing is visible rather than implied by a green run.";
    }

    part pf47 : ProprietyFinding {
        :>> id = "c1a30003-0830-4c1e-9f10-000000000047";
        :>> title = "federation-suite as built x classCoverage";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "suite:federation";
        :>> lens = ProprietyLens::classCoverage;
        :>> verdict = ProprietyVerdict::sound;
        :>> rationale = "pf42 was the round's UNSOUND verdict: eight claims that all tested a project at birth and none at its second week. F9 was added for exactly that, and it earned the verdict immediately - it found issue314, where `keel migrate` resynced every shipped engine file over the project's copy and SILENTLY REVERTED the project's own adoption choice. A project that had deactivated a process had it switched back on by an engine upgrade, and the same path can switch one off, which is an unsigned control change of precisely the kind the keystone lock exists to prevent. The claim that a suite tests the easy day is refuted by the suite failing on the hard one.";
        :>> defeater = "TRIED AND FAILED: the obvious refutation is that F9 tests migration mechanics rather than federation, but the defect it found is federation-specific by construction - it can only occur where the engine and the project are separate artefacts moving at different speeds, which is what a federation IS. Sound.";
    }

    part pf48 : ProprietyFinding {
        :>> id = "c1a30003-0830-4c1e-9f10-000000000048";
        :>> title = "federation-suite as built x changeRobustness";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "suite:federation";
        :>> lens = ProprietyLens::changeRobustness;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "pf43 asked for assertions against COMMAND OUTPUT rather than file layout, and F3 now reads `keel business` while F4/F5 read `keel process show`. F2 and F3 still read DIRECTORIES (.engine/reference/decisions, .engine/decisions) because no command reports them - the mitigation was applied where it was available and not invented where it was not. Each surviving path assertion is paired with a positive population check, so a moved directory fails loudly rather than passing emptily, which is the part of pf43 that was fully discharged.";
        :>> defeater = "STANDS, reduced: a rename of `.engine/reference/decisions` breaks F2/F3 loudly rather than silently, which is the tolerable failure. Fully discharged only by a command that reports the reference population - worth building when something else needs it, not for this suite alone.";
    }

    part pf49 : ProprietyFinding {
        :>> id = "c1a30003-0830-4c1e-9f10-000000000049";
        :>> title = "federation-suite as built x circumventability";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "suite:federation";
        :>> lens = ProprietyLens::circumventability;
        :>> verdict = ProprietyVerdict::conditional;
        :>> rationale = "pf44 was the round's second UNSOUND: a silent skip reported as green. As built, F6 runs unless KEEL_FEDERATION_OFFLINE is DECLARED, and an absent `gh` with no declaration PANICS rather than skipping - so the default posture is now fail-loud. That is a real improvement. It is not a full discharge: the manifest test asserts only that a function EXISTS per claim, not that it EXECUTED, so a claim could be `#[ignore]`d or return early on some future condition and the manifest would still read complete. The failure mode pf44 named - a green run that never touched the real contract - is narrowed, not closed.";
        :>> defeater = "STANDS in the reduced form: existence-checking is not execution-checking. The honest closure is a run manifest the suite WRITES and a check READS - the same shape as the no-op coverage floor (D0261) - which is more machinery than this round earned and is recorded as the next increment rather than claimed as done.";
    }

    part pf50 : ProprietyFinding {
        :>> id = "c1a30003-0830-4c1e-9f10-000000000050";
        :>> title = "federation-suite as built x measurability";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> target = "suite:federation";
        :>> lens = ProprietyLens::measurability;
        :>> verdict = ProprietyVerdict::unsound;
        :>> rationale = "pf45 asked that 'is the project confused about whose purpose it serves' be made measurable rather than left as judgment, and F3b answers it with a text search for keel's domain vocabulary. The search terms are a HAND-PICKED LIST OF FOUR ('keystone lock', 'propriety panel', 'the engine's own', \"keel's Needs\"). That is the overfit lens applied to the measurability fix: the list was chosen by the same author who knows which terms he did not leak, it passes trivially on a scaffold that ships no .tracking content at all, and any future leak using a term not on the list is invisible. The claim F3b makes - keel's vocabulary does not leak - is much wider than what it measures, which is 'these four strings are absent'.";
        :>> defeater = "DEMONSTRATED by construction, not merely possible: adding a fifth keel-specific term to a fresh project's .tracking would pass F3b unchanged. Routed to work. The defensible form derives the vocabulary rather than listing it - for instance, terms frequent in KEEL's OWN tracking corpus and absent from a generic project - so the measure grows with the engine instead of with the author's memory.";
    }
}
