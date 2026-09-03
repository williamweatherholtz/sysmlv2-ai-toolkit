// ENGINE CONTROL STRUCTURE - the AUTHORED RESIDUE of STPA step 2 for keel itself (st066).
// Everything derivable is a view (Invariant 1). Control actions and feedback are COMPUTED by the
// safety viewpoint's renderer (dcControlStructureIsComputed) from facts that already exist: hook
// events in .claude/settings.json, git hooks in .githooks/, CI and channel steps in
// .github/workflows/*.yml, the CliCommand facts (D0271: effect = writes -> control action, reads ->
// feedback), the guard set and control-map.sysml (refusals), github-actors.toml (declared deciders),
// and the remote's branch protection (an external fact fetched, never copied). What CANNOT be derived
// and is therefore authored here: which controlled processes exist, which controller anchors the
// computed actions attach to, what each controller BELIEVES (a process model is a judgment), and
// which process each hazard is a state of.
package EngineControlStructure {
    private import EngineElement::*;
    private import EngineSafety::*;

    // ---- Controller anchors: the authorities (titles only; their actions are computed) ----
    part ctHuman : Controller {
        :>> id = "bda48e53-ae1f-46ea-bf1b-d7377a84eaa7";
        :>> title = "Human director (wweatherholtz)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part ctAgent : Controller {
        :>> id = "d4afd6c5-1fdc-4449-9cec-6061af3b4c7f";
        :>> title = "AI agent (claudeOpus5 / claudeFable5, Claude Code session)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part ctHooks : Controller {
        :>> id = "3a66096d-226c-45d2-a424-4ff5fb848d04";
        :>> title = "keel binary at the Claude Code hook boundary";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part ctCommitGate : Controller {
        :>> id = "c19ed41e-89b5-4df3-9fdd-2a7aef818d02";
        :>> title = "keel binary at the git hook boundary (.githooks)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part ctCI : Controller {
        :>> id = "3e8cb27d-fb78-4959-87c6-86a060d91183";
        :>> title = "GitHub Actions ci.yml";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part ctRemote : Controller {
        :>> id = "47c9294e-7e74-440c-8340-3a9accd7c353";
        :>> title = "GitHub remote (origin/main, branch protection)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part ctConsole : Controller {
        :>> id = "703ce3de-e465-47ed-9a02-9ae6e908589f";
        :>> title = "keel serve on 127.0.0.1:7777 (the human's oversight lens)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }

    // ---- Controlled processes: what the authorities act on ----
    part cpModel : ControlledProcess {
        :>> id = "74870947-1e13-438c-bd4a-ca81e87196ae";
        :>> title = "The recorded model (.tracking + .engine instance files in the working tree)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part cpMainRef : ControlledProcess {
        :>> id = "c2bb8c38-81b2-488d-b2c8-e9df79645527";
        :>> title = "The shared history (origin/main)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part cpEnforcementSurface : ControlledProcess {
        :>> id = "36f06417-776d-477c-b794-fe7a17870f1d";
        :>> title = "The enforcement surface (guards.rs, adherence.rs, .githooks/, .github/workflows/*.yml, .engine/processes, .engine/skills, contracts)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part cpDeliverable : ControlledProcess {
        :>> id = "48e33b69-656f-4271-845d-ec5f85625e2e";
        :>> title = "The deliverable (keel-cli source and tests)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part cpWork : ControlledProcess {
        :>> id = "0a1e13c3-627d-44a0-a4c4-d936b47d247b";
        :>> title = "The work in flight (sprints, ceremonies, the frontier)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }
    part cpAgentTurn : ControlledProcess {
        :>> id = "04631a34-6d12-4413-b4cf-35417a8f4201";
        :>> title = "The agent's turn (one Claude Code response, its tool calls and edits)";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
    }

    // ---- Process models: what each controller believes, and which beliefs are known false ----
    part pmHuman : ProcessModel {
        :>> id = "aac29139-0beb-4c88-a0a9-4ba44aa8ac06";
        :>> title = "What the human believes";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> heldBy = ctHuman;
        :>> beliefs = "That an auto-accepted Decision is fine unless overridden; that a green gate means the model is honest (true) and the work is complete (false by design, D0098); that the DoD verified is the DoD agreed (FALSE today - issue343: the owner may rewrite it after the pass); that a pass with a receipt was produced by what the receipt names (unchecked - CI does not re-run receipts).";
    }
    part pmAgent : ProcessModel {
        :>> id = "5927f4af-ea4a-4c01-8611-bc54b02d47eb";
        :>> title = "What the agent believes";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> heldBy = ctAgent;
        :>> beliefs = "That the ready list is the priority (true, D0052); that a green Stop gate ends the turn honestly (true for FORM); that its own judgment on demo/inspect/analyze suffices as verification (the belief st065 withdraws); that a non-fork classification is its call (true, and the channel acts on it).";
    }
    part pmHooks : ProcessModel {
        :>> id = "76e33b3a-7aac-4840-ae7c-8e4d2c6a6398";
        :>> title = "What the hooks believe";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> heldBy = ctHooks;
        :>> beliefs = "That the session is an agent session because the environment says so (agent-mutable); that the binary they resolve is the pinned one (true only where KEEL_BIN or PATH agrees with the pin - issue 43 downstream); that a clean fast tier means the edit was safe to continue (form only).";
    }
    part pmCI : ProcessModel {
        :>> id = "8380019d-4ce9-4a76-80a7-e50d717ff0af";
        :>> title = "What CI believes";
        :>> createdAt = "2026-09-02"; :>> createdBy = "claudeOpus5";
        :>> heldBy = ctCI;
        :>> beliefs = "That the pushed range is the whole story (true - audit-history walks it); that its verdict is read (false often enough to have a Decision, D0266); that its pinned binary is the engine under test (true here, where it builds from source).";
    }

    // ---- Every hazard in engine-safety.sysml is a state of one of these processes ----
    dependency from ehz1 to cpMainRef;
    dependency from ehz2 to cpModel;
    dependency from ehz3 to cpModel;
    dependency from ehz4 to cpModel;
    dependency from ehz5 to cpEnforcementSurface;
    dependency from ehz6 to cpModel;
    dependency from ehz7 to cpEnforcementSurface;
    dependency from ehz8 to cpModel;
    dependency from ehz9 to cpDeliverable;
    // The structure answers the statement that asked for it.
    #DerivedFrom dependency from cpModel to st066;
}
