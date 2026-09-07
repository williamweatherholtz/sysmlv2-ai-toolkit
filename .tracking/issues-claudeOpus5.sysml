// Per-actor issues records for claudeOpus5 (issue210/srDcPerActorWriteTargets): new records
// route here so concurrent contributors never conflict at a shared append anchor.
package IssuesClaudeOpus5 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part issue292 : Issue {
        :>> id = "cea7937f-97e0-4482-8a06-4cbfddb8aa79";
        :>> title = "the shipped reference decision copy is parsed as instance data, importing 235 foreign attestations and 236 unresolvable evidence SHAs";
        :>> createdAt = "2026-08-27";
        :>> createdBy = "claudeOpus5";
        :>> description = "keel init remaps the engine's 236 decisions to .engine/reference/decisions/ and Sprint 104's record states they are NOT scanned as instance - the rust views scan .engine/decisions. That was never true: collect_sysml (lib.rs:75) has no exclusions, so the reference copy has always been parsed and registered into the downstream project's model. Sprint 104 implemented the PATH remap and not the scanning boundary it declared. Measured in a fresh init fixture: the downstream model contains 235 TestResults judged by wweatherholtz - keel's human, not a stakeholder of that project - and 236 judgedAgainst SHAs that do not resolve in that repository (sampled e146621: 'Not a valid object name'). So every downstream project's verification surface is inflated with another project's human attestations, and its evidence chain contains 236 references that D0129 requires to resolve and which cannot. D0237 fixed the symptom this surfaced (the package collision on the first recorded decision) and deliberately did not fix this. Note the constraint on any fix: .engine/rules/rules.sysml carries 10 real #JustifiedBy edges to dNNNN targets, so the reference copy cannot simply be dropped from the walk - the edges would dangle. Aggravating, and the reason this shipped: both gates covering init (sprint 104's live check and sprint 115's init smoke test) assert init-then-validate and never record a fact, which is the one state in which every defect in this class is invisible.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from scopeReferenceCopyOutOfTheInstanceModel to issue292;

    part issue293 : Issue {
        :>> id = "42488fde-9b75-44e7-b5e4-c83032b5fed4";
        :>> title = "keel activate/deactivate rewrites activation.toml and loses charteredBy plus non-switchable processes";
        :>> createdAt = "2026-08-27";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found because the KEYSTONE LOCK refused a commit, which is the control working exactly as designed: I ran `keel deactivate knowledge-graph-memory` and `keel activate knowledge-graph-memory` to verify a kill switch, believed I had restored the file, and the pre-commit gate reported a locked-file change with no co-committed Decision. The diff showed two losses from one round trip. (1) `charteredBy = 'd0226'` was DELETED along with its explanatory comment - that is the D0225 charter reference, and the comment it removed says a set without a charter is a collection of switches nobody can account for. (2) `decision-authoring` was dropped from `active`, because it is reported `[always]` (asserts no guard, nothing to switch off) and the writer emits only switchable processes. The second is a semantic change, not untidiness: an ABSENT section means everything is active (D0138/D0164), but a PRESENT list naming 10 of 11 processes means the 11th is INACTIVE - so silently omitting a process from a list that exists can deactivate it. Both losses are in a file under the keystone lock, so any legitimate use of `keel activate` now either loses these facts or requires a signed Decision to re-add them, which is a write path fighting its own control. Nothing detected either loss; the lock detected the FILE changing.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from activateDoesNotLoseFacts to issue293;

    part issue294 : Issue {
        :>> id = "08e84944-fdf2-467b-8575-cd7456e119dc";
        :>> title = "four-letter domain words never seed recall: they are dropped for length before frequency is consulted";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "A prompt token shorter than five characters is dropped for LENGTH before its corpus frequency is ever consulted (prompt_tokens), so four-letter DOMAIN words never seed recall: gate, hook, land, push, view. Five-letter hooks seeds and four-letter hook does not, which is arbitrary from the reader side and cannot be predicted by anyone using it. The five-character floor exists because a four-character floor admits been, does, made, that and this, and the English function-word list was sized against the longer floor. Found because a unit test asserted that gate would be dropped as TOO COMMON and it was instead dropped as TOO SHORT - the test premise was wrong, not the code, and nothing else would have surfaced it. Severity Medium rather than High: the identifier path and longer synonyms still work, so this narrows recall rather than breaking it, and the eight-case comparison gives a way to measure any fix instead of guessing.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from shortDomainWordsCanSeed to issue294;

    part issue295 : Issue {
        :>> id = "a97e2219-d1db-4a5e-b771-cc25bcecdd33";
        :>> title = "recall never indexes body text, and its ranker is alphabetical in 43 of 50 cases";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by an adversarial panel (IR-fundamentals lens), confirmed against the code. seeds_for matches an item's name, its title and an Alias term - nothing else - so the five body fields the same module reads for DISPLAY (decision, description, rationale, actionText, consequences) are invisible to RETRIEVAL. Demonstrated: a prompt about provenance and defaulted dates contributed ZERO seeds for `provenance`, the most informative word in it, because it appears in dozens of bodies and in no name or title. Second defect, structural: agreement is the primary sort key but reaches max 1 in 43 of 50 bench cases, so the key is constant and ranking degenerates to (hops, name) - alphabetical - which is precisely the failure the agreement comment says it was introduced to fix. MAX_PROMPT_WORDS caps votes at three and the rarity ceiling removes the mid-frequency words most likely to co-occur, so the vote can rarely happen. BM25 measured on the SAME fields scores 37/50 (74%) hit@20 versus the shipped 24/50 (48%); the field ceiling is 44/50. SQLite FTS5 provides bm25() as its default ranking function, so rejecting BM25 while proposing SQLite was incoherent. The lexical layer is not exhausted, it is barely attempted.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from recallIndexesBodiesAndRanksProperly to issue295;

    part issue296 : Issue {
        :>> id = "a5088939-bd58-4cba-8321-462d35171f70";
        :>> title = "the recall benchmark's control was a strawman, precision was never measured, and the 14-point gap is not significant";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found independently by two panel lenses and confirmed by re-running the harness. The control matches name and title only while queries are drawn from BODY_FIELDS, so it was structurally denied the field the queries came from; with bodies included the same rarest-word control scores 50/50 and real git grep 14/15 - both themselves leakage, since queries are verbatim body words. The harness cannot discriminate in either direction, so the 48%-vs-34% result carries no information and should not have been used to ship anything. Worse for a PUSH feature: precision was never measured at all, and sampling 17 realistic prompts put it near 9% - one of eleven pushes topical. The 2x2 is 13 both / 11 KG-only / 4 grep-only / 22 neither, giving McNemar exact p = 0.119 and a net advantage of 7 cases rather than 24. And the harness carries a dead statistic (`both`, always false, never printed), which is evidence it went unreviewed while being the sole evidence for a proposal to delete the subsystem.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from recallBenchmarkCanDiscriminate to issue296;

    part issue297 : Issue {
        :>> id = "99351a32-1e98-4479-8cdb-fb0e676ad5b9";
        :>> title = "three surfaces still assert a confidence gate that has not existed since it was removed, including a passing DoD";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "confident() discards all four parameters and returns seed_count > 0, so the only silenced prompts are those with no informative token - already rejected upstream by prompt_tokens. The gate is gone; its prose is not. The main.rs doc comment still lists kill switch 3 as 'a LOW-confidence prompt is not injected at all'; the PASSING kgPromptPathInjects DoD still reads 'Injection is gated on a CONFIDENCE verdict - silence beats noise'; and the printed HIGH/LOW label is unreachable at LOW whenever any row is shown, so it carries no information while reading as though it does - the panel observed 'confidence HIGH' stamped on a payload 14 of whose 17 rows were off-topic. This is the fourth instance this session of prose asserting a property the code lacks, and the first where I wrote both the prose and the code within two commits. Same class, also in my own comments: the ceiling described as '~60' when it computes to 68, `gate` counted as 5,736 on one line and 5,806 on another, and the ceiling credited with dropping `gate` when the length rule drops it first.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from retireTheConfidenceProse to issue297;

    part issue298 : Issue {
        :>> id = "eaf25067-5cbe-4026-806d-7199dad74d6c";
        :>> title = "D0243 changed an accepted process's meaning with no marker and without editing the process definition";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the source-fidelity panel lens and confirmed: D0243 carries ZERO #ProspectiveChange/#SafetyChange markers where its sibling D0242 carries two, and .engine/processes/knowledge-graph-memory.sysml mentions neither Decision. Yet D0243 declared '.knowledge/ an optional refinement rather than a prerequisite' and rejected 'require a curated subject vocabulary, AS THE SOURCE PROCESS'S STEPS 1-2 IMPLY' - i.e. it changed what two accepted, human-owned steps mean. The keystone lock did not fire because the guarded artifact was never edited: the amendment was made AROUND the control rather than through it. D0161 had already written the rule broken here - 'A design may amend a process; it may not quietly disagree with one.' The substance of D0243 is sound and measured; the governance is not, and a guard that only notices edits cannot notice a disagreement.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from processAmendmentsReachTheDefinition to issue298;

    part issue299 : Issue {
        :>> id = "4ed10cb4-e5ab-45ce-8cdd-3f4931ddb75a";
        :>> title = "identifier seeding fails on three of six natural spellings, and it is the one path measured to work";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "Confirmed by the panel and reconfirmed directly: `d0242`, `issue294` and `what does D0242 say` seed correctly, while `issue 294`, `D-0242` and `decision 242` push nothing at all - is_identifier_token requires the digit run to abut the prefix with no separator. Both the cost-benefit and IR lenses independently named identifier lookup as the feature's genuine value (`keel why d0133` returns the decision, its acceptance Test, its TestResult and file paths - precise and provenance-carrying, which grep does not give), and the cost-benefit lens recommends making it the ONLY injection trigger. Under that recommendation this brittleness becomes the system's whole failure rate rather than a corner case.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from identifierMatchingAcceptsNaturalSpellings to issue299;

    part issue300 : Issue {
        :>> id = "7eabeb2d-2325-4276-94e7-d1acffa3961d";
        :>> title = "The intake write path accepts 12 of the schema's 15 ImplicationKind members";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-28 while triaging st023. schema/core/intake.sysml:39-59 declares 15 ImplicationKind members. keel-cli/src/intake_write.rs:57 hardcodes a 12-element allowlist that omits verifiedRequirement, designChange and implementationChange - the three members the schema comments call out as deliberately RENAMED or added, and which .engine/processes/intake.sysml:36 and both copies of the intake SKILL.md instruct the triager to use. A real story of mine was refused today for using designChange, which is the correct member for it. THE HARM IS THE ONE THE PROCESS EXISTS TO PREVENT: intake.sysml says a kind the vocabulary cannot express is a CHANGE to the vocabulary and never a forced fit, so a triager following the skill hits a refusal and the path of least resistance is to force-fit into architecture or bug - a wrong triage, which the skill states is worse than an absent one because it routes real direction to the wrong place and looks handled. THIS IS THE issue119/issue120 CLASS, EXACTLY. That pair was resolved by building schema.rs so the binary reads its vocabulary from the schema instead of restating it: KNOWN_EDGES mirrored EdgeKind, ENGINE_MARKERS mirrored the metadata defs, RISK_ORDER mirrored the severity enum. IMPLICATIONS is a fourth constant of the same kind that was written after the fix and did not use it. FIX: read ImplicationKind from schema/core/intake.sysml through schema.rs, as the other three vocabularies already are, so the allowlist cannot drift again; a downstream project that extends the enum then works without a binary change. CONTROL: the schema-vocabulary guard should assert that every vocabulary the binary validates against is schema-sourced, so a fifth hardcoded list is a red gate rather than a discovery.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcIntakeHasAWritePath to issue300;

    part issue301 : Issue {
        :>> id = "1d8bfa7a-ed75-4621-97c0-26d9f901c859";
        :>> title = "The unit manifest carries machine-absolute path keys, and publishing it to a shared library would break every other machine";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-28 while committing the intake-vocabulary fix. .engine/contracts/installed-units.toml records per-file content hashes keyed by path, and four keys under the decision-channel unit are MACHINE-ABSOLUTE: file.C:__SL__Users__SL__WilliamWeatherholtz__SL__claude_code__SL__sysmlv2-ai-toolkit__SL__.github__SL__scripts__SL__record_decision.sh and three siblings. Every other key in the file is repository-relative (file.processes__SL__intake.sysml). WHY THIS IS NOW URGENT RATHER THAN LATENT: D0250 makes the library a git REPOSITORY that other machines clone. A committed manifest naming this machine's home directory was previously an ugliness confined to one tree; published to a library that a second machine pulls, the keys resolve to nothing there, so the three-way merge that D0183 relies on compares against files it cannot find. The defect is in the one file whose entire purpose is portability. FIX: keys are repository-relative always; a unit-extras entry outside the project root is either rejected at export or recorded relative to a declared anchor, never absolutised. VERIFY: a unit exported on one machine and imported on another reconciles by hash with no path rewriting; a manifest containing an absolute key fails a check rather than being silently tolerated.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from storyLibraryManifestIntegrity to issue301;

    part issue302 : Issue {
        :>> id = "b54cc34a-5c24-4ed6-8df5-2b329b9582e7";
        :>> title = "Unit versions bump with no content change, so every update decision reads as noise";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-28 while committing. The intake unit in .engine/contracts/installed-units.toml went version 42 -> 43 during a session in which none of its three recorded file hashes changed, and the file churns in essentially every recent commit. A version that moves without content moving is a FALSE UPDATE SIGNAL. WHY IT MATTERS MORE UNDER D0250: import --update decides on version, and srActivatedUnitVersionIsPinned makes a project's recorded unit version the thing that does not move until an explicit update. Once the library is a repository other machines pull, a project deciding whether to take an update reads a number that changes for reasons unrelated to content - so the honest answer to should I update becomes unavailable exactly where it starts to matter. This also interacts with the no-op churn: a manifest rewritten every commit makes the diff useless as a review surface for what actually moved. FIX: the version advances only when at least one recorded file hash changes; a re-stamp that finds identical hashes leaves the version alone. VERIFY: re-running the stamping path twice with no edits leaves installed-units.toml byte-identical; a single file edit advances exactly that unit's version and no other.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from storyLibraryManifestIntegrity to issue302;

    part issue303 : Issue {
        :>> id = "5c15d85a-35ca-41dc-a133-f5f9e99ffba7";
        :>> title = "Eight of nine controls are classified unprobeable by a VIEW's constraints, and read as unverifiable";
        :>> createdAt = "2026-08-29";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-29 while answering a demand for REAL assurance. keel controls reports 9 declared controls: 1 ARMED, 8 not-machine-checkable, with uncoveredHazards [] and hazardsWithADisarmedControl [] - a clean bill of health resting on ONE probe. Each reason in control-arming.toml is individually careful, and the AGGREGATE is the issue240 failure at a higher level: truthful on the question asked, misleading on the question a reader has. THE SHARED ERROR: not-machine-checkable BY A READ-ONLY VIEW was generalised into not checkable at all. keel controls must not spawn processes, write files or make network calls, so it genuinely cannot establish most of these. A TEST may do all three. The arming question was answered against the constraints of the surface that happened to ask it, not against whether the property is verifiable. PROVEN ON THE FIRST ONE TRIED: ctlWriteLock was classified nothing to probe on the reasoning that no configuration can disarm it. That rules out being DISARMED and says nothing about whether the lock SERIALISES - and issue184/issue185 record four concurrent writers landing two issues with all four exiting 0, a day on which the same reasoning was equally true. A 30-line integration test with six real concurrent processes now proves it: all six land, all six exit 0, six distinct ids. The control is genuinely armed, and now demonstrably so rather than assumed. REMAINING, each needing its own probe rather than a reclassification: ctlPrePushBehind (a two-clone fixture can attempt a push - audit-history already built one), ctlReverify (run it against a fixture), ctlPreWriteTiers (invoke the hook, assert it denies), ctlFireLedger, ctlAuditHistory, ctlBranchProtection, ctlReleaseGateJob. FIX: a third arming state - proven-by-test, distinct from both checkable-by-view and not-checkable - plus a probe per control that admits one. The control-arming contract gains provenBy naming the test, so a reader can re-derive rather than trust the reason text.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    // RE-POINTED 2026-08-29 (D0255). It pointed at sprint 485, whose DoD said it resolved issue303
    // 'in part' - and resolution is COMPUTED from resolver completion, which has no partial state, so
    // a passing DoD closed an issue whose own closeOut gate said it stays open. The prose asserted a
    // nuance the computation cannot see. The resolver is now the OPEN task that actually finishes it.
    #Resolves dependency from dcProveTheRemainingControls to issue303;

    part issue304 : Issue {
        :>> id = "a3f95a97-c977-483f-b288-fda52817697f";
        :>> title = "The control map has never been reconciled against the enforcement the binary implements";
        :>> createdAt = "2026-08-29";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-29 (D0254 clause C). The control map declared NINE controls. The launcher's dirty-tree refusal - implemented in launcher::prepare, load-bearing for every Need about bounded process launching, and now proven by test - was not among them; a grep for launch, dirty or singleWriter across .tracking/architecture/control-map.sysml returned nothing. It was found by asking about one mechanism, not by any check. THE CLASS, distinct from the two already named: declared-but-never-fired (D0217) is visible to a reader who looks, and declared-and-unprobeable (D0253) is visible as a classification, but IMPLEMENTED-BUT-UNDECLARED is visible to nobody. keel controls computes the hazard/control diff over DECLARED controls and DECLARED hazards, so an undeclared control cannot appear as a gap, and neither can a hazard only that control covers. Worse, the measure improves as the map gets less complete: fewer declared controls with no gaps reads better than more declared controls with gaps. That is the issue088/issue240 shape again - truthful about the question asked, misleading about the question a reader has. NOT ESTIMATED HERE, deliberately: the map has ten entries and the binary has 53 guards, six hook events, a write lock, a launcher, a push gate and a release gate. Putting a guessed number of missing controls in the record would be the habit this whole line of work has been correcting. FIX: reconcile the map against the binary's actual enforcement surface, one mechanism at a time, and make the reconciliation itself a check so the map cannot silently fall behind the code again - the doc-sync shape applied to controls rather than prose.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    // RE-POINTED 2026-08-29 (D0255), same cause as issue303 above: sprint 486 declared ONE control
    // and said 'in part'. Sprint 487 reconciles the map against control-events.toml and lands the
    // standing check, which is what actually resolves it.
    #Resolves dependency from storyControlMapReconciled to issue304;

    part issue305 : Issue {
        :>> id = "86fb8720-b9d7-4720-ab17-e94e7d6fd7ed";
        :>> title = "A DoD saying it resolves an issue IN PART closes it outright, because resolution has no partial state";
        :>> createdAt = "2026-08-29";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-29 by reading back the model after two sprints I authored myself. Resolution is COMPUTED from a resolver's completion (D0077): a #Resolves edge plus a passing resolver DoD closes the issue. There is no partial state. I wrote 'RESOLVES issue303 in part' and 'RESOLVES issue304 in part' into two sprint DoDs, and both issues computed as CLOSED the moment those DoDs passed - while each sprint's own closeOut gate said in plain words that the issue STAYS OPEN. Two surfaces of the same model, one computed and one prose, disagreeing, with the computed one winning silently and the prose one being the true statement. THE SHAPE, and it is the recurring one: prose asserting a nuance the computation cannot see. The engine's own rule is that resolution is computed and never a prose RESOLVED note - this is the mirror image, a prose NOT-YET-RESOLVED note that the computation cannot honour. It is also self-serving in the dangerous direction: the issue count went DOWN while the work did not get done, and nothing flagged it. FIX, two parts. (1) An issue's resolver must be an item whose completion actually resolves it; 'in part' is not expressible, so the honest options are to SPLIT the issue or to point the edge at the item that finishes it - both done here for issue303 and issue304. (2) A CONTROL, because this will recur every time a large issue is chipped at: flag a DoD whose text claims partial resolution ('resolves ... in part', 'partially resolves', 'in part') while carrying a #Resolves edge, since that combination is always either a mis-pointed edge or an issue that needs splitting.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from storyControlMapReconciled to issue305;

    part issue306 : Issue {
        :>> id = "b7c19dc5-be2c-4ab7-b8c1-c846ed18a4c0";
        :>> title = "The pre-push hook refused every fresh project's bootstrap push of main";
        :>> createdAt = "2026-08-29";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-29 by the ctlPrePushBehind arming probe - a two-clone fixture attempting a REAL push through the REAL .githooks/pre-push - on its first run. The hook fetches origin main to verify behind-ness, and 'git fetch origin main' FAILS when the remote has no main yet; the fail-loud clause (K2) then refuses. So the BOOTSTRAP push - creating main on an empty remote, the first thing every keel init project does after git init and adding a remote - was refused, with a message directing the user to keel land, which would hit the same wall. The hook conflated 'cannot reach the remote' with 'the remote has no main YET'. THE ASYMMETRY THAT CAUSED IT: ref DELETION was already skipped (local_sha all zeros, one line above) while ref CREATION was not (remote_sha all zeros) - half of 'deleting or creating refs is not a behind-ness question' was implemented and the comment claimed both. IN-FIELD because every downstream project hits it: the self-build never did, since main existed on its remote before the hook was installed - the same reason it survived unnoticed. FIX (shipped with this issue, D0256 co-committed per the keystone lock): skip when remote_sha is all zeros, exactly as deletion is skipped. THE SECOND LESSON from the same probe: a PLAIN behind push never reaches this hook at all - git's own client-side fast-forward check refuses it first - so the hook's actual added value is the push git would ACCEPT: a FORCED one. The probe now pushes --force, testing what the control is FOR (the D0129 history-rewrite class) rather than a case git already catches. The arming contract's old claim that this 'can only be established by attempting a push, which a read-only view must not do' was true of the view and false of a test.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    // Re-pointed at filing time: the FIX ships with sprint 488, so the resolver is the story that
    // completes now - not the long-running probe task, which would hold a fixed defect open (the
    // inverse of the issue305 trap, where prose held a closed one open).
    #Resolves dependency from storyFourMoreControlsProven to issue306;

    part issue307 : Issue {
        :>> id = "bdee3068-393b-40d7-91f2-8f4ed566e017";
        :>> title = "guard 53 passes a traversal key: relative-with-dotdot escapes the project exactly as an absolute key does";
        :>> createdAt = "2026-08-29";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-29 by the control-propriety panel (pf02, lens classCoverage), by mutating the guard's motivating defect: the class is 'a manifest key that resolves outside the receiving project', and manifest-key-portability checks only the ABSOLUTE members (drive letter, leading slash, tilde). A relative key with traversal segments - file...__SL__..__SL__elsewhere decodes to ../../elsewhere - escapes the project root identically and PASSES. Demonstrated against the predicate by hand. This is the mutation-testing question doing exactly what the research said it would: the guard killed the specimen and three shape-variants, and the fourth variant class slipped. FIX: the same predicate also flags any decoded key containing a '..' path segment; arming re-proven by injection.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from storyProprietyRound1 to issue307;

    part issue308 : Issue {
        :>> id = "4ac6c679-4a3e-4b93-844e-4034fa9024c1";
        :>> title = "provenBy is a receipt-shaped testimony: nothing verifies the named test file exists";
        :>> createdAt = "2026-08-29";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-29 by the control-propriety panel (pf33, lens changeRobustness), by construction: no check exists to fail. provenBy = 'a/test/that/never/existed.rs' passes every gate, and so does a stale pointer after a rename. A dangling provenBy is WORSE than none - it is a receipt-shaped testimony sitting in the very contract (control-arming.toml, D0253) that exists to separate receipts from testimonies. The keystone lock authenticates the ACT of writing it, not the CONTENT. FIX: control-map-reconciled (guard 54) additionally resolves every provenBy path in control-arming.toml against the tree; a missing file is a violation naming the entry. The qualitative half - whether the named test actually EXERCISES that control - remains judgment (pf35) and stays with the panel.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from storyProprietyRound1 to issue308;

    part issue309 : Issue {
        :>> id = "fb3b9bcd-de99-463e-9205-92423dd4cad1";
        :>> title = "Root extras imported into .engine: the dot heuristic fit the specimen and misplaced everything else";
        :>> createdAt = "2026-08-29";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-08-29 by the kickoff DEMO, live: importing the exec-summary unit landed templates/ and tests/ under the receiving project's .engine/ - inert, in the wrong place, while reporting '5 file(s) from upstream'. restore_dst inferred repo-residence from a LEADING DOT, which fit the only extras that existed when it was written (.github/workflows, the decision-channel unit) and misplaced every dotless root extra. The pf02 overfit class, verbatim: the rule killed the specimen and none of its variants. FIX: residence travels WITH the bundle - export writes the unit's declared extras (repo-relative) into unit.toml, and restore_dst_with restores a declared extra where it lived; the dot rule stays as the fallback so pre-extras bundles import exactly as before. Both import loops thread the same list (two loops with one rule between them being the recorded cause of the LAST placement defect). CONTROL: a unit test pins declared-extra-at-root, engine-file-under-.engine, dot-fallback, and no-declaration-old-behaviour. Proven live: re-published, re-imported, extras restored at the demo project's root.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from storyKickoffReady to issue309;

    part issue310 : Issue {
        :>> id = "cf924c95-0287-4152-ba14-484678d2ee02";
        :>> title = "Regenerate-from-model writers silently drop what the engine's view cannot represent";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "Generalised from issue293's retro (sprint497). keel activate regenerated activation.toml from a template plus two computed lists, so it deleted every fact the engine's view could not represent: comments, the charteredBy reference, and the whole category of non-switchable [always] processes. NOTHING DETECTED IT - the keystone lock caught the FILE changing, and only because that file happens to be locked. A write path whose only audit is the control guarding its output is unaudited for every output that is not locked. THE CLASS: any command that rebuilds a human-editable file from the engine's model of it. sync-claude is the next candidate and already claims the correct shape (it merges user entries in settings.json rather than overwriting) - claimed, not currently proven by a test that a hand-added user entry survives a regeneration. SCOPE: enumerate the writers that regenerate a human-editable file, and hold each one's preservation claim with a round-trip test of the sprint497 shape (write, regenerate, assert byte-identical apart from the intended change).";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from activateDoesNotLoseFacts to issue310;

    part issue311 : Issue {
        :>> id = "ae5000ae-cc76-4b89-b268-60c3332c5873";
        :>> title = "The release process tags AFTER recording, so the tag and the recorded commit disagree by construction";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the rigor audit (2026-08-30) reading the release-recorded warning that had been standing since v0.3.1 shipped. The Release item recorded c4e306c (the content commit) while tag v0.3.1 points at c4d5dbf (the commit that RECORDED the release). Neither was a typo: the release process records the Release item, commits, and THEN tags HEAD - so the tag necessarily lands one commit after the commit the record names, for every release. The instance is corrected (the record now names the tagged commit, which is what users actually run); the PROCESS defect is this issue. Fix direction: tag the commit the record NAMES, not HEAD - or record the release and tag that same commit, so the two cannot diverge. Whichever is chosen must be held by a test that a release's recorded commit resolves to its own tag, since the guard that caught this is warning-tier and stood unread for days behind 61 undischargeable warnings (D0261).";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcNoOpInvarianceControl to issue311;

    part issue312 : Issue {
        :>> id = "7937936b-24d5-417b-98ee-686f98669997";
        :>> title = "decision-scaffolding has no way to say 'the change landed WITH the Decision', so self-executing decisions warn forever";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found by the rigor audit. The guard warns when an accepted #ProspectiveChange Decision has no inbound tracked-item edge, on the premise that it 'promises process change but charters no work'. For a decision whose change IS the commit - d0211 froze the enforcement surface, d0247 reclassified STPA, d0248 cut six clauses, d0257 shipped the propriety process and its skill, all landing in the same commit as the Decision - there is no future work to charter and never will be. The premise is right for promises and wrong for self-executing changes, so those four warn permanently: the SAME defect as the legacy-actor lines this audit just collapsed (D0261), regenerating itself in another channel. The fix is the shape this codebase already reached twice - THREE buckets, not two (issue173, quoted in keel hardening: the two-bucket version made a judgment look identical to a missing guard). A decision needs a way to declare 'self-executing, the change is in this commit', reviewable and disagreeable, exactly like process-enforcement.toml's declaredUnenforceable. NOT FIXED HERE, deliberately: the guard is under the keystone lock and adding a bucket is a weakening unless the declaration is itself checkable, which needs a Decision and a design - hardening it today would either lock out legitimate self-executing decisions or add an unchecked escape hatch. What IS fixed here: the three decisions that genuinely promised unbuilt mechanisms (d0201 Option B device HMAC, d0252 migration rollback, d0249 ungrounded-ratio trigger) now carry tracked work.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcUngroundedRatioTriggers to issue312;

    part issue313 : Issue {
        :>> id = "b5381977-0f45-42fd-93cf-16c70b9b04bd";
        :>> title = "A fresh keel project cannot make its first commit at a deep path: reference-decision filenames blow Windows MAX_PATH";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND BY RECONNAISSANCE for the federation test platform, before any suite existed. keel init at a 150-character directory produces .engine/reference/decisions/0137-a-release-is-verified-by-running-the-published-asset.sysml at 249 characters; git add -A then fails with 'unable to index file ... fatal: adding files failed' and the project cannot make its FIRST COMMIT. Discriminated against the alternative explanation: the SAME scaffold at C:/kt1 stages 484 files with exit 0, so this is path length, not a corrupt scaffold. Windows MAX_PATH is 260 and git refuses without core.longpaths=true, which keel init does not set. 150 characters of parent path is not exotic - a user under C:/Users/<name>/Documents/Projects/<client>/<project>/ is most of the way there before keel adds 100 of its own. THE SHAPE OF THE FIX IS A CHOICE, not obvious: set core.longpaths in the project keel init creates (only helps if keel owns the git init), shorten reference filenames on copy (changes a stable path downstream readers may cite), or DETECT the margin at init and refuse loudly rather than let the failure surface as an opaque git error later. Refusing loudly is the honest floor: a project that cannot commit is not initialised, whatever else succeeded.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcNoOpInvarianceControl to issue313;

    part issue314 : Issue {
        :>> id = "39a31cb5-85c4-461f-9972-582a56813240";
        :>> title = "keel migrate silently reverts a project's adoption choices: engine resync overwrites project-owned contracts";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND BY THE FEDERATION SUITE (F9), which exists because the propriety panel returned UNSOUND on the design for testing a project only at birth (pf42). step_engine_resync copies every shipped engine file over the project's copy, and .engine/contracts/ is shipped - so keel migrate overwrote activation.toml, adoption-profile.toml, engine-version.toml, installed-units.toml, github-actors.toml and parser-coverage-baseline.toml with the engine's defaults. DEMONSTRATED on a real project: deactivate render, commit, migrate - and render was ACTIVE again. THE DIRECTION THAT MATTERS MORE is the other one: where the engine's default has FEWER active processes than the project, migrate silently DISARMS controls the project had on, with no Decision anywhere - the unsigned control-weakening the keystone lock and audit-adherence exist to catch, arriving through a path neither of them watches. github-actors.toml is worse still: it declares WHO MAY DECIDE on the decision channel. FIXED: is_project_owned_contract - the engine ships a default so a fresh init has one, and resync ADDS it if absent but never overwrites it. The rule is 'a keel COMMAND writes this file'. Held by F9, which now asserts the deactivation survives the upgrade. THIS DESCRIPTION WAS REPAIRED (2026-08-30): its first version was written with backticks inside a double-quoted shell argument, so the shell EXECUTED them and substituted their stdout into the record - and also ran deactivate against this repository, whose activation.toml was restored from git. Third occurrence of a documented trap; routed to issue315.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcNoOpInvarianceControl to issue314;

    part issue315 : Issue {
        :>> id = "b7350b86-7701-4ead-bfb9-156c2bb29ac4";
        :>> title = "A documented trap fired a third time because its fix was applied to the instance, not the class";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "THIRD OCCURRENCE of a trap that already has a fix. Prose passed to a write command as a double-quoted shell argument has its backticks EXECUTED by the shell, and the command's stdout is substituted into the governance record in place of the author's words. D0224 fixed this for `record decision` by adding --from FILE after it happened twice. `record issue` was left on the argument path, and on 2026-08-30 it fired again while recording issue314: the shell ran `keel deactivate render` and `keel migrate`, substituted their output into the description, AND modified this repository's keystone-locked activation.toml, which had to be restored from git. The record was repaired by hand and says so. THE LESSON IS NOT 'be careful with backticks'. That is manual vigilance, which D0047 rejects as a control, and my own memory already carries the warning verbatim - it did not help. The lesson is that D0224 fixed the INSTANCE (decisions) and not the CLASS (every write path that takes prose as an argument). FIXED HERE: `record issue --description-from FILE`, mirroring the decision path, with the argument form still accepted for one-liners and its usage text naming the hazard. STILL OPEN as the class: any other write path taking prose by argument. The honest closure is a check that no write command accepts a long prose field by argument alone - which needs an inventory of write paths, so it is named here rather than claimed.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcNoOpInvarianceControl to issue315;

    part issue316 : Issue {
        :>> id = "7ae8dd2e-20aa-4029-9213-a8e767b9423d";
        :>> title = "reprocess-candidates cannot finish, so the antiquated-data question has never been answered on this project";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND BY THE VINTAGE PANEL (pf55), by measurement rather than inspection: keel reprocess-candidates does not finish. Timed twice - 240 seconds, exit 124, zero bytes of output - because it walks all 501 delivery stories and runs several git subprocesses PER STORY (govern.rs:362 all_delivery_stories, then govern_resolve -> git grep + git log per item). WHY IT MATTERS MORE THAN A SLOW COMMAND: this is the lens that answers 'which items were authored under a process version a SAFETY change later superseded' - that is, exactly the antiquated-data question. Because it never completes, that answer has never been computed on this project. The per-item lens (governing-version) works and is correct, but it is only usable by someone who ALREADY suspects a particular item, which D0261's own retro named as the shape of a metric that is not a control. THE FIX IS NOT A DESIGN CHANGE - the derivation is sound on four of five propriety lenses. It is a batched traversal: one git log --name-only pass building a commit -> touched-definition map and a decision -> intro-commit map, then an in-memory lookup per item, instead of a subprocess per item. VERIFY BOTH WAYS: the batched result must be IDENTICAL to the per-item result on a sample of items where the slow path can still be run to completion, and the whole-corpus run must finish - a fast lens that quietly answers differently is worse than a slow correct one.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcNoOpInvarianceControl to issue316;

    part issue317 : Issue {
        :>> id = "2c850de4-e479-4139-beaa-3ab746009abc";
        :>> title = "governing-version answers DELIVERY for every item type it does not cover, instead of unknown";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND BY THE VINTAGE PANEL (pf52). governing_def_for falls back UNCONDITIONALLY to .engine/workflows/delivery.sysml when an item has no #CharteredBy edge (govern.rs:19 and :39), and reprocess-candidates iterates only all_delivery_stories (govern.rs:362). CONSEQUENCE: asking for the vintage of a Need, SystemRequirement, Issue, Test, Statement, Decision or critique does not return 'unknown' - it returns DELIVERY, confidently and wrongly. A Need is governed by the Business workflow, not Delivery. A lens that answers the wrong question fluently is worse than one that refuses, because nobody checks a fluent answer. MEASURED SCOPE: 447 of 501 delivery stories carry a #CharteredBy edge, so derivation has real input for that ONE item type. No other item type in the corpus carries one. The human's intuition - that antiquated data carries no indication of what process authored it - is CORRECT for every item type except Story. FIX DIRECTION, and the choice matters: either (a) resolve per item KIND, with a declared kind-to-workflow map so a Need resolves to Business, or (b) return UNKNOWN when no charter edge exists, which is honest and immediately less useful. (a) is better only if the map is declared and checkable rather than another hardcoded constant restating something the model already knows - the issue300 class. Whichever is chosen, the unconditional Delivery fallback must go: it manufactures an answer.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    // Re-pointed 2026-08-30: issue316 was filed against the nearest AVAILABLE resolver because
    // `record issue` requires one, which is the very antipattern the github-intake process warns
    // about (an issue routed to a resolver that will not resolve it looks handled and is not).
    #Resolves dependency from dcBatchedGovernanceIndex to issue317;

    part issue318 : Issue {
        :>> id = "219bc82c-091c-4777-ba90-1434843bbabd";
        :>> title = "keel land reports success on a push whose CI it never looked at";
        :>> createdAt = "2026-08-30";
        :>> createdBy = "claudeOpus5";
        :>> description = "CI was red for seven hours across five pushes and nothing noticed. The human noticed. WHY EVERY SIGNAL SAID FINE. The local gate is thorough - validate, 56 guards, check-engine, rules --enforce, clippy - and it passed every time, because the three failing cases were ENVIRONMENT-dependent and this machine has a configured library, a global git identity and an authenticated gh. `keel land` reports 'landed' on a successful PUSH, which is where its knowledge stops. Green locally, push succeeded, command said landed, CI red. THE GAP IS PRECISE: land makes a claim about a GIT OPERATION while reading as a claim about the tree's standing. A contributor who runs land and sees 'landed' has been told the thing worked. FIX DIRECTION, and it is a real design choice rather than an obvious patch. (a) land POLLS the resulting run and reports its verdict - accurate, but turns a fast command into a two-minute one and needs a network path that may not exist. (b) land reports the run URL and the PREVIOUS run's verdict, cheap and immediate but tells you about the last push rather than this one. (c) land records the pushed SHA as awaiting CI and the NEXT keel invocation reports any awaited SHA whose run has since failed - cheap, asynchronous, and it surfaces the failure at the start of the next session, which is when someone can act. (c) is the shape that fits how this project is actually used, and it needs no new network call in the hot path. WHATEVER IS CHOSEN MUST NOT BE A REMINDER. D0047 rejects manual vigilance as a control, and my own working memory already carried 'check gh run after EVERY push' verbatim - it did not help for seven hours. The check has to be mechanical and it has to fire where someone will read it.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcBatchedGovernanceIndex to issue318;

    part issue319 : Issue {
        :>> id = "675a3b5d-97f9-473b-801c-70bf9481dc12";
        :>> title = "An unrecognised flag becomes the ROOT, so a typo reports every guard green over a phantom directory";
        :>> createdAt = "2026-08-31";
        :>> createdBy = "claudeOpus5";
        :>> description = "INGESTED FROM GITHUB as GH#14 (st058), reported from field use. `keel guard --read` treated the unrecognised flag as the ROOT argument, gated a directory that does not exist, found no files, and reported every guard PASS with 0 scanned. TWO DEFECTS COMPOUNDING, which is why it is worse than either alone: silent argument mis-parsing, and pass-at-zero. Together they produce a GREEN RUN OVER NOTHING - output indistinguishable from a clean tree, on a command whose entire job is to tell you whether the tree is clean. REPRODUCED before fixing, on a fresh strict-profile project: every guard PASS, 0 scanned. NOT AN ISOLATED TYPO: the identical shape appeared hours earlier while building github-ingest, where the trailing value of --at was taken as the root and the command failed with an opaque filesystem error. Two independent instances of 'an argument that is not a path becomes the path' in one day is a class, not an accident - so the fix is a shared refusal applied to guard, validate and check-engine rather than a patch where it was noticed. FIXED: refuse_flag_as_path names the offending argument and states why refusing beats answering. Held by unknown_flag_is_refused.rs, which asserts BOTH that the three commands refuse with no verdict line AND that an ordinary root still scans the corpus - a refusal that broke real usage would have traded one vacuous pass for another.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcUnknownFlagIsRefused to issue319;

    part issue320 : Issue {
        :>> id = "46806a42-912a-4810-b6f0-3d4e54d928c2";
        :>> title = "Every issue id I cited in prose today was off by one, because I piped the command that told me through tail -1";
        :>> createdAt = "2026-08-31";
        :>> createdBy = "claudeOpus5";
        :>> description = "EVERY ISSUE ID I CITED IN PROSE TODAY, from the migrate-adoption defect onward, was off by one. Six distinct defects, cited in Decisions D0263/D0265/D0266/D0268, in CLAUDE.md, in four source files, in two sprint records and in a propriety round: 33 wrong references in total, corrected here. ROOT CAUSE, PROVEN NOT GUESSED: `keel record issue` prints the allocated id on its FIRST line - 'recorded issue320 -> ...' - and even prints the resolver edge to author. I piped every one of those calls through `| tail -1`, which showed only the closing 'run keel validate to confirm' line and discarded the id. I then wrote down the number I ASSUMED had been allocated. The command told me the truth every time and I truncated it away. THIS IS THE THIRD TIME TODAY that output truncation caused a defect: `| head -4` killed a migration mid-apply and left a half-written tree; `| tail -1` hid an id fourteen times; and the same reflex hid the ingest count earlier. The pattern is piping a command's output through a truncating filter and then reporting on what I did not read. WHAT CAUGHT IT, AND WHAT DID NOT. `edge-endpoints` caught it the moment a TYPED EDGE named a non-existent issue320 - that guard works exactly as designed. Nothing caught the 33 PROSE citations, because a reference inside a description string is not an edge. So the model's typed structure is verified and its prose is not, and prose is where most cross-references actually live. CONTROL GAP, stated as the honest finding rather than a resolution to be careful: a guard could check that every `issueNNN` / `dNNNN` token appearing in authored prose resolves to a declared item. It would have caught all 33 immediately. It is not built here because the false-positive surface needs thought - prose legitimately discusses ids that were superseded or that belong to another project - and shipping a noisy guard would repeat the D0261 mistake of adding lines nobody can act on.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcUnknownFlagIsRefused to issue320;

    part issue321 : Issue {
        :>> id = "7e18cd4f-8544-4640-a0d0-53398ef50bd6";
        :>> title = "The CLI has 111 dispatch arms and ~30 of them are the same act: compute a lens and print it";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "INGESTED FROM THE HUMAN (st063), on the summary page: 'remove cruft from CLI. tons of commands, when a smaller subset should do, and parentheticals embedded in descriptions. tighten it up - what core functional hooks do we need in the CLI? a lot of these seem to be variations of an idea cemented as separate cli hooks. and critique all descriptions to trim them down to the real root of what is being done and what a user needs to know to run it.' MEASURED, and they are right about the scale. 111 dispatch arms in main.rs. About 30 of those are READ-ONLY LENSES - assumptions, attestation-coverage, authority-queue, concern-coverage, contentions, controls, coverage, critique-coverage, dispositions, hardening, indicators, intake, open-issues, orphans, outstanding, question-coverage, recent, reprocess-candidates, rootedness, sitting-coverage, suspect, tier-satisfaction, verification, why, workflows and more. Every one of them is the same act: compute a view of the model and print it. Each got its own top-level verb. THE DUPLICATION IS DEEPER THAN THE COMMAND LIST. 16 of the 32 declared VIEWPOINTS also have a dedicated top-level command of the same name - so a lens is declared in one mechanism and implemented in another. And the two are only half-connected: `keel view orphans` FAILS with 'view file not found' while `keel orphans` works. Two mechanisms for one idea, neither complete. DONE IN THIS PASS, because it breaks nothing: 55 help rows carried a decision-id parenthetical - (D0195), (issue171/D0169) - which is provenance for a maintainer, not something a user needs in order to run the command. All 55 stripped; the citations remain in the source doc comments, where someone looking for the reason will actually look. NOT DONE, and this is the part that needs a decision rather than initiative: collapsing ~30 verbs into one router is a BREAKING change to a surface other things call by name. Measured cost: 118 references across .engine skills and processes invoke these commands directly, and guard 39 (tool-reference) fails a process that names a tool it did not receive. So the migration is real work with a real failure mode, not a rename. THE SHAPE I WOULD PROPOSE, for their call: one `keel show <lens>` routing to the existing implementations, with all current names kept as aliases for a deprecation window so nothing breaks on the day it lands; then the alias list shrinks as the 118 call sites move. That collapses the visible surface from 111 arms to roughly twenty verbs - orient/status/whats-next, show, gate, record, audit, github, library, process, init/migrate/sync/land, serve/render/report/deck, actor/enroll, hook - without removing a single capability.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    // Re-pointed: record issue auto-triaged this onto a task that is already DONE. The honest
    // resolver is the consolidation task, which the human has not yet authorised - so the issue
    // carries the analysis and waits, rather than looking handled.
    #Resolves dependency from dcCliSurfaceConsolidation to issue321;

    part issue322 : Issue {
        :>> id = "af25e075-18f6-4678-bae2-2b2aadcb6e2c";
        :>> title = "The prose-by-argument trap fired a fourth time, because the fix was applied per-command not per-class";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOURTH OCCURRENCE of the same trap, and the reason it recurred is precisely diagnosable: the fix was applied per-command instead of per-class. D0224 added `--from FILE` to `record decision` after shell backticks in a double-quoted argument were EXECUTED into a governance record twice. issue315 recorded the third occurrence, in `record issue --description`, which also ran `keel deactivate` against this repository; that one was fixed with `--description-from FILE`. `add-task --dod` was the remaining prose-by-argument path and nobody looked, so today it ate `keel show <lens>` out of a task's Definition of Done, leaving the sentence 'one routing to the existing implementations' - a DoD that reads as finished prose and is missing the thing it specifies. WHAT MAKES THIS DIFFERENT FROM CARELESSNESS: after the third occurrence I wrote the rule into CLAUDE.md, into the skill, and into a Decision. All three said never pass prose as a double-quoted shell argument. The instruction was present, correct, and prominent - and the command I needed had no file-taking flag, so following the rule was impossible for that call. A rule that cannot be obeyed on some path is not a control on that path. FIXED: `add-task --dod-from FILE`, with the usage text naming the hazard on the inline form. That is now every write path that takes prose. THE REMAINING GAP, stated rather than claimed closed: nothing CHECKS that a write path taking a long prose field offers a file alternative. A fifth such path added tomorrow would reopen this. That check is cheap in principle - enumerate the write commands, assert each prose flag has a `-from` sibling - and it is not built here because the honest resolver is the CLI consolidation work (issue321), which will change what the write paths are.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcCliSurfaceConsolidation to issue322;

    part issue323 : Issue {
        :>> id = "dae5054f-9128-40e5-98de-3ed72a35b88c";
        :>> title = "Re-publishing an imported unit re-mints its identity and resets it to v1, so the drift report affirms there is no update";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while publishing an improvement to the exec-summary skill, and it defeats the drift report shipped one day earlier. WHAT HAPPENED, step by step and all verified on disk. This project imported `exec-summary` from the library; the import wrote an install record keyed by the LIBRARY's unit id a2428932. The skill was then improved and re-published. Export looked the unit id up in THIS project's .engine/contracts/unit-ids.toml, found a DIFFERENT id - a55244a1 - and wrote that into the library's unit.toml, replacing the original identity. Because install_record(a55244a1) finds nothing (the record is filed under a2428932), next_unit_version fell to its None arm and wrote version = 1. SO THE PUBLISHED UNIT CHANGED CONTENT, CHANGED IDENTITY, AND KEPT ITS VERSION NUMBER. The library's SKILL.md is now 151 lines carrying the new sufficiency rule; unit.toml still says version = 1. WHY THIS IS WORSE THAN A NUMBERING BUG. Every consuming project compares versions to decide whether it is behind - that is exactly what `keel status`'s drift section does, shipped yesterday against srDriftIsReportedPerUnit. A project holding the old v1 sees 'v1 available, v1 installed, nothing behind' while the content differs entirely. The report does not merely miss the update; it AFFIRMS that there is none. And the identity change means even a corrected version number would not help: the unit a project installed no longer exists under that id. TWO DEFECTS, and they should not be conflated. (1) A unit's identity is re-minted on export from a project that IMPORTED it, instead of being carried from the install record. (2) The version resets when the id lookup misses, silently, because None means 'new unit' and a missed lookup is indistinguishable from one. FIX DIRECTION. Export must prefer the id in the INSTALL RECORD over a locally minted one - a unit's identity belongs to the unit, not to the project that happens to be exporting it - and a version lookup that misses on a process the project demonstrably has installed should REFUSE rather than restart at 1. Whatever is chosen must be held by a round trip: import a unit, change one file, publish, and assert the identity is unchanged AND the version incremented. That test would have failed today.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    // MIS-TRIAGED ON ARRIVAL and corrected here: this pointed at dcCliSurfaceConsolidation, which is
    // the resolver of the two issues recorded immediately before it and has nothing to do with unit
    // identity. The `issues` guard passed anyway, because it checks that a #Resolves edge EXISTS
    // (guards.rs:970-986) and never that the resolver can resolve it — so a false triage reads as
    // triaged. issue322 above points at the same task DELIBERATELY and says so in its own text; that
    // is what a correct one looks like, and the difference is not visible to any control today.
    #Resolves dependency from dcUnitIdentityIsCarriedNotMinted to issue323;

    part issue324 : Issue {
        :>> id = "9cdbc5c6-28d9-41fb-a5d9-80fa8e7960f4";
        :>> title = "Migrate's dirty-tree refusal and D0251's skew refusal form a circular lock, sprung by a file the engine wrote itself";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "REPORTED BY A PEER SESSION working in another keel project (their issue021), and VERIFIED HERE IN SOURCE before recording rather than taken on their word. Three individually-correct controls compose into a lock with no non-bypassing exit, and the file that springs it is written BY THE ENGINE. THE CYCLE, each leg read at file:line. (1) `keel migrate` refuses on a dirty tree - migrate.rs:652-656, and the check at migrate.rs:670-687 runs `git status --porcelain -- .tracking .engine`, which reports UNTRACKED files, so a brand-new file trips it. (2) Committing that file to clean the tree fails - the pre-commit hook runs `keel validate`, and validate REFUSES under engine-version skew because a gate surface must not return a verdict from an engine the project did not pin (main.rs:226-230, D0251). (3) Skew clears by running the pinned binary or by `keel migrate` - main.rs:188. A project whose only available binary is the newer one has exactly one exit, and it is leg (1). WHY IT IS THE ENGINE'S OWN DOING. The uncommitted file in the reported case was `.tracking/obligations/red-yield-<short>.sysml`, auto-written by `record_obligation` (write.rs:471-481) when the Stop hook's second red pass yielded, under D0176/K7. That function's own doc comment reasons about deadlock - 'one Issue per file so recording can never deadlock on the very file being repaired' - and the reasoning is correct at the FILE level and blind at the TREE level: a new untracked file in a tree that cannot be committed is exactly as blocking as an edit to the file under repair. And the trigger correlates with the hazard: an obligation is recorded precisely when a project is already unhealthy, which is also when it is most likely to be behind its pin. NOT REPRODUCIBLE IN THIS REPO, and that is why it went unseen. check_preconditions returns Refusal::SelfBuild for any tree holding keel-cli/Cargo.toml (migrate.rs:664-666), so migrate never runs here at all. This class of defect can only be found downstream, which makes the peer's report the evidence and not a nuisance. WHAT THE HUMAN ACTUALLY DID: `git stash push -u -- <path>` then migrate then `git stash pop` - the escape migrate's own message names. It works and bypasses nothing. The outcome worth preventing is the contributor who reaches for `--no-verify` instead, which disables every other check in the hook to get past one. FIX DIRECTION, and the two rejected alternatives are the argument for the chosen one. (a) Have the obligation recorder STAGE its own file: does not help - staged is still uncommitted, and `git status --porcelain` reports it either way. (b) Degrade pre-commit validate to a warning under pure skew: weakens a gate to work around a different gate, which is how a control gets hollowed out one exception at a time. (c) Let migrate proceed when every uncommitted path under .tracking/ is an engine-authored obligation record: this is the only shape that breaks the cycle WITHOUT weakening anything, and it is defensible on migrate's own stated rationale - the refusal exists because 'a half-migrated tree mixed with uncommitted edits cannot be told apart afterwards', and a brand-new additive file in a dedicated directory CAN be told apart. It must stay narrow: the carve-out is `.tracking/obligations/` only, untracked-or-added only, and migrate must NAME the files it tolerated so the exemption is visible rather than silent.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcObligationDoesNotLockTheTree to issue324;

    part issue325 : Issue {
        :>> id = "aed57d5d-fa22-46eb-96e6-ca64393d708a";
        :>> title = "add-task wrote DoD prose into a SysML literal unsanitized, so a quoted phrase left the backlog unparseable";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND BY HITTING IT, one command after recording issue322 about this exact class. `add-task --dod-from` interpolated its prose RAW into a SysML string literal (write.rs:1193) while every other prose write path in the same file bound sanitize_field. My DoD contained the phrase THE REMEDY IS NOT (quote)fill in the placeholder(quote), the inner double quote closed the literal, and .tracking/backlog.sysml stopped parsing at 418:398. WHAT MADE IT WORSE THAN A MALFORMED FIELD. The tree was UNPARSEABLE, so the next command could not read the model to do anything at all - including to report the problem. `keel record issue` failed with 'cannot read the model to check the resolver', which is the write API refusing correctly and for the wrong-looking reason: the resolver was fine, the corpus was broken by the previous write. A write path that can render the model unreadable takes every other control down with it, because they all parse before they check. THE SAME SHAPE AS issue322, ONE DAY LATER AND FROM THE SAME CAUSE. issue322 recorded that the prose-by-argument trap fired a fourth time because the fix had been applied PER-COMMAND rather than per-class, and its remedy was `add-task --dod-from FILE`. That flag was added THIS MORNING - by me - and it was wired to read a file without routing the result through the sanitizer the other write paths use. So the fix for a per-command defect was itself applied per-command. The rule I wrote in issue322's own text ('a fix applied per-command leaves the next path open') described what I was about to do. FIXED AT THE WRITE PATH, and SWEPT rather than patched: before changing the line I grepped every interpolation into a SysML literal in write.rs. record_issue_locked binds `let s = sanitize_field` and uses it on title, date, author, description, severity and resolver; record_decision does the same for its six fields; record_obligation binds `esc`; the notes, view-attribute and claim paths all call it. add_task_locked was the only one that did not. That is now closed and the count is zero. RESIDUAL, stated: nothing CHECKS that a new interpolation routes through the sanitizer. A fifth prose field added tomorrow could skip it exactly as this one did, and the only signal would be a corrupted tree. The honest control would be a lint or a type that makes an unsanitized value unusable in a literal - not built here, and named so it is not mistaken for closed.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDodProseCannotBreakTheModel to issue325;

    part issue326 : Issue {
        :>> id = "8df99ffa-9adc-4556-b797-c5a7506dd12e";
        :>> title = "decision-channel ships two CI workflows as extras that build the keel crate, so every consumer importing it acquires two permanently red workflows";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "REPORTED BY A PEER SESSION that imported decision-channel v6 into a non-engine project (their issue022), and VERIFIED HERE before recording rather than taken on their word. The unit ships two CI workflows as declared extras, and NEITHER can run anywhere but the engine's own repository. VERIFIED AT file:line. `.engine/contracts/unit-extras.toml [decision-channel] files` declares all four extras, including .github/workflows/decision-issue.yml and .github/workflows/decision-record.yml; the library's decision-channel unit ships exactly those four files alongside the process, skill and registry. Both workflows run `cargo build --release -p keel-cli --bin keel` - decision-issue.yml:27 and decision-record.yml:49. A consuming project has no Cargo.toml and no keel-cli crate, so the build step cannot succeed. The peer additionally reports both returned startup_failure on their very next push, which is their observation and not one I can reproduce from here; the build-step defect stands independently of it. SO EVERY CONSUMER THAT IMPORTS decision-channel ACQUIRES TWO PERMANENTLY RED WORKFLOWS. That is not merely noise. A project whose Actions tab is red by default trains its maintainers to ignore red, which is the same failure that let this repo's own CI stay red for seven hours across five pushes (issue318). Shipping a red workflow is shipping a disabled alarm. SAME FAMILY AS issue301, DIFFERENT HALF. Both are unit EXTRAS behaving wrongly once they leave the engine repo: issue301 was about how extras were KEYED (absolute paths naming the author's machine), this is about whether they are RUNNABLE where they land. The generalisation worth holding: an extra is the only part of a unit that executes in the consumer's environment, and nothing checks that it can. THE DEEPER ROOT, and it is the reason this is not a one-line fix. Every CI workflow keel ships was written for a repository that CONTAINS the keel crate. The engine's own in-workflow gate must build from source - it exists to gate recorder commits with the code in that commit - so the workflow is correct where it lives and wrong everywhere it is sent. A unit cannot ship a file that is right in both places unless the file itself distinguishes them. NOTE ON THE NAIVE FIX. Replacing the build with a download of the LATEST binary reintroduces the failure from the other side: the project's pin is BINDING and validate/guard REFUSE under engine-version skew (main.rs:226-230, D0251), so a consumer's CI running a newer binary than its pin would refuse every gate step. The install must read .engine/contracts/engine-version.toml and fetch that exact version.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcShippedCiRunsWhereItLands to issue326;

    part issue327 : Issue {
        :>> id = "3ae25be1-7641-405f-9e16-2d4e9215b864";
        :>> title = "keel init scaffolds a CI gate whose install step is three echoes, so a fresh project's own gate has never run";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "REPORTED BY A PEER SESSION as a pre-existing failure in their own project, explicitly not attributed to me - and it turns out to be mine. VERIFIED HERE at claude_surface.rs:162-185. WHAT keel init SCAFFOLDS. CI_TEMPLATE emits .github/workflows/keel-gate.yml whose `install keel` step is three echo statements and nothing else: it echoes 'Install the released keel binary for linux and put it on PATH', echoes the releases URL, echoes 'Then delete this echo block'. The very next step runs `keel validate .`. So the workflow reaches a command that is not on PATH and fails with `keel: command not found` on the project's FIRST push. WHY THIS IS WORSE THAN AN UNFINISHED TEMPLATE. The four steps after the placeholder are validate, guards, rules and audit-history - this file IS the downstream project's layer-3 verification, the hook-independent gate K15 exists to provide precisely because nothing remote should assume a local hook ran. A fresh project therefore starts life with its own gate never executing, and with a red Actions tab that reads as 'the placeholder nobody deleted' rather than 'your gate is not running'. The redness has a ready explanation that is not the true one, which is the property that makes a broken control stay broken. AND THE PLACEHOLDER CANNOT SIMPLY BE FILLED IN. Downloading the latest release would put the runner in D0251 skew against the project's pin, and validate/guard REFUSE under skew on a gate surface (main.rs:226-230) - the workflow would go from `command not found` to a refusal, still red, now for a subtler reason. The install must read .engine/contracts/engine-version.toml and fetch that version. FEASIBLE AS OF TODAY, which is worth recording because it may not have been when the placeholder was written: the engine repository is PUBLIC (gh repo view), so a runner needs no secret to download; and release.yml:49-76 attaches a per-platform asset per tag, keel-linux-x86_64 for the runner this template targets. Releases exist through v0.3.1. Nothing blocks a real install step. RELATED TO issue326 BY ROOT: both are shipped CI that assumes the engine's own repository. This half assumes a human will finish the file; that half assumes a keel-cli crate is present. Neither has ever been observed to run green anywhere but here.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcShippedCiRunsWhereItLands to issue327;

    part issue328 : Issue {
        :>> id = "65d4fda8-57dd-43bc-8b57-be7806525856";
        :>> title = "keel migrate silently discards a project's edits to an engine-shipped file, reporting only a count";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND BY BUILDING THE REALISTIC MIGRATION FIXTURE, which is exactly what that fixture exists for - the first property it added to a fresh scaffold produced a defect on the first run. OBSERVED, not reasoned: a project customised an engine-shipped file (.engine/skills/actor-enrollment/SKILL.md, one appended comment line), committed, and ran keel migrate. The customisation was present before and GONE after, and migrate's entire output about it was 'wrote 2 file(s)'. THE ISSUE314 CLASS, ONE FILE-KIND OVER. issue314 was migrate silently reverting a project's ADOPTION choices, fixed by exempting six project-owned contracts from the resync. This is the same shape for the project's edited CONTENT: the resync writes engine files in place, and anything the project put there is discarded with no receipt. WHAT IS AND IS NOT THE DEFECT, because the distinction decides the fix. Overwriting is arguably the correct POLICY - engine files belong to the engine, and a project wanting durable local content has the unit mechanism for exactly that. Preserving instead would leave a project pinned to a stale engine file forever with no signal, which is worse. So the defect is not the replacement; it is the SILENCE. A count cannot be acted on: it does not say which files, so it cannot tell a project that the edit it was relying on no longer exists. WHY NOT A THREE-WAY MERGE, which is the obvious richer answer: it needs the engine content at the project's OLD vintage, and keel deliberately stores no vintage stamp - the vintage IS which migration steps still match (migrate.rs module docs, and the reasoning there is sound: a stored stamp can drift from the tree it claims to describe). There is no baseline to merge against, and inventing one would be a bigger change than the problem warrants. FIXED: report_written names every file the migration wrote, repo-relative, under a line saying plainly that a local edit to any of them is now gone and git diff will show what changed. That hands the reader the one tool that settles it, at the moment pm3Reconcile already tells them to review before committing. NOTE ON HOW IT WAS ALMOST MISSED. The first probe of this behaviour was INCONCLUSIVE and looked like a pass: it ran at a deep scratchpad path, hit Windows MAX_PATH mid git-add, left the tree uncommitted, and migrate then refused on a dirty tree - so the customisation survived because the migration never ran. Reading that as 'the customisation is preserved' would have recorded the opposite conclusion. The issue313 class biting a test rather than a user, and the reason the fixture now roots itself shallowly on purpose.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcMigrationFixtureIsRealistic to issue328;

    part issue329 : Issue {
        :>> id = "a77eec16-afca-49a5-b5a0-f758ffe3a6f0";
        :>> title = "A non-zero count asserted by substring broke twice, and the first repair left an identical live instance";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "THE SAME TEST DEFECT FIRED TWICE, AND THE FIRST REPAIR LEFT AN IDENTICAL LIVE INSTANCE UNTOUCHED. Two tests asserted a non-zero count by NEGATED SUBSTRING - !text.contains('0 <label>'). That form is wrong for every count ending in zero, because '580 tracked file(s)' CONTAINS '0 tracked file(s)' as a substring. TIMELINE, which is the reason this is recorded rather than just fixed. It first fired earlier today when a guard's scanned count reached 370 and federation.rs went red; that site was repaired and a comment left explaining the trap. It fired again this evening when the tracked-file count reached 580, reddening status_distinguishes_unknown - and the sweep that followed found unknown_flag_is_refused.rs:58 carrying the identical predicate, live, untouched by the first repair, waiting for its own count to end in a zero. WHAT IT ACTUALLY COSTS. Only false REDS, not false greens: when the count genuinely is zero the assertion still fails, which is what it is for. So this is a nuisance-class defect rather than an honesty-class one - but it cost two debugging detours, and the second one arrived mid-way through a five-step plan where a red suite is indistinguishable from a real regression until someone reads it. THE PATTERN IS THE FINDING, NOT THE STRING. This is the third time in one day that a defect recurred because its fix was applied PER-INSTANCE rather than PER-CLASS: issue322 (the backtick trap fired a fourth time because --from was added per-command), issue325 (add-task's --dod-from was added without routing through the sanitizer every other prose path used), and now this. In all three the first repair was correct and local, and the class stayed open. The generalisable rule: after fixing a defect, the next action is to GREP FOR THE SHAPE, not to move on - and the grep costs seconds where the recurrence costs a debugging session each time. FIXED, and the correction is a control this time (D0047). Both sites parse the number and compare it. no_test_asserts_a_count_by_substring walks every .rs under tests/ and fails on any negated-substring count assertion, naming file and line - so a third instance is refused at the point of writing rather than discovered when a counter happens to reach a multiple of ten.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcCountAssertionsParseTheNumber to issue329;

    part issue330 : Issue {
        :>> id = "15921bc7-47ab-4730-a0c4-0b9bf467a61b";
        :>> title = "An authorised breaking rename cannot land: editing human-owned viewpoint renderers violates D0108, leaving them stale violates the renderer guard";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "A BREAKING CHANGE THE HUMAN AUTHORISED CANNOT BE LANDED WITHOUT VIOLATING A GUARD, AND NOTHING ANTICIPATED IT. D0273 collapses 35 lens verbs into `keel show <lens>` and removes the old spellings. Ten declared Viewpoints in .engine/views/viewpoint-registry.sysml carry a `renderer` field naming one of those verbs, and those ten items are owned by wweatherholtz. BOTH DIRECTIONS FAIL A CONTROL, and neither guard is wrong. EDIT the renderer strings: `guard:ownership` fails 10 times. D0108 - a non-owner ADDS items and typed edges or SUPERSEDES, never overwrites in place. Verified by running it: 10 violations, all on wweatherholtz-owned items; the 12 viewpoints owned by claudeOpus5 pass, so the guard is discriminating correctly rather than misfiring. LEAVE them stale: `guard:viewpoint-renderer` fails. classify_renderer resolves the lens behind the router, so a renderer naming a removed verb is 'unknown' - which is the guard working, since the declaration would otherwise point at nothing. THE SANCTIONED THIRD PATH DOES NOT RESOLVE IT. D0108 offers SUPERSEDE to a non-owner, but neither guard excludes superseded items - checked in the source, not assumed - so a superseding viewpoint would leave the original still declaring a dead renderer and still failing the renderer guard. The escape hatch D0108 provides does not reach this case. WHY THIS IS A GOVERNANCE GAP RATHER THAN A BUG IN EITHER GUARD. A viewpoint's renderer is not a JUDGMENT its owner made; it is a POINTER to a mechanism the engine owns and has now renamed. D0108 exists to stop a non-owner changing what someone MEANT, and updating the spelling arguably preserves their meaning rather than altering it. But that distinction is mine to argue and not mine to act on: talking myself past an ownership control to land my own change is precisely the self-modification class D0209 and issue236 exist to prevent, and the reasoning would be equally available to any future change that found the guard inconvenient. WHAT IS BLOCKED. The D0273 work is complete and green on every other axis - 31 of 31 comparable lenses byte-identical against the pre-collapse binary, 463 tests passing, clippy clean, 55 of 56 guards green - and is NOT COMMITTED, because committing it would mean landing a red gate. Steps 1 to 3 of the D0275 migration plan are landed; step 4 is held here. WHAT WOULD RESOLVE IT, for the record and without choosing between them: the owner makes the ten edits; or the ownership guard learns that a mechanically-renamed reference is not a field edit (an enforcement-surface change needing its own signed Decision); or D0108 gains a stated rule for engine-driven renames of owned items, which is the general form and the one that would also cover the next rename. WHERE THE WORK IS. The D0273 change is complete, green on every other axis, and NOT COMMITTED - it sits on `git stash@{0}`, labelled. Applying it will conflict on .tracking/issues-claudeOpus5.sysml, because THIS issue was recorded before the stash and re-recorded after it; keep one copy. The decision surface is published at https://claude.ai/code/artifact/79c42da9-bd0e-4761-9c18-ecc9f8ad28d6 and carries the ten exact edits.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcCliSurfaceConsolidation to issue330;

    part issue331 : Issue {
        :>> id = "0ef7db3e-7e53-4fa2-88f8-2df83ca34223";
        :>> title = "decision-scaffolding passes a multi-clause Decision once ONE clause is chartered, so partial delivery is invisible";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "CONTROL DEFECT, direction UNDER-REPORTING (D0278). `decision-scaffolding` checks that an accepted #ProspectiveChange Decision has SOME inbound tracked-item edge - one is enough. A Decision with several independent clauses therefore satisfies it the moment ONE clause is chartered, and partial delivery of a compound Decision becomes structurally invisible. OBSERVED: D0252 was accepted on 2026-08-30 with two clauses. Clause B (a migration that cannot finish rolls back) was chartered by dcMigrationRollsBack and built. Clause A (a unit declares the CAPABILITIES it invokes, and an install lacking one refuses by name) had its guard half already shipped under D0183 and its COMMAND half never built at all - the token `commands` appeared nowhere in process_cmd.rs. The guard passed for nine days while half an accepted Decision sat undelivered, and the gap surfaced only because D0273's clean break needed exactly that missing mechanism. INVALIDATION - what it passed while broken: every `decision-scaffolding` PASS on any multi-clause Decision, across 67 scanned. NOT RE-CHECKED, and worth being precise about why it is hard to re-check: no view lists Decisions by clause, because CLAUSES ARE PROSE. There is no authored fact saying D0252 has two of them. So the population cannot even be enumerated without reading 276 Decisions. FIX DIRECTION, and the honest one is not a guard change: per-clause charter edges would make delivery computable, which is a schema question (a Decision would need addressable clauses) and belongs in a proposal rather than a patch. Until then the guard is correct about what it checks and must not be read as checking more.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDecisionClausesAreAddressable to issue331;

    part issue332 : Issue {
        :>> id = "2d29fe40-a4aa-47e1-9807-bb0ee2d459fd";
        :>> title = "The ownership guard attributes edits to the machine's bound actor, not the session's, so it over-reports against items the session owns";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "CONTROL DEFECT, direction OVER-REPORTING (D0278). The `ownership` guard (D0108: a non-owner ADDS or SUPERSEDES, never overwrites in place) attributes an edit to the actor bound to the MACHINE rather than to the actor the session is running as. A session that sets KEEL_ACTOR - the per-session mechanism D0129 explicitly sanctions - is therefore reported as a non-owner editing items it in fact owns. OBSERVED, with both numbers: this machine is bound to `claudeFable5`; this session runs as `claudeOpus5`. Running `keel guard` under KEEL_ACTOR=claudeOpus5 reported 10 ownership violations, all genuinely wweatherholtz-owned. The Stop hook, which runs WITHOUT the env var, reported those 10 plus 12 more against viewpoints owned by claudeOpus5 - items this session may edit. 12 false violations against 10 real ones: the majority of the gate's output was wrong. INVALIDATION - what it passed while broken: nothing silently. This is the over direction, so its errors are refusals rather than false greens, and no verdict needs re-checking. The cost is different in kind and is the reason it is still High: an over-reporting BLOCKING gate is one people learn to work around, and this one blocks the turn. It also actively obscures the real finding - the 10 genuine violations arrived buried in 22, which is how a true signal is lost inside its own noise (the D0214 failure through a different door). FIX DIRECTION: the guard should read the effective session actor the same way the write paths do (crate::actor::resolve, which honours KEEL_ACTOR before the machine binding) rather than the binding alone. That is a guard-source change under the keystone lock, so it needs its own signed Decision - which is the stated reason for carrying it rather than fixing it in the turn that found it.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcControlDefectsAreFixed to issue332;

    part issue333 : Issue {
        :>> id = "3f4b3167-70d6-4022-9138-dba5f83308fc";
        :>> title = "The issues guard passes a mis-triage - it checks a #Resolves edge EXISTS, never that the named resolver could resolve it";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "CONTROL DEFECT, direction UNDER-REPORTING (D0278). The `issues` guard enforces that every Issue carries a #Resolves edge, and enforces nothing about whether the named resolver could possibly resolve it. Predicate at keel-cli/src/guards.rs:970-986: it sources the untriaged set from the declared issuesTriagedRule and reports only issues with NO edge. An edge to an unrelated task is indistinguishable from a correct triage. WHAT MAKES THIS THE UNDER DIRECTION rather than a missing feature: the guard PASSES a state it exists to catch. Triage is the thing being enforced, and a false triage is untriaged with an edge on it. OBSERVED, not theorised: issue323 (unit identity re-minted on republish) was recorded with resolver dcCliSurfaceConsolidation - the CLI consolidation task, carried over from the previous record call - and the guard passed it. dcCliSurfaceConsolidation cannot resolve a unit-identity defect and its DoD never mentions it. Caught by reading, not by any control. INVALIDATION - what it passed while broken: every `issues` PASS since the guard was written, over a corpus now at 329 issues. NOT RE-CHECKED. The exposure is not cosmetic: `keel whats-next` ranks the frontier partly by the severity of issues a task resolves, so a mis-pointed edge inflates a task's apparent priority and lets the real issue sit behind work that will not close it. WHY IT IS HARD, stated so the fix is not assumed cheap: issue322 points at that SAME task deliberately and says so in its own text, because the CLI consolidation genuinely is its resolver. So the distinguishing signal between a correct and an incorrect edge is PROSE, and a guard that fired on prose similarity would be wrong about half the time. The defensible shapes are (a) require the resolving task's DoD to NAME the issue id - a convention change with a migration cost across 329 issues, or (b) report a WARNING rather than a violation where the resolver's text does not mention the issue, accepting noise in exchange for visibility. Neither is a one-line fix, which is why this is registered and carried rather than fixed in the turn that found it.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcControlDefectsAreFixed to issue333;

    part issue334 : Issue {
        :>> id = "85676c61-4aae-44aa-9b2c-126a4f9ac785";
        :>> title = "The turn gate is fail-open - the Stop hook ALLOWS on its own error, so the control against a rogue turn holds only when nothing goes wrong";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "PANEL FINDING, Critical (rigorTopologyCritique). The Stop hook - the turn gate, the control the human relies on to stop a rogue turn - is fail-OPEN. On its own error it prints a loud warning and ALLOWS. So do post-edit, subagent-stop and the CI gate. Of 12 enforcement points in keel hardening's own table, TWO are fail-closed: pre-write and the pre-commit git hook. On harness timeout every hook resolves to ALLOW. WHY THIS IS THE ANSWER TO 'WE STOP AI AGENTS FROM GOING ROGUE SOMETIMES'. The gate is real and it works when nothing goes wrong. Its failure mode is permission. A slow guard, a locked binary, a crashed validate - the conditions of a session under load, which is when a rogue turn is most likely - all end the turn with a green light. Authority that evaporates under load is not authority. WHAT IS AND IS NOT FIXABLE. The harness's timeout-resolves-to-allow is the harness's and stays a recorded residual (the fire-ledger detects an expected fire that did not happen). The hook's OWN error path is keel's choice, and it currently chooses to allow. That is the fix. RESOLVER: dcTurnGateFailsClosed. Deny on error, the way pre-commit already does.";
        :>> discoveredInField = false;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcTurnGateFailsClosed to issue334;

    part issue335 : Issue {
        :>> id = "5992bb60-7227-4a20-ac22-82dd842ee652";
        :>> title = "retro_backlog is vocabulary-keyed and satisfied by any task name, so retro findings are never examined";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "CONTROL DEFECT, direction UNDER-REPORTING (D0278) - the fourth found today and the one behind the other three going unrouted. retro_backlog (guards.rs:1601-1628) fires only when a retro's text contains the literal tokens AVOIDABLE-ISSUE, AVOIDABLE ISSUE or LESSON:. It is then satisfied by names_tracked_item, which returns true if ANY dc-prefixed name appears anywhere in the file - and every sprint file names the task it just delivered. The check is satisfied by construction. OBSERVED: every retro written today said FINDING. None matched. Three control defects, two migration-path findings and a keystone-coverage gap went into retro procedureText, the guard examined none of them, and D0131 - retro findings become tracked items - held in prose only. The guard also accepts its `changed` parameter and discards it (`let _ = changed`), which is the tell: it was designed to ask whether THIS COMMIT adds a tracked item and never did. INVALIDATION - what it passed while broken: every retro since it was written, 500+ sprints. NOT RE-CHECKED; a sweep for analyze-method gates whose commit added no Issue or task is possible from git and is the honest follow-up. It is also a WARNING, not a violation, so even a match blocks nothing. RESOLVER: dcRetroFindingsRoute.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcRetroFindingsRoute to issue335;

    part issue336 : Issue {
        :>> id = "f39529df-768e-493d-926a-2f8c861c3de1";
        :>> title = "A process is a document with an optional guard - steps bind to no predicate, and analyses live in String fields no control can read";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "PANEL FINDING, High, two critiques with one root (rigorProcessAuthorityCritique, rigorFactLifecycleCritique). A process in keel is a document with an optional guard, not a machine. MEASURED from keel hardening: 37 processes, 16 assert at least one guard, 10 are declared human-judgment, 11 are UNDECLARED with no guard and no reason - and five of those eleven were authored in the last two days, so authoring is outrunning enforcement and only the author's memory converts one into the other. 'Enforced' means the process asserts a guard SOMEWHERE, not that each step has a check: a ProcessStep is an actionText paragraph with no binding to any predicate, so a process cannot refuse to advance past a skipped step. Only agile-workflow's ceremony steps have per-step gates, which is the one place `keel advance --to` can refuse. THE SAME ROOT UNDERNEATH THE FACT-SLIPPING. Everything analytical in the model is a String field - rationale, consequences, description, procedureText, actionText - and no control can read a String for meaning. A Decision's clauses are prose, so partial delivery is invisible. A retro's findings are prose, so routing is a grep. A resolver's fit is prose. The human's phrase was exact: analyses of facts slip - not out of the repository, where they are all committed, but out of the ENFORCEABLE model into fields only a reader can check. THIS IS WHY THE PROCESS AUTHORITY IS WEAK: a well-written custom process ships as a unit, lands with its skill, and is followed exactly as far as the reader chooses. The process-enforcement contract's own comment names the missing mechanism - 'the constraint IS checkable and no guard was written' - and cannot make anyone write one. RESOLVER: dcProcessStepsBindToChecks - a PROPOSAL to the human, because both halves are schema questions under the frozen core.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcProcessStepsBindToChecks to issue336;

    part issue337 : Issue {
        :>> id = "2f7526e2-db6d-4c6a-bcea-eb10267e2846";
        :>> title = "The session's actor never reaches the hooks, so hook-run guards attribute every edit to the machine binding";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "PANEL FINDING, High (rigorSelfReferenceCritique), and a CORRECTION of issue332. issue332 says the ownership guard reads the machine's bound actor rather than the session's. Read at source, that is wrong: actor::resolve (actor.rs:97-112) prefers explicit, then KEEL_ACTOR, then the binding, and ownership calls it (guards.rs:2498). The guard is correct. THE ACTUAL DEFECT: the Stop hook is spawned by the harness in an environment where KEEL_ACTOR was never set - it is exported per-command inside the session's shell - so resolve has nothing to prefer and falls through to the machine binding. This machine is bound to claudeFable5; this session is claudeOpus5. Result: 22 edits attributed to the wrong actor and 12 items the session owns flagged as non-owner edits, burying the 10 real ones. WHY THE CORRECTION MATTERS beyond accuracy: issue332 gave 'guard source under the keystone lock' as the reason for carrying rather than fixing. actor.rs and the hook plumbing are NOT on D0209 clause 2's surface. The reason was wrong, so the carry was unjustified - the process I wrote today names that as the unacceptable case. RESOLVER: dcSessionActorReachesHooks - a session-scoped binding, keyed by the session id the hook payload already carries, that resolve reads between KEEL_ACTOR and the machine binding.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSessionActorReachesHooks to issue337;

    // issue337 CORRECTS issue332: the guard's predicate is right, the session's identity never
    // reaches the hook. issue332 stays as the record of what was believed and why it was wrong.
    #Supersede dependency from issue337 to issue332;
    // ...and the fix that resolved issue337's correct diagnosis resolves issue332's wrong one: the
    // defect they both point at is gone. issue332's text stays as the record of what was believed.
    #Resolves dependency from dcSessionActorReachesHooks to issue332;

    // ── issue330 resolved: the owner's word, and the mechanism that turned out to already exist ──
    verification issue330Accept : Test {
        :>> id = "e2b14c07-58d9-4f36-a0c1-71f8b2d4e953";
        :>> title = "The owner authorises the ten renderer edits (human word, recorded verbatim)";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz, 2026-09-01, in response to the issue330 decision page listing the ten exact edits and three options. Their words, verbatim and in full: 'accept. continue'. READ AS: the owner authorises the ten renderer edits to their Viewpoints and directs the work to proceed - the page's recommended option was that they make the edits themselves, and 'continue' says they want them made. WHAT THIS DID NOT NEED TO DO, found after they answered: D0108's own sanctioned exemption already covered this case. The ownership guard suspends for a commit that co-commits a migration transform under .engine/tools/migrations/, because D0067 REQUIRES bulk transforms that cross every ownership boundary at once - and a mechanical rename of 86 call sites across 31 files is exactly that shape. The first attempt had done the rename with a scratchpad script, which is why the exemption did not apply and the human was asked. Their authorisation is recorded because it was given; the mechanism that lands the change is the committed transform, which is what D0067 asked for all along.";
    }
    part issue330AcceptR1 : TestResult { :>> id = "f3c25d18-69ea-4047-b1d2-82a9c3e5fa64"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3bb0125"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "wweatherholtz"; }
    #Verify dependency from issue330Accept to issue330;

    part issue338 : Issue {
        :>> id = "e4f198cb-0b4e-4355-a6cf-0935ca64b800";
        :>> title = "D0273 promised fewer than 25 verbs and the lens collapse yields 66 - the criterion was never derived from the Decision's own measurements";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "D0273's consequences state that the visible verb count falls BELOW 25. The lens collapse landed at 66: 100 dispatch arms, minus 35 lens verbs moved into the `keel show` router, plus the router itself. Read back from `keel hardening` helpCoverage.dispatched, not estimated. THE DEFECT IS IN HOW THE CRITERION WAS WRITTEN, not in the work. D0273's own context section measured '43 read-only lenses of roughly 111 dispatch arms', which already implies the post-collapse count is near 66 - the arithmetic was available in the same paragraph and was not done. So an accepted Decision carries a verification criterion that its own numbers refute, and had nobody checked, the collapse would have been recorded as meeting it. Found by the person doing the work reading the number back; no control compares a Decision's stated criterion against its stated measurements, and none plausibly could, since both are prose. WHAT THIS IS NOT: a reason to redo the collapse. The lens family - the thing the human named, 'variations of an idea cemented as separate cli hooks' - is collapsed and byte-identical. The gate, record and orient/surface families were never measured or argued in D0273, so reaching 25 means either collapsing them too (a second breaking change) or amending the Decision to state what was achieved. RESOLVER: dcRemainingCliFamilies - measure the remaining families and put the choice to the human as a fork. Sprint 512's retro recorded the generalisable rule: a numeric criterion in a Decision must be DERIVED from the numbers already in that Decision, or it is a wish with a number attached.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcRemainingCliFamilies to issue338;

    part issue339 : Issue {
        :>> id = "69935de0-41da-4cc3-9066-641ef6893e23";
        :>> title = "A test exported from the live repository on every run, bumping the real install record";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "A TEST WROTE TO THE LIVE WORKING TREE ON EVERY RUN. adoption_check_can_fail.rs:54 ran `keel process export intake` with cwd None; `export` locates the repository root and, per D0183 ('the ORIGIN records its own exports too'), writes .engine/contracts/installed-units.toml. So each `cargo test` that found the intake unit's content changed bumped its version in the REAL project - v43 became v44 during today's suite, and the file showed up modified after staging with no human or command having touched it. WHY IT MATTERS beyond hygiene: a version number in a committed contract is a fact consumers compare against (srDriftIsReportedPerUnit). A test silently advancing it means the number stops meaning 'the content changed and someone published' and starts meaning 'the suite ran'. It also produced a phantom out-of-band write that the hook flagged as possibly human-authored, which cost a diagnosis. THE GENERAL SHAPE, and it is worth a sweep rather than one fix: any test that invokes the binary with no cwd inherits cargo's, and every keel command that walks to a repository root will find the live one. The other export-invoking tests (federation, library_publish, capability_refusal_bites) pass an explicit fixture; this one did not. RESOLVER: dcTestsDoNotWriteTheLiveTree.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcTestsDoNotWriteTheLiveTree to issue339;

    part issue340 : Issue {
        :>> id = "1039a969-82ac-494f-acc2-f5ec0ea152e6";
        :>> title = "publish commits a manifest-only change at an unchanged version with a message that reads as a duplicate publish";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> description = "The library log carries two commits titled 'publish library-stewardship v2' (643a188 and 707fefe), and the same for control-propriety v1 and github-intake v1 - three units re-published today at unchanged versions. NOT a version defect: next_unit_version compares file hashes and the files were identical, so holding the version was right. The manifest changed - every unit.toml gained `commands = [...]` when D0252 clause A's command declaration landed - and publish commits whenever the bundle differs, without saying WHAT differed. The issue302 no-op check reads 'did the files change', not 'did the bundle change', so a manifest-only change is neither a no-op nor a version bump and the commit message cannot tell the reader which it was. Low, one-time per manifest schema change, and worth fixing because a library log that says the same version was published twice is a log a consumer learns not to read. RESOLVER: dcPublishSaysWhatMoved.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcPublishSaysWhatMoved to issue340;

    part issue341 : Issue {
        :>> id = "19e1f750-9937-4bcb-8e1e-72001f5fd6b6";
        :>> title = "An accepted Decision's text can change after sign-off and still compute as accepted: acceptance binds to a commit, never to what was accepted";
        :>> createdAt = "2026-09-02";
        :>> createdBy = "claudeOpus5";
        :>> description = "MEASURED: keel-cli/src/govern.rs acceptance_judged_against extracts the acceptance's commit SHA and that is the only binding an acceptance has to what was accepted; a grep of the write path and guards for a body digest or an edited-after-acceptance check finds nothing (checked 2026-09-02 during the agent-corral landscape spike). So the text a human signed at commit X and the text the tree carries at HEAD can differ, and the Decision still computes as accepted. keel refuses this exact drift elsewhere: evidence whose judgedAgainst no longer resolves is suspect, and a confirmation flip on chat words carries a quote receipt (D0198). Consent has no such receipt. WHY IT MATTERS: the landscape names this the rug-pull class (mcp-scan: the tool description you consented to is not the one that executes) and the operand-binding gap (PAuth: OAuth grants an operator, the call carries operands). A model that owns a Decision may edit its fields under D0108 - the owner of most Decisions is the AI that proposed them. FIX: dcAcceptanceBindsToTheText - compare the accepted fields at the acceptance SHA with HEAD, violation on difference, acceptance reported suspect.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAcceptanceBindsToTheText to issue341;

    part issue342 : Issue {
        :>> id = "e2269ef3-26b5-4708-b8aa-83b1c25df255";
        :>> title = "Standing consent is revision-unbound and non-expiring: an auto-acceptance names D0207 but not the revision of the rule or the guard set it was granted under";
        :>> createdAt = "2026-09-02";
        :>> createdBy = "claudeOpus5";
        :>> description = "MEASURED: the auto-accept step in .github/workflows/decision-issue.yml writes the AUTO-ACCEPTED token, the standing words verbatim and the override thread URL; it records neither the commit of the D0207 file nor the guard count or adherence revision at acceptance time (checked 2026-09-02). So a standing consent granted under one rule set silently covers acceptances under any later rule set, and an override on the thread is not bound to the acceptance it overrides. WHY IT MATTERS: every product in the landscape spike that has thought about consent lifecycle has moved AWAY from this shape - Entra Agent ID time-bounds grants and re-opens approval on extension; CapLease shows even single-use tokens replay under agent retry without durable state over the confirmation event; the IETF compliance-receipts draft makes policy_digest and expires_at mandatory. The unauthenticated commenter on the override thread asked precisely this question, which the recording Action correctly refused to record - the refusal was right, the question stands. THIS IS A FORK, not a defect with one fix: the human chose standing consent (D0207) knowingly, and the options trade interruptions against attributability. FIX: dcConsentIsScopedAndExpires writes the costed proposal and puts it to the human.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcConsentIsScopedAndExpires to issue342;

    part issue343 : Issue {
        :>> id = "fbca1f37-5314-44a7-945d-d2e1575248d7";
        :>> title = "A done task's own DoD can be rewritten after its pass and the task stays done: criterion drift is computed for dependencies only, never for the task itself";
        :>> createdAt = "2026-09-02";
        :>> createdBy = "claudeOpus5";
        :>> description = "MEASURED 2026-09-02 by experiment: a done task's own DoD (instanceAwareDoD) was widened with an unverified claim, keel orient was run, the task appeared in no suspect entry and remained done; the edit was reverted. By reading: orient.rs criterion_suspects iterates data.deps only - a dependency's criterion changing since the verified commit makes the dependent suspect, the task's own criterion changing does not. proactive.rs names own-criterion drift as a post-edit ADVISORY, non-blocking, so nothing computed carries it forward. WHY IT MATTERS: the human's direction is trust-but-verify with the DoD as the gold standard; a standard the owner may rewrite after the pass, with the pass still standing, is not a standard. Under D0108 the owner of most tasks is the AI that will pass them. Same class as issue341 (acceptance binds to a commit, not to the accepted text), for the DoD instead of the Decision. FIX: dcOwnDoDDriftIsSuspect - compare own criterion at judgedAgainst with HEAD in criterion_suspects.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcOwnDoDDriftIsSuspect to issue343;

    part issue344 : Issue {
        :>> id = "6ae2e51b-0677-4198-8a95-cfd0786dedde";
        :>> title = "CLAUDE.md invariant 8 states the CliCommand facts exist; D0271 deferred them and D0273 never picked them up - zero instances in the tree";
        :>> createdAt = "2026-09-02";
        :>> createdBy = "claudeOpus5";
        :>> description = "MEASURED 2026-09-02: grep for CliCommand instances across .engine, .tracking and keel-cli/src finds only the schema (cli.sysml) and two Decisions; zero instances. D0271 says so itself - 'NOT YET AUTHORED: the 90 instances and the two-way guard ... held until the human answers whether the lens family collapses'. The human answered (D0273), the collapse landed (65 commands, 35 lenses, held equal to dispatch by a unit test), and the instances were never authored. CLAUDE.md invariant 8 nevertheless states 'Every command is a CliCommand carrying its family, effect and stability' as a present fact. WHY IT MATTERS: a doc claim ahead of reality is the class D0105 and doc-sync exist to prevent; and these facts are the source the computed control structure needs to tell a write action from feedback. FIX: dcCliCommandFactsAuthored - author the instances from the dispatch, guard both ways, correct the invariant's wording and count in the same commit.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcCliCommandFactsAuthored to issue344;

    part issue345 : Issue {
        :>> id = "00b6c2c9-b173-4ba8-83d3-1ffca59505fc";
        :>> title = "priority-inversion asks the author to 'say so' and reads nothing they could say it in - a deliberate ranking cannot be recorded";
        :>> createdAt = "2026-09-02";
        :>> createdBy = "claudeOpus5";
        :>> description = "MEASURED 2026-09-02: guards.rs priority_inversion emits 'if that is deliberate say so, otherwise reorder' and consumes no edge, marker or field in which it could be said; 30 warnings stood after the human's stated first step (st066) was ranked at the head, and the reason - a recorded Statement - had no way to reach the guard. A warning nobody can discharge trains blindness to the ones they can (the actors guard says this about itself). FIX: dcDeliberateRankIsRecorded - a #PrioritizedBy edge to the ranking Statement or Decision, read by the guard.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcDeliberateRankIsRecorded to issue345;

    part issue346 : Issue {
        :>> id = "cb8dfb2b-2a85-4f17-8add-376d707c5423";
        :>> title = "record decision --from accepted an unknown marker silently and wrote an unmarked Decision - the process-change guard then refused the edit it was meant to authorise";
        :>> createdAt = "2026-09-02";
        :>> createdBy = "claudeOpus5";
        :>> description = "MEASURED 2026-09-02 while recording D0283: the --from file carried marker: prospective-change; the Decision was written with NO marker and no message; the process-change guard then refused the co-committed guards.rs edit for want of a marker. main.rs record-decision recognised only process-change and safety-change and treated anything else as none. D0282 (2026-09-02) landed unmarked the same way and, being accepted history, stays as written. A write path that drops a field silently is the issue118 class on the command line. FIX: dcUnknownMarkerRefused - refuse by name, write nothing; test in the same commit.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcUnknownMarkerRefused to issue346;

    part issue347 : Issue {
        :>> id = "3fe6c2a4-76c5-4ac7-84ff-d0fa786f8aa5";
        :>> title = "Record-time standing consent (D0291) can auto-accept a Decision derived from an untrusted story, laundering D0264's plan-only tier";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while triaging twelve untrusted Statements (public repo, D0264) the same hour D0291 landed. D0264 says an untrusted story may reach work only through a Decision a human accepts; guard 56 checks a Decision is among the story's targets. D0291 makes keel record decision accept every NON-FORK Decision at record time under standing consent. Composed, a non-fork Decision proposed from an untrusted story is accepted by the agent's own record call, with no human act - the guard sees an accepted Decision and is satisfied, and D0264's plan-only tier is laundered through standing consent. Not yet exercised: the twelve stories are triaged and unrouted, and their reporter is the project owner. But the path exists. FIX: dcUntrustedTaintClosure already charters the taint walk from sourceTrust = untrusted through #DerivedFrom/#Implicates; record time must consult it - a Decision derived from an untrusted story is a FORK by construction (the human chooses), never auto-accepted.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcUntrustedTaintClosure to issue347;

    part issue348 : Issue {
        :>> id = "8725fcd4-c00d-43b7-a3f1-8d79ef9af091";
        :>> title = "The generated hooks resolve keel from PATH before the pin, so a project's turns can run a different engine vintage than its gates";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#43 (st086, untrusted, owner's own downstream report). Every hook claude_surface generates opens by taking KEEL_BIN else `command -v keel` - PATH first - while the scaffolded pre-commit hook probes .keel/bin/keel before PATH (D0230). Two resolution orders for one pin: a project pinned on purpose ran its turns on an unreleased PATH build. This is the D0251 skew the pin exists to prevent, at the turn boundary. Resolver: dcHooksResolveThePinFirst.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcHooksResolveThePinFirst to issue348;

    part issue349 : Issue {
        :>> id = "21eaf3df-2c39-45dc-a37b-6b4d81168a63";
        :>> title = "keel migrate silently deletes project-authored sections in engine-shipped contract files";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#44 (st085, untrusted, owner's own report). migrate rewrites .engine/contracts files wholesale; a project's own [exec-summary] section in unit-extras.toml was deleted and the published unit lost its declared payload silently. Adjacent to issue314 (project-owned contracts survive resync), which covered whole files, not sections within an engine file. Resolver: dcMigratePreservesProjectSections.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcMigratePreservesProjectSections to issue349;

    part issue350 : Issue {
        :>> id = "c5f29355-9fc7-46af-8451-35214a53202d";
        :>> title = "adoption-check reports FAIL for every unit, and the reason it prints is a guard PASS line";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#45 (st084, untrusted, owner's own report, se-templates on 0.3.1). Every unit including known-good ones reported FAIL; the printed reason was a guard PASS line. The issue329 class (a substring test on guard text) in adoption_check.rs. A gate that is always red is routed around (D0214). Resolver: dcAdoptionCheckReportsTheRealVerdict.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAdoptionCheckReportsTheRealVerdict to issue350;

    part issue351 : Issue {
        :>> id = "183ecd77-1bcf-4c27-b5dd-94b099ed5e29";
        :>> title = "guard:claude-surface-drift byte-compares output-styles/keel.md against an embedded literal, so every fresh clone on Windows fails it";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#46 (st083, untrusted, owner's own report). sync-claude writes LF; git on Windows checks out CRLF; the guard byte-compares against the embedded literal and fails every fresh Windows clone. Resolver: dcSurfaceDriftIgnoresLineEndings.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSurfaceDriftIgnoresLineEndings to issue351;

    part issue352 : Issue {
        :>> id = "f4161393-22b6-4b8f-85ba-6acaafc57f87";
        :>> title = "A forward-only rule ships upstream's adoption date as a literal, so it is RETROACTIVE for every downstream project that migrates into it";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#47 (st082, untrusted, owner's own report, Penumbra 0.3.0 -> 0.3.1). Two rules carry acceptQuotesDelegatedWords(2026-08-22) / confirmationQuotesOrAttested(2026-08-23): this repo's dates, shipped, so every downstream project is retro-failed from the day it migrates. The general form of issue068. Resolver: dcForwardOnlyCutoffIsTheProjectsOwn.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcForwardOnlyCutoffIsTheProjectsOwn to issue352;

    part issue353 : Issue {
        :>> id = "f1c13472-4838-41f2-a4ef-286903b43c66";
        :>> title = "keel append-result inserts the TestResult INSIDE a multi-line verification block, and the error it causes points at the wrong thing";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#48 (st081, untrusted, owner's own report, Penumbra 0.3.1). The insert point is the line matching `verification <T>DoD : Test {`, correct only when the DoD is a one-liner; a multi-line DoD gets the result inserted inside the block and validate's error names the wrong construct. Resolver: dcAppendResultFindsTheClosingBrace.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAppendResultFindsTheClosingBrace to issue353;

    part issue354 : Issue {
        :>> id = "57a4fd46-ab2e-406c-b3bb-4a848a94a5ba";
        :>> title = "add-task's def lookup breaks on a DoD whose string contains a block-opener brace: the write path scans braces without honouring strings";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-03 while routing the intake: keel add-task refused with 'action def not found: EngineBuild' after a DoD was recorded whose text contained the literal characters of a block opener inside its string (the phrase verification T DoD colon Test followed by an open brace). keel check and keel validate both reported the file clean - the real parser respects strings - but the write path's own scan (write.rs parse_file / action_def_exists) does not, so the def vanished from its view and every subsequent add-task failed. Bisected: removing that one DoD line restored add-task. ESTABLISHED against the source 2026-09-03 (write.rs find_action_def_close): the scan counted every brace character, so a DoD string containing a brace pushed the depth off by one. TWO SYMPTOMS from one scan, depending on file shape: in this repository's backlog the def's close was never found and add-task REFUSED; in a small fixture the scan overshot to the package close and the next task was written at PACKAGE LEVEL, outside its def, silently - the misplacement is the worse symptom and was found only by reproducing on both shapes. The real keel_parser was never at fault (keel check and whats-next saw the def throughout). Same scanner class as GH#48 / issue353 (the result insertion point). Resolver: dcAppendResultFindsTheClosingBrace - one string- and comment-aware scanner for both.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAppendResultFindsTheClosingBrace to issue354;

    part issue355 : Issue {
        :>> id = "a7147f16-6ce5-485d-9a2e-dc79ac5c8d03";
        :>> title = "Deactivating a process turns off its guard but still deploys its SKILL, so an agent is advertised a channel the project switched off";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#49 (st080, untrusted, owner's own report, Penumbra 0.3.1). sync-claude builds the surface from the skills registry and consults activation nowhere; a deactivated process's skill stays deployed and an agent has no way to learn the state. Resolver: dcDeactivatedSkillSaysSo (option B: deploy it stated).";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcDeactivatedSkillSaysSo to issue355;

    part issue356 : Issue {
        :>> id = "07a5529a-e30f-40cd-b84b-19a25ee77386";
        :>> title = "red-yield obligation truncates its captured diagnostic at a fixed 726 chars, mid-token";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#50 (st079, untrusted, owner's own report). The Stop hook's obligation writer cuts description at a fixed width; three records at exactly 726 chars, mid-token. Resolver: dcRedYieldKeepsTheDiagnostic.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcRedYieldKeepsTheDiagnostic to issue356;

    part issue357 : Issue {
        :>> id = "5931f6dc-a850-4883-9279-9279ddb640a7";
        :>> title = "Two builds of the same version silently disagree on the .claude surface; drift report names no binary";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#51 (st078, untrusted, owner's own report). Version alone is not identity for a dirty build; two 0.3.1 binaries on one machine generated different surfaces and the drift report could not say which. Resolver: dcSurfaceStampNamesTheBuild.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcSurfaceStampNamesTheBuild to issue357;

    part issue358 : Issue {
        :>> id = "b51c124b-2c0d-461c-9ae7-b715916648dd";
        :>> title = "Stop-hook advisory: configurable summary (outstanding-decisions count + names) instead of the [oversight] nag";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#52 (st077, untrusted, owner's own report). The per-turn [oversight] advisory reads as noise; the owner wants one line with the outstanding decisions and how to accept. Resolver: dcStopAdvisoryIsOneLine.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcStopAdvisoryIsOneLine to issue358;

    part issue359 : Issue {
        :>> id = "faa82bb8-cc34-48b1-8761-33c7b035f59b";
        :>> title = "keel accept from a TTY writes --note bare, but acceptQuotesDelegatedWords needs a single-quoted span - so the one command only a human may run lands red";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#53 (st076, untrusted, owner's own report, Penumbra 0.3.1). The human ran keel accept six times from their own terminal with a plain-sentence note; all wrote; the guard then reported six violations for want of a quote receipt. The TTY IS the gesture the rule already accepts when cited - the command did not cite it. Resolver: dcTtyAcceptCitesTheGesture.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcTtyAcceptCitesTheGesture to issue359;

    part issue360 : Issue {
        :>> id = "2d871d6a-bc1e-4872-b2f7-7078cc74c001";
        :>> title = "keel land has no evidence the suite ran at HEAD: a sprint landed on guards alone and CI was red for twenty minutes on a test the suite runs locally";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "2026-09-02: sprint 521 (the stpa-diagram process) was landed after validate and guards but WITHOUT the full suite; the suite had last run at sprint 520. CI failed twenty minutes later on adoption_check_can_fail: the shipped process named a tool the scaffold did not ship - a test the suite runs locally in seconds. Nothing refused the land: keel land gates validate and guards, not the deliverable's tests, and no receipt records whether the suite ran on the tree being pushed. CI caught it, and CI is detective (branch protection requires no status check - computed, keel show control-structure). Recorded in the fix commit's message only, until now. FIX: dcLandRequiresASuiteReceipt.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcLandRequiresASuiteReceipt to issue360;

    part issue361 : Issue {
        :>> id = "871a1757-0a84-436d-ac61-99d521b29792";
        :>> title = "Six enforced guards had no catalogue row and nothing checked the catalogue was complete (control added 2026-09-02; Issue recorded late)";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-09-02 while authoring the CLI facts: six enforced guards (manifest-key-portability, control-map-reconciled, sprint-closure, untrusted-routing, control-defect-registry, cli-surface-declared) had no row in .engine/docs/guards.md, the file that calls itself the catalogue - five of them older than a week. doc-guard-count polices the total-count claim and nothing policed the rows. Fixed the same day with the rows and a test (guard_catalogue_tests::every_guard_has_a_catalogue_row) that fails the build on the next omission - the CONTROL half of D0047 - but no Issue was recorded, which is the RECORD half. Recorded now; the resolver is the sprint that fixed it.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcCliCommandFactsAuthored to issue361;

    part issue362 : Issue {
        :>> id = "77dea60d-488b-48ef-97d2-37d3d1669a1e";
        :>> title = "A fresh scaffold ships this repository's standing consent and recording delegations as its own policy";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-09-03 (sprint 527 retro, recorded there as prose and nowhere else): keel init ships attestation-policy.toml with THIS repository's grants in it - standingConsent = d0207 and the two delegatedRecording lines, each quoting the owner's words from August - so a fresh project declares a standing consent and two recording delegations nobody granted it. Inert today only because the scaffold also declares no decider (two absences cancelling). The D0219 class: project data shipped as engine data. FIX: dcScaffoldShipsNoGrants.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcScaffoldShipsNoGrants to issue362;

    part issue363 : Issue {
        :>> id = "409e6d40-d7f2-4620-99de-e4a5137fa41b";
        :>> title = "record decision cannot declare what a Decision supersedes or derives from, so a reversing Decision lands edgeless";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found 2026-09-03 (sprint 526 retro, prose only): D0289 reversed layer 3 of D0178 and landed with no #Supersede edge, no dependency on D0192, no #DerivedFrom - the human asked whether it had to be superseded, which is how the gap was seen. record decision has no way to declare what a Decision supersedes or where it came from; decision-requirement-link warns only when the prose names an item by id. FIX: dcRecordDecisionDeclaresSupersedes.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcRecordDecisionDeclaresSupersedes to issue363;

    part issue364 : Issue {
        :>> id = "f6b55dfb-877f-46c3-819f-53ef57adfbb4";
        :>> title = "retro-backlog under-reports: 'already tracked' is accepted as a justification without any item being checked, so retro findings go untracked";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "CONTROL DEFECT (D0278), direction UNDER. The retro-backlog guard (guards.rs RETRO_NO_ITEM_JUSTIFICATIONS) accepts the phrase 'already tracked' as a justification for a retro that adds no item, without checking that anything tracks the finding. Established against the source, guards.rs:1611 and :1681: the justification match is a substring test and consults no item. INVALIDATES: every retro-backlog PASS on a retro carrying that phrase. Concretely observed this week: sprint 526's retro said the supersede-field gap 'belongs to issue336' (it does not; issue336 is about ProcessStep predicates) and sprint 527's retro said the scaffold-ships-grants finding 'belongs to dcVerificationByAuthority' (that task's DoD does not mention it) - both passed, and both findings were untracked until the human asked. NOT re-checked across earlier retros: a scan for the phrase is the first step of the fix. Registered in control-defects.toml so every retro-backlog verdict carries the note until fixed. FIX: dcRetroAlreadyTrackedMustName. SECOND SHAPE, found by the fix's own arming test: retro_texts split the sprint file on the WORD verification, so any retro whose text contained it was cut in half and never examined - 27 of 510 retro gates in this repository, 20 of them with no justification at all, passed unread. Fixed in the same sprint (blocks start only at a line-leading verification); the 27 are NOT re-checked here and the count is the invalidation.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcRetroAlreadyTrackedMustName to issue364;

    part issue365 : Issue {
        :>> id = "916b17a2-ce1a-4527-b34d-8b0bd7a985d1";
        :>> title = "disableAllHooks in a repo-scope settings file silences every keel hook and the drift guard reports clean";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND while answering st090 (would keel work better as an extension with forced hooks), 2026-09-03. CHECK: grep disableAllHooks over keel-cli/src and .engine/docs/guards.md returns nothing; the Claude Code settings reference documents disableAllHooks with scope 'any file' and hook lists MERGE across scopes, so a project-scope key turns off user-, plugin- and (absent allowManagedHooksOnly) managed-scope hooks too. MECHANISM: claude_surface.rs compares the keel-OWNED entries (hooks by event, outputStyle) and lets user entries survive by design; a foreign top-level key is a surviving user entry, so `'disableAllHooks': true` beside intact hook entries reads as NO DRIFT. CONSEQUENCE: the in-loop tier (post-edit fast gate, stop gate, protected-path check, pre-bash advisory) is silenced by one line the agent may write, with the guard that exists to notice hook-surface tampering reporting clean; CI still re-derives validate and every guard on the tree (the third leg holds), so dishonest state is caught at push, not at the edit - detection is delayed, not lost. This is the same class the landscape spike (D0282) named in Kiro: blocking hooks the controlled party can edit. PREVENTING CHANGE: dcDriftGuardRefusesDisableAllHooks. The wider question - hosting the hooks outside the agent's write path at all - is us090 / dcHooksOutsideTheAgentsWritePath, not this issue.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDriftGuardRefusesDisableAllHooks to issue365;

    part issue366 : Issue {
        :>> id = "0d451672-03a3-4be1-a548-afa0deabca10";
        :>> title = "record decision and add-task wrote unparsable files over a Windows path backslash and reported success";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-03 recording D0296. CHECK: `keel record decision --from` with a context field containing the literal path C:/Program Files/ClaudeCode/managed-settings.json wrote the file, printed 'recorded' and 'accepted at record time', and the next `keel validate .` reported `0296-...sysml:18:2724: invalid escape sequence '/P'` - every guard that reads decisions then FAILED (acceptance-events, charter). MECHANISM: sanitize_field (write.rs) neutralises double quotes and newlines but not backslashes, and the record path writes without tokenizing the result. CONSEQUENCE: a write API whose contract is 'prose verbatim, safely' (issue255) produced an unparsable governance record and reported success; on a Windows host any pasted path does this. Same class as issue354 (structure vs text) on the WRITE side. PREVENTING CHANGE: dcSanitizeMakesBackslashesSafe - escape or replace, and tokenize-before-write so the command refuses instead of reporting a success validate will reverse.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSanitizeMakesBackslashesSafe to issue366;

    part issue367 : Issue {
        :>> id = "cf6322fe-abe5-420a-9169-751a2dfaf38a";
        :>> title = "the ranked recall payload is beaten by grep plus one hop on the linked-record set, and lost two hand-set answers two hops away";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-04 by the corrected harness (issue296) on the first ranker that indexes bodies (issue295). CHECK: recall_bench.py, linked-record set, 50 cases: ranked KG 41/50 in a mean 21 shown rows; naive control 7/50; the SECOND control - the rarest-word grep hits plus everything they link to - 49/50 in a mean 16 UNRANKED candidates. The trivial listing beats the traversal-and-rank on the task the graph exists for, in a smaller set. On the 8-case hand-written set the same ranker scores 5/8 against the previous ranker's 7/8 (measured against a rebuilt shipped binary on the same day): both d0129 questions are lost because d0129 is two hops from every element that carries the words (requirement -> need -> decision) and the one-hop lift cannot reach it. MECHANISM: the walk expands from the 30 best lexical seeds and the lift spreads only one hop from seeds, so a target with none of the query's words competes with 29 lexically-matching seeds for ~20 rows; the control has one rare word, a handful of hits, and lists their links. CONSEQUENCE: the payload the human is handed is worse than what they would get by hand on this task shape, and the graph's traversal is not yet earning its place over grep. PREVENTING CHANGE: recallRanksLinkedRecordsAsWellAsGrepDoes; the harness is the arbiter and the hand set is read, not fitted.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from recallRanksLinkedRecordsAsWellAsGrepDoes to issue367;

    part issue368 : Issue {
        :>> id = "987724d7-5f71-4fc2-9a30-fdb699b812c1";
        :>> title = "two computed views stamped viewStatus twice - a duplicate JSON key on the field that exists to be unambiguous";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-04 while receipting issue285 from a live server: GET /api/computed/whats-next and /api/computed/verification returned a body beginning viewStatus computed, viewStatus computed - the same key twice. MECHANISM: those two views stamp their own status inside computed_binding and the choke point with_status (serve.rs) stamps every cached body again without looking. CONSEQUENCE: duplicate keys are accepted by most JSON parsers and resolved differently by each (first wins, last wins), so the one field the N-C2 requirement says every render path must carry unambiguously was the one field that was ambiguous; on the stale path the inner computed would have hidden the outer stale in a last-wins parser. PREVENTING CHANGE: with_status drops an inner viewStatus and re-stamps with its own verdict; the test with_status_never_duplicates_the_key covers the four body shapes including stale-over-computed. Fixed in the same sprint as dcEveryAdvertisedViewIsBound, which resolves it.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcEveryAdvertisedViewIsBound to issue368;

    part issue369 : Issue {
        :>> id = "84844a68-8160-4b46-8932-fa78108cca90";
        :>> title = "a launcher test fails intermittently under the full suite and passes alone - a control that lies one run in five";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-04, twice in one day during full-suite runs: cargo test --release stopped early with one failure - once uncaptured (397 passed), once named: finish_writes_a_record_carrying_every_declared_field in tests/launcher_controls_actually_fire.rs. The test passes alone and in the following full run. A test that fails one run in five is a control that lies one run in five: a red suite the author re-runs to green trains everyone to distrust red, and a green suite that would have been red on the next run hides the defect it is there to show. MECHANISM: not yet established - the fixture is process-id-unique and the failure is intermittent under parallel test threads, so shared state (the gate finish runs, the ledger it reads, or the working directory) is the first suspect. PREVENTING CHANGE: dcLauncherFinishTestIsDeterministic - capture the failure text with --no-fail-fast, name the cause in the test, prove twenty consecutive runs.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcLauncherFinishTestIsDeterministic to issue369;

    part issue370 : Issue {
        :>> id = "efaafb6e-0fa5-4122-b51b-51812636521a";
        :>> title = "twelve enforced guards had no constraint-def identity while the rules file said every guard has one";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-04 while building the vintage fixture (issue265): guard-constraints.sysml states (D0195 clause 1) that EVERY enforced guard has a constraint-def identity - the model identity the control map, a unit's assert and the K8 capability handshake bind to. Computed from GUARD_NAMES against the file: 60 guards, 48 constraint defs, TWELVE guards without one - evidence-cited, gating-workflow-history, process-applicability, doc-guard-count, manifest-key-portability, control-map-reconciled, sprint-closure, untrusted-routing, control-defect-registry, cli-surface-declared, decision-amends-process, unit-extras-present. Eight predate guard 49; the last three are this week's. CONSEQUENCE: a unit could not claim those guards by assert, and the rule that says every guard has an identity was false on a fifth of them with nothing checking it. PREVENTING CHANGE: the twelve are added under D0302 within dcVintageFixture, the sprint that found them; the durable control is every_guard_has_a_constraint_def_identity in guards.rs, which computes GUARD_NAMES against the file's constraint defs and fails on any guard without one - built in the same sprint, so the file's rule is now held rather than stated.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcVintageFixture to issue370;

    part issue371 : Issue {
        :>> id = "c8ae66e8-f272-4ca6-9a6c-bb6643d550e2";
        :>> title = "seventeen accepted Decisions carried text edited after acceptance - all editorial, none silently: found by the guard on its first run";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-04 by the first run of guard acceptance-binds-to-text (D0308): 17 of 301 accepted Decisions carry decision, rationale or consequences text that differs from the text at the SHA their acceptance binds to. Every one is editorial, read by diff: rollout notes appended to consequences after acceptance (d0092 21a3c5f, d0093 90b29b8, d0095 e33148b, d0222 43a7b10); the acceptance-state sentence replaced or the word PROPOSED changed to ACCEPTED inside the text (d0112 a642269, d0130 7740eb0, d0131 ee2512d, d0132 d6f9da5, d0146 d0147 d0148 a6b7123, d0149 01fce65, d0151 1ba3c5c, d0153 8507239); guard counts renumbered when a guard was removed (d0263 6583e08, d0265 2ca2acb, d0266 429fc87). None changes what was agreed - and that is a judgment, which is why each is RE-BOUND with a recorded acceptance result naming the commit and the change, under the standing consent that covers non-fork acceptances, rather than allowlisted. Ten further Decisions read as drifted on the guard's first draft and were formatting only (indentation between plus-joined string segments) - the comparison now reads string values. PREVENTING CHANGE: dcAcceptanceBindsToTheText - the guard itself, hard, and keel accept --rebind as the only remedy path.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcAcceptanceBindsToTheText to issue371;

    part issue372 : Issue {
        :>> id = "5a332da7-13ca-4bca-9ffb-efdf62bba8bd";
        :>> title = "already tracked was accepted eight times while the tracked item sat below the fold - the retro process flagged the heredoc defect and nothing made the flag escalate";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-05 when the human asked why the heredoc defect was still occurring and whether retros were flagging issues. THEY WERE: dcPreBashSeesHeredocSource was created 2026-09-03 (sprint 531) as the resolver of a recurring heredoc failure, a memory said the same, and every retro afterwards wrote 'no new item - already tracked: dcPreBashSeesHeredocSource' - which the retro-backlog guard accepted, because D0294's fourth shape checks that a justification NAMES an existing item and nothing checks that the named item is being worked. The item had been placed below the High-issue resolvers to quiet a priority-inversion warning, and the defect recurred at least eight times in two days (sprints 540-550), each time caught by a wrong test result or an unparsable file rather than prevented. MECHANISM: 'already tracked' is a valid justification the FIRST time; the second time the same open item is named, the finding has recurred, and an item that keeps absorbing recurrences without moving is a reminder in an item's clothes (D0047) - and no control computed the count. CONSEQUENCE: the process that exists to turn recurring defects into controls accepted eight recurrences as tracked, and the human saw the defect keep happening. PREVENTING CHANGE: guard recurrence-escalates (D0310) - two or more retro citations of an open item is a violation unless the item is in the next three ready items or blocked on a proposed Decision; and the heredoc defect itself is closed by D0309's deny, which is dcPreBashSeesHeredocSource's delivery. Resolver: dcPreBashSeesHeredocSource, the item whose stasis is the defect.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcPreBashSeesHeredocSource to issue372;

    part issue373 : Issue {
        :>> id = "227eb2d8-92df-42fe-804b-32952ba1e49b";
        :>> title = "a fork written without OPTION markers auto-accepts under standing consent - fork detection is lexical and the author holds the lexicon";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-05 by stpa-self run 1 (UCA-R1, ucaRecordForkDisguised): standing-consent auto-acceptance at record time (D0291) decides fork-or-not by ONE lexical shape - deck::fork_options reads OPTION X (label) enumerations - so a Decision that weighs alternatives in prose without that marker auto-accepts on the spot, and a direction the human owns is recorded as accepted by an AI (EHZ3). The author controls the lexicon that decides whether the human is asked. No control detects the disguised shape; D0303's clause detector counts (1)(2) enumerations for a different purpose. PREVENTING CHANGE: dcForkShapeIsDetected - a wider fork detector that holds the Decision proposed and asks the author to write it as a fork or to say it is not one, measured for precision over the accepted corpus first. Constraint CC-R1 (ccRecordForkShape) is bound to this Issue until then.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcForkShapeIsDetected to issue373;

    part issue374 : Issue {
        :>> id = "95eb87b3-0010-487a-94e3-0639f1cda8b9";
        :>> title = "an append-result receipt proves a sentence was typed, not that the run happened - a suite count was written before the suite finished (2026-09-04) and no control saw it";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-05 by stpa-self run 1 (UCA-A1, ucaResultReceiptIsTestimony): `append-result --evidence` satisfies guard 52 (attestation-substance, D0232) with WHATEVER the agent writes after `// RAN:`. The receipt proves a sentence was typed, not that a run happened or finished. It happened in this tree on 2026-09-04: I wrote a suite count into a receipt while the background suite was still running, and only my own read-back caught it - no control did. The pass then computes completion from evidence that did not yet exist (EHZ4). PREVENTING CHANGE: dcExternalFactGate - a result class that closes only on an external fact keel queries (a CI run id for the judged SHA, verified by CI on push), never on the agent's assertion; the fork D0312 decides how far that authority model extends to the other methods. Constraint CC-A1 (ccResultReceiptIsExternal) is bound to this Issue until then.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcExternalFactGate to issue374;

    part issue375 : Issue {
        :>> id = "66f65784-fae8-4e0f-8aab-ada90c5a1d5a";
        :>> title = "a DoD pass is judged against the commit BEFORE the work lands - judgedAgainst names a tree that does not contain what was verified";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> description = "FOUND 2026-09-05 by stpa-self run 1 (UCA-A2, ucaResultBoundToTheWrongTree): the ordinary DoD pass is recorded with `append-result --sha HEAD` while the verified work is still uncommitted, so judgedAgainst names a commit whose tree does not contain the work. Every reader that dereferences the SHA - deliverable-drift suspicion in orient, reverify, the attestation census - reads the claim against a tree where it is false, and drift is measured from the wrong baseline (EHZ4). This is the result-side twin of issue341, which D0308 closed for acceptances with the introducing-commit fallback. PREVENTING CHANGE: dcResultBindsToItsLandingCommit - the same fallback for results, and `append-result` naming the tree the pass will read against when the working tree is dirty. Constraint CC-A2 (ccResultBindsToLandingTree) is bound to this Issue until then.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcResultBindsToItsLandingCommit to issue375;

    part issue376 : Issue {
        :>> id = "f3c5dbfd-2053-4618-b732-b982bfbbdcc7";
        :>> title = "Standing-consent auto-accept quotes this repository's human and ships its grants to every adopter (GH#57)";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#57 (st094, us093; untrusted tier, filed by the declared decider - D0314). The engine's standing-consent path fabricates an attestation in any project but this one. VERIFIED on main: keel record decision builds the AUTO-ACCEPTED note from a string literal of THIS repository's human's words (main.rs cmd_record, the D0291 arm); keel init ships attestation-policy.toml verbatim with standingConsent = d0207 and delegatedRecording = d0192 (the file is not an instance contract); keel migrate resyncs it wholesale (not in PROJECT_OWNED). A fresh project with one declared decider therefore auto-accepts every non-fork Decision in its decider's name, quoting consent that decider never gave - the D0106 / confirmation-authenticity class, produced by the tool. The library unit decision-channel v6 has the same defect in record_decision.sh (--by wweatherholtz, the quote inline), routed to dcLibraryUnitRetirement. Resolver: dcStandingWordsAreProjectLocal.";
        :>> discoveredInField = false;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcStandingWordsAreProjectLocal to issue376;

    part issue377 : Issue {
        :>> id = "5c603b85-3e2e-42c4-b951-57faccbdfc1c";
        :>> title = "migrate's removed-type scanner reads string literals as schema; a quoted Rust path blocks a healthy project (GH#59)";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#59 (st092, us091; untrusted tier, decider-filed). keel migrate refuses a healthy downstream project because a removed type is named in PROSE: types_as (migrate.rs) treats text ending in a colon before the name as a typing colon, so the Rust path separator in ureq::Agent passes, and step_removed_types scans whole lines including quoted string literals; REMOVED_ENUMS uses line.contains the same way. VERIFIED by reading the functions. There is no override surface, and the three workarounds (edit the critic's words, re-add a dead type, skip migrate) each corrupt something. Blast radius: every REMOVED_TYPES name that is an ordinary word (Task, Risk, Role). The reporter implemented and falsified the fix locally against d82c512. Resolver: dcRemovedTypeScannerReadsSchemaNotProse.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcRemovedTypeScannerReadsSchemaNotProse to issue377;

    part issue378 : Issue {
        :>> id = "8e46cf98-63f1-4a6e-b2b3-d0f72825d772";
        :>> title = "pre-commit and Claude hooks probe a cache path init never writes, then fall back to PATH silently (GH#55)";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#55 (st096, us095; untrusted tier, decider-filed). Three surfaces enforce one pin and each resolves the binary differently: keelw and keel init write .keel/bin/<version>/<asset>; PRECOMMIT_HOOK (lib.rs) and claude_surface RESOLUTION_ORDER probe .keel/bin/keel(.exe) - a path nothing writes - and then fall back to PATH with no line saying which binary gated. VERIFIED on main. Measured downstream: the same tree passed under keelw (release) and failed 14 viewpoint-renderer violations under the hook (+dirty PATH build); the mirror case produced a false green. D0316 (this week) fixed the ORDER of the Claude hooks' probes and left the layout mismatch - the hooks now correctly prefer a file init never creates. The K2 comment in the hook says an absent gate must not pass silently; a wrong gate that passes silently is the same failure. Resolver: dcOneBinaryResolutionForEverySurface.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcOneBinaryResolutionForEverySurface to issue378;

    part issue379 : Issue {
        :>> id = "c97accbf-d57b-43bd-936c-3b77b53a5924";
        :>> title = "init seeds the wrapper cache from whatever binary ran it, and keelw never verifies a cache hit (GH#54)";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#54 (st097, us096; untrusted tier, decider-filed). init_wrapper (main.rs) copies std::env::current_exe into .keel/bin/<CARGO_PKG_VERSION>/<asset> unconditionally, so a +dirty local build is filed as the pinned release for the project's life; keelw execs a cache hit without comparing it to keel-wrapper.toml, so the committed release checksum was in the tree and nothing ever checked the cached file against it. VERIFIED by reading both. Measured downstream: two binaries both answering 0.3.1, 900 KB apart, one spelling lenses keel show <lens> and one keel <lens>; the same tree gated clean on one and 14 violations on the other, and the project's CI (which downloads by tag) was the only honest signal. The wrapper's own header says trust-on-first-use is no verification; a seeded cache is trust-on-first-use by another route. Resolver: dcSeededCacheIsVerified.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSeededCacheIsVerified to issue379;

    part issue380 : Issue {
        :>> id = "75f228d9-80d7-4dda-b965-1402898d3872";
        :>> title = "onboard asserts a charter whose Decision the project does not hold; the overwrite behind it is fixed on main but unreleased (GH#56)";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#56 (st095, us094; untrusted tier, decider-filed). A v0.3.1 keel migrate rewrote a downstream project's chartered activation.toml with the engine's own - re-arming two processes the project had declined for stated reasons - and keel onboard then reported the set CHARTERED by d0226, a Decision the project does not hold. Two facts: (1) the overwrite is the issue314 class and is FIXED on main since 6583e08 (2026-08-30: activation.toml is in migrate's PROJECT_OWNED set), which is not an ancestor of v0.3.1 (c4d5dbf, 2026-08-29) - adopters on the release still have it until a release ships the fix; (2) the dangling charter is NOT fixed anywhere: nothing checks that charteredBy resolves in the project's own decisions, so onboard prints a provenance claim it cannot read as fact. The reporter's aside that engine-shipped viewpoint-registry.sysml carries this repository's owner ids is a real observation and is not resolved here. Resolver: dcCharterMustResolve.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcCharterMustResolve to issue380;

    part issue381 : Issue {
        :>> id = "a9a871d6-f20e-4294-8a3d-6d9ba360efd9";
        :>> title = "the published decision-channel unit dies on its first run in a new repo: the keel-standing label is never bootstrapped (GH#58)";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "GH#58 (st093, us092; untrusted tier, decider-filed). keel-lib's published decision-channel unit v6 ships decision-issue.yml, which bootstraps the labels blocks-work and decision, and open_decision_issues.py, which creates the standing thread with a third label keel-standing under check=True - so the channel's first run in any new repository dies inside gh issue create with a CalledProcessError that names no cause. VERIFIED: git ls-tree origin/main in the library shows both files under decision-channel/.github. It is masked by two earlier blockers (org allowlist, cargo build) and never reproduces in the origin repository, where the label exists. This repository deleted the channel in sprint 527 (D0291) and cannot republish the unit from a landed tree (library-stewardship rule 1); the same unit carries GH#57's fabricated-attestation script. Resolver: dcLibraryUnitRetirement - the unit is retired in the library with both reasons and its replacement, rather than patched in place.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcLibraryUnitRetirement to issue381;

    part issue382 : Issue {
        :>> id = "3a1d6e58-1c4b-41b6-ab38-74a64af0ab91";
        :>> title = "The decision report's numbers were typed, not computed - three of five figures wrong within a day, and one claim simply false";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "The standing decision report asserted numbers I typed rather than numbers the engine computed, and they were wrong within a day. Measured 2026-09-06 against the authored CLI facts: the page's diagram said 105 facts / 68 top-level / 34 read-only lenses; the tree says 106 / 69 / 33 - three of five figures wrong, because `keel suite` was added that morning. The page also asserted that the release workflow had not run since the last release and was unverified; `gh run list --workflow=release.yml` shows it ran 2026-08-29, succeeded in 4m56s, and cut that very release. It further priced the full suite at about four minutes, a number nothing records - suite.rs stamps only a wall-clock start, so the receipt has no duration. D0215 (accepted) states that a number the engine can compute may not be typed into the doc surface and that the enforcement is DELETION rather than reconciliation; the report is a doc surface and every number on it was typed. Resolver: dcDecisionReportIsArguable, whose contract requires the builder to emit a facts block and every digit-run in reader prose to trace to it.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDecisionReportIsArguable to issue382;

    part issue383 : Issue {
        :>> id = "515e08ce-6a90-40ca-af8b-c9228230279b";
        :>> title = "The report asked for ratification while framing it as a choice: half the queue already shipped, no panel offered a do-nothing option (recurrence of issue234)";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "The standing decision report asked for ratification while presenting it as a decision. Measured over the five panels published 2026-09-06: 4 of the 8 items already ship in the binary - each Decision's own consequences says so in as many words ('the code ships because ...') - so the only act available to the reader on those four is REVERT, and no panel priced a revert; 0 of 5 panels offered a do-nothing option, so every choice set was a menu of the author's actions; and 2 of 12 option costs carried a quantity, the rest being adjectives. The owner's verdict (st099): 'mostly just administrative justification for the decision ... a decision isn't really to accept A decision or B decision'. THIS IS THE SECOND OCCURRENCE of the class: issue234 (2026-08-23) recorded the same complaint about decision cards, and its fix rendered the Decision's own context and decision text - more of the same prose inside the same frame - which is why it returned on a different surface. Per D0311 a finding named twice ranks High. Resolver: dcDecisionReportIsArguable, which requires a state chip (does this already ship, and what does the revert cost), a mandatory status-quo option, and a quantity in every cost.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDecisionReportIsArguable to issue383;

    part issue384 : Issue {
        :>> id = "e81ecd9f-fb0b-4024-9a9a-4b4dc2f8b5dd";
        :>> title = "migrate printed 40 passing lines and hid the violation that reverted the update";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "keel migrate reported the first 40 lines of a failed gate, which are all PASS, so the violation that caused the revert was invisible - in the printed report AND in the recorded attempt, which stored the same clipped text. Found 2026-09-06 on the 0.4.0 version bump: ten migration tests failed, every printed guard read PASS, and the actual cause (claude-surface-drift: the plugin rendering was stale because the manifest stamps the engine version and the version had just changed) sat below the cut. Diagnosis took four extra passes and a --no-verify apply to see a line the tool already had. This is the red-yield truncation class one command over: a diagnostic clipped by POSITION rather than by RELEVANCE. Fixed here: migrate::gate_failure_lines selects the lines naming a failure, violation, error or refusal, caps at 20 with a count of the rest, and falls back to the tail when nothing names itself; the recorded attempt stores the same selection.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcMigrateReportsTheFailingLines to issue384;

    part issue385 : Issue {
        :>> id = "f865aed2-beb0-43c4-8330-a0988d461a58";
        :>> title = "the release gate has no GitHub token, so the v0.4.0 tag published nothing";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "The release gate is the one gating workflow that does not get a GitHub token, so the release for v0.4.0 gated red on a test that passes everywhere else and published nothing. WHAT HAPPENED: the tag v0.4.0 was pushed at 2c288d8. The release workflow's gate job ran cargo test --workspace --release with no env block. The federation suite's live-upstream claim shells out to gh api against this repository; gh refuses inside GitHub Actions without GH_TOKEN and prints 'To use GitHub CLI in a GitHub Actions workflow, set the GH_TOKEN environment variable'. The test failed 10 passed 1 failed, the gate concluded failure, the build matrix was skipped by needs: gate, and no release object exists for v0.4.0 - gh release view v0.4.0 returns 'release not found'. The same test passes on every push because ci.yml supplies github.token, with a comment explaining that the alternative, declaring the path offline, is strictly weaker. WHY IT SURVIVED: releases are rare. This is the issue260 class one workflow over - guard 51 was built after the release gate had been red for a whole release cycle undetected, and it checks that every gating workflow runs the guard set, not that the two gating workflows agree on the environment they run the same tests in. A test whose precondition is supplied in one workflow and not the other is green in the surface that runs constantly and red only in the surface nobody watches. WHY IT MATTERS BEYOND ONE TAG: the project's own engine pin was already committed as 0.4.0, and keelw resolves that pin to a release asset. A pin naming a version whose release does not exist means a fresh clone of this repository cannot obtain the binary its own gates require. PREVENTING CHANGE: the release gate gets the same token, with the reason recorded beside it as ci.yml does. The deeper control - that two gating workflows running the same command must supply the same environment - is a guard-51 extension and is proposed, not assumed, in the accompanying Decision.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcReleaseGateHasTheToken to issue385;

    part issue386 : Issue {
        :>> id = "aafdd184-2394-40b5-9a9a-c04fbfdb3648";
        :>> title = "keel suite reports fail over a file lock: it rebuilds the binary running it";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "keel suite rebuilds the binary that is running it, so on Windows it reports 'fail - 0 passed, 0 failed' over a file lock and writes that verdict into the receipt. WHAT HAPPENED: run as ./target/release/keel.exe suite . against a tree whose Rust sources had changed, the command shelled out to cargo test --release --no-fail-fast. cargo tried to relink target/release/keel.exe, which was the executing image, and Windows refused: 'error: failed to remove file ... Access is denied. (os error 5)'. No test ever ran. The command printed 'keel suite: fail - 0 passed, 0 failed' and wrote .keel/metrics/suite-receipt.toml over the deliverable fingerprint recording that outcome. Re-running from a copy - cp target/release/keel.exe target/release/keel-suite.exe - works, which is the same remedy CLAUDE.md already documents for keel serve (issue150). WHY IT MATTERS: this is not a false green, the verdict is fail, and D0356 already withdrew the push gate so no push is blocked by it. What is wrong is the KIND of the verdict. A run that never compiled the tests is recorded in the same field, with the same shape, as a run where the code is broken; the receipt claims the deliverable at that fingerprint was measured and found failing when it was not measured at all. The only evidence of the real cause is one line inside a log file the command names but does not read back. An operator who trusts the printed line concludes the code is red. THE FAMILY: issue150 is the same shape one command over - keel serve holds target/release/keel.exe and blocks every rebuild, which is why the handbook says to serve a copy. The generalisation neither has yet: a command that builds the deliverable must not be running from inside the directory the build writes. PREVENTING CHANGE: keel suite handles its own image - runs the tests from a copy, or detects that the executing binary sits in the target directory the build will write to and refuses with that named cause and the remedy, writing no receipt. A receipt is a claim that a measurement happened; a run that could not start has nothing to record.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcSuiteSurvivesItsOwnImage to issue386;

    part issue387 : Issue {
        :>> id = "d39a21e9-34b9-415a-b035-518ddd7557c6";
        :>> title = "release-recorded binds a tag to a Release record by substring-in-title and can name the wrong one";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "release-recorded binds a tag to a Release record by substring-in-title, so it can name the wrong record and did. WHAT HAPPENED: the v0.4.0 Release record was corrected to say it published nothing, and its title ended 'the payload shipped as v0.4.1'. The guard matches a tag to a record with title.contains(tag) and takes the FIRST match, so the tag v0.4.1 matched the v0.4.0 record and the guard warned that 'tag v0.4.1 points at b171cd7 but its Release item records commit 2c288d8 - the recorded release and the shipped tag disagree'. Both records were correct; the matcher was not. It was resolved by rewording the title, which fixes this instance and leaves the rule intact. THE DEFECT IS THE BINDING, NOT THE TITLE: a Release record identifies its tag by prose that happens to contain the tag string. That fails in two directions. A title mentioning any other version misroutes, as here - and a title is exactly where a release NOTE naturally mentions the previous or next version. And the containment is unanchored, so the tag v0.4.1 also matches a record titled v0.4.11, which will arrive on its own with no warning at all: the first matching record wins and the disagreement it reports will be about the wrong pair of commits. WHY IT MATTERS MORE THAN A WARNING: this guard is the one control asserting that what shipped is an authored fact (D0191). A control that can bind the wrong record produces two failure shapes, both bad: a spurious disagreement, which trains the reader to dismiss it, and a spurious AGREEMENT, where a record for one version silently vouches for another version's tag. PREVENTING CHANGE: bind the tag explicitly. A Release record should carry the tag it names as its own field, and the guard should match on that field rather than searching prose; a tag with no record naming it stays the violation it already is. Until then the containment should at least be anchored so one version string cannot match another's prefix.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcReleaseRecordNamesItsTag to issue387;

    part issue388 : Issue {
        :>> id = "44e79606-df93-40ef-8fa8-6adfff739c22";
        :>> title = "a test asserts on the source text of the code it polices, and broke for the second time when that code moved";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "A test in main.rs asserts on the literal SOURCE TEXT of the code it polices, so deleting that code breaks the test rather than the property - and this is the second time the same test has broken that way. WHAT HAPPENED: the_turn_boundary_blocks_only_on_dishonest_state reads its own file with include_str!('main.rs') and asserts that the source contains the string 'return match keel_cli::view::pending_acceptances(root)'. When D0359 removed the Stop hook's outstanding-decisions line, the code that string described stopped existing and the test failed - not because the property it names had been violated, but because the mechanism it had memorised was gone. The property, that only dishonest state can reach the block and that a green turn returns before it, was true throughout. WHY IT IS A RECURRENCE: the test's own doc comment says it replaced a predecessor that failed the same way, in its words 'a test bound to a mechanism rather than to a property', when the D0270 oversight advisory was deleted. Two deletions, two breakages, one file. Each time the fix was to rewrite the assertion; neither time was the pattern itself addressed. WHY IT MATTERS: a test that greps source text is not merely brittle - it inverts what a test is for. It passes when the text is present regardless of whether the behaviour holds, and it fails when the text moves regardless of whether the behaviour broke. It cost a full suite run to discover here, and its failure mode is indistinguishable at a glance from a real regression, which is worse in a codebase where the suite is the thing consulted before a release. PREVENTING CHANGE, and it is NOT what I first said. I claimed in the retro that this could not be mechanically forbidden. It can be detected: a check over the test surface for a test that both reads program source (include_str! of a .rs file) and asserts on substrings of it. What needs deciding is not whether it is detectable but whether the rule should be a refusal or a warning, because some source-reading checks in this repository are legitimate controls - the doc-claim and catalogue tests read files on purpose. The resolver weighs that; a behavioural equivalent for this specific test already exists in stop_says_nothing_about_decisions, which runs the binary against a real scaffold and asserts the empty output.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcTestsBindToPropertiesNotSource to issue388;

    part issue389 : Issue {
        :>> id = "472fdbbe-f5e4-4771-9c3b-352240617bc8";
        :>> title = "the computed-facts script typed the release tag it measures against";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "The computed-facts script TYPED the release tag it measures against, so every 'since the last release' number would have described a superseded release the moment a new one shipped. WHAT HAPPENED: facts.py asked git about one tag BY NAME in seven places - the tag date, the commit count, the day count, and the CI-run filter all named the same literal. While that tag was the newest release the numbers were right. v0.4.1 shipped this morning; the next run of the script would have reported the commits, days and CI runs since a release two versions back, and reported them with a `how` string that reads as authoritative precisely because it names a real git command anyone can run. Caught before publication only because I read the facts table and noticed the tag was stale. WHY IT IS THE DEFECT THE FILE EXISTS TO PREVENT: the script's own docstring says 'Nothing is ever guessed, and no answer is hardcoded', and the whole reason the standing brief is built from it is D0215 - a computable number may not be typed into a doc surface. A typed IDENTIFIER is the same failure one level up: the number is computed honestly, against the wrong thing, and nothing in the output says so. THE FIX: the tag is derived - `git describe --tags --abbrev=0 --match v*` - and emitted as its own fact, so a reader sees which release the numbers are about instead of trusting that the script and the repository agree. Each dependent fact carries the derived tag in its unit. THE CONTROL, and its own first failure: facts.py now refuses to run while its source contains a version literal. This is the one place where reading source text is the right implementation, because the property IS about source text. The first version of that check scanned only the source ABOVE itself - and the release facts sit below it, so it passed over exactly the defect it was written for. A deliberate probe caught it. The rule is now phrased without naming any version, so the whole file is scanned with nothing to exclude; verified clean, then failing on an injected literal, then clean again.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcFactsNameNoVersion to issue389;

    part issue390 : Issue {
        :>> id = "ddc0fc38-b148-406c-8170-883b9dc7db28";
        :>> title = "an exhibit legend overprinted itself and a note was cut mid-word, both silently";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "Two exhibit builders lost text without saying so: a legend overprinted itself and a note was cut mid-word at the canvas edge. WHAT HAPPENED, both found by rendering the standing brief and looking at it rather than by any check: (1) The stacked-bar legend placed its entries at even intervals - W divided by the number of segments - which assumes every label is the same width. The first real label set was not: 'already running, needs your signature' ran under its own count and under the swatch beside it, so two of the four categories were unreadable. (2) A logic exhibit's note is one SVG <text> element. SVG does not wrap, and a line longer than the viewBox is simply not drawn past the edge - no error, no ellipsis, no visual cue. The note ended '...rather than to a list someone m' and a reader would take that as the whole sentence. WHY THIS CLASS MATTERS MORE THAN ITS SEVERITY SUGGESTS: the entire argument for building these pages from data is that a figure cannot drift from the facts the way a hand-drawn one does. That guarantee covers the NUMBERS and stops at the LAYOUT - a figure can carry perfectly correct data and still tell the reader something false by hiding half of it. Silent truncation is the worse of the two, because an overlap looks broken and a truncation looks finished. FIXED: the legend advances by each entry's measured width and wraps to a second row rather than overrunning; the note is broken on word boundaries and the figure grows to fit the lines it needs. Both fixes are in the shared modules, so every future brief inherits them. WHAT IS NOT FIXED, and is the reason this is tracked rather than closed silently: nothing CHECKS that a rendered exhibit fits its canvas. The brief contract checks titles, labels and ids; it does not measure. A probe written during this fix - sum each text element's rendered length against the viewBox width - reported five false positives on right-anchored labels, so the naive version of that check is worse than none. Doing it properly means measuring in a browser, which the contract checker does not currently do; whether that is worth its cost is the resolver's question.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcExhibitsDoNotHideText to issue390;

    part issue391 : Issue {
        :>> id = "8d6db4cb-6e4f-4f1d-bf40-ad2a0bd436ca";
        :>> title = "two failing recall cases return an element that answers the question and are scored misses";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> description = "In two of the three hand-set cases still failing, the payload contains an element that ANSWERS the question and is scored a miss, because ground truth names one specific record. WHAT THE HARNESS MEASURES: for each of eight hand-written questions, is the element named as ground truth among the rows the payload shows. Ground truth is fixed before the run and must not be refitted - that rule exists for good reason and is not in question here. WHAT WAS OBSERVED, on the current ranker, both checkable by re-running the two prompts: (1) 'why must we never rebase or force-push?' - ground truth d0129. The payload's FIRST row is the SystemRequirement srDcMergeOnlyIntegration, whose title reads 'Integration preserves commit ancestry - merge only, never squash, rebase' and whose rationale states why: a passing TestResult counts as done only while its judgedAgainst SHA resolves. A model handed that row can answer the question. It is scored 0 for that case, and d0129 sits two hops behind it as requirement --satisfy--> need <--derivedFrom-- decision. (2) 'what is an obligation and what discharges one?' - ground truth d0176. The word 'obligation' appears ONCE in the whole of d0176. The payload returns a UseCase titled 'A supervisor discharges accumulated authority in one batched sitting', the Need behind it, and d0276 (which is about an obligation record) - records that do discuss obligations. The target barely uses the query's vocabulary, so no lexical or graph mechanism reaches it; an alias under .knowledge/ would, and adding one for a test case is precisely the fitting the criterion forbids. WHY THIS IS RECORDED RATHER THAN ACTED ON: the fix that would raise the score is to loosen ground truth, and an agent editing the ground truth of the test it is failing is the worst available move. It is equally wrong to leave the observation in a chat message. So: the number stands as measured (6/8 against a required 7/8, recorded on the DoD result), and the QUESTION - whether 'the answering element' means the governing Decision specifically, or any element that answers the question - is a measurement-definition question for the human. WHAT WOULD SETTLE IT, either way: if the definition stays as it is, then case 2 is unreachable without an alias and the criterion should say so, because a bar no mechanism can clear is not a target but a permanent red. If the definition becomes 'an element that answers', then the two cases need a stated rule for what counts as answering - and that rule must be written before it is applied, not chosen after seeing which rows came back.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcRecallHitDefinitionIsStated to issue391;
}
