// =====================================================================
// D0041 — toolchainWatch: SysML v2 toolchain feature adoption decision.
// Sprint 14; accepted 2026-06-16 by wweatherholtz (sprint execution).
// =====================================================================
package Decision0041 {
    private import EngineWork::*;

    part d0041 : Decision {
        :>> id = "5f2a3b4c-6d7e-4f8a-9b0c-1d2e3f4a5b6c";
        :>> title = "D0041: Defer expose/render, state-def-body, and derive/refine/trace toolchain features";
        :>> createdAt = "2026-06-16";
        :>> authoredBy = "claudeSonnet46"; :>> createdBy = "claudeSonnet46";
        :>> status = DecisionStatus::accepted;
        :>> context = "toolchainWatch sprint evaluated three feature areas: (1) expose/render in view/viewpoint — SysML v2 spec feature, requires SysIDE live kernel. (2) state def body syntax — still blocked in pilot 0.59.0 (D0031). (3) derive/refine/trace — SysML v1 keywords; v2 uses :> idiomatically (deferred per crNativeWins). (4) verify-by — not a standard SysML v2 keyword; native verification def + verify target is equivalent.";
        :>> decisionText = "ALL DEFERRED. Current stack (Rust parser + pilot 0.59.0 validators + query.py) is stable and self-sufficient. expose/render incompatible with text-file-first approach; state def body still fails in pilot 0.59.0; derive/refine/trace are v1-only. Re-evaluate when a new pilot release ships with exposed-body state def support or standalone render API. Accepted: wweatherholtz 2026-06-16 at commit aba9e6f.";
        :>> consequences = "No toolchain changes in this sprint. sysmlv2-syntax-notes.md updated to note toolchainWatch verdict. WorkflowDefinition.transitions stays String[*] until state def body syntax is supported in a new pilot release.";
    }
}
