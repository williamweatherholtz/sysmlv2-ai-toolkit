// =====================================================================
// ProjectDeliveryS107 — Sprint 107: engineRename (D0095/D0067). Rename the PRODUCT sysmlv2 -> keel:
// the binary (`keel`), both crates+dirs (sysmlv2-cli -> keel-cli, sysmlv2-parser -> keel-parser, via
// git mv), all code-level crate identifiers (sysmlv2_cli/_parser -> keel_cli/_parser, 39+39 refs), and
// every LIVE command/doc reference (CLAUDE.md, .engine docs/skills/README/processes/views/registry,
// README, the pre-commit hook, help text, the embedded scaffold). KEPT (language / record, never
// fabricated): the `.sysml` extension, "SysML v2" prose + sysmlv2-syntax-notes.md, and all historical
// .tracking/** + .engine/decisions/** references. Bulk migration discipline (D0067): control totals
// reconciled, green at every layer.
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS107 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from engineRenameStory to d0095;

    part engineRenameStory : Story {
        :>> id = "63940516-2c7d-4e8f-c107-51627384c001";
        :>> title = "Sprint 107: engineRename (5 pts) — product sysmlv2 -> keel (binary + crates/dirs + live refs); D0095/D0067 migration";
        :>> createdAt = "2026-06-23";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::refactor;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS107 {
        action storyEngineRename;
        verification storyEngineRenameDoD : Test { :>> id = "63940516-2c7d-4e8f-c107-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0095 rename (full scope, human-chosen) under D0067 bulk-migration discipline. STRUCTURAL: git mv sysmlv2-parser -> keel-parser + sysmlv2-cli -> keel-cli; workspace + crate Cargo.toml package names; [[bin]] name sysmlv2 -> keel; the inter-crate path dep; 39 sysmlv2_parser + 39 sysmlv2_cli code identifiers -> keel_*. LIVE SWEEP (s/sysmlv2/keel/g over 55 files): CLAUDE.md, .engine/{README,docs/usage,skills/*,processes/*,views/viewpoint-registry,skills-registry,deliverable-manifest,contracts,schema/core/element}, README.md, the pre-commit hook (target/release/keel.exe), main.rs help/doc, console.html, the classify_renderer 'keel ' prefix (kept consistent with the renderers). KEPT (auto-safe + by exclusion): the `.sysml` extension (no 'v2' substring), 'SysML v2' prose (capitalized), .engine/docs/sysmlv2-syntax-notes.md (the language doc), and ALL historical .tracking/** + .engine/decisions/** prose (the record — never fabricated). CONTROL TOTAL: 0 residual lowercase product 'sysmlv2' outside the record/language areas. VERIFY (live): cargo build (keel.exe + keel-parser); clippy --workspace -D warnings; cargo test 46 lib; `keel validate .` 120 clean; `keel guard` ALL PASS (viewpoint-renderer matches the new 'keel ' prefix); kernel validate_schema 15/15 + validate_instances 111/111; `keel init /tmp/x` scaffolds a fresh keel-branded project that guards ALL PASS."; }
        part storyEngineRenameDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c107-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "98d2c74"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
    }

    verification engineRenameRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c107-51627384c004"; :>> title = "Sprint 107 refine gate (DoR)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0095 (named Keel) + the human chose the FULL rename scope (binary + crates/dirs). Resolver: engineRename. estimatedPoints=5. #CharteredBy d0095. D0067 migration: scope the keep/change boundary (product/command -> keel; language `.sysml`/SysML-v2 + historical record -> keep), reconcile control totals, green at every layer. DoD accepted."; }
    part engineRenameRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c107-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "98d2c74"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineRenameStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c107-51627384c006"; :>> title = "Sprint 107 standup gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: structural first (git mv + Cargo + code identifiers) -> confirm green build (the only thing that can break compilation) -> broad s/sysmlv2/keel/g over the LIVE surface (excluding .tracking, .engine/decisions, sysmlv2-syntax-notes.md; .sysml + 'SysML v2' auto-safe by no-v2/caps) -> rebuild -> keel validate + guard + kernel + init re-check. No blockers."; }
    part engineRenameStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c107-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "98d2c74"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineRenameImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c107-51627384c008"; :>> title = "Sprint 107 implement gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Renamed binary + both crates/dirs + 78 code identifiers + 55 live files. VERIFY (live): cargo build green (keel.exe); clippy -D warnings green; cargo test 46 lib; keel validate 120; keel guard ALL PASS (13); kernel schema 15/15 + instances 111/111; keel init -> fresh project guard ALL PASS; 0 residual product 'sysmlv2' outside record/language."; }
    part engineRenameImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c107-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "98d2c74"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineRenameReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c107-51627384c00a"; :>> title = "Sprint 107 review gate"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The keep/change boundary held precisely: lowercase product `sysmlv2` -> keel everywhere it is invoked/named, while `.sysml` (no 'v2'), 'SysML v2' (capitalized), the SysML-v2 syntax-notes doc, and ALL historical .tracking/decision prose were preserved (the record was not fabricated, D0067). The viewpoint renderers + classify_renderer prefix moved together (guard still green). The embedded scaffold + CLAUDE.md re-embed keel-branded (init re-check green). The methodology (SysML v2) stays named where it denotes the language; only the product became Keel. Clean."; }
    part engineRenameReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c107-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "98d2c74"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineRenameCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c107-51627384c00c"; :>> title = "Sprint 107 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyEngineRename DoD recorded pass; the product is now Keel (binary keel, crates keel-cli/keel-parser); all layers green (build/clippy/test/keel validate/guard/kernel/init re-check). Self-evidencing (D0051). D0095 realized — the engine ships as Keel."; }
    part engineRenameCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c107-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "98d2c74"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }

    verification engineRenameRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c107-51627384c00e"; :>> title = "Sprint 107 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-23"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: doing the STRUCTURAL rename first (dirs+Cargo+code-identifiers) and confirming a green build BEFORE the cosmetic doc sweep isolated the only compile-breaking step; the keep/change boundary exploited a happy lexical fact — lowercase `sysmlv2` is a substring of neither `.sysml` (no 'v2') nor 'SysML v2' (capitalized) — so a single global s/sysmlv2/keel/g over the live surface was SAFE without touching the language or the record; excluding .tracking + .engine/decisions honored D0067's never-fabricate-history rule; moving the viewpoint renderers + the classify_renderer prefix in the same sweep kept the guard consistent. AVOIDABLE-ISSUE SCAN: no avoidable defect — every layer (build/clippy/test/validate/guard/kernel/init) was re-verified green; the binary-rename risk (the pre-commit hook + the embedded scaffold) was handled in-sweep (hook -> keel.exe) + re-verified (init re-check). NON-blocking note (NOT debt): the `sysmlv2-syntax-notes.md` filename keeps 'sysmlv2' deliberately (it documents the SysML v2 language); historical .tracking prose still says 'sysmlv2 orient' as the record of what was run then. Estimate 5 pts held. The engine is now Keel end-to-end (live surface); the methodology stays SysML v2."; }
    part engineRenameRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c107-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "98d2c74"; :>> judgedAt = "2026-06-23"; :>> judgedBy = "claudeOpus"; }
}
