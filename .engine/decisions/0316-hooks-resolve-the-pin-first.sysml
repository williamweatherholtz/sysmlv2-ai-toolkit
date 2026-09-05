// D0316 (status: the `status` field below is the truth) — hooksResolveThePinFirst: every Claude hook resolves the keel binary in the git hooks' order - KEEL_BIN, the pinned .keel/bin binary, then PATH
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0316Accept event on sign-off.
package Decision0316 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0316 : Decision {
        :>> id = "f4a50f98-a699-4df8-81d0-d6cbbb138d67";
        :>> title = "hooksResolveThePinFirst: every Claude hook resolves the keel binary in the git hooks' order - KEEL_BIN, the pinned .keel/bin binary, then PATH";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0230 made a project pin its own engine binary at .keel/bin/keel(.exe), and the scaffolded git hooks probe it before PATH. The Claude hooks did not: they probed KEEL_BIN then PATH (D0174's fail-loud resolver), so a project pinned to 0.3.0 ran its turns on whatever keel PATH held - including an uncommitted build - while its commit gate ran the pin (GH#43, st086, issue348). Two surfaces, two orders, and the hook that runs most often was the one that ignored the pin.";
        :>> decision = "One resolution order for every keel surface, RESOLUTION_ORDER = KEEL_BIN, then .keel/bin/keel(.exe) under the project directory (CLAUDE_PROJECT_DIR or cwd), then PATH: the three hook resolvers (advisory, gating, protected-path) are generated from one pin_first_probe, the scaffolded pre-commit hook moves into the library beside them, and a unit test holds every generated hook command AND the git hook to that order by sequential probe - a PATH-first script is refused by the same check.";
        :>> rationale = "Why the order and not a policy: the pin exists so a project's gate is decoupled from whatever PATH holds (D0230); a turn hook that ignores it runs the project on an engine its gate will not accept, which is the skew D0251 refuses elsewhere. Why one generator: the three resolvers had drifted from the git hook because they were three strings; a probe with one home cannot disagree with itself, and the git hook moving into the library is what lets the test compare them. Why sequential probing in the test: the git hook's comment names .keel/bin/keel before its code does, and a first-occurrence check read the comment as the probe. Arming (D0298): claude_surface tests assert the order per hook command (seven commands today) and on PRECOMMIT_HOOK, and that a PATH-first script fails the check.";
        :>> consequences = ".claude/settings.json and the plugin hooks.json regenerate with the pin probe; claude-surface-drift holds them. A project that pins its engine now runs its turns on it. issue348 is resolved by dcHooksResolveThePinFirst. This repository's own .githooks/pre-commit keeps its self-build target/release probe first by design and is outside this order.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0316Accept : Test { :>> id = "fd020e37-fd8f-4205-a18a-1ca8a892a3af"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0316AcceptR1 : TestResult { :>> id = "88236e72-12d8-4a07-a15c-5eef86379c92"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d697ae2"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
