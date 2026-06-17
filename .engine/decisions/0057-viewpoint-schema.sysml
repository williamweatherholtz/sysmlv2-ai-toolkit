// =====================================================================
// D0057 — viewpoint-schema: §2.5 sign-off to unfreeze schema/core for the Viewpoint
// type (D0056 increment 1). Adds part def Viewpoint + a declared registry; re-freezes
// schema after. Accepted (schema unfreeze) 2026-06-16 by wweatherholtz ("agreed on all
// points. proceed and unfreeze").
// =====================================================================
package Decision0057 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0057 : Decision {
        :>> id = "61728394-0a5b-4c6d-976f-1324a5b6c7ec";
        :>> title = "D0057: schema sign-off — add part def Viewpoint + viewpoint registry (formalViewpoints increment 1)";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0056 accepted the DIRECTION (formalize viewpoints; justified by reuse x auditability x audience) and gated the frozen-schema execution on an explicit §2.5 sign-off. The human gave it: 'agreed on all points. proceed and unfreeze.' This records that sign-off (who/when) per CLAUDE.md §4 and applies increment 1.";
        :>> decision = "ACCEPTED schema unfreeze (wweatherholtz 2026-06-16) for ONE change: add `part def Viewpoint :> Element` to schema/core/computed.sysml with attributes concern/audience/sources/renderer; created .engine/views/viewpoint-registry.sysml declaring 8 viewpoints (orient, whats-next, traceability, safety, requirements, decisions, baselines, suspect), each with concern + audience + sources + renderer; validate_instances extended to cover .engine/views/; pre-commit + _ID_TYPES updated; schema/core is RE-FROZEN after this change. This is INCREMENT 1 (formalViewpointsSchema).";
        :>> rationale = "Lightweight, engine-registry style — analogous to AISkill, NOT native viewpoint def which can't carry queryable data and whose expose/render don't parse in the pilot. The 8 declared viewpoints give the formally-defined safety/requirements/whats-next/traceability viewpoints requested. Further schema edits need a new sign-off. INCREMENT 2 (formalViewpoints) rewires report.py/orient to be declaration-DRIVEN renderers + adds a concern-coverage view (which stakeholder concerns lack a viewpoint) — remains gated/queued. NON-ADOPTIONS from D0056 stand (no Capella/Kitalpha; no literal OA/SA/LA/PA; 42010 correspondence rules optional).";
        :>> consequences = "schema/core gains part def Viewpoint (then re-frozen). Views are now declarable as first-class registry entries with concern/audience/sources/renderer — auditable and extensible by declaration. The renderer-rewiring + concern-coverage computation (increment 2, formalViewpoints) is the remaining high-value work. validate_instances now validates .engine/views/. Reaffirms D0054 (keep it friction-light) + D0055 (engine is systems modeling of the work/process).";
    }

    verification d0057Accept : Test { :>> id = "86191820-ab99-45d5-b408-c4a07e436778"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0057 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0057AcceptR1 : TestResult { :>> id = "3f1d8f5b-2081-4158-a270-8fae6c8c1c0b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
