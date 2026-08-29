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
}
