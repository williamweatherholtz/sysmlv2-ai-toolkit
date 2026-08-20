// =====================================================================
// D0095 — the engine is named KEEL. "sysmlv2" is the METHODOLOGY (SysML v2 text), not the product.
// The keel is the hidden structure that lets a ship carry FULL SAIL and drive fast + true instead of
// capsizing — discipline isn't a restraint on the AI, it's what converts raw power into clean forward
// motion (the POSITIVE reframe of "leash"/control). Short, ownable, excellent CLI verb
// (`keel orient`/`keel init`/`keel serve`). The RENAME itself is DEFERRED to a tracked bulk migration
// (engineRename, D0067) — captured now per section-2.4 (record decisions even when they cause no action).
// Accepted 2026-06-23 by wweatherholtz (chose "Keel" from the naming brainstorm; "that was the tweak. proceed").
// =====================================================================
package Decision0095 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // D0070: the engineRename execution edits process-def files (cosmetic command-name refs), so this
    // governing decision is marked #ProspectiveChange (the keystone — a process-def change is decision-backed).
    #ProspectiveChange part d0095 : Decision {
        :>> id = "95a6b7c8-1d6e-4f70-a829-516273849595";
        :>> title = "D0095: the engine is named Keel (rename deferred to the engineRename migration)";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The engine has been called 'sysmlv2', but that is the METHODOLOGY it is built on (SysML v2 text), not the product's identity. It needs a real name reflecting its POSITIVE value: the discipline that makes AI-driven work powerful, distilled, and cleanly driving. The human explored the space (leash = control but negative; jetfuel = power) and asked to focus on the positive outcome — discipline that ENABLES power.";
        :>> decision = "Name the engine KEEL. The keel is the hidden structure that lets a ship carry full sail and drive fast + true instead of capsizing — the discipline is not a restraint on the AI, it is what converts raw power into clean forward motion (the positive reframe of 'leash'). Short, ownable, and an excellent CLI verb (`keel orient` / `keel init` / `keel serve`). The RENAME itself (binary `sysmlv2` -> `keel`, the cargo package, every doc, the embedded init scaffold, CLAUDE.md) is DEFERRED to a tracked bulk migration (engineRename) under the D0067 expand/migrate/contract discipline with reconciled control totals — NOT performed now. Recorded per section-2.4 (capture the decision even though it causes no immediate action).";
        :>> rationale = "ALTERNATIVES (naming brainstorm): leash — REJECTED (frames the discipline as restraint, the negative the human wants to avoid); jetfuel / octane / thrust / ignition — power, but miss the discipline-ENABLES-power reframe; vector (power + direction) and mainspring (powerful + precise) — strong runners-up; flywheel / forge / crucible / refinery / throughline / spine. CHOSEN: Keel — it best captures 'discipline -> power' (the keel lets you fly full sail without capsizing) AND has the best CLI ergonomics (4 letters, clean command). CRITERIA: positive-outcome-framing x discipline-enables-power x ownable x short-CLI-verb.";
        :>> consequences = "The engine's name is Keel. The rename is the engineRename backlog item — a D0067 bulk migration (binary/package/docs/embedded-scaffold/CLAUDE.md; expand -> migrate -> contract; control totals reconciled; green at every step), to be chartered as its own sprint. No action until then; 'sysmlv2' remains the working command name in the interim. The D0094 console + the broader product surface carry the Keel identity once the rename lands. ROLLOUT: [2026-06-23] engineRename EXECUTED (Sprint 107) — binary -> keel, crates -> keel-cli/keel-parser (git mv), 78 code identifiers + 55 live files swept; `.sysml`/'SysML v2'/historical records preserved; this commit co-records the process-def edits per the D0070 keystone. The methodology stays SysML v2; the product is Keel.";
    }

    verification d0095Accept : Test { :>> id = "95a6b7c8-1d6e-4f70-b829-516273849595"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of d0095 (the engine is named Keel — the discipline-enables-power reframe; the rename itself deferred to the engineRename D0067 migration) by selecting 'Keel' from the naming options and saying 'that was the tweak. proceed' on 2026-06-23."; }
    part d0095AcceptR1 : TestResult { :>> id = "95a6b7c8-1d6e-4f70-c829-516273849595"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6e8bb7a"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "wweatherholtz"; }
}
