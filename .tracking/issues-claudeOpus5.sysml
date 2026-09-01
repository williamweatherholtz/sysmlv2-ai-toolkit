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
    #Resolves dependency from dcControlDefectsAreFixed to issue331;

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
}
