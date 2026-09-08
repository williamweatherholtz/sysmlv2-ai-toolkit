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

    part pmCommitGate : ProcessModel {
        :>> id = "d64667a8-0917-48ae-8487-f5b77792cd68";
        :>> title = "What the commit gate believes";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> heldBy = ctCommitGate;
        :>> beliefs = "That the staged tree is the tree being committed (true - it gates the index, not the working copy); that the binary core.hooksPath resolved is the pinned engine (true only through the probe, D0343/GH#55 - PATH used to decide it); that HEAD after the commit is the SHA its results will be judged against (false whenever the post-commit land merges first); that a green fast tier means the model is honestly formed, never that the work is complete (D0098).";
    }

    part pmRemote : ProcessModel {
        :>> id = "34c14f6f-8ba2-44fc-8342-dc32b7d8f4d2";
        :>> title = "What the remote believes";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> heldBy = ctRemote;
        :>> beliefs = "That whoever holds the push credential is the actor named in the commit (false - the credential is a machine's, the commit's author field is prose); that a ref update is safe when it is a fast-forward (true for history, silent about content - it reads no gate); that a required status check is what makes CI preventive (true, and none is required here, so CI is detective - remote.reading).";
    }

    part pmConsole : ProcessModel {
        :>> id = "0a0cc6ff-6162-4f75-a3a6-74c5803a799c";
        :>> title = "What the console believes";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> heldBy = ctConsole;
        :>> beliefs = "That a tap signed by the paired device is the human's judgment (true as far as a device can prove - D0334/D0335 bind the device, not the person); that the working tree it serves is the tree the agent's session sees (true - and the reason its writes must be diffed out of the agent's commit); that the queue it renders is the whole set awaiting the human (true - authority-queue is the same lens the brief reads).";
    }

    // ---- The fifth element type (Handbook p.25; D0363): what crosses the boundary that is neither
    // a control action nor feedback. Inbound at a controller shapes its process model without being a
    // measurement of any process; outbound leaves the system for a party that is not a controller here.
    part oioModelApi : OtherInputOutput {
        :>> id = "9df92981-4085-45b4-8adf-76fe4864bec8";
        :>> title = "the model's completions";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "the Anthropic API - the agent's every judgment is a sample from it (issue392: two of four verdicts flipped on identical inputs)";
        :>> atController = ctAgent;
    }

    part oioGithubIssues : OtherInputOutput {
        :>> id = "9b139613-da8c-4c77-8e8c-d9fe68e81b8c";
        :>> title = "issues filed on the repository";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "anyone who can open an issue - an unauthenticated stranger when the repository is public, which is why D0264 routes by visibility";
        :>> atController = ctAgent;
    }

    part oioSessionEnvironment : OtherInputOutput {
        :>> id = "593f857f-f2e9-414a-88a9-6c5ab88b3e5c";
        :>> title = "the harness's session environment";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "Claude Code - the hook event payload, KEEL_BIN, PATH and the settings precedence decide which binary fires and whether it fires at all (D0296, GH#43)";
        :>> atController = ctHooks;
    }

    part oioHostClock : OtherInputOutput {
        :>> id = "9c73f5b1-86e1-48a1-b29e-e6ca96ad15fc";
        :>> title = "the host clock and file mtimes";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "the operating system - the guard receipt is keyed on mtimes and refuses a path younger than two seconds (D0371), and every judgedAt is the host's date";
        :>> atController = ctHooks;
    }

    part oioHooksPathConfig : OtherInputOutput {
        :>> id = "3b39d700-8eb4-498b-b244-167360dc4529";
        :>> title = "core.hooksPath and the resolved binary";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "the machine's git config - machine-local, uncommitted, and the whole reason the commit gate runs at all; a clone without it commits ungated";
        :>> atController = ctCommitGate;
    }

    part oioToolchain : OtherInputOutput {
        :>> id = "8569c65e-9d72-4a0c-a396-d7fb326f569d";
        :>> title = "the runner image and the Rust toolchain";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "GitHub-hosted runners, rustup and crates.io - CI builds the engine under test from source on whatever they serve that day";
        :>> atController = ctCI;
    }

    part oioPushCredential : OtherInputOutput {
        :>> id = "0b994ca6-031e-45ef-85ad-eea6c76a979a";
        :>> title = "the push credential";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "the machine's credential helper - the remote authenticates a token, and the token belongs to a machine both the human and the agent push from";
        :>> atController = ctRemote;
    }

    part oioPairedDeviceKey : OtherInputOutput {
        :>> id = "3fc79442-d78f-4318-bed5-32eec9bace6a";
        :>> title = "the paired device's key";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "the browser that paired with the code the serving terminal printed (D0334) - a device, and whoever holds it";
        :>> atController = ctConsole;
    }

    part oioUncommittedEdits : OtherInputOutput {
        :>> id = "f5d5fabf-a742-4943-9ab9-429cc0178e62";
        :>> title = "out-of-band edits to the working tree";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::inbound;
        :>> externalParty = "an editor, the console, or a second session writing the same working tree - they reach the model through no keel command and no hook sees them (S-F9)";
        :>> atProcess = cpModel;
    }

    part oioDecisionBrief : OtherInputOutput {
        :>> id = "3266acaa-861f-47c9-b06f-d91d4242e379";
        :>> title = "the published decision brief";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::outbound;
        :>> externalParty = "the claude.ai artifact host - the one page the pending set is published to (D0359), read outside every control here";
        :>> atController = ctAgent;
    }

    part oioReleaseBinary : OtherInputOutput {
        :>> id = "1c17492b-68b4-4266-b375-a465fb014e7c";
        :>> title = "the released binary";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> direction = FlowDirection::outbound;
        :>> externalParty = "downstream projects - keelw and keel init fetch the release asset the tag names, and a defect here becomes theirs (D0275, D0302)";
        :>> atController = ctRemote;
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
