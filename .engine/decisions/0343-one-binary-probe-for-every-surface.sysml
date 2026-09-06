// D0343 (PROPOSED — NOT YET ACCEPTED) — oneBinaryProbeForEverySurface: the scaffolded pre-commit hook and every generated Claude hook embed one probe that finds the pin's own cache before PATH, and each surface says which binary gated
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0343Accept event on sign-off.
package Decision0343 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0343 : Decision {
        :>> id = "78963072-d171-4814-94ee-7cbfaf0f01fe";
        :>> title = "oneBinaryProbeForEverySurface: the scaffolded pre-commit hook and every generated Claude hook embed one probe that finds the pin's own cache before PATH, and each surface says which binary gated";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "GH#55 (st096, us095, issue378). keelw and keel init write the binary cache at .keel/bin/<version>/<asset>; the scaffolded pre-commit hook and, after D0316, the Claude hooks probed .keel/bin/keel(.exe) - a path nothing writes - and fell through to PATH with no line saying so. Measured downstream: one tree passed under keelw and failed 14 violations under the hook, and the mirror case was a false green. D0316 had fixed the ORDER of the Claude hooks' probes and left the layout mismatch: two probe texts, maintained apart, agreeing on a file that does not exist.";
        :>> decision = "One POSIX-sh probe, claude_surface::pin_probe_sh, resolves the binary for every surface: KEEL_BIN, then a released binary dropped at .keel/bin/keel(.exe), then the pin's own cache .keel/bin/<engine pin>/<asset> with the pin read from engine-version.toml, then PATH; the scaffolded pre-commit hook is rendered from it (precommit_hook is a function, not a constant) and every generated Claude hook embeds the same text, held equal by test; the pre-commit hook prints the binary it gated with and that binary's version line before gating; every hook fire records bin and build in the fire-ledger and keel status shows the last. NOT A FORK: the reporter's three candidates were calling keelw, teaching the hook the cache layout, or refusing without PATH - this takes the second for the hook and the Claude hooks alike, keeps PATH last so this self-build and a contributor without a pin still gate, and makes the fallback loud, which is what made it dangerous.";
        :>> rationale = "Why one probe text rather than one order: D0316 held the ORDER equal by test and the two texts still disagreed on the path - a list of steps is not a probe, the shell text is. Why not call keelw from the hooks: keelw downloads on a miss, and a commit hook that reaches the network on a cold cache is a different failure class; the hooks now find what keelw already put there. Why keep PATH: this repository has no pin cache and gates from its build, and refusing would break every contributor without one - the fix for a silent fallback is to say it, every time. Arming (D0298): the order test holds the hook and every Claude hook to the same probe text; a behavioural test runs the probe through sh against a directory whose only binary is .keel/bin/9.9.9/keel-linux-x86_64 with no keel on PATH and gets that path back; a live scaffold with a planted pin-cache binary and PATH=/usr/bin ran its pre-commit hook against the planted binary and printed via ./.keel/bin/0.3.1/keel-windows-x86_64.exe.";
        :>> consequences = "A fresh project's commits and turns run the release keelw fetched, and the hook says so on every commit. Marked safety-change: the scaffolded hook text and the hook set are enforcement surface, so under D0337 this waits for the human's acceptance; the code ships because every arm gates and states what gated. issue378 resolves on dcOneBinaryResolutionForEverySurface's pass.";
    }
}
