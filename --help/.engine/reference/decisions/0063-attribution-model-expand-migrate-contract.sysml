// =====================================================================
// D0063 — attribution-model migration shape: EXPAND / MIGRATE / CONTRACT.
// Schema sign-off for the additive expand step (attrModelSchema, re-scoped) AND
// the recorded amendment to D0061's decomposition: the breaking rename/retire is
// deferred from task 1 to a new CONTRACT task, so each step commits green.
// NOTE: authored in the CURRENT decision pattern (authoredBy/context/decisionText/
// consequences) for consistency with the other 62 decisions — migrated by
// attrModelInstances (D0061 task 2). The new `decision`/`rationale` fields this
// Decision adds are exercised by instances in the migrate step, not here.
// Accepted 2026-06-17 by wweatherholtz (chose "Expand / migrate / contract" when
// shown that a rename cannot be committed without migrating all 389 authoredBy +
// 85 decisionText occurrences at once; pre-commit validate_instances would block it).
// =====================================================================
package Decision0063 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0063 : Decision {
        :>> id = "63940516-2c7d-4e8f-b93a-627384950662";
        :>> title = "D0063: attribution-model migration = expand/migrate/contract; schema sign-off for the additive expand (createdBy + decision + rationale, all [0..1])";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "attrModelSchema (D0061 task 1) was worded as a breaking RENAME (authoredBy->createdBy) + RETIRE (decisionText). Investigation showed authoredBy appears 389 times across 63 files, decisionText 85 times across the 62 decision files, and 6 tools read these fields. The pre-commit hook runs validate_instances (kernel, checks instances against schema) + validate_actors (reads authoredBy) — so a rename in the schema alone fails pre-commit until every instance + tool is migrated in the SAME commit. That couples task 1 (schema) to task 2 (instances) and task 3 (tools) at commit time and forces one large, higher-risk change. Expand/migrate/contract is the standard practice for breaking schema changes in a live, validated system (parallel-change / Fowler): add the new shape additively, migrate readers + data, then remove the old shape — each step independently green.";
        :>> decision = "DECISION: execute the attribution-model migration as EXPAND -> MIGRATE -> CONTRACT. (1) EXPAND (attrModelSchema, re-scoped, DONE here): ADD createdBy to Element + the 3 Tracked* bases and decision + rationale to Decision, ALL as [0..1], KEEPING authoredBy/decisionText/context — purely additive, so all existing instances stay valid and the step commits green; validate_schema 14/14 green. (2) MIGRATE (attrModelInstances + attrModelTooling): populate createdBy on every instance + split decisionText into decision+rationale per decision (KEEP context); update the 6 readers (query.py, report.py, validate_actors.py, capture_user.py, Rust parser/indexer/validate where applicable) to read the new fields; old fields still present, so still green. (3) CONTRACT (attrModelContract, NEW task): once nothing authors or reads authoredBy/decisionText, drop them from the schema and tighten createdBy/decision/rationale toward [1..1]; validate_all green. AMENDS D0061: the retire/rename moves OUT of task 1 INTO the new contract task; task 1 is now the additive expand.";
        :>> rationale = "ALTERNATIVES: (A) atomic big-bang — do the rename + migrate all 389+85 occurrences + 6 tools + per-decision split in one commit — rejected: large, higher-risk, tree uncommittable mid-migration. (B EXPAND/MIGRATE/CONTRACT, CHOSEN): incremental, each step green and reviewable, decisionText split done lazily per decision. CRITERIA: each-commit-green x risk x reviewability x friction (D0054).";
        :>> consequences = "schema/core/element.sysml + work.sysml expanded (createdBy on Element + 3 Tracked* bases; decision + rationale on Decision; all [0..1]; old fields marked DEPRECATED in comments). Backlog: attrModelSchemaDoD re-scoped to the additive expand; new action attrModelContract + DoD added, ordered AFTER attrModelInstances + attrModelTooling. Authoring convention unchanged for now (keep authoredBy + decisionText; the Decision Analysis ALTERNATIVES/CRITERIA tail still lives in decisionText until the split). sysmlv2-syntax-notes.md Decision-fields note updated to the in-flight state. schema/core stays frozen-governed: this Decision is the recorded sign-off for the change (D0002). No instance or tool changed in this step — those are the migrate step, separately committed and green.";
    }

    verification d0063Accept : Test { :>> id = "8199b966-580d-4613-9ae3-c91c0d492ff0"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0063 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0063AcceptR1 : TestResult { :>> id = "08714ea3-3d90-45da-a7d9-5f725adb28bf"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
