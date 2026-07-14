// =====================================================================
// D0112 — PROPOSED (not accepted): direction for retiring the JVM kernel from the .engine validation
// path. Deliverable of the issue066 research spike (jvmRetirementSpike). Design artifact:
// docs/proposals/2026-07-14-jvm-retirement-issue066.md. Awaits an explicit human acceptance gate
// before any implementation (D0106) — recorded here as status=proposed, NO acceptance event.
// =====================================================================
package Decision0112 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0112 : Decision {
        :>> id = "10a1b0c1-4112-42a3-ab5c-516273849712";
        :>> title = "D0112 (proposed): retire the JVM kernel from the .engine path incrementally (Option C), not via full Rust SysML-semantics reimplementation";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::proposed;
        :>> context = "D0048 kept the JVM kernel as the .engine schema/workflow/instance SysML oracle because the Rust validator does not cover the deep SysML-v2 semantics; the cost is conda + a ~20s JVM start on every .engine commit plus environment fragility (issue066). The issue066 spike measured what the kernel validators actually assert: validate_instances.py already has TWO pure-Python checks (an `import EngineWork` lint + a missing-`:>> id` check) and one kernel step that fails on error keywords. Those keywords split into PARSE errors (unexpected/no viable/extraneous/mismatched/wasn't expected — already caught by the Rust parser) and NAME/TYPE-RESOLUTION errors (unresolved/cannot/couldn't — the true kernel-only delta). Rust already resolves cross-file references for .tracking (D0048), just not aimed at .engine or as a full type-conformance checker.";
        :>> decision = "PROPOSED direction (Option C of the spike, not yet accepted): (1) port the two pure-Python checks to Rust guards (zero kernel; immediate); (2) extend the Rust cross-file reference resolver to .engine (type/name EXISTENCE across schema+instances, reusing the .tracking machinery) to catch the common `unresolved` class; (3) DEMOTE the kernel to an optional CI-only deep-semantics backstop for the residual (full type-conformance/specialization validity). Full kernel retirement (Option A — reimplement the SysML type system in Rust) is explicitly REJECTED as premature; it stays a later step contingent on measuring that the kernel-only residual catches nothing real.";
        :>> rationale = "ALTERNATIVES weighed in docs/proposals/2026-07-14-jvm-retirement-issue066.md: (A) full Rust SysML semantic checker — REJECTED as high-cost/high-risk and disproportionate to how rarely .engine changes; (B) status quo — REJECTED as it keeps the conda/JVM pain on the engine path; (C CHOSEN-AS-PROPOSAL) targeted incremental port + kernel-as-CI-backstop — attacks the real cost (local conda/JVM) at low risk, keeps an honest deep-semantics backstop, and preserves the retire-fully option once measurable; (D) alternative SysML checker library — feasibility UNKNOWN, worth a brief scan but not assumed. CRITERIA: attacks-real-cost x low-risk x honest-backstop x reversible/incremental x no-premature-boil-the-ocean.";
        :>> consequences = "This Decision is a DIRECTION, not an implementation — it changes nothing until accepted (D0106 human gate; recorded here as status=proposed with NO acceptance event, never AI-fabricated). If accepted: it decomposes into a small first sprint (the two pure-Python checks -> Rust guards; measurably no kernel needed for those) then the .engine reference resolver; the kernel stays for CI deep checks. If rejected/deferred: the status quo (D0048) stands and issue066 is closed by this recorded analysis. The spike (jvmRetirementSpike) is DONE on delivery of this proposed Decision + the design artifact, resolving issue066; the follow-on implementation is separately chartered and gated.";
    }
}
