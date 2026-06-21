// =====================================================================
// ProjectDeliveryS76 — Sprint 76: parserPackageGuard (resolves issue027; D0047). Make the Rust
// parser FAIL LOUD on items declared outside the top-level package (it silently dropped them, so a
// stray-brace file parsed 'clean' while losing content — found authoring D0083). The parser is the
// guard: every validate/check/orient parses, so it's enforced everywhere incl pre-commit.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS76 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from parserPackageGuardStory to d0048;

    part parserPackageGuardStory : Story {
        :>> id = "63940516-2c7d-4e8f-b076-51627384ad01";
        :>> title = "Sprint 76: parserPackageGuard (2 pts) — parser fails loud on out-of-package items; resolves issue027";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS76 {
        action storyParserPackageGuard;
        verification storyParserPackageGuardDoD : Test { :>> id = "63940516-2c7d-4e8f-b076-51627384ad02"; :>> method = VerificationMethod::test; :>> procedureText = "Resolves issue027 (D0047): parse() now requires the token stream be exhausted after parse_package — any out-of-package/trailing item errors (ParseError::Expected 'end of file; one package per file; all items must be inside the package'). The parser is the permanent guard (every validate/check/orient parses). VERIFY: stray-brace file now FAILS `sysmlv2 check` with a clear message (was silently clean); regression test content_after_package_is_rejected; all 88 real files still clean; clippy --workspace -D warnings + cargo test (31 lib) + rust validate green; issue027 computes RESOLVED."; }
        part storyParserPackageGuardDoDR1 : TestResult { :>> id = "40cb5f34-b514-4ede-a5c7-675505501ee6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "492c5e6"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification parserPackageGuardRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b076-51627384ad03"; :>> title = "Sprint 76 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: issue027 — the parser silently drops items after the top-level package (found when a stray brace in D0083 exiled its accept event; `check` reported clean). Per D0047 a recurrable fail-quiet defect must get a permanent automated guard. Fix: parse() asserts Eof after parse_package. The parser itself is the guard (upstream of all tooling). No legitimate file has >1 package (verified) so no false positives. estimatedPoints=2. #CharteredBy d0048 (Rust authority). DoD accepted."; }
    part parserPackageGuardRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b076-51627384ad04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "492c5e6"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification parserPackageGuardStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b076-51627384ad05"; :>> title = "Sprint 76 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: in parse() (parser.rs), after parse_package, peek the next token; if not Eof, return ParseError::Expected{expected='end of file (one package per file; all items must be inside the package)', got, line, col}. Add regression test content_after_package_is_rejected. Verify all real files still parse + a stray-brace fixture now errors. No blockers."; }
    part parserPackageGuardStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b076-51627384ad06"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "492c5e6"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification parserPackageGuardImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b076-51627384ad07"; :>> title = "Sprint 76 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the Eof-after-package check in parse() + regression test. VERIFY (live): a fixture with a part after the package -> `sysmlv2 check` ERROR 'expected end of file (one package per file; all items must be inside the package), got Part' (was silently clean); cargo test 31 lib pass (incl content_after_package_is_rejected); clippy --workspace --all-targets -D warnings green; `sysmlv2 validate .` 88 files clean (zero false positives — no real file has >1 package). issue027 resolver (this) done -> issue027 computes RESOLVED."; }
    part parserPackageGuardImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b076-51627384ad08"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "492c5e6"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification parserPackageGuardReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b076-51627384ad09"; :>> title = "Sprint 76 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. Minimal, correct fail-loud fix: parse() now exhausts the stream, turning a fail-QUIET defect (silently dropped content) into a fail-LOUD parse error with file:line:col. The parser being upstream of validate/check/orient/guards means it's enforced everywhere — no separate forward-guard needed (D0047 satisfied by the validator-level check). Safe: verified no legitimate file has trailing/out-of-package content (no >1-package files), and all 88 parse clean. Regression-tested. issue027 resolves by computation (#Resolves from this action). This is the assurance loop completing: a defect the assurance machinery caught is now permanently guarded."; }
    part parserPackageGuardReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b076-51627384ad0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "492c5e6"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification parserPackageGuardCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b076-51627384ad0b"; :>> title = "Sprint 76 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyParserPackageGuard + backlog parserPackageGuard DoDs recorded pass; parser fails loud on out-of-package items; regression test added; clippy + cargo test + rust validate + guard all green. issue027 computes RESOLVED (resolver done). Self-evidencing (D0051). D0047 honored: the fail-quiet defect now has a permanent automated guard (the parser itself)."; }
    part parserPackageGuardCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b076-51627384ad0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "492c5e6"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification parserPackageGuardRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b076-51627384ad0d"; :>> title = "Sprint 76 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: textbook D0047 loop — a correction (the D0083 brace slip) revealed a recurrable fail-quiet defect (parser dropping out-of-package items), which is now a permanent automated guard (the parser fails loud) + a regression test, not patched-and-forgotten. The deeper win: the assurance machinery CAUGHT its own author's structural bug (attestation-coverage flagged the missing accept event), and that catch is now generalized so the next stray brace fails at parse for everyone. CAUGHT (avoidable-issue candidate): the original parse() never checked for Eof after the package — a latent gap that predated this session; fixed at the root. No new issue. Estimate 2 pts held. The parser is now fail-loud on this class; the assurance arc (coverage->critique->readiness->charter-time->three-tier->orient-perf->target-revision->parser-guard) is fully closed and self-hardening."; }
    part parserPackageGuardRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b076-51627384ad0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "492c5e6"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
