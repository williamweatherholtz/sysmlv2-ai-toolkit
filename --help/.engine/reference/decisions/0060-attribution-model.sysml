// =====================================================================
// D0060 — attribution-model: the settled data/metadata + attribution model. Item base =
// id + name + content + creation-provenance; attribution-beyond-creation = EVENTS
// (TestResults), not fields; decisionText restructured + acceptance becomes an event.
// Records the design converged 2026-06-16/17; execution = attributionModel migration
// (pre-authorized breaking change, queued). Accepted 2026-06-17 by wweatherholtz.
// =====================================================================
package Decision0060 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0060 : Decision {
        :>> id = "61728394-0a5b-4c6d-d619-40516273841c";
        :>> title = "D0060: data vs metadata vs attribution-events — the settled item model";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "A long design dig (D0053-D0058 + this thread) on what is data vs metadata on a tracked item, and how to attribute roles (author/approver/tester/owner). SysML v2 rationale: intrinsic features (a thing's structure/behavior/content) = data; @annotation/metadata def = about-the-element provenance/classification. Cross-metatype specialization is impossible (D0055), so per-metatype Tracked* bases duplicate id/title/createdAt/authoredBy. The dig found the original 'collapse 4 fields into @Tracked' framing was wrong on multiple counts.";
        :>> decision = "SETTLED MODEL. (1) DATA on the item base: `id` (immutable identity) + `title`/name (human name; may duplicate; NOT an identifier — distinct from computed `displayLabel`) + the item's intrinsic CONTENT. (2) METADATA (provenance, universal): `createdAt` + `createdBy` (who RECORDED the item, when) — rename `authoredBy`->`createdBy`. (3) ATTRIBUTION BEYOND CREATION IS NOT ITEM STATE — it is EVENTS or ASSIGNMENTS: (a) attestations/authority (approval, acceptance, third-party test, witness, independent V&V) are EVENTS = TestResult records (method + judgedBy + judgedAt + judgedAgainst), linked by verify — never fields, never null columns; the process (gate/DoD) declares which attestations are REQUIRED and a coverage view flags missing ones; (b) standing responsibility (owner) is an assignment = a typed edge / `owner` field; who-implemented is COMPUTED (git/TestResult executor), not stored. (4) DECISIONS: decisionText-as-free-prose is DEPRECATED in favor of structured fields (decision statement + context + alternatives/criteria [D0058] + consequences); a Decision's ACCEPTANCE moves out of prose into a downstream `confirmation` TestResult; the Decision record (reasoning) STAYS first-class data — only its acceptance is absorbed into the TestResult.";
        :>> rationale = "'author' was ambiguous, hence the createdBy rename. ALTERNATIVES considered for role attribution: (A) fixed optional columns (approvedBy, thirdPartyTestedBy, ...) — rejected: sparse-column anti-pattern, un-enumerable, every new role = a schema change, null can't distinguish not-required from missing; (B) dynamically-attached ad-hoc metadata annotations — rejected: a by/at pair IS an event, and ad-hoc annotations drift without typed registry + guards; (C) CHOSEN: attestations = TestResult events (already first-class, append-only, provenance-bearing, verify-linked), ownership = edge, implemented = computed. CRITERIA: correctness (an attestation is an act, not a property) x consistency with existing invariants (atomic items + typed edges; TestResults are separate appended records, D0012) x no-null-columns x process-governs-requirement.";
        :>> consequences = "Execution = the `attributionModel` backlog migration (pre-authorized breaking change; own focused sprint): rename authoredBy->createdBy; deprecate decisionText (restructure into decision/context/alternatives/criteria/consequences); route Decision acceptance through a confirmation TestResult; keep id+title(name)+content as data; confirm attestations are TestResults (mostly already true) + add a process-required-attestation coverage view. Supersedes the D0053 'collapse 4 fields' framing. Pairs with formalViewpoints (the coverage view) and decisionAnalysis (D0058 alternatives/criteria, already in effect). This Decision itself follows the new convention (structured + alternatives/criteria); its acceptance is recorded here pending the confirmation-TestResult mechanism the migration introduces.";
    }

    verification d0060Accept : Test { :>> id = "135383bc-cae1-4740-a210-db7dd72a4819"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0060 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0060AcceptR1 : TestResult { :>> id = "b0a15061-2aed-4d6e-aedd-313ac19bcca9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
