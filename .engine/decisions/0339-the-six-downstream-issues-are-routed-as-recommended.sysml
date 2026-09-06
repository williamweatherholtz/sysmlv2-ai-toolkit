// D0339 (status: the `status` field below is the truth) — sixDownstreamIssuesRouted: GH#54-59, filed by the declared decider from downstream adoption, are routed as recommended - six tracked Issues with resolver tasks, one of them Critical
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0339Accept event on sign-off.
package Decision0339 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0339 : Decision {
        :>> id = "664216ce-4666-4336-b0e8-0761bcc85aa3";
        :>> title = "sixDownstreamIssuesRouted: GH#54-59, filed by the declared decider from downstream adoption, are routed as recommended - six tracked Issues with resolver tasks, one of them Critical";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The first live currency pass (D0338) pulled six new open issues on the public repository as untrusted Statements (st092-st097). All six were filed by williamweatherholtz, the single declared decider in github-actors.toml, from two downstream projects moving onto a newer engine. Triaged into us091-us096, each with a recommendation. The tier says a human accepts before anything becomes work (D0264); D0292 already settled the same situation for GH#42-53 on the owner's word ('all work. these are essentially from me'), and D0314 made the rule general: a declared decider's utterance is the human's own direction, not a stranger's, so the untrusted label does not taint what derives from it. Every one of the six was VERIFIED against main before routing, and two were found wider than filed: GH#57's fabricated-attestation class exists in the binary's record-time path, not only in the library script; GH#55 persists after D0316 because both hook surfaces probe a cache path init never writes.";
        :>> decision = "The six stories are routed as recommended: us093 (GH#57) to issue376, Critical, resolver dcStandingWordsAreProjectLocal; us091 (GH#59) to issue377, High, resolver dcRemovedTypeScannerReadsSchemaNotProse; us095 (GH#55) to issue378, High, resolver dcOneBinaryResolutionForEverySurface; us096 (GH#54) to issue379, High, resolver dcSeededCacheIsVerified; us094 (GH#56) to issue380, High, resolver dcCharterMustResolve, with the overwrite itself recorded as fixed on main at 6583e08 and unreleased; us092 (GH#58) to issue381, Medium, resolver dcLibraryUnitRetirement, which also carries GH#57's library half. The six resolvers enter the backlog above every Medium item, Critical first, so the ranked frontier works them next. NOT A FORK: the routing follows the recorded recommendations, and each resolver's design is its own Decision when it is built.";
        :>> rationale = "Why route rather than plan-only: the speaker is the declared decider (D0314), the same ground D0292 stood on, and the reports are precise enough to verify - each was, and the verification is in the story's triage note. Why one Decision for six: it is one act, the routing of one pass's intake; the resolvers are separately chartered. Why Critical for GH#57: the engine itself produces an acceptance in a human's name quoting words they never said, which is the class D0106 and confirmation-authenticity exist to make impossible. Arming (D0298): guard 56 untrusted-routing fails an untrusted story routed to work with no Decision among its targets - this Decision is that target for all six; guard 63 untrusted-taint follows the label through derivation and stops at the decider.";
        :>> consequences = "Six Issues open with resolvers; the frontier gains six items at the top. GH#54-59 are answered on GitHub with their tracked ids once the routing lands. A release carrying the fixes since v0.3.1 is a deploy-workflow fact this Decision states and does not perform: adopters on the release still meet issue314's overwrite.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0339Accept : Test { :>> id = "a3586b5c-a3ef-4220-9265-af7009364532"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0339AcceptR1 : TestResult { :>> id = "bd71965c-b8b4-4e7c-8e41-fefb6dd58cf4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5ab6573"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
