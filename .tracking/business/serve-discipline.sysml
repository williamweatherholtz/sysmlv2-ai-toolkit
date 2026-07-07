// =====================================================================
// ProjectBusinessServeDiscipline — Business-workflow artifacts for the keel-serve AI-discipline tooling
// (D0109 accepted). FIRST real dogfood of the business-architecture skill on a genuine design task
// (closes issue061's skill-run half). Phase 1: Brief + Needs. Needs-before-architecture: the human
// accepts these Needs before any SystemRequirement / architecture Decision / code (the Business gate).
// =====================================================================
package ProjectBusinessServeDiscipline {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part serveDisciplineBrief : Brief {
        :>> id = "5e40d1c0-0109-4b7a-9c1e-7c5d1f0e2a09";
        :>> title = "keel-serve AI-discipline tooling — brief";
        :>> createdAt = "2026-07-07";
        :>> createdBy = "wweatherholtz";
        :>> problem = "AI agents drift: they stray off task, ignore defined processes, skip documentation, and 'do what they said they wouldn't' (evidenced this self-build: D0106 violated after adoption; the issue058 dual-truth). Discipline today is SOFT — skills + reminders the agent can ignore (a linter, not a compiler). Freeform prompt-in / freeform-act-out leaves too much to interpretation.";
        :>> opportunity = "Move discipline into the ACTION SPACE at keel's interaction boundary (D0109): let the human launch only DEFINED processes (invalid actions unrepresentable), show + approve the parsed routing before execution (closing D0106's conversational residual that a commit gate cannot — issue059), and validate/repair AI output against its expected structure at generation time. Realized as `keel serve` increments over the already-declared model (one source of truth; subscription billing via the existing claude -p bridge).";
        :>> constraintsNote = "Extend keel serve (accepted harness), not a new AI IDE. Single-user (the human) — friction (D0054) is acceptable if it buys control. Reuse keel's declared processes/skills + schemas as the launchable set; do not duplicate. Inherits claude -p's system prompt (minimal-prompt is out of scope — not the discipline lever, per D0109).";
    }

    // ── Needs (solution-free outcomes; MoSCoW). Accept these BEFORE architecture (needs-before-architecture). ──

    requirement nServeLaunchDefinedOnly : Need {
        :>> id = "5e40d1c0-0109-4b7a-9c1e-000000000001";
        :>> title = "Only defined processes are launchable";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::operator;
        :>> statement = "When directing an AI task, the operator can invoke only a DEFINED process/skill from keel's declared model; a freeform / un-routed action is not offered — invalid actions are unrepresentable at the boundary.";
        :>> priority = Priority::must;
    }
    requirement nServeApproveBeforeExecute : Need {
        :>> id = "5e40d1c0-0109-4b7a-9c1e-000000000002";
        :>> title = "Parsed routing is shown and approved before execution";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::operator;
        :>> statement = "Before an AI task runs, the operator sees how the input was parsed + which process it routed to, and explicitly approves — the agent never silently acts on its own interpretation.";
        :>> priority = Priority::must;
    }
    requirement nServeValidatedOutput : Need {
        :>> id = "5e40d1c0-0109-4b7a-9c1e-000000000003";
        :>> title = "AI output is structurally validated + repaired before it lands";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::operator;
        :>> statement = "AI output that violates its expected structure/schema is caught and sent back for repair at generation time — before it reaches a commit or the model — so malformed results never silently persist.";
        :>> priority = Priority::must;
    }
    requirement nServeBoundedContext : Need {
        :>> id = "5e40d1c0-0109-4b7a-9c1e-000000000004";
        :>> title = "Each task runs in bounded, single-purpose context";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::operator;
        :>> statement = "Each AI task executes with narrow, single-purpose context so the agent does not drift from a diffuse, long-running goal; large work is decomposed into small launches.";
        :>> priority = Priority::should;
    }
    requirement nServeReuseModel : Need {
        :>> id = "5e40d1c0-0109-4b7a-9c1e-000000000005";
        :>> title = "Launchable set + schemas come from the declared model";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> source = NeedSource::internal;
        :>> statement = "The set of launchable processes and their expected output structures are read from keel's already-declared model — one source of truth — never a separately-maintained list that can drift.";
        :>> priority = Priority::should;
    }

    // ── Business gate: human acceptance of the Needs (needs-before-architecture). ──
    verification serveDisciplineNeedsAccept : Test { :>> id = "5e40d1c0-0109-4b7a-9c1e-00000000000a"; :>> title = "serve-discipline Needs accepted (Business gate)"; :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz accepts the 5 serve-discipline Needs (nServeLaunchDefinedOnly / nServeApproveBeforeExecute / nServeValidatedOutput [must], nServeBoundedContext / nServeReuseModel [should]) via the explicit 'yes' on 2026-07-07 — the Business gate. Architecture (SystemRequirements) may now be derived from these accepted Needs."; }
    part serveDisciplineNeedsAcceptR1 : TestResult { :>> id = "5e40d1c0-0109-4b7a-9c1e-00000000000b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5c0b17f"; :>> judgedAt = "2026-07-07"; :>> judgedBy = "wweatherholtz"; }
}
