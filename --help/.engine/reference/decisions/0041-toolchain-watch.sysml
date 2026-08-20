// =====================================================================
// D0041 — toolchainWatch: SysML v2 toolchain feature adoption decision.
// Sprint 14; accepted 2026-06-16 by wweatherholtz (sprint execution).
// =====================================================================
package Decision0041 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0041 : Decision {
        :>> id = "5f2a3b4c-6d7e-4f8a-9b0c-1d2e3f4a5b6c";
        :>> title = "D0041: Defer expose/render, state-def-body, and derive/refine/trace toolchain features";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeSonnet46";
        :>> status = DecisionStatus::accepted;
        :>> context = "toolchainWatch sprint evaluated three feature areas: (1) expose/render in view/viewpoint — SysML v2 spec feature, requires SysIDE live kernel. (2) state def body syntax — still blocked in pilot 0.59.0 (D0031). (3) derive/refine/trace — SysML v1 keywords; v2 uses :> idiomatically (deferred per crNativeWins). (4) verify-by — not a standard SysML v2 keyword; native verification def + verify target is equivalent.";
        :>> decision = "Defer all three evaluated SysML v2 toolchain feature areas: expose/render in view/viewpoint, state def body syntax, and derive/refine/trace. Re-evaluate when a new pilot release ships exposed-body state def support or a standalone render API.";
        :>> rationale = "The current stack (Rust parser + pilot 0.59.0 validators + query.py) is stable and self-sufficient. expose/render requires a SysIDE live kernel and is incompatible with the text-file-first approach; state def body syntax still fails in pilot 0.59.0 (D0031); derive/refine/trace are SysML v1 keywords (v2 uses :> idiomatically per crNativeWins) and verify-by is not a standard v2 keyword (native verification def + verify target is equivalent).";
        :>> consequences = "No toolchain changes in this sprint. sysmlv2-syntax-notes.md updated to note toolchainWatch verdict. WorkflowDefinition.transitions stays String[*] until state def body syntax is supported in a new pilot release.";
    }

    verification d0041Accept : Test { :>> id = "2d2cd214-5328-4747-bc41-57bbb79df897"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0041 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0041AcceptR1 : TestResult { :>> id = "060e8617-4743-4642-b867-d4a867a83adc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
