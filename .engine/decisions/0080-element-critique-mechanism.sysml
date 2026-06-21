// =====================================================================
// D0080 — antagonistic critique as a lens-tagged verification: the critique mechanism (schema +
// process + skill). Implements D0079's critique-as-verification. Frozen-schema change to
// schema/core (new VerificationMethod::critique + Severity/CriticKind/CritiqueLens enums +
// lens/critiquedBy/severity attributes) — human-accepted per the schema-freeze rule.
// #ProspectiveChange: introduces a process-def (keystone co-commit, D0070). Accepted 2026-06-20.
// =====================================================================
package Decision0080 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0080 : Decision {
        :>> id = "63940516-2c7d-4e8f-a080-516273849670";
        :>> title = "D0080: critique mechanism — antagonistic critique IS a lens-tagged verification (schema/core: critique method + Severity/CriticKind/CritiqueLens enums + lens/critiquedBy/severity; element-critique process + skill)";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0079 (computed-assurance doctrine) framed antagonistic critique as a LENS-TAGGED VERIFICATION so it reuses all assurance machinery (verify/suspect/confirmation/coverage/issue-loop) rather than a parallel subsystem, and named D0080 as the schema+process consequence. assuranceCoverage (Sprint 68) then exposed the concrete gap this targets: all 28 Needs/SystemRequirements are uncovered (zero verify edges, zero requirement charters) — the model cannot yet demonstrate any requirement was examined, let alone survived scrutiny. The engine already has a whole-engine architectural-critique process, but it audits the ENGINE ITSELF (schema/tooling/invariants) ad hoc and ends in CR proposals — nothing adversarially verifies the TRACKED elements (needs/requirements/decisions) and records it against the model. schema/core is frozen (CLAUDE.md §2.5), so the schema additions need explicit human acceptance.";
        :>> decision = "Model antagonistic critique as a method=critique VERIFICATION. SCHEMA (schema/core, human-accepted): add VerificationMethod::critique; add enums Severity {Critical, High, Medium, Low}, CriticKind {aiModel, human, tool}, CritiqueLens {completeness, correctness, ambiguity, testability, feasibility, consistency, necessity} (the requirement-quality canon); add optional attributes lens : CritiqueLens[0..1], critiquedBy : CriticKind[0..1], severity : Severity[0..1] to verification def Test; add severity : Severity[0..1] to Issue (findings carry severity). PROCESS (.engine/processes/element-critique.sysml, cadence perItem, EngineElementCritique — distinct from the engine-level architectural-critique): scope (pick targets, prioritize the coverage gap set; pick required canon lenses) -> critique ((element x lens) by an INDEPENDENT critic: a method=critique Test + TestResult #Verify-linked to the target; result judgedBy != target author; outcome pass=survived / fail=finding) -> finding (each fail -> an Issue with severity, triaged via the D0078 #Resolves loop) -> disposition (human gate at >= Medium: ACT / ACCEPT-RISK (confirmation) / DISMISS (mooting Decision); Critical must be fixed or risk-accepted; Low AI-dispositioned) -> staleness (a critique goes suspect when its target changes after the critique commit -> re-critique at HEAD; re-open accept-risk dispositions whose basis changed). Deploy via the element-critique SKILL (D0059). Required lenses are BROAD across needs/requirements/decisions/architecture (D0079).";
        :>> rationale = "Critique-as-verification is the key economy (D0079): ONE mechanism (verify edge + TestResult + suspect + confirmation + coverage + #Resolves) serves critique, vs a duplicate critique subsystem. Optional attributes ([0..1]) keep every existing Test parsing unchanged (additive schema). Severity on the Issue is the correct home — a finding's impact is a property of the finding, and it lets the same D0079 taxonomy drive both critique findings and ordinary issues; the human-disposition gate at >= Medium falls straight out. Independence (critic identity != author) is what makes a critique adversarial rather than self-affirming. Keeping element-critique DISTINCT from architectural-critique avoids conflating 'verify the tracked model' with 'audit the engine'. ALTERNATIVES: (A) a standalone critique subsystem (own store/state) — REJECTED (duplicates suspect/coverage/confirmation/issue-loop; D0079 alt B); (B) extend the existing architectural-critique to elements — REJECTED (different target, cadence, and output: per-element recorded verification vs ad-hoc engine CR proposals); (C CHOSEN) critique = lens-tagged verification, additive schema + a dedicated element-critique process/skill. CRITERIA: reuse-one-mechanism x additive-not-breaking x independence-enforced x compute-don't-store. Lens taxonomy = the requirement-quality canon (human-selected over the existing critic-skill set) because it is element-agnostic and maps to INCOSE-style requirement quality.";
        :>> consequences = "Schema/core change (element.sysml: critique method + 3 enums; verification.sysml: lens/critiquedBy/severity on Test; work.sysml: severity on Issue) — validate_schema/instances (kernel) green; existing instances unaffected (additive). New process-def (.engine/processes/element-critique.sysml) + deploying skill (.engine/skills/element-critique/SKILL.md) + skills-registry entry; #ProspectiveChange marks the keystone process-change co-commit (D0070). Enables the next milestone steps: critiqueCoverage (per-element x required-lens coverage + a `guard critique`; staleness via suspect) and assuranceReadiness (composite gate). The 28 uncovered requirements/needs are the first critique targets. doc-sync: CLAUDE.md (verification methods / the critique process+skill), the severity taxonomy reference, usage docs, skills-registry. Pairs with D0079 (doctrine), D0050 (suspect/staleness), D0077/D0078 (issue loop), D0066 (attestation/confirmation), D0059 (deploying skill), D0016/D0051 (human-judgment confirmation).";
    }

    verification d0080Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a080-516273849671";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0080 (critique mechanism: antagonistic critique = a method=critique lens-tagged verification; schema/core additions — critique method + Severity/CriticKind/CritiqueLens enums + lens/critiquedBy/severity attributes; the element-critique process + deploying skill; findings -> severity-carrying Issues via the #Resolves loop; human disposition gate at >= Medium; staleness via suspect) by selecting 'Accept as proposed' and the 'Requirement-quality canon' lens taxonomy in the D0080 acceptance dialogue on 2026-06-20.";
    }
    part d0080AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a080-516273849672";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "3c93a66";
        :>> judgedAt = "2026-06-20";
        :>> judgedBy = "wweatherholtz";
    }
}
