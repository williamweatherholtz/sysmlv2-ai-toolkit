// Intake for 2026-08-28 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260828 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st023 : Statement {
        :>> id = "1bb50688-15a5-4a16-8fa1-3f3cc45c15ed";
        :>> title = "Verdict on the bottom-up needs pass: create U1-U12 as Needs; STPA is a process; leverage Result for arming; back-identify and consolidate latent needs";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> text = "I like your #61-6 items.  Though I actually think #1 is pretty good!  The truth is that the point of keel really is to manage AI work - the sysml part is just a means to an end, if a very defensible one.  I do think the difference between use cases and needs can be very thin, and that these are more need statements than empirically-driven use cases.  So sure, create them as needs.  #3 U11 - Rust has means of identifying this through error handling, no?  It's a sematic distinction that seems well aligned with this.  We should leverage Result.  Your #4 - STPA is really a process.  It's not a need explicitly - some confusion early on while we were building and I didn't have processes defined very strongly.  One that I expect to use regularly, but not a core function in the way sysmlv2 representations are.  #2 - decisions & needs: we do need stronger backidentifying of needs from decisions & use cases.  We state them, and they can strongly imply a need, but that need isn't entered.  Still - we don't suffer much pratically from it because having a need stated directly is really just a source for requirements - it's not that big of a deal if it's missing.  It's still a deal though - if we have latent needs, we should be identifyign them to strengthen the direction of our program.  Having a use case for ''typing A'', another for ''typing B'' , etc. could be consolidated into the need for having a keyboard.  That would be a tight, concise paring down of real functional needs.  ANd that is important.  Better as a process than as a core function?  Maybe everything is really a process...";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-28"; :>> channel = StatementChannel::chat;
    }

    part st024 : Statement {
        :>> id = "84d19883-2f6e-4738-aa9b-0f25ff3422a8";
        :>> title = "Reframe keel as a per-project versioned dependency shipped with the project, and make templates + AI skills portable the same way";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> text = "speaking of needs, i want to change keel's framing a bit.  I want independent keel shipped with every project, (as a submodule?) so that i can avoid needing it on path, and it seems proper to version keel per project (since it's a dependency) anyway, no?  also - i want more things to be portable, a set of templates from a template project i want to transfer along with it as a submodule, other AI skills, ... best practices here?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-28"; :>> channel = StatementChannel::chat;
    }

    part us028 : UserStory {
        :>> id = "a26ff5b5-4a98-43b0-86f7-b4f2e39e9e12";
        :>> title = "keel scope is AI-work management; SysML representation is the means";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "keel framed as an engine for managing AI work, with SysML v2 representation as a defensible means rather than the end";
        :>> soThat = "scope decisions like where the viewer Needs live follow from what keel is for, instead of from what it is built on";
        :>> implication = ImplicationKind::scopeConstraint;
        :>> triageNote = "scopeConstraint and not architecture: it does not choose a structure, it bounds what is in and out of keel. It is the stated reason the 19 nViewer Needs belong in the sibling viewer repo, and it contradicts the projectBrief problem statement, which frames keel as a fix for scattered requirements documents. Not correction, because they are not retracting a recorded fact - they are stating a scope boundary that the Brief was authored before.";
    }
    #DerivedFrom dependency from us028 to st023;

    part us029 : UserStory {
        :>> id = "c21ade9c-9aa2-46bd-878e-051f2297300c";
        :>> title = "Author the twelve derived capabilities as Needs";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the twelve capabilities the bottom-up pass found with no Need behind them authored as Needs";
        :>> soThat = "the eighty-six percent of work chartered by Decisions with no Need root has a stated direction to answer to";
        :>> implication = ImplicationKind::need;
        :>> triageNote = "need and not useCase: they explicitly said the line is thin and that these read as need statements rather than empirically-driven use cases, and directed that they be created as needs. Their word governs the type.";
    }
    #DerivedFrom dependency from us029 to st023;

    part us030 : UserStory {
        :>> id = "155c6b79-b853-49ad-864a-f0f68dc0fbaf";
        :>> title = "STPA is a process keel deploys, not a Need and not a core function";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "STPA recorded as a process I expect to use regularly, explicitly not as a Need and not a core function of the engine";
        :>> soThat = "the safety schema stops reading as an unrooted capability gap, and core function stays what SysML v2 representation is";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not scopeConstraint: they are not bounding scope, they are classifying an existing artifact - STPA stays and is expected to be used regularly. Not none, because it resolves a live ambiguity the needs pass raised: 15 declared safety types with 3 instantiated and no Need. They also gave the cause - processes were not strongly defined when the safety schema was authored - which is a fact about the record and belongs with the classification. Open tail this does NOT settle: whether schema/core-adjacent safety types should travel inside the stpa process unit rather than sit beside the frozen core schema.";
    }
    #DerivedFrom dependency from us030 to st023;

    part us031 : UserStory {
        :>> id = "483338c2-1f81-4f85-baf6-cec88c6c6e6f";
        :>> title = "Back-identify latent Needs from Decisions and UseCases and consolidate them";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "a repeatable way to recover the Needs that Decisions and UseCases imply but never state, and to consolidate many narrow ones into the single functional Need behind them";
        :>> soThat = "the program has a tight concise set of real functional needs giving it direction, rather than latent needs scattered across 415 decision-chartered stories";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not need: they asked for the MECHANISM that produces Needs, not for a Need. They named the shape themselves - a use case for typing A and another for typing B consolidate into the need for a keyboard - which is abstraction, not extraction, and is why this cannot be a one-shot pass. D0040 forces the same conclusion independently: it will recur, so it must be a process before it is executed. Their own framing question, process or core function, is triaged separately as a question.";
    }
    #DerivedFrom dependency from us031 to st023;

    part us032 : UserStory {
        :>> id = "bc1f77ab-01a0-4189-86b9-7f40771f347a";
        :>> title = "Is need derivation better as a process than a core function - and is everything really a process";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "an answer on whether need derivation belongs as a process rather than a core engine function, and on the wider claim that maybe everything is really a process";
        :>> soThat = "the boundary between what the engine computes and what a process instructs is stated rather than assumed case by case";
        :>> implication = ImplicationKind::question;
        :>> triageNote = "question and not process: it implicates no change on its own, it asks for a position. Recorded separately from the derivation story because that story is actionable whichever way this is answered, and folding an unanswered framing question into an actionable story is how the question quietly goes unanswered.";
    }
    #DerivedFrom dependency from us032 to st023;

    part us033 : UserStory {
        :>> id = "79c592fa-82ad-47dd-abeb-135ae38fc26c";
        :>> title = "keel ships with every project as a per-project versioned dependency, not from PATH";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "project author";
        :>> iWant = "an independent keel shipped with every project and versioned per project, so no project depends on a keel being on PATH";
        :>> soThat = "a project pins the engine version it is gated by, the way it pins any other dependency";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture and not process: it changes how the engine is DELIVERED and where its version is pinned, which is structure, not how work is done. It bears on accepted D0175 (install from the Releases page only, no package managers, no auto-update, no install scripts) and on engine-version.toml, which today records the vintage for a parity warning and does not pin anything - so this is an amendment to a recorded direction and needs a Decision, not an edit. Their submodule suggestion is raised as a question mark and is triaged with the best-practice question, not baked into this story.";
    }
    #DerivedFrom dependency from us033 to st024;

    part us034 : UserStory {
        :>> id = "305dd733-c35e-4f95-a6f5-d01d3d39cdf2";
        :>> title = "Templates and AI skills travel with a project the way processes do";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "project author";
        :>> iWant = "a template project set of templates, AI skills and similar content to transfer along with a project";
        :>> soThat = "starting a project inherits the working material, not just the engine";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture and not process: the portable UNIT and its transfer mechanism are structure. Distinct from the keel-binary story because the answers differ - a process unit is content a project edits locally, and D0183 already moves it with a versioned identity and a three-way merge against per-file hashes, which a submodule cannot do. Filing them as one story would hide that the right mechanism is not the same for both.";
    }
    #DerivedFrom dependency from us034 to st024;

    part us035 : UserStory {
        :>> id = "57ea86fd-fb41-43a6-bc79-0cdb8ec30468";
        :>> title = "What is the best practice for shipping the engine and its portable content with a project";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "project author";
        :>> iWant = "a recommendation on the mechanism - submodule or otherwise - for shipping keel and its templates and skills with a project";
        :>> soThat = "the two architecture stories are decided against stated practice rather than against the first mechanism that came to mind";
        :>> implication = ImplicationKind::question;
        :>> triageNote = "question and not architecture: they asked what the practice is, they did not direct a mechanism - the submodule appears twice with a question mark attached. Recording it as an architecture story would convert my answer into their instruction, which is the exact substitution D0216 exists to prevent.";
    }
    #DerivedFrom dependency from us035 to st024;

    part st025 : Statement {
        :>> id = "c894a147-d242-45df-9020-0d092ecbfab3";
        :>> title = "Implement all including the schema change; keel as a full-service AI manager with ambient cross-project processes, downstream-updatable templates and skills, GitHub-issue intake and possible autonomous polling";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> text = "implement all, including schema change.\n\nI think I want to use keel as the full-service AI manager. I have templates, skills, processes, output-formats that I like to use, and I want to be updateable from downstream. i want an easy way for these use cases to be supported, including when they encounter a breaking change in keel. and in general I want all processes to be immediately accessible across projects. import/export doesn't quite serve that - it adds the extra step for me to manage all of that. maybe we build in a process to add GitHub issues to keel? and in keel proper, a polling git pull that can periodically get issues and act on them autonomously? that'd be cool imo (except when keel crashes?). but for the moment, keel is private iirc so others cant make tons of issues atm.\n\ngive me a critique of how best to proceed";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-28"; :>> channel = StatementChannel::chat;
    }

    part st026 : Statement {
        :>> id = "79769ee1-8d16-4b13-abb0-2efc348578b5";
        :>> title = "Processes transfer universally but transfer is not activation; library data syncs silently with no per-update permission; pushback on the autonomous daemon and on implement-all accepted";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> text = "I want *processes* to be universally transferred, but that doesn't mean activated. still - it is data that I want silently synced without needing per-update permission.\n\nagreed with your pushback do #5 and #6. continue";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-28"; :>> channel = StatementChannel::chat;
    }

    part us036 : UserStory {
        :>> id = "24ad566f-3a20-41a3-b360-00b66ce12063";
        :>> title = "Carry the declared-versus-armed control distinction in Rust Result rather than a declaration file";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the distinction between a control that is declared and a control that can actually fire expressed through Rust error handling, using Result";
        :>> soThat = "cannot-fire is a typed outcome the compiler tracks, not a claim a TOML file makes about the code";
        :>> implication = ImplicationKind::designChange;
        :>> triageNote = "designChange and not architecture: it changes how one existing mechanism represents a state, not the system decomposition. Not implementationChange, because it changes the contract of every control probe - control-arming.toml declares arming as data today, and moving it into Result makes the probe signature the source of truth, which is a contract change needing a Decision. RECORDED LATE AND ON PURPOSE: this story was refused on 2026-08-28 because the write path accepted 12 of the schema 15 ImplicationKind members (issue300). It was held rather than force-fitted into architecture, and is recorded here as the first use of the repaired vocabulary.";
    }
    #DerivedFrom dependency from us036 to st023;

    part us037 : UserStory {
        :>> id = "4dc4af80-0638-4482-a485-8fcc3607c228";
        :>> title = "keel is the full-service AI manager and holds the working material, not only the engine";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "keel to carry my templates, skills, processes and output-formats as first-class portable content";
        :>> soThat = "starting or resuming work anywhere brings the way I work with it, not just the engine that gates it";
        :>> implication = ImplicationKind::scopeConstraint;
        :>> triageNote = "scopeConstraint and not architecture: it widens what is IN keel scope, and the mechanism is a separate question they asked me rather than directed. It bears directly on the CLAUDE.md invariant that the deliverable domain vocabulary never enters the engine - templates and output-formats are deliverable-shaped, so the scope widening is safe only if keel moves and versions them as OPAQUE payloads it never interprets.";
    }
    #DerivedFrom dependency from us037 to st025;

    part us038 : UserStory {
        :>> id = "a5a542ec-30bb-4473-ac99-71b26f1c2937";
        :>> title = "Portable content survives a breaking change in keel";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "project author";
        :>> iWant = "templates, skills and processes to keep working when they meet a breaking change in keel";
        :>> soThat = "an engine upgrade does not silently strand the working material that depends on it";
        :>> implication = ImplicationKind::verifiedRequirement;
        :>> triageNote = "verifiedRequirement and not scopeConstraint: it states a contract that can be verified - install a unit published against a newer engine into an older project and the outcome must be a stated refusal, not a gate failure discovered later. Two pieces are missing today: a declared engine-compatibility range per unit, and a migrate path for the library. keel migrate brings a PROJECT to a binary vintage and nothing migrates a unit.";
    }
    #DerivedFrom dependency from us038 to st025;

    part us039 : UserStory {
        :>> id = "e7e2783a-46f8-40fa-99c4-16cd93443134";
        :>> title = "GitHub issues become a keel intake channel";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "a process that brings GitHub issues into keel";
        :>> soThat = "work raised where people raise it enters the model instead of staying outside it";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not architecture: the mechanism already exists as the decision channel (D0205/D0207) with github-actors.toml refusing an unmapped login, so this adds a CHANNEL to the intake process rather than a subsystem. Their own words framed it as a process. Their note that keel is private and so nobody can raise many issues yet is recorded as the reason this is cheap now and not urgent - the failure modes are free to find at zero volume.";
    }
    #DerivedFrom dependency from us039 to st025;

    part us040 : UserStory {
        :>> id = "0017c912-c474-4c49-81a0-5a7a00f6cf09";
        :>> title = "A polling git pull that periodically fetches issues and acts on them autonomously";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "keel to poll, pick up issues and act on them without me present";
        :>> soThat = "work moves while I am away";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture and not process: an unattended actor with authority is a structural change to the trust model, not a way of working. Every control in keel today fires synchronously with a human-initiated action, and a daemon has no session actor, so it needs a registered identity and a declared authority ceiling before it can write anything. They raised it with a question mark attached and named the crash risk themselves. DEFERRED by st026, which accepted the pushback: a scheduled READ-ONLY sync and report ships first, and autonomous action waits on a Decision naming the daemon actor, its ceiling, and PR-only landing.";
    }
    #DerivedFrom dependency from us040 to st025;

    part us041 : UserStory {
        :>> id = "bd7943a1-f1be-4da0-b702-7f0dfbb4b2bb";
        :>> title = "Processes transfer universally; transfer is not activation";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "every process transferred to every project, without that transfer activating it";
        :>> soThat = "everything I have is available everywhere while only what a project chose actually governs it";
        :>> implication = ImplicationKind::scopeConstraint;
        :>> triageNote = "scopeConstraint and not architecture: it draws the boundary the mechanism must respect rather than choosing the mechanism. It is the resolution of the conflict raised in the critique - ambient availability and per-project pinning are compatible exactly when availability and governance are separated, and this states that separation as their intent rather than my inference.";
    }
    #DerivedFrom dependency from us041 to st026;

    part us042 : UserStory {
        :>> id = "e8424b39-5ca1-4fb7-97f1-9cbf7cd0de5b";
        :>> title = "Library content syncs silently with no per-update permission";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "portable process and template data synced silently, without being asked to approve each update";
        :>> soThat = "keeping the working material current costs me nothing";
        :>> implication = ImplicationKind::verifiedRequirement;
        :>> triageNote = "verifiedRequirement and not scopeConstraint: silent-and-unprompted is a checkable behaviour of the sync path. It is safe only BECAUSE of the preceding story - inert library content can update silently, whereas silently updating an ACTIVATED unit would change a project gate without its tree changing, which is the property audit-history depends on. The two stories must be read together and are recorded from the same statement for that reason. Open and NOT settled by this: whether an already-installed unit also auto-updates. I read it as no.";
    }
    #DerivedFrom dependency from us042 to st026;

    part us043 : UserStory {
        :>> id = "5b0188e3-98ca-43fd-b1bd-df8d05230e5d";
        :>> title = "Authorization to implement the queued set including the frozen-core schema change";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the queued work implemented, the TestResult schema change included";
        :>> soThat = "the pinning story is checkable rather than asserted";
        :>> implication = ImplicationKind::attestation;
        :>> triageNote = "attestation and not priority: it is a sign-off that unlocks a frozen schema/core edit, which invariant 5 says requires explicit human acceptance. Their exact words were: implement all, including schema change. Recorded as its own story so the acceptance record for the schema Decision can cite a tracked utterance rather than a paraphrase of conversation.";
    }
    #DerivedFrom dependency from us043 to st025;

    part us044 : UserStory {
        :>> id = "95a133af-0a21-43bf-96b6-2975c55efded";
        :>> title = "Acceptance of the pushback on the autonomous daemon and on implementing everything at once";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the critique positions on the polling daemon and on implement-all adopted";
        :>> soThat = "the daemon waits behind a read-only stage and the work proceeds in waves";
        :>> implication = ImplicationKind::attestation;
        :>> triageNote = "attestation and not correction: they are not retracting anything, they are agreeing to two recommendations. Their exact words were: agreed with your pushback do #5 and #6. Item 5 was the polling daemon, item 6 was implement-all; recorded so the deferral of the daemon has a cited source rather than resting on my summary of it.";
    }
    #DerivedFrom dependency from us044 to st026;

    // ── Routing (intake step 5): what each triaged story became ──────────────────────────────────
    // #Implicates points story -> downstream item and is what `keel intake` counts as ROUTED;
    // #DerivedFrom points the other way, from the derived item back to the story, and is what D0216
    // requires so a Need cannot pass as unsourced. Both exist and they are not redundant: one says
    // this story produced that, the other says that item rests on this utterance.
    // Stories still unrouted here are waiting on downstream items that do not exist yet, which is a
    // stated debt rather than an omission - `keel intake` reports it either way.

    #Implicates dependency from us028 to d0246;

    #Implicates dependency from us029 to nIntakeBeforeInterpretation;
    #Implicates dependency from us029 to nTurnStartRecall;
    #Implicates dependency from us029 to nEnginePinAndEvolution;
    #Implicates dependency from us029 to ucRepoHoldsSeveralProjects;
    #Implicates dependency from us029 to nForeignTreeVerification;
    #Implicates dependency from us029 to nPassDisclosesItsSource;
    #Implicates dependency from us029 to nMonitoredNotGated;
    #Implicates dependency from us029 to nDecisionPromisesTracked;
    #Implicates dependency from us029 to nAuthoringIsLowFriction;
    #Implicates dependency from us029 to nParserIsTheTrustedBase;
    #Implicates dependency from us029 to nDeclaredControlDistinguishableFromArmed;
    #Implicates dependency from us029 to nDocSurfaceCannotOutliveTheChange;

    part st027 : Statement {
        :>> id = "d4633aa9-5173-461d-884e-0755817e54e6";
        :>> title = "Design the remaining waves as sprints with full architecture and the project's BDD/TDD verification rigor";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> text = "sounds great. let's design this out carefully in sprints, with full architecture (and our verification rigor - BDD/tdd etc.)";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-28"; :>> channel = StatementChannel::chat;
    }

    #Implicates dependency from us030 to d0247;

    // Six stories, two Needs. The consolidation is the human's own principle applied to their own
    // words (st023: a use case for typing A and another for typing B consolidate into the need for a
    // keyboard), and it is visible here as four stories pointing at ONE Need rather than four Needs.
    #Implicates dependency from us034 to nWorkingMaterialTravels;
    #Implicates dependency from us037 to nWorkingMaterialTravels;
    #Implicates dependency from us041 to nWorkingMaterialTravels;
    #Implicates dependency from us042 to nWorkingMaterialTravels;
    #Implicates dependency from us038 to nPortableContentSurvivesEngineChange;

    // us033 routes to an EXISTING Need rather than a new one: the outcome it asks for is already
    // stated, so it corroborates that Need instead of duplicating it.
    #Implicates dependency from us033 to nEnginePinAndEvolution;

    part st028 : Statement {
        :>> id = "cb0ce260-e2b0-4582-a910-c64f5e7f5b6d";
        :>> title = "Use the exec-summary HTML template and its skill in se-templates, not the Typst requirements-doc";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> text = "should be an html template now, along with a skill to use in the .Claude folder iirc";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-28"; :>> channel = StatementChannel::chat;
    }

    part st029 : Statement {
        :>> id = "a3d4a9ad-bc31-4df0-9b0a-59a96570b1bc";
        :>> title = "Fork 1 answered: the portable-content library is a git repository, not a machine-local store";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> text = "#1 yeah i think repo for sure.\n\nproceed";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-28"; :>> channel = StatementChannel::chat;
    }

    part us045 : UserStory {
        :>> id = "88c4fbd9-5a57-401a-97a1-474474852bd2";
        :>> title = "The portable-content library is a git repository";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the library that carries processes, skills, templates and output formats to be a git repository rather than a machine-local store";
        :>> soThat = "it syncs across machines, keeps its own history, and can be improved from any project that uses it";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture and not scopeConstraint: it CHOOSES a structure among the two I put to them, rather than bounding what is in scope. It is the answer to a question I asked, so the choice is theirs and the consequences below are mine to derive. Their hedge - i THINK repo - is preserved in the Statement verbatim rather than smoothed into certainty; the closing for sure is what makes it a decision rather than a lean.";
    }
    #DerivedFrom dependency from us045 to st029;

    #Implicates dependency from us045 to d0250;

    part st030 : Statement {
        :>> id = "8d7e4957-4e69-4129-b351-81f3401b028c";
        :>> title = "Forks 2 and 3 answered: all five recommended options selected, no notes";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> text = "[x] An exact version, e.g. engine = 0.3.0 - RECOMMENDED\n[x] Fetch the pinned release asset, verify its checksum, exec - RECOMMENDED\n[x] Writes and gates refuse; read-only views warn and proceed - RECOMMENDED\n[x] Extend the existing capability handshake: a unit declares what it NEEDS - RECOMMENDED\n[x] Roll back entirely and report per file what could not migrate - RECOMMENDED\nMy notes:";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-28"; :>> channel = StatementChannel::chat;
    }

    part us046 : UserStory {
        :>> id = "d299ee58-f599-4ccb-8c32-fce12ba15475";
        :>> title = "The engine pin names an exact version, is fetched and verified, and refuses on writes";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "project author";
        :>> iWant = "an exact pinned engine that the wrapper fetches and verifies, refusing writes and gates on a skew";
        :>> soThat = "the same tree produces the same verdict on every machine, and a new machine works on first clone";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture and not designChange: it fixes the DELIVERY structure of the engine and amends an accepted Decision (D0175 on the fetch), rather than reshaping an implementation. Three of their five selections belong to this one story because they are one mechanism - a pin that names nothing resolvable, or resolves and then never bites, is not a pin.";
    }
    #DerivedFrom dependency from us046 to st030;

    part us047 : UserStory {
        :>> id = "bce20c04-a347-4021-a3fa-af0272c2e8c5";
        :>> title = "A unit declares capabilities, and a failed migration rolls back";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> asA = "project author";
        :>> iWant = "unit compatibility expressed as declared capabilities, and a migration that cannot finish to leave the unit untouched";
        :>> soThat = "a unit refuses to install for a reason that names what is missing, and a half-migrated unit never exists";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "architecture and not verifiedRequirement: the requirements srUnitDeclaresEngineRange and srUnitMigrationPath already exist and are accepted; this chooses the MECHANISM that discharges them. Kept separate from the pin story because the two forks are independent - the library could carry content under either compatibility scheme.";
    }
    #DerivedFrom dependency from us047 to st030;

    #Implicates dependency from us046 to d0251;
    #Implicates dependency from us047 to d0252;
}
