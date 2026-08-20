// =====================================================================
// D0058 — decision-analysis: capture structured alternatives + criteria in Decisions
// (ISO 42010 / NPR 7123.1 Decision Analysis / ADR). Convention, not schema. This
// Decision SELF-DEMONSTRATES the convention in its own decisionText.
// Accepted 2026-06-16 by wweatherholtz (issue016; autonomous flow).
// =====================================================================
package Decision0058 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0058 : Decision {
        :>> id = "61728394-0a5b-4c6d-a619-405162738419";
        :>> title = "D0058: Decision Analysis convention — record alternatives + criteria, not just the verdict";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Benchmark (D0054/issue016): ISO/IEC/IEEE 42010 makes architecture Decisions AND Rationale first-class and recommends recording the alternatives NOT chosen + the criteria; NASA NPR 7123.1 names Decision Analysis as one of its 17 technical processes. Our Decision items captured context/decisionText/consequences (the verdict + why) but not the STRUCTURED options-considered / selection-criteria / rejected-with-reason (the ADR pattern). Trade rationale was being lost.";
        :>> decision = "Adopt a Decision Analysis CONVENTION: when a Decision chose between real options, decisionText includes a structured tail 'ALTERNATIVES: (A) ... rejected: ...; (B) ...; (C) chosen. CRITERIA: ...'. Documented in sysmlv2-syntax-notes.md (Decision authoring). Applies going forward; skip for record-only / no-alternative decisions.";
        :>> rationale = "ALTERNATIVES: (A) add structured `alternatives`/`criteria` ATTRIBUTES to the Decision schema — rejected: touches frozen schema/core (needs a §2.5 sign-off) and adds authoring friction (D0054) for marginal gain over text; (B) a separate ADR file/type per decision — rejected: duplicates the existing Decision record + fragments the audit trail; (C) CHOSEN: a lightweight text convention inside the existing decisionText String — no schema change, no friction, fully queryable as text, reversible. CRITERIA: schema-stability (keep core frozen) x authoring-friction (D0054, the #1 risk) x rationale-completeness (ISO 42010) x reversibility — (C) dominates on all but the most rigorous structured-query axis, which we do not yet need.";
        :>> consequences = "Decisions with real trade-offs now record alternatives + criteria in decisionText (this Decision is the first, self-demonstrating). No schema change; schema/core stays frozen. sysmlv2-syntax-notes.md Decision section updated. Closes issue016. Feeds the `decisions` viewpoint (D0056) — alternatives become part of what that lens surfaces. If a future need for structured QUERY over alternatives emerges, revisit alternative (A) with a sign-off.";
    }

    verification d0058Accept : Test { :>> id = "d7330ce8-ef1a-444c-a1c6-7a75429ae4b7"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0058 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0058AcceptR1 : TestResult { :>> id = "63b80dc9-baba-49ce-b596-93b6e4c6c2e6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
