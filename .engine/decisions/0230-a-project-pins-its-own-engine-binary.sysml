// D0230 (status: the `status` field below is the truth) — pinnedFirst: a project's gate runs the binary that project chose, not whatever is on PATH
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0230Accept event on sign-off.
package Decision0230 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0230 : Decision {
        :>> id = "93d1272f-5841-4706-a307-4539c6065aeb";
        :>> title = "pinnedFirst: a project's gate runs the binary that project chose, not whatever is on PATH";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "penumbra's commit gate was running THIS repository's working build. `keel` on PATH resolved to target/release/keel in a sibling directory, reporting version 0.3.0+dirty - so a broken or half-finished build here would have broken that project's gate, and any guard I added mid-session applied to their commits without them adopting anything. That is how a guard landed this session failed 22 of their processes on contact (issue259). The author chose to decouple them: pin penumbra to a release. The scaffolded hook already honoured a KEEL_BIN environment variable, but an environment variable is not a pin - it has to be set correctly in every shell, by every contributor, forever, which is the manual-vigilance shape D0047 rules out.";
        :>> decision = "The scaffolded pre-commit resolves its binary PINNED-FIRST: an explicit KEEL_BIN wins, then a repo-local `.keel/bin/keel` (or `.keel/bin/keel.exe`), then PATH. Dropping a released binary at `.keel/bin/keel` IS the pin - no wrapper, no script, nothing to keep in sync - and `.keel/` is machine-local, so each contributor installs their own copy rather than a binary being committed. A project that wants the old behaviour simply does not create the directory, so this changes nothing for any existing project until it opts in.";
        :>> rationale = "The pin had to be a PATH rather than a rule, because the failure mode is silent: a gate running the wrong binary still prints a pass. That is the same class as the release workflow checking out shallow (issue260) - a check that reports success it did not earn - and the fix has the same shape, a literal in the right place rather than a habit. Repo-local rather than committed because a binary in git is a different problem, and machine-local because a contributor on another platform needs a different build. Ordering matters and is deliberate: KEEL_BIN first so an override still works for a bisect or a one-off, the pin second, PATH last so an unpinned project is unaffected. The engine-version parity warning (D0190) stays advisory and now becomes useful rather than noisy, because a pinned project's declared version and running binary should agree, and a divergence means the pin slipped.";
        :>> consequences = "A pinned project's contributors must each install the binary once, and a contributor who does not gets the LOUD not-found failure the hook already emits with the release URL - not a silent skip. The pin is only as current as whoever last installed it, so a project can sit on an old engine indefinitely; that is the intended trade of decoupling, and the parity warning is what surfaces it. This does not pin keel itself, which builds from source by design and whose own hook probes target/release first. Verified: a fresh `keel init` scaffold renders the pinned-first resolution, and a stub binary placed at .keel/bin/keel is the one the hook actually invokes for validate, guard and rules - checked by running the hook, not by reading it.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0230Accept : Test { :>> id = "e7b64223-e98e-43f6-9498-ead84c6f825d"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/29."; }
    part d0230AcceptR1 : TestResult { :>> id = "b3292cd5-fc4e-4135-839b-b9466a4016e7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b75e238"; :>> judgedAt = "2026-08-25"; :>> judgedBy = "wweatherholtz"; }
}
