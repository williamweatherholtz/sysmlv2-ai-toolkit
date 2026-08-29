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
}
