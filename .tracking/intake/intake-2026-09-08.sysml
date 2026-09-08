// Intake for 2026-09-08 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260908 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st101 : Statement {
        :>> id = "b384e3f8-559b-4a49-94a6-c0a33de84b33";
        :>> title = "GH#67 [feature] TOML views cannot traverse a PROJECT-declared marker edge (only engine edge kinds are known)";
        :>> createdAt = "2026-09-08"; :>> createdBy = "githubRecorder";
        :>> text = "**Observed (keel 0.3.1).** A project declares its own marker edges in `.engine/schema/core/relationships.sysml` (`metadata def Causes; ... LeadsTo; Inverts; Mitigates; RealizedBy; Analyzes; Enforces; Senses; Explains; Satisfies` - each with the D0139(B) `doc` justification) and uses them 200+ times in its STPA model. A TOML view that tries to traverse them fails:\n\n```\nview error: view 'safety' references unknown edge kind 'causes' (known: allocate, capability, charteredby, contains, controls, covers, dependency, dependson, derivedfrom, dispositions, ..., verify, view)\n```\n\nand `edges = [''dependency'']` does NOT include marker-form dependency statements either (a `[select] type = ''UnsafeControlAction''` + `traverse dependency both closure` returned only the 22 UCAs).\n\n**Expected.** Either (a) a project-declared `metadata def` becomes a traversable edge kind by name (the declaration is type-checked already - D0133 - so the vocabulary is known to the engine), or (b) `dependency` covers every `#Marker dependency from A to B;` statement regardless of marker, with `[traverse.target]` available to filter. Without one of these a project's own edge vocabulary is authored, validated, guarded - and invisible to every declared view; the safety viewpoint here had to fall back to the SR -> Test `verify` slice.\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-07). Context: https://github.com/williamweatherholtz/obsidian-selfsync/blob/main/.engine/views/safety.view.toml\n";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/67";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st102 : Statement {
        :>> id = "10166395-761f-4589-b3f6-f71489f0dc7f";
        :>> title = "GH#66 [feature] Propagate suspicion from architectural elements to dependent STPA analysis artifacts (reassess-on-change)";
        :>> createdAt = "2026-09-08"; :>> createdBy = "githubRecorder";
        :>> text = "Downstream project decision (obsidian-selfsync D0017), carried here because it is a keel change.\n\n**Ask.** Extend the suspicion computation to propagate along the analysis->architecture linkage edges: when an architectural element (Controller, ControlAction, Feedback, Sensor, ProcessModel) is modified at a commit LATER than a dependent analysis element (UnsafeControlAction, LossScenario, SafetyRequirement) was last REASSESSED, mark that analysis element suspect (surfaced in `keel orient`, the safety view and `keel show suspect` with a reason). Follow `#Analyzes` / `#RealizedBy` / `#Senses` directly and `#Explains` / `#Mitigates` / `#Inverts` / `#Enforces` transitively, to fixpoint. Hazards and Losses are NOT flagged by an architecture change (only by a loss/stakeholder/boundary change).\n\n**Needs.** A per-element `reassessedAtCommit` stamp on the analysis elements in EngineSafety (schema addition, human sign-off), the propagation to fixpoint, and the surfacing. Verified by a keel test that edits a controller and asserts its UCAs/requirements go suspect while the hazards do not.\n\n**Why.** Without it an STPA silently diverges from the code it abstracts: obsidian-selfsync has a 25-SR STPA whose control structure is a first-class model with 100+ marker edges into the architecture, and today nothing tells the author that a controller changed under an analysed UCA. The project keeps `stpaSuspicionPropagation` open on this.\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-07). Full report with context: https://github.com/williamweatherholtz/obsidian-selfsync/blob/main/docs/upstream/2026-09-07-keel-toolkit-defect-reports.md";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/66";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st103 : Statement {
        :>> id = "508dab29-3ea7-444d-a54a-1c64742445bd";
        :>> title = "GH#65 [docs] Stale engine-shipped docs after D0222 / per-actor issue files";
        :>> createdAt = "2026-09-08"; :>> createdBy = "githubRecorder";
        :>> text = "`.engine/docs/usage.md` still says ''Register an AI skill | Add an AISkill/Agent to skills-registry.sysml''\n(now per-skill `registry.sysml`, D0222) and ''Track an issue | Author a part in .tracking/issues.sysml''\n(now `keel record issue` -> `.tracking/issues-<actor>.sysml`). Engine-owned, so not fixed downstream.\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-07). Full report with context: https://github.com/williamweatherholtz/obsidian-selfsync/blob/main/docs/upstream/2026-09-07-keel-toolkit-defect-reports.md";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/65";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st104 : Statement {
        :>> id = "3f8e90c0-af91-4cf1-9935-79e383ff97d3";
        :>> title = "GH#64 [bug] `sprint-closure` exempts an unstamped task by the TEXT of its action-line comment";
        :>> createdAt = "2026-09-08"; :>> createdBy = "githubRecorder";
        :>> text = "**Observed.** `action dodBackfillStamp;` (open, unstamped, in the OPEN section of the def) failed\n`sprint-closure` (''work has moved on to a later sprint''). Changing ONLY the trailing comment to\n`action dodBackfillStamp;  // resolver for issueHistoricalDoDsUnstamped` made the guard PASS; moving the\ntask elsewhere in the def did not. So the exemption is decided by comment text (presumably ''resolver for'' /\n''OPEN''), which a `.sysml` comment is not a typed fact. A task that is genuinely open should be exempt by a\ntyped signal (e.g. it is the `#Resolves` source of an open Issue, or it sits in the highest-numbered\nsprint as D0260 states) — not by whether the author wrote a particular phrase after `//`.\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-07). Full report with context: https://github.com/williamweatherholtz/obsidian-selfsync/blob/main/docs/upstream/2026-09-07-keel-toolkit-defect-reports.md";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/64";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st105 : Statement {
        :>> id = "a3a5b968-3e9f-4413-a034-c7ea61e746e5";
        :>> title = "GH#63 [bug] `keel add-task --def X` appends the task OUTSIDE the action def when the def is not the last block in the package";
        :>> createdAt = "2026-09-08"; :>> createdBy = "githubRecorder";
        :>> text = "**Observed.** `.tracking/delivery/delivery.sysml` is `package ProjectDelivery { action def SelfSyncDelivery {...}  <#Capability stories, edges...> }`.\n`keel add-task --file ... --def SelfSyncDelivery --task coversEdgesHoisted --method inspect --dod-from F` (and a\nsecond call) appended `action coversEdgesHoisted; verification ...DoD ...` immediately before the PACKAGE's closing\nbrace — after package-level `#Covers` edges and `#Capability` stories — i.e. outside the def it was told to\nextend. `keel validate` accepted it and `keel guard` was green (the task still resolved its issue), so nothing\nsignalled the mis-placement; `sprint-closure` then judged the open task by its position after every shipped\nsprint (''work has moved on to a later sprint''). The stock template happens to end the file with the def, which\nis presumably why the anchor works there.\n\n**Expected.** Insert before the named def's own closing brace (or refuse with ''def close not found''), never at\nthe package tail. Also: `keel record issue` numbers a new record as `issueNNN` by counting existing `issueNNN`\nnames in the per-actor file, so after a rename (issue001 -> a semantic name) the next record is `issue001` again;\nminting from a counter or the id would avoid the collision-by-rename.\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-07). Full report with context: https://github.com/williamweatherholtz/obsidian-selfsync/blob/main/docs/upstream/2026-09-07-keel-toolkit-defect-reports.md";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/63";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st106 : Statement {
        :>> id = "5409af49-ec73-4def-9f51-fc5fdbf484f6";
        :>> title = "GH#62 [bug] `open-issues` treats a finding with a `#Dispositions` edge as closed even when the disposition Test has NO TestResult";
        :>> createdAt = "2026-09-08"; :>> createdBy = "githubRecorder";
        :>> text = "**Observed.** A `method=confirmation` Test carrying `disposition = DispositionKind::acceptRisk`,\ndeliberately WITHOUT a TestResult (the owner had not yet attested), linked to a finding with\n`#Dispositions dependency from <test> to <issue>;`. `keel show open-issues` then dropped the issue\nfrom BOTH the open and resolved lists; `keel show dispositions` did not list it as a finding.\nDeleting the single edge line made the issue reappear as open; restoring it hid it again.\n\n**Expected.** Per D0092 a disposition is a typed RECORDED JUDGMENT — the judgment is the TestResult\n(who/when/against what), not the edge. An unattested disposition should leave the finding OPEN,\nexactly as an unstamped DoD leaves a task open. As it stands, an item awaiting the owner's word is\ninvisible to the owner.\n\n**Local mitigation.** The edge is withheld until the attestation is recorded (comment in\n`.tracking/verification/acceptance.sysml` carries the exact edge to author at that moment); issue\n`issueUnattestedDispositionClosesFinding`, resolver `keelDispositionLensReport`.\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-07). Full report with context: https://github.com/williamweatherholtz/obsidian-selfsync/blob/main/docs/upstream/2026-09-07-keel-toolkit-defect-reports.md";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/62";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st107 : Statement {
        :>> id = "f97e872b-a0a0-4c27-a82e-f764c7f96687";
        :>> title = "GH#61 [bug] `keel migrate --dry-run` reports 0 blockers for a change that deletes a project's schema additions";
        :>> createdAt = "2026-09-08"; :>> createdBy = "githubRecorder";
        :>> text = "**Observed.** On a July-2026-vintage tree with 25 `metadata def` declarations in\n`.engine/schema/core/relationships.sysml` (18 engine + 7 project markers added by an accepted project\nDecision D0045), `keel migrate . --dry-run` reported ''481 files / 481 edits / 0 blockers''. The live\n`keel migrate .` overwrote the file with the stock copy: 25 -> 18 declarations, and the project's\ninstance data then failed `keel validate` (marker-vocabulary 0 -> 228 violations, attribute-vocabulary\n0 -> 1216). Rolled back at the time (project D0048), later redone under a committed transform that\nre-adds the project's declarations by name (project D0050).\n\n**Expected.** A dry-run that would overwrite an engine-shipped file the project has EXTENDED (its\ncurrent content is not byte-equal to any stock vintage) should list that file as a blocker — or at\nleast as ''project-modified, will be overwritten'' — rather than 0 blockers. The same applies to the\nshared skills registry (8 project-only AISkill registrations were dropped) and to project-only\nprocess files whose shape the new schema rejects (`part X : Process` vs `action def Process`,\n`:>> order` no longer declared).\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-07). Full report with context: https://github.com/williamweatherholtz/obsidian-selfsync/blob/main/docs/upstream/2026-09-07-keel-toolkit-defect-reports.md";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/61";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st108 : Statement {
        :>> id = "ad100f6f-6ed3-4838-b8c2-b9c3db272cc7";
        :>> title = "GH#60 [bug] keel-parser skips a `dependency` statement nested inside an `action def` body — silently";
        :>> createdAt = "2026-09-08"; :>> createdBy = "githubRecorder";
        :>> text = "**Observed.** `.tracking/delivery/delivery.sysml` had 74 `#Covers dependency from <sittingReview> to <task>;`\nstatements at indent 8, inside `action def SelfSyncDelivery { ... }`. `keel show marker-census` reported\n`Covers` edges = 0 (proseMentions 86); `keel show sitting-coverage` reported 0 covered sprints; the\nparser-coverage guard reported 81 skipped statements = 7 successions + 74 dependency. Package-level\n`#Verify dependency` statements in the same project (114) were all counted.\n\n**Reproduction.** Four probe files run through `keel check`: (a) package-level marker dependency,\n(b) two marker dependencies on one line, (c) a marker dependency with a trailing `//` comment,\n(d) a dependency nested inside a `part`/`action` body. ALL four report `1 file(s) checked clean`.\nOnly (d) is skipped by the census. Hoisting the 74 statements to package level: Covers edges 74,\nskipped count 7, sitting-coverage counts covering reviews.\n\n**Why it matters.** The sitting review is the single human gate in the sprint discipline (D0049); the\nlens that reports its coverage read a false zero for the life of the record, and `keel check` gave no\nsignal. A construct the parser does not read should be a parse error (or the nested form should be\nparsed), never a clean check. The parser-coverage ratchet catches the COUNT but not the location —\nlisting the skipped statements' file:line in the warning would have made this a two-minute fix.\n\n**Local mitigation.** Edges hoisted (obsidian-selfsync commit 4a991a54, `coversEdgesHoisted`);\nbaseline kept at the stock 7; issue `issueNestedCoversEdgesSkipped` (#ProcessDefect) records it.\n\n---\nFiled from obsidian-selfsync (keel 0.3.1, 2026-09-07). Full report with context: https://github.com/williamweatherholtz/obsidian-selfsync/blob/main/docs/upstream/2026-09-07-keel-toolkit-defect-reports.md";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/60";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st109 : Statement {
        :>> id = "e1b9d43f-b791-4fec-b89a-65e8520891b8";
        :>> title = "the brief is still far too verbose";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> text = "the decision brief is still far too verbose.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::chat;
    }

    part st110 : Statement {
        :>> id = "4695f3fd-33b3-4ec2-9c74-ec352a362c17";
        :>> title = "make the brief use caveman";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> text = "make it use caveman?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::chat;
    }

    part us098 : UserStory {
        :>> id = "1ee95b35-83eb-4827-94ce-d735d2d28e26";
        :>> title = "the brief reads in far fewer words";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> asA = "decider reading the published brief";
        :>> iWant = "each page to state its ask and argument within the budgets the brief contract already declares";
        :>> soThat = "I can answer it in one read";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "the contract table sets title 18 / ask 70 / provenance 60 words; check_brief (scripts/check_templates.py:212) holds none of them, and the page just published passed clean at 1334 words with a 26-word title";
    }
    #DerivedFrom dependency from us098 to st109;

    part us099 : UserStory {
        :>> id = "4fcce93c-5b27-4912-a43e-fe52c3804317";
        :>> title = "the brief is written in caveman register";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> asA = "decider reading the published brief";
        :>> iWant = "the brief written terse: no filler, no hedging, short synonyms, one idea per sentence, with a ceiling on the whole page";
        :>> soThat = "the page costs one read";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "a register and a whole-page ceiling are new contract clauses, not a checker defect; they change what the surfacing process publishes, so they go through a Decision";
    }
    #DerivedFrom dependency from us099 to st110;

    part st111 : Statement {
        :>> id = "75610dae-5e28-425f-9651-311a7f1a039c";
        :>> title = "Objective tests need no agent panel; the human validates the subjective; process hooks, checklists and rubrics for small agents";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> text = "for the process agent one - looks like some of these items are just poorly measured, and that the measurement itself should be critiqued.  We don't really need a panel of agents to perform the same test, especially if that test is objective.  Where it's not objective, I should probably be included in validating the result at some point, if it's genuinely subjective.  Otherwise, I don't see a necessity to constantly be running agents to double check a subjective assertion (e.g. interpreting my text).  Maybe we need a more determinstic way to track processes?  Maybe we provide githook-like hooks into processes.  Do they fire before the prompt (i don't think this is possible)?  before work is done (e.g. sprint planning)?  after as a post-processing (e.g. sprint retro)?  there's got to be a good way to make a simple checklist for these processes and to make simple rubrics so lower-model agents can run them as necessary (e.g. user said ''X'' - is the user asking for a view?  if so, quote the text indicating so.  Is the user asking to record? if so, quote the text indicating so, etc.).";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-08"; :>> channel = StatementChannel::chat;
    }

    part us100 : UserStory {
        :>> id = "dd8a539e-b8c4-40a7-83e0-5a0084687bb6";
        :>> title = "The measurement is critiqued before the measured";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "a check that is repeatedly disagreeing with itself to be treated as a poorly designed measurement and critiqued as such";
        :>> soThat = "the project does not spend on rerunning a bad instrument to average its noise";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "their words: some of these items are just poorly measured, and that the measurement itself should be critiqued";
    }
    #DerivedFrom dependency from us100 to st111;

    part us101 : UserStory {
        :>> id = "8c9c8405-b7ba-47a0-9736-cd6b4e4e060f";
        :>> title = "Objective tests run once; the human samples the subjective";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "an objective test run once and deterministically, never by a panel of agents, and a genuinely subjective verdict brought to me for validation at some point instead of re-run by more agents";
        :>> soThat = "agent spend goes to work, not to double-checking interpretations of my text";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "their words: We don't really need a panel of agents to perform the same test, especially if that test is objective. Where it's not objective, I should probably be included in validating the result";
    }
    #DerivedFrom dependency from us101 to st111;

    part us102 : UserStory {
        :>> id = "df430c56-06f2-4921-8180-17c1b1f2bfd1";
        :>> title = "Processes fire hooks like git does";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "processes tracked deterministically through hook points - before work such as sprint planning, and after as post-processing such as retro - the way git hooks fire around a commit";
        :>> soThat = "a process runs because a hook fired, not because an agent remembered";
        :>> implication = ImplicationKind::architecture;
        :>> triageNote = "their words: Maybe we provide githook-like hooks into processes. before work is done (e.g. sprint planning)? after as a post-processing (e.g. sprint retro)?";
    }
    #DerivedFrom dependency from us102 to st111;

    part us103 : UserStory {
        :>> id = "c50d4154-8319-41b5-94fd-be3bd622169d";
        :>> title = "Checklists and rubrics a small model can run, quoting its evidence";
        :>> createdAt = "2026-09-08"; :>> createdBy = "claudeOpus5";
        :>> asA = "project owner";
        :>> iWant = "each process reduced to a simple checklist and a rubric that a lower-capability agent can run, where every verdict quotes the text that decided it - is the user asking for a view? quote it; asking to record? quote it";
        :>> soThat = "process checks are cheap, repeatable and auditable from the quote rather than from an agent's judgment";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "their words: make simple rubrics so lower-model agents can run them as necessary (e.g. user said X - is the user asking for a view? if so, quote the text indicating so)";
    }
    #DerivedFrom dependency from us103 to st111;
}
