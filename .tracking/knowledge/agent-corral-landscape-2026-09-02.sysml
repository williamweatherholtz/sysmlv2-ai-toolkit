// KNOWLEDGE RECORD (D0196) - the agent-corral landscape spike and what keel takes from it (D0282).
package KnowledgeAgentCorralLandscape20260902 {
    private import EngineElement::*;
    private import EngineVerification::*;

    verification kAgentCorralLandscape : Test {
        :>> id = "96ce526d-9776-47c7-94c0-15cba914a66c";
        :>> title = "Landscape: fifteen projects corralling AI agents in three families - nobody checks substance, standing consent is the distrusted artifact, keel's three legs exist together nowhere else";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "REPLICATION (D0196). PROCESS: adversarialPanelReview (D0187, def pin 64f2dea). TRIGGER: an unauthenticated GitHub login commented on the standing-consent override thread (issue 29, 2026-09-02T10:12Z) asking whether keel records consent scope and rule revision on each auto-accepted Decision, linking an in-path MCP authorization product; the recording Action refused to record it (login not in github-actors.toml) - correctly. The human asked what other projects in this space are doing and what keel can learn. ROSTER: three parallel read-only web-enabled lenses - (A) in-path intermediaries and authorization brokers, (B) in-repo and harness-level agent discipline, (C) runtime-formal enforcement, capability sandboxes and tamper-evident audit. Each lens: five projects in a fixed schema (layer, mechanism, provenance, what keel could steal, what keel does that it does not, verbatim evidence with URL), then a synthesis. ONE round; the three syntheses converged on the same three findings without a refutation round. VERIFICATION OF THE LENSES' CLAIMS ABOUT KEEL, before any became a finding: acceptance binds to a commit SHA and never to the accepted text (govern.rs acceptance_judged_against; no digest, no edited-after-acceptance guard) - CONFIRMED, issue341; the agent can run keel accept naming a human (main.rs cmd_accept refuses in agent sessions unless TTY-human or console queue, best-effort and env-mutable by recorded design; write.rs refuse_ai_judgment checks the NAME's kind) - PARTLY, the channel is the strong path; no gate satisfiable only by an external fact (grep gh run, run_id, workflow_run in src: ingest and headless_ask only) - CONFIRMED; the hook fire-ledger is not reconciled in CI (hooks.jsonl read by pm.rs and launcher.rs only) - CONFIRMED BUT NARROWER than claimed, since CI re-derives validate and every guard on the tree so a bypassed hook cannot hide dishonest state; auto-acceptance records no rule revision (decision-issue.yml auto-accept step) - CONFIRMED, issue342. THE FIFTEEN: agentgateway, ToolHive, Snyk Agent Scan, Entra Agent ID, Arcade.dev (in-path); Beads, GitHub Spec Kit, OpenSpec, Superpowers, Kiro (in-repo); AgentSpec, Progent, CaMeL, PAuth, halo-record (formal and ledger). Runners-up: CapLease, IETF compliance-receipts draft, in-toto agent-decision RFC, Wassette, OWASP Agentic Top 10, ruflo. THREE FINDINGS: (1) nobody checks substance - in-path products answer may-this-call-happen, in-repo products does-the-artifact-exist, formal systems does-the-pending-call-satisfy-a-predicate; the partial exceptions are PAuth recomputing an operand at the server and Beads closing a gate on a real CI run; (2) standing consent is the artifact the field distrusts - Entra time-bounds and re-attests, CapLease shows single-use tokens replay under agent retry without durable confirmation state, the IETF draft mandates policy_digest and expires_at - and keel's D0207 is the outlier; (3) keel's three legs - computed state, blocking guards at the harness boundary, independent re-derivation on infrastructure the agent cannot modify - exist together nowhere: Beads has computed state in a gitignored Dolt database with an actor defaulting to unknown and an unauthenticated human gate, Kiro has blocking hooks the agent can edit, agentgateway and ToolHive have separation without a model of the work; the in-repo family is a context family (280k+133k+52k stars of prompt-only discipline) and Superpowers ships keel's verify-don't-assert clause verbatim with zero hooks behind it, which is also keel's one clause with no control (D0151). THE TRIGGER ANSWERED: an MCP intermediary whose own page says the in-path proxy is on the roadmap - today an MCP server the agent may simply not call; its ledger is real, its enforcement is not yet, its question was legitimate. DECISION D0282: six ideas adopted in rank as backlog items (dcAcceptanceBindsToTheText, dcConsentIsScopedAndExpires, dcExternalFactGate, dcSkillsRouteBehaviourally, dcUntrustedTaintClosure, dcCommitDeltaView), two mechanisms fed into the existing fork dcProcessStepsBindToChecks (a closed predicate registry a step binds to by name - AgentSpec; a parseable requirement grammar a guard can refuse - EARS via Kiro), two declined with reasons (Progent's SMT narrowing-versus-expansion over guard predicates - correct in principle, a rewrite for a defect class not yet observed; halo-record's capture completeness - narrowed on check). KEY LEARNING: keel is further along the form axis than anything examined and no further along the substance axis than the field; the field's one live critique of keel is the shape of its consent, not the shape of its enforcement. ARTIFACTS: docs/reviews/agent-corral-landscape-2026-09-02.md (synthesis + three full lens reports with URLs); D0282; issue341, issue342; six backlog items.";
    }
    part kAgentCorralLandscapeR1 : TestResult {
        :>> id = "6fa8d2cd-3597-46b6-b0bb-1065b80683f9";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "52b434e93bc09f719a94f7457890400cc8a1614f";
        :>> judgedAt = "2026-09-02";
        :>> judgedBy = "claudeOpus5";
        :>> notes = "Three lenses converged in one round; every claim about keel's own state checked against source, one narrowed. Fifteen projects with URL-cited evidence.";
    }
    dependency from kAgentCorralLandscape to adversarialPanelReview;
    dependency from kAgentCorralLandscape to d0187;
    dependency from kAgentCorralLandscape to d0196;
    dependency from kAgentCorralLandscape to d0282;
    dependency from kAgentCorralLandscape to landscapeSpikeStory;
    dependency from kAgentCorralLandscape to issue341;
    dependency from kAgentCorralLandscape to issue342;
    dependency from kAgentCorralLandscape to dcAcceptanceBindsToTheText;
    dependency from kAgentCorralLandscape to dcConsentIsScopedAndExpires;
    dependency from kAgentCorralLandscape to dcExternalFactGate;
    dependency from kAgentCorralLandscape to dcSkillsRouteBehaviourally;
    dependency from kAgentCorralLandscape to dcUntrustedTaintClosure;
    dependency from kAgentCorralLandscape to dcCommitDeltaView;
    dependency from kAgentCorralLandscape to dcProcessStepsBindToChecks;
}
