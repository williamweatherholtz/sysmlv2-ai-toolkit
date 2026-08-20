// =====================================================================
// D0074 — full-Rust is the target for the toolchain; Python is transitional (prototyping).
// New authority-bearing tooling is written in Rust with fail-loud guarantees; the existing
// Python views + guards are ported on a sequenced plan. Clarifies/extends D0048 (which made
// Rust the authority for the core but left the boundary implicit).
// Accepted 2026-06-18 by wweatherholtz ("we go full rust eventually ... stronger performance and
// compatibility guarantees, and fails more loudly ... rust with strong guarantees, please").
// =====================================================================
package Decision0074 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0074 : Decision {
        :>> id = "63940516-2c7d-4e8f-a074-516273849656";
        :>> title = "D0074: full-Rust toolchain is the target; Python is transitional; fail-loud Rust conventions + a sequenced port plan (extends D0048)";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0048 promoted Rust to AUTHORITY for the core (validate / orient / whats-next / write API) but did not say what happens to the rest of query.py (the view/reader layer) or the Python pre-commit guards — the boundary was implicit, and new views (governance, audit) kept landing in Python by default. The user resolved the ambiguity: go full Rust. Rationale given: stronger performance + compatibility guarantees, and it fails more loudly; Python is fine for prototyping but the engine is past prototyping.";
        :>> decision = "FULL-RUST is the target end state for the toolchain. Python is TRANSITIONAL: new authority-bearing tooling is written in Rust; existing Python (the query.py view/reader layer + the pre-commit guard scripts) is ported to Rust on a sequenced plan, not rewritten ad hoc. FAIL-LOUD Rust conventions (enforced in CI clippy): #![forbid(unsafe_code)]; deny clippy::unwrap_used, expect_used, panic, indexing_slicing, todo, unimplemented, and truncating `as` casts; -D warnings; typed errors (thiserror in libs, anyhow at the binary edge) with Result + `?` everywhere; NO silent fallbacks (an unwrap_or_default that masks a missing fact is banned); #[serde(deny_unknown_fields)] on all config/declaration structs so typos fail at parse; validate at boundaries and return errors, not defaults; non-zero exit on any error. PORT PLAN (sequenced, each a tracked sprint): M1 = the Rust TOML view executor (D0075) + the typed-edge extraction it needs; M2 = port the remaining query.py query-views (issues/orphans/charter/governance/attestation-coverage) to Rust; M3 = port the Python pre-commit guards (ceremony/coverage/charter/process-change/acceptance) to Rust; M4 = retire query.py (resolving the issue012 orient undercount, since the Rust orient becomes the only one).";
        :>> rationale = "A single authoritative implementation with compile-time + lint guarantees beats two drifting ones (the issue012 query.py-vs-rust orient divergence is the warning). Fail-loud (deny unwrap/panic, no silent fallbacks) makes latent errors crash visibly instead of corrupting state silently — the same value the loud validators already provide, extended to all tooling. A SEQUENCED port (not a big-bang rewrite) keeps each step green and reviewable (Fowler strangler-fig). ALTERNATIVES: (A) keep Python as the permanent view/guard layer (the boundary-option-a I floated) — rejected by the user: weaker guarantees, two implementations, drift; (B) big-bang rewrite to Rust — rejected: high risk, long red period; (C CHOSEN) Rust-first for new work + a sequenced strangler-fig port. CRITERIA: one-authoritative-impl x fail-loud x green-at-every-step x performance/compatibility.";
        :>> consequences = "New tooling lands in Rust under these conventions (a clippy config + CI enforcement is the first concrete artifact, M0). The Rust/Python boundary is now DECIDED (port, not keep). query.py + the Python guards are explicitly transitional, tracked for porting (M2/M3) and eventual retirement (M4). Pairs with D0048 (Rust authority), D0054 (friction — the port must not raise authoring friction; the write API + fast validation stay), D0075 (the view model the M1 executor implements). Does not change any process-def file, so no process-change marker required (D0070). The migration increments are added to the backlog as tracked tasks.";
    }

    verification d0074Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a074-516273849657";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0074 (full-Rust target; Python transitional; fail-loud conventions; sequenced port plan M1-M4) via 'i think we go full rust eventually. stronger performance and compatibility guarantees, and fails more loudly ... rust with strong guarantees, please (e.g. deny unwrap, etc...)' + selecting '#3 Rust now + plan the full port' on 2026-06-18.";
    }
    part d0074AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a074-516273849658";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "909764e";
        :>> judgedAt = "2026-06-19";
        :>> judgedBy = "wweatherholtz";
    }
}
