// D0228 (status: the `status` field below is the truth) — guardScope: guard 50 enforces the APPLIES-WHEN fact only where project-onboarding was adopted
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0228Accept event on sign-off.
package Decision0228 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0228 : Decision {
        :>> id = "21ec574e-ea89-495d-881c-b736d284e7cf";
        :>> title = "guardScope: guard 50 enforces the APPLIES-WHEN fact only where project-onboarding was adopted";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Hours after landing guard 50, the first project keel was adopted onto failed it 22 times out of 22. The guard requires every declared Process to carry an APPLIES-WHEN condition; penumbra runs a 20-August engine snapshot whose processes predate the convention, has never adopted project-onboarding, and would therefore have had its commit gate broken by a control it never chose. That is precisely the failure D0164 names - a project that never adopted a control has not violated it - and I wrote the guard the same day while quoting D0164 in a different context. It was invisible until an actual adoption ran, which is the third time in four sprints that the self-build could not feel a defect and a second project could.";
        :>> decision = "The guard is SCOPED TO ADOPTION: it enforces nothing unless `.engine/processes/project-onboarding.sysml` is present, and reports 0 scanned when it is not. Not silently skipped - the zero is printed, so an out-of-scope guard is visible rather than a vacuous pass. Deliberately NOT solved by having projectOnboarding claim the guard: a claim makes it switchable in a project that HAS the unit, but a project that lacks the unit has no claim either, so guard_state would return Core and enforce it anyway - the exact opposite of what is needed. Presence of the process is the fact that decides it.";
        :>> rationale = "An APPLIES-WHEN line exists to let onboarding recommend a process for or against. Where onboarding is not adopted, the fact serves nothing and its absence harms nobody, so enforcing it is friction with no purchase - D0054's top risk, arriving as a red gate on a newcomer's first commit, which is the worst possible first experience of an adopted engine. The honest-state test (D0098) also fails: a project without the convention is not making a FALSE claim about itself, it is simply not playing this game. The zero-scanned report matters as much as the scoping, because this session has twice shipped checks that passed on an empty population and read as clean - the difference here is that the emptiness is DECLARED by a condition anyone can read, and printed rather than hidden.";
        :>> consequences = "A project adopts the obligation exactly when it adopts the process, which is the right coupling but does mean a project can hold processes without conditions indefinitely and never be told - the guard will not nag them, and that is intended. keel itself is unaffected: 25 scanned, still enforced. Verified by running the real case rather than reasoning about it: penumbra went from 22 violations to [guard] ALL PASS with the fixed binary, and keel's own count is unchanged at 25. The unit test asserts both directions plus the out-of-scope zero. Resolves issue259.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0228Accept : Test { :>> id = "b2a1ed87-d5dd-4652-bd40-f7cac568691b"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/26."; }
    part d0228AcceptR1 : TestResult { :>> id = "0dfa83bb-02b2-4e3d-bacb-5c588683d6bd"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f4417e9"; :>> judgedAt = "2026-08-25"; :>> judgedBy = "wweatherholtz"; }
}
