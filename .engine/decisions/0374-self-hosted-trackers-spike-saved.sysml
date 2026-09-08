// D0374 (status: the `status` field below is the truth) — selfHostedTrackers: the comparison with Linear, Jira and the self-hosted field is saved as a spike, and no tracker is adopted
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0374Accept event on sign-off.
package Decision0374 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0374 : Decision {
        :>> id = "ab51c6cd-86c2-4f7d-a52e-b21cabf0fbfb";
        :>> title = "selfHostedTrackers: the comparison with Linear, Jira and the self-hosted field is saved as a spike, and no tracker is adopted";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human, 2026-09-07: 'contrast what we're doing with his linear and Jira. best practices for those with AI agents? is linear self hostable?' then 'I do like the self hosting surface. how really comparable are the competing self hosted options?' then 'ok. save this as a spike for now.' The comparison was answered in conversation with every external fact checked against a named source that day; a conversation is not a record, so this Decision and docs/reviews/self-hosted-trackers-2026-09-07.md hold it. What was found: Linear cannot be self-hosted; Jira Data Center reaches read-only on 28 Mar 2029 and its agents are cloud-only; GitLab, Plane, OpenProject, Huly, Taiga, Redmine and Vikunja can be self-hosted, and each stores status as a field someone sets, which is the one thing keel does not do (§1 invariant 1, D0018). Two claims could not be verified from public documentation and the review says so: whether Plane's agent framework functions on the AGPL Community build, and which GitLab tier gates Duo on self-managed.";
        :>> decision = "The spike is saved: the review at docs/reviews/self-hosted-trackers-2026-09-07.md is the record of what was compared, what was verified and what was not, and this Decision is its charter. No tracker is adopted, trialled or integrated by it; keel's position - truth in text, state computed, done is a passing test against a resolvable SHA - is unchanged. If a hosted surface for humans and outside collaborators is ever wanted, that is a later Decision carrying its own trial install and the two unverified claims resolved first. NOT A FORK: the human asked to save the spike, not to choose a tracker, and this records the finding and that nothing changes.";
        :>> rationale = "The self-hosting surface the human likes transfers to any of the tools compared; the computed-truth bet does not transfer to any of them, and it is the bet this project is. The three near-misses of the same session - a suite reporting 0 passed 0 failed over a file lock (issue386), a write filing work where nothing looks for it, a benchmark that could not see the mechanism - would have been caught by none of the tools compared, because none has an opinion about whether a claim is true. Saving the comparison as a record rather than re-deriving it later is D0040: the question will recur, and the answer's external facts are dated, so a reader must be able to see when they were checked.";
        :>> consequences = "docs/reviews/self-hosted-trackers-2026-09-07.md exists and names its check date and sources; sprint 605 is chartered by this Decision as the research sprint that recorded it. The two UNVERIFIED claims stay unverified until someone decides adoption is worth a trial install, at which point they are the first thing that trial answers. Nothing in the engine, the process set or the tracking model changes.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0374Accept : Test { :>> id = "36953679-1a3e-47da-b123-483da91da623"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0374AcceptR1 : TestResult { :>> id = "52e07dcd-c6f7-4224-8039-641acf569c94"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a08f2a1"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
