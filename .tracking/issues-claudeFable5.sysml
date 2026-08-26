// Per-actor issues records for claudeFable5 (issue210/srDcPerActorWriteTargets): new records
// route here so concurrent contributors never conflict at a shared append anchor.
package IssuesClaudeFable5 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part issue214 : Issue {
        :>> id = "a6db1f9a-75cd-4d20-8ff8-c0030e6ca1fa";
        :>> title = "per-actor routing probe A";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "issue210's live verification probe, kept as the record of its own routing: recorded through keel record issue AFTER the per-actor change and required to land in issues-claudeFable5.sysml with the next globally-allocated number - which it did, making this Issue both the test and the evidence. Resolved by dcPerActorAppendFiles alongside issue210 itself.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcPerActorAppendFiles to issue214;

    part issue215 : Issue {
        :>> id = "67cedf7a-c673-485e-a85d-3e4fc0efc046";
        :>> title = "The allocation layer is stale and actively served: June components, 26 edges for 102 SRs, a retired tool described as the single source of state";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, found independently by all five panelists. The allocIfaceGate confirmation was TRUE at abb033a (June) and is false at HEAD with nothing re-firing the obligation - the K10 reprocess gap applied to the engine's own architecture phase; trace-need serves the stale closure as current truth (EHZ8's open instance). Direction Decision D0194 queued (supersede + charter-chain/registry mechanism + coverage leg).";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAllocationRefresh to issue215;

    part issue216 : Issue {
        :>> id = "93fb02ab-fa22-43df-ab67-5c9d10424323";
        :>> title = "pmMeasureDoD was stamped pass with its promotion-review criterion unmet - a hollow pass on the anti-hollow-pass invariant";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, automotive finding 1 (their flip set). The DoD's text requires 'at least one advisory-to-blocking promotion or non-promotion decision cites >= 3 sprints of this evidence'; no such decision exists, and the ledger was 2 sessions old when R1 was stamped (judgedBy claudeFable5 - my stamp, the exact EHZ4 class). REOPENED honestly: pmMeasureDoDR2 fail at HEAD returns pmMeasure to the frontier, where it waits for the promotion/non-promotion Decision the guided profile's evidence bar (D0180) defines. The machinery clauses all hold; the item is open on the review clause alone.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from pmMeasure to issue216;

    part issue217 : Issue {
        :>> id = "d216018a-8fee-40cf-8b63-cbbdc0af30f0";
        :>> title = "31 accepted decisions carried a PROPOSED header - prose restating the status field, gone stale";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, automotive finding 5 (D0105 dual-truth class; it briefly misled the reviewer). FIXED in-sprint: keel accept now rewrites the scaffolded header on acceptance, and the 31 existing files were swept (the header now points at the status field instead of restating it).";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from storyPanelRound1 to issue217;

    part issue218 : Issue {
        :>> id = "d7736a07-a61a-4242-aa91-039e241368c7";
        :>> title = "The code registry omitted the human channel, the K15 backstop, the git spawn point, the hook dispatch, and the run lifecycle";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1 (SE finding 3, ARCH finding 6, AERO finding 4): against the registry's own criterion - units whose failure would matter - the AI write channel was risk-classified while serve.rs (human-judgment endpoints), history.rs (audit-history), gitx.rs, cmd_hook, and launcher.rs were not. FIXED in-sprint: five CodeElements added with riskClass/invariantSafety/stpaRole judgments (registry 12 -> 17).";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from storyPanelRound1 to issue218;

    part issue219 : Issue {
        :>> id = "f0a76d4a-0a1c-4df6-9674-37714ff0e106";
        :>> title = "The hook watchdog exited 0 with no ledger line - the one fail-SILENT branch inside the instrumented layer";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, robotics finding 4: a wedged hook vanished without a trace, invisible to D0180's single instrumentation path and to K2's fail-loud inventory. FIXED in-sprint: the watchdog appends event=hook-watchdog-timeout before its mandated exit 0; declared in control-events.toml; guard 45's two-way diff covers it.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from storyPanelRound1 to issue219;

    part issue220 : Issue {
        :>> id = "0f6bddc4-7bb5-42b2-912e-459c5ce07d13";
        :>> title = "view.rs is a flagged god-module with no chartered follow-through";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, architecture finding 3: 8159 lines, ~235 definitions, depended on by guards/orient/claims/arch, marked #Restructure by the registry (Ca=4, I=0.20) with no backlog item or accept-risk Decision - a standing flag with no resolving edge, the untriaged-flag failure the engine forbids for Issues. Chartered: dcViewRsRestructure (split along lens seams, or record the accept-risk Decision).";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcViewRsRestructure to issue220;

    part issue221 : Issue {
        :>> id = "d0c6bfe8-365e-4f80-b88a-b9d3069b4eb4";
        :>> title = "EHZ9's controls trace to no standing requirement - removing the release gate would violate nothing recorded";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R2, aerospace finding 1 (MINOR, unanimous-accept residue): the ungated-outward-publish hazard's controls (release gate, pre-push check, land's merged-tree gate) have no invariant/SR anchor - the K-set has no outward-publish member. Resolver: D0195 clause 8 (added round 2) charters the SR anchor under dcControlMapView's delivery.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcControlMapView to issue221;

    part issue222 : Issue {
        :>> id = "a33e001f-489b-4a17-a3ba-b71705c4d9b5";
        :>> title = "GATE_ORDER hand-duplicated the declared ceremony successions - two homes for one fact";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R2, robotics finding 2 (MINOR): guards.rs GATE_ORDER is a compiled constant duplicating the succession chain declared in .engine/workflows/delivery.sysml; post-ingestion both live in the model and a CR editing the declared order would leave the guard silently enforcing the old one. FIXED in-sprint: gate_order_matches_the_declared_successions parses the declared chain at compile time and fails the build on divergence.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from storyPanelRound1 to issue222;

    part issue223 : Issue {
        :>> id = "77e4e057-9c16-4e55-8d37-e699c8a057af";
        :>> title = "The deck presents a fork Decision as a bare Sign card, soliciting an unbindable acceptance";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "d0192 enumerates OPTION A and OPTION B and states 'accepting this Decision means choosing ONE'; the deck rendered it as a single Sign button. wweatherholtz tapped it (inbox row uid 5b1d2b0e, At 2026-08-22T06:54:40Z, deck HEAD 3c9645e) and the tap could not be recorded - the gesture was solicited and then wasted, exactly the authoring-friction class D0054 names. The control (D0047): the deck detects OPTION-enumerating proposed decisions and renders one sign button per option, stamping the chosen option into the note so the recorded acceptance carries the choice.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcDeckForkOptions to issue223;

    part issue224 : Issue {
        :>> id = "1a148a03-fc88-4705-8bc9-f36c9d6a9477";
        :>> title = "The recording delegation covers acceptances only while the practice also records confirmation flips";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "srK06a completeness critique (srK06aCompl1, sprint 421): D0192 OPTION A declared the delegation and its quote-substance check for decisionAcceptance, but the identical practice - AI records the human's judgment on their verbatim words - also flips method=confirmation DoD results (sprint 420's dcHumanChannelDelegationRuleDoD flip is the live example), and no [confirmationRecord] policy class or substance demand exists there. Same asymmetry D0192 closed, one level down. Design note: a DoD result carries no note field, so where the quote lives is a design question, not a copy of the acceptance check.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcConfirmationRecordDelegation to issue224;

    part issue225 : Issue {
        :>> id = "9000ce80-8947-4391-9976-c4f5d76dcc0f";
        :>> title = "Parser proptest identifier strategy generates reserved words - latent CI flake";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "The sprint-421 push went CI-red on prop_part_string_attr: the [a-zA-Z_][a-zA-Z0-9_]{0,31} strategy generated 'doc', which the lexer keywords and the parser correctly refuses as a bare identifier. No commit in the push touched the parser - the random seed found a latent generator defect. Control (D0047): the lexer exposes is_reserved_word (single source: the keyword match itself) and the strategy filters through it, so the generator cannot drift from the grammar; a deterministic test pins the refusal.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcProptestReservedWords to issue225;

    part issue226 : Issue {
        :>> id = "b9aa7ad7-e1b0-40cb-9462-d629c5863ad8";
        :>> title = "The enforcement contract claimed D0108's ownership rule was unchecked - stale since sprint 293";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "process-enforcement.toml [distributedCollaboration] said 'Nothing checks it today. This is the entry I would most expect a reviewer to act on' - but the ownership guard has existed since sprint 293 (1b2fd4f, 2026-08-15): staged-diff vs HEAD, item-level field comparison against the bound actor, D0067 migration keystone exemption. A dual truth (D0018): the hardening view faithfully reported a stale declaration, and sprint 421 chartered dcOwnershipDiffGuard to build a guard that already existed. The contract text was never swept when the guard landed - a doc-sync miss on a CONTRACT surface. Control: the unit now asserts the constraint (D0139(D) unit-ownership), so the enforcement map computes from the model instead of restating it in prose.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcOwnershipDiffGuard to issue226;

    part issue227 : Issue {
        :>> id = "275707d4-a2be-4827-a539-98dc7ae8a677";
        :>> title = "authority-queue and sitting-coverage disagree about the sitting-review obligation";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Process-value panel (human-factors lens), verified live by the convener: authority-queue reports '279 sprint(s) awaiting a sitting review, waitingDays 65, escalated true' while sitting-coverage reports due 6 with 313 grandfathered under the D0155 human attestation. Two computed views, one obligation, two answers - the exact dual-computation class D0018 forbids, in the pair of views the human uses to size their own duty.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSittingViewParity to issue227;

    part issue228 : Issue {
        :>> id = "66136aa6-703d-48ac-91db-602246631061";
        :>> title = "Batch acknowledgments count as read reviews; the human override rate is unmeasured; escalation is toothless";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Process-value panel, 4 of 9 lenses convergent. VERIFIED: 65 of 105 covered sittings (62 percent) were signed in two batch chat utterances against deck cards rendering a title plus the literal string 'unreviewed'; zero reject/needs-work sitting verdicts exist in the record; the 279-sprint queue has been escalated 65 days while ~145 sprints landed on top of it. sitting-coverage counts a batch acknowledgment identically to a read review - the same one-number conflation the EXAMINED/EXERCISED split exists to prevent. The record is honest about what happened (verbatim batch words); the COUNT is not.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcOversightRedesign to issue228;

    part issue229 : Issue {
        :>> id = "d182fb95-f1e5-470a-9313-d703d2f9fe7b";
        :>> title = "Claims gate selection but not landing, and claimedAt is authored where nothing else is trusted";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Process-value panel (multi-agent lens), two concrete interleavings: (1) A claims, goes stale, B claims and works, A resumes and LANDS - no holdership re-check exists at land, so both land green and duplication surfaces only on contradiction; (2) holdership = earliest un-expired claimedAt, and claimedAt is claimer-authored, never validated against the introducing commit's ancestry - a backdated claim steals holdership deterministically on every clone. The repo's own doctrine (D0013: git ancestry is the clock) supplies the fix and is not applied to claims.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcClaimLandCheck to issue229;

    part issue230 : Issue {
        :>> id = "4bfd8897-7f1c-4358-87bf-2776c1b97ba4";
        :>> title = "Zero advisory blocks is uninformative: the ledger cannot distinguish deterrence from dead weight";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Process-value panel - empiricist, skeptic and AI-safety lenses independently: the fire-ledger records fires (1,164 pre-bash, 0 blocks) but not advisory content or whether the flagged action changed, so D0197's declared revisit condition is structurally untriggerable and the 'advisory tiers deter' hypothesis has no counterfactual arm. A monitor whose catches cannot be counted is indistinguishable from a blind one.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcAdvisoryInstrumentation to issue230;

    part issue231 : Issue {
        :>> id = "982b984a-105c-41e7-bfea-fec614644769";
        :>> title = "The parser TCB has no ratchet: the skipped-statement count can grow silently";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Process-value panel (formal-methods lens). The parser is the trusted computing base of every view and guard - a statement class it skips is invisible to the whole enforcement stack (incident lineage: issue102 56 absent use-case usages, issue109 dangling endpoints only the kernel saw, issue119 edge-kind drift both directions, issue221-era bare-dependency skip). parser-coverage WARNS at 7 skipped today, but nothing goes red if the count grows, and no property-based round-trip pins skip-freedom over generated corpora.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcParserCoverageRatchet to issue231;

    part issue232 : Issue {
        :>> id = "9ddea0e0-6144-4ea2-be50-b51f0f83497f";
        :>> title = "claim-ancestry's verdict depended on clone depth - CI-red on its first push";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Guard 47 resolved a claim's introducing commit with git log -S; on CI's default depth-1 checkout the oldest VISIBLE commit is the clone boundary, so the guard flagged claudeOpus5's honest 2026-08-15 claim as backdated (intro misread as 2026-08-22) while the full-history local clone passed - a depth-dependent verdict, the exact K15 machine-dependence the guard cites D0013 to prevent, caught by CI within minutes of landing. Control: the guard SKIPS LOUDLY on shallow repos (absence of history is a state, not a violation - the enrollment-binding precedent) and ci.yml checks out fetch-depth 0 so the real check runs where it matters.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcClaimLandCheck to issue232;

    part issue233 : Issue {
        :>> id = "16880abf-9d8b-43fa-b453-d092aaa1ae3d";
        :>> title = "Fork cards highlighted every option as selected - eleven contradictory taps on one decision";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> description = "Their report, 2026-08-23: 'deck shows multiple sign options as being selected.' Root cause: the accept-styling CSS keyed on data-v=accept, shared by EVERY option button on a fork card, so any accepted fork lit all its options; the human fought the UI and the inbox recorded eleven d0201 rows with contradictory options (A/B/C within seconds) - none of which can bind. The unbindable-gesture class (issue223) recurring one level down: the tap bound correctly in DATA (data-opt-chosen) but lied in PIXELS.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDeckJudgeableCards to issue233;

    part issue234 : Issue {
        :>> id = "8cad950c-c902-44a2-b2ba-a857c3539872";
        :>> title = "Decision cards carried no deciding context - a signature could only be a trust gesture";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> description = "Their words, 2026-08-23: 'there is not sufficient context to make these decisions generally... it's not even clear what's really being proposed because there are so many references and obscure change or value implications.' The acceptance card rendered title + 'proposed - needs your signature' and nothing else; the D0200 judgeable-cards finding applied to decisions as much as sittings, and the panel's rubber-stamp evidence was partly THIS.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDeckJudgeableCards to issue234;

    part issue235 : Issue {
        :>> id = "d3ea8a30-3790-4151-92ca-85f0c455bb43";
        :>> title = "The substance guard rejects a one-letter fork answer - the channel's own gesture wasn't recognized";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "D0208 redTeamScope was answered 'A' on GitHub issue #3; the recorder ran keel accept (note: OPTION A - their words verbatim 'A' (GitHub comment <url>...)) but confirmation-authenticity FAILED: 'A' is no >=10-char quoted span and the scanner's gesture keywords were only deck/console. A GitHub comment is 2FA-authenticated, timestamped, immutably logged - the D0205/D0201-B conclusion that it subsumes the device HMAC - so a cited GitHub comment IS a valid channel gesture. The guard was refusing the very channel this project just built. Fix: quotes_conversational_words recognizes a github comment / github.com citation as a gesture; nothing pushed, the human's answer is safe in the thread and re-records via the sweeper once the fix lands.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcGithubGestureSubstance to issue235;

    part issue236 : Issue {
        :>> id = "3862f78b-2c11-46e2-868a-69454d6792fe";
        :>> title = "An agent can disarm a blocking guard rule with no signed decision - the monitor was modifiable by the monitored";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "D0208 control evaluation (OPTION A, scripted probes in a throwaway worktree), the panel-predicted finding made concrete: committing a flip of confirmationAuthenticityRule from RuleSeverity::blocking to warning in .engine/rules/rules.sysml passed process-change (scanned 0 - the rules dir was outside is_process_def), engine-lint, and the whole gate. An agent could silently downgrade any control it enforces. The AI-safety lens named exactly this ('the agent builds its own guards... the monitor is modifiable by the monitored'); the evaluation the human authorized found it on the first pass. Control: .engine/rules/ joins the process-change keystone (dcRulesUnderKeystone), so weakening a rule needs a human-signed marked Decision.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcRulesUnderKeystone to issue236;

    part issue237 : Issue {
        :>> id = "61c9980c-962e-4ff3-8159-9ca65d652c1f";
        :>> title = "A dangling judgedAgainst SHA passes the commit gate when the result is unreferenced";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "D0208 control eval: a TestResult with judgedAgainst = 000...0 (a SHA that never resolves) passed validate+guard. The tracked case is caught elsewhere - orient invalidEvidence flags a done task whose evidence SHA dangles, and CI audit-history re-derives per commit - but the commit gate itself does not check SHA resolvability, so an orphan garbage-SHA result lands green. LOW because an unreferenced result computes nothing; recorded so the gap is visible rather than assumed covered.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcDanglingResultAtGate to issue237;

    part issue238 : Issue {
        :>> id = "c1263af4-f7ed-4c81-b28b-9ce399d88a90";
        :>> title = "A decision accepted before its push bypasses the githubChannel - the human loses the override surface";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> description = "Caught by the human: I told them D0209 was overridable 'on its GitHub issue' but no issue existed. D0205's githubChannel (open_decision_issues.py) opens an override-thread issue only for PROPOSED decisions. When a decision is accepted locally (serve API / keel accept) before it is pushed, it is already 'accepted' the moment the workflow runs, so the proposed-only opener skips it and never creates the override thread. Confirmed: d0205=1 issue, d0206=1, d0207=0, d0208=1, d0209=0 - the two I had accepted locally were exactly the two with no override surface. The channel D0205 built to give the human a standing override could be silently bypassed by my own acceptance path. Manually opened #4 (D0207) and #5 (D0209) as the immediate remediation; the durable control is dcOverrideThreadEveryAccept.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcOverrideThreadEveryAccept to issue238;


    part issue239 : Issue {
        :>> id = "3c604c51-a958-4827-a79b-c9d1a9c22830";
        :>> title = "The honest-surface invariant is bound to the console and unbound on the CLI/JSON authority";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Adversarial panel (9 lenses, 2026-08-24) on the adoption/portability/elicitation brief. The panel's most valuable reframing: three of the four drift mechanisms the brief named are not independent - they are three violations of ONE already-accepted invariant. nConsoleNeverMisleads (N-C2, human-accepted 2026-08-16, 'looks great, proceed') requires that the absence of information always be distinguishable from information ABOUT absence, that a view which could not be computed never render as a negative result, and that stale content never be shown as current. Its ONLY satisfying requirement, srConsoleViewStatusExplicit, is scoped to 'every view the surface renders' - the HTML console. But CLAUDE.md section 1.8/D0093 make CLI/JSON the authority and the automation substrate, and every violation found this session was on that side: a catalogue that denies 12 processes which exist, a wiredness proxy reporting an unreachable hook as armed, a ratio published without its denominator or its own floor caveat. The answer to 'why are reports out of sync with truth' is therefore not 'add a drift control' but 'an invariant you already signed is scoped to the wrong surface'.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcHonestSurfaceEverywhere to issue239;

    part issue240 : Issue {
        :>> id = "7f2e6774-4daf-4656-b80b-eca4076211dd";
        :>> title = "The commit gate was disarmed while three shipped surfaces reported it armed - the predicate is a proxy, not the control";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Found by the panel's mechanisms lens and confirmed directly: core.hooksPath was set to `nul` in .git/config, so git resolved hooks to a nonexistent directory and .githooks/pre-commit, pre-push and pre-merge-commit could not run at all - for an unknown period covering this entire session, during which I repeatedly told the human 'the hook passed'. The substance was in fact checked, because I ran keel validate/guard/gate by hand and CI re-derives on push, but the mechanism I NAMED was dead. Three shipped surfaces asserted otherwise: keel hardening reports the pre-commit point present=true failMode=fail-closed, keel orient printed no hook warning, and keel controls reported uncoveredHazards=[] against ctlCommitGate. Root cause: wiredness is computed as 'git config core.hooksPath returned a non-empty value' - a proxy for reachability rather than reachability. issue116's bypass vector is blocked in the action space and wide open in the detector. hooksPath restored to .githooks; the detector defect is what this issue tracks.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcArmedNotDeclared to issue240;

    part issue241 : Issue {
        :>> id = "d5671140-539e-4605-ace0-aa92b967c79f";
        :>> title = "12 of 23 processes cannot be listed, shown or exported - a must-requirement violation that three critiques passed as green";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "The human asked whether processes are transferable. Measured: 23 process definition files, 11 in the exportable palette; `keel process show intake` and `keel process export intake --out DIR` both exit 2 with 'error: no process intake' while .engine/processes/intake.sysml exists, and `keel process search intake` returns 0 matches. Cause: process_cmd.rs rows() iterates activation::unit_names(), and a unit is formed only when a process declares a guard (activation.rs `if !guards.is_empty()`), so the 12 guard-less processes - intake, definition-of-done, definition-of-ready, adversarial-panel-review, architectural-critique, review-critique, migration, report, indicator, diagram, introduction, obligation-review - are structurally invisible to the portability layer. Those 12 are guard-less by ACCEPTED reasoning (process-enforcement.toml declares them checkable=false with stated reasons), so the exclusion is permanent, not pending. This violates srPortModularProcessUnit, an accepted MUST requirement that a unit carry its definition, deploying skill and rules/guards so it can be moved between projects whole - and it is the un-swept half of issue149, whose disclosure remedy reached activation/activate/deactivate but not the process catalogue. Aggravating: three critiques dated 2026-08-21 passed srPortModularProcessUnit with zero findings, two of them citing a live run of the very command, and tier-satisfaction reports 103/103 verified - a false green inside the verification substrate.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcPaletteIsDeclared to issue241;

    part issue242 : Issue {
        :>> id = "9e95436c-79e7-4194-990c-4b2cb91716e0";
        :>> title = "A core guard can be demoted to inactive by one line in a guard-less process, and the D0209 adherence gate cannot see it";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Found by the panel's operations lens, which reproduced it live: adding `assert constraint probe : staleGateProse;` to the guard-less .engine/processes/report.sysml removed stale-gate-prose from the core (never-deactivatable) guard set, made report [INACTIVE] with 1 guard, and turned the guard's run into 'PASS - 0 scanned, 1 warning' with `keel guard stale-gate-prose` reporting 'NOT ACTIVE ... do NOT block'. The mechanism is guard_state: it returns Core only when NO unit claims the guard, so a process claiming a core guard converts it into that process's switchable property - and the CLI's own remediation text ADVISES the edit ('or give it an `assert constraint` so it becomes switchable'). The same class applies to narrowing activation.toml's active list, which disarms every guard the removed process owns. Both are covered by the commit keystone, but D0209 clause 1 exists precisely because a bypassed hook leaves no trace at commit time - and the hook was in fact bypassed (issue240) - while audit-adherence's signature() reads only .engine/rules/ trees. So the control built this session for the issue236 self-modification class has a hole the class walks straight through.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAdherenceCoversActivation to issue242;

    part issue243 : Issue {
        :>> id = "e930b3da-2faa-437f-a814-24645f807ba4";
        :>> title = "Every new project inherits this repo's adoption declaration and exchange identities from keel init";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "The human asked whether a new project is guided to pick its processes. Worse than un-guided: it inherits someone else's choices. keel init scaffolds the embedded .engine verbatim, so a freshly inited tree's .engine/contracts/activation.toml is BYTE-IDENTICAL to the self-build's (verified by diff) - 11 named active processes plus all 32 viewpoints - and `keel activation` in the new tree reports 'declared manifest: yes' for a declaration that project never made. That falsifies CLAUDE.md's own adoption guarantee, that an absent file or section means everything is active so a project that never adopted a control has not violated it: the file is not absent, it is another project's. The new tree also inherits unit-ids.toml, the self-build's EXCHANGE identity for units it never exported. init already special-cases deliverable-manifest.txt to a starter for exactly this reason, so the shape of the fix exists.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcInitContractReset to issue243;

    part issue244 : Issue {
        :>> id = "fb9cf296-1df6-4486-8bfb-cc98621377ab";
        :>> title = "80 warnings ride under [guard] ALL PASS unread, including a live recorded-release contradiction";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "The panel's reducer named this the drift mechanism with the most evidence, and it is the sharpest answer to the human's question about reports being out of sync with truth. Measured: `keel guard` prints '[guard] ALL PASS' while emitting 80 warnings across 10 guards (actors, ceremony, viewpoint-renderer, manifest-coverage, attestation-substance, decision-requirement-link, sequence-multiplicity, parser-coverage, decision-scaffolding, release-recorded). One of them is itself a shipped, detected, unacted-on report-vs-truth contradiction: 'tag v0.2.0 points at e2c812f but its Release item records commit 425145d - the recorded release and the shipped tag disagree (D0191)'. I read ALL PASS and moved past those warnings repeatedly this session. This is why the panel refused a new drift detector: the engine already detects more than it triages, so another detector makes the felt problem worse. The fix is subtractive - surface the warning population where it is read.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcWarningsAreRead to issue244;

    part issue245 : Issue {
        :>> id = "d5d642d4-9b74-4094-825a-770ecb104925";
        :>> title = "keel's elicitation skill is in no catalogue, transferable by nothing, and authors Needs from unrecorded utterances";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "The human asked whether input is recorded as statements then translated to stories, and said pulling latent needs out of a user is critical for the average user. Two findings, converged on by three lenses. First, the elicitation machinery already EXISTS and is in use: the business-architecture skill runs a design/scoping dialogue Brief->Personas->Needs->UseCases and has produced 11 Briefs, 15 Personas, 69 UseCases and 75 Needs - so a new interview process would be a fourth overlapping surface, not a missing capability. Second, that skill has NO process definition, so it appears in no catalogue (process list, activation, hardening all miss it), is enforced by nothing, and can be exported by nothing - and the process-skill guard is one-directional, requiring every process to have a skill but never every skill to have a process, so no control notices. Compounding: it authors Needs straight out of a conversation whose utterances are never recorded as Statements, which is the mechanical cause of the unsourced population and exactly the D0157 defect (a Need written wider than the demand cannot be checked against what the human actually said). Separately measured: intake has recorded nothing since 2026-08-21 - all 17 Statements are dated 08-20/21 - across four days containing four panels, and the human's own six questions in this session were never intaken.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcElicitationThroughIntake to issue245;

    part issue246 : Issue {
        :>> id = "2ab1381b-ef8b-45ce-b114-96c47ade67ea";
        :>> title = "Counts typed into prose drift from the counts the engine computes, with no control between them";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Named independently by four panel lenses as a distinct drift mechanism, and the brief under review committed it twice itself. Live instances: CLAUDE.md and .engine/docs/guards.md both state '45 forward guards' while keel guard runs 48; the shipped activation.toml header still directs readers to .engine/contracts/process-units.toml, a file D0141 deleted with no tombstone, and keel init copies that ghost into every new project; my own brief said 30 skills where there are 37 and cited GitHub issues #1-#6 where the channel threads are #2-#7. This is distinct from two views disagreeing and from an unverifiable prose assertion: it is a number a reader will trust sitting next to a number the engine can compute, with nothing comparing them. hardening.helpCoverage already does exactly this reconciliation for commands (88/88), so the counter-measure is proven and cheap.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcDocCountsReconciled to issue246;


    part issue247 : Issue {
        :>> id = "3d89e4ae-6ec5-4484-8f58-b485c0dabef9";
        :>> title = "The frontier WIDENED on a model-read error, and the code did the opposite of what its own comment promised";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Found while building dcHonestSurfaceEverywhere (issue239). orient's ready computation filtered out SUPERSEDED tasks (issue100) and tasks BLOCKED on a proposed Decision (issue112), and both call sites read `.unwrap_or_default()`. On an Err that yields an EMPTY filter set, so NOTHING is filtered and every retired task and every human-blocked task RETURNS TO THE FRONTIER. The comment immediately above the superseded line said, verbatim, 'Conservative on error: failing to read the model must not silently make everything ready again' - so the code did precisely what its own comment forbade, and the second site carried the same promise. This is the worst instance of the class because the frontier is AUTO-FOLLOWED (D0052): a transient parse error could schedule work a Decision has explicitly retired. It is also the honest-surface invariant (N-C2) violated on the authority substrate rather than the console - a computation that could not run rendering as a confident, and wider, answer. Compounding it, `keel whats-next` printed nothing and exited 0 whether the frontier was genuinely empty or a filter had failed, so COMPUTED-EMPTY and COULD-NOT-COMPUTE were byte-identical on the one answer the AI follows. FIX: the narrowing filters are extracted into one place that RECORDS failures; a failure empties `ready` (fail closed - an empty frontier plus a stated reason is honest, a superset presented as the answer is not); orient emits an explicit answerStatus of COMPUTED or COULD-NOT-COMPUTE with reasons; whats-next REFUSES with exit 1 on a failure and says COMPUTED-EMPTY out loud when the answer is genuinely empty.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcHonestSurfaceEverywhere to issue247;


    part issue248 : Issue {
        :>> id = "55eebc17-dbe3-440a-98fe-a0d1934d5c3a";
        :>> title = "A unit test asserted a MACHINE property and turned main red for six commits";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "The regression test added in sprint447 for issue240, this_repos_commit_gate_is_armed, asserted that this repository has an ARMED local git hook. That is a property of a MACHINE, not of the repository: a CI checkout configures no core.hooksPath and correctly does not need one, because CI is the hook-INDEPENDENT layer (K15/D0179) that exists precisely so nothing has to assume a hook ran. Every ci run from the issue243 commit onward failed on it - six commits red - while the paired decision-issue workflow kept succeeding, which is why the failures were easy to skim past in a run list showing both. WHAT MAKES THIS WORTH RECORDING RATHER THAN JUST FIXING: my own sprint447 review gate had already named the risk in almost these words - it couples a unit test to machine-local git config, and will fail for a contributor who has not run the setup step - and I wrote that I judged it acceptable and deliberate. The review pass found the defect and the author overrode the reviewer, which is a failure of the review process rather than of the analysis. FIX: assert the CONDITIONAL invariant - if core.hooksPath is configured at all, it must resolve to a directory holding pre-commit - which still catches the exact issue240 state (configured to a null device, unreachable) while an unconfigured checkout is not a violation.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcTestsAssertRepoNotMachine to issue248;


    part issue249 : Issue {
        :>> id = "bb2266b1-9fc8-4096-bcb6-095872fdc97c";
        :>> title = "A git config set inside a probe WORKTREE disarms the parent repo - this is how issue240 happened";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Reproduced ACCIDENTALLY while probing the controls-arming work, which settles issue240's origin. A linked worktree SHARES the common .git/config, so `git -C <worktree> config core.hooksPath nul` does not scope to the worktree - it rewrites the parent repository's config and disarms the real commit gate. That is almost certainly the exact mechanism by which issue240 arose: an earlier red-team probe set hooksPath to a null device to simulate a bypass, and the setting outlived the worktree. The correct probe form is the PER-COMMAND override `git -c core.hooksPath=/dev/null <cmd>`, which the audit-adherence probes used and this one did not. WHAT MATTERS MORE THAN THE MECHANISM: the detection control already existed and worked. The conditional test added for issue248 failed within a minute of the probe, naming the state, so the disarmed gate was caught by a check rather than by luck - the second time that control has fired on a real disarming, both times self-inflicted. This is the D0047 shape working as intended: rather than trying to prevent a probe-methodology mistake by vigilance, DETECT the resulting state regardless of cause. Recorded so the mechanism is written down once instead of being rediscovered a third time.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcArmedNotDeclared to issue249;


    part issue250 : Issue {
        :>> id = "34fb807f-efe4-4d99-893f-781dd5c10231";
        :>> title = "audit-adherence never checked the FIRST commit of a push, so a single-commit push was never checked at all";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Found while building the chartered dcNoFalseCleanRange, and the charter was WRONG about the fix. The visible symptom was that audit-adherence printed 'fewer than 2 commits in range - nothing to compare. Clean.' and exited 0, a false clean inside the gate built to prevent false cleans; the chartered remedy was to make that a usage error. That would have been a serious mistake: CI invokes --since github.event.before, so a SINGLE-COMMIT PUSH lists exactly one commit, and erroring would have failed CI on the most common push there is - the issue089/issue090 trap of a new hard check breaking every normal case on day one. THE REAL DEFECT, one layer down: the comparison seeded its baseline from shas[0] and then compared windows of two, so shas[0] was only ever a BASELINE and was NEVER ITSELF CHECKED. The first commit of every push therefore escaped the gate, and a single-commit push escaped entirely. A weakening introduced in the first commit of a push was invisible to the one control built to catch exactly that class (D0209 clause 1), which had been true since the day it shipped. FIX: seed the baseline from the PARENT of the range's first commit, so every commit in the range including the first is compared; a root commit (no parent) is stated as its own baseline rather than guessed at; and only a genuinely EMPTY range is reported as nothing to do. VERIFIED: a weakening committed as a single commit with the hook bypassed now returns exit 1 naming the rule, where the same probe previously reported Clean; real history passes at both --since HEAD~1 and --since HEAD~20.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcNoFalseCleanRange to issue250;


    part issue251 : Issue {
        :>> id = "401a8361-8e26-444c-84da-3b3933da5553";
        :>> title = "The decision channel could not be adopted: it left its mechanism behind and welded one project identity into it";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Raised by the human asking to activate the GitHub decision channel on asirobots/penumbra, with the constraint: do not force me to be owner of every new project, be modular and careful. FOUR defects, three of which would have broken or misfired on the FIRST adoption rather than degrading. (1) TRANSFER: keel process export decision-channel produced three files - definition, skill, unit.toml - while the five that ARE the channel (two workflows, two scripts, the decider table) stayed home; the guard travelled BY NAME and the machinery not at all. That violates srPortModularProcessUnit, an accepted MUST that a unit move without leaving its enforcement behind, and is the same class as issue241 and issue245. (2) OWNER-AS-PERSON: the issue opener assigned to github.repository_owner, which is not a person - for asirobots/penumbra it is an ORG and gh issue create --assignee <org> fails outright. (3) IDENTITY IN SOURCE: the decider login was a literal in decision-record.yml in three places, so every adopting project had to edit the workflow, forking the mechanism and touching its own frozen enforcement surface. (4) WORST, found only by a round-trip test: keel init shipped this project github-actors.toml verbatim, so every new project silently inherited MY login as its authorized decider and would have recorded acceptances in my name - the issue243 class landing on the single most consequential instance fact in the tree. TWO FURTHER BUGS surfaced by the round trip, both of which had been REPORTING SUCCESS: export derived destinations by stripping .engine, and for a file outside .engine the strip failed so the absolute-path fallback made join discard the base, making it copy the file onto itself; and the --update loop had its own .engine-join, writing .github/workflows/x.yml into .engine/.github/workflows/x.yml - inert, in the wrong place - while printing 6 file(s) from upstream.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcChannelUnitTravels to issue251;


    part issue252 : Issue {
        :>> id = "509efa73-6b5f-4084-8340-fdbabc80da83";
        :>> title = "An adopted unit landed with a RED gate: the entry binding skill to process could not travel";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Found by actually adopting the decision-channel unit on asirobots/penumbra instead of testing the export in this repository. The bundle carried the process definition, the SKILL.md and the mechanism, but the entry that binds skill to process lives in .engine/skills/skills-registry.sysml - one shared file holding all 37 skills - and stayed home. process-skill therefore FAILED in penumbra on the very first run: process .engine/processes/decision-channel.sysml has NO deploying skill (inert process). A new project's first experience of an adopted unit was a red gate clearable only by hand-editing a shared engine file, which is a fork of the mechanism and exactly the adoption friction D0054 calls the top risk. The rules layer had already met this and solved it by carrying rules BY NAME rather than copying a shared file; skills had no equivalent. FIX (D0220): a skill may declare its deployment beside itself in any .sysml under .engine/skills/, and both readers scan the directory. TWO further defects surfaced in the doing: duplicate-identity refused the copy on the first edit, forcing a MOVE rather than a duplicate; and moving the entry silently dropped the export from 6 files to 5, losing the skill entirely, because activation::deploying_skills also read only the central file - caught by re-checking the export rather than trusting the count.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSkillRegistersBesideItself to issue252;


    part issue253 : Issue {
        :>> id = "8735fd75-aae9-4d2d-9f0e-211ef3b07a70";
        :>> title = "23 of 24 units would have landed RED in any project adopting them - and nothing computed that";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Raised by the human asking for a final transferability pass BEFORE migrating to penumbra: fix keel first, I want a good transition. Twice in the preceding hours a unit had been DECLARED transferable and was not - the decision-channel left its whole mechanism behind (issue251), then left behind the declaration binding skill to process (issue252) - and both were found by ACTUALLY ADOPTING, never by reading the export. Computing the answer rather than reading it showed 23 of 24 units would land RED: each carried its process definition and its SKILL.md while the entry BINDING them stayed in the single shared skills-registry, which cannot travel with one unit, so any adoption into a project lacking that process failed process-skill on arrival. Two further hazards surfaced only by adopting into a simulated older project: an incoming per-skill declaration DUPLICATES a central one still present there, failing duplicate-identity - which is penumbra exactly - and a carried file citing .engine/tools/ leaves a dead reference because init never ships engine-dev-only tooling, including one process still naming the Smartsheet tap-inbox that D0206 RETIRED. FIX (D0222): keel process audit makes transferability a computation any project can reproduce; a committed transform moves all 36 declarations beside their skills; import retires a superseded central declaration and says so; the stale references are swept.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcTransferabilityIsComputed to issue253;


    part issue254 : Issue {
        :>> id = "fa98990d-ef7e-458d-91cb-b5cfb105e36d";
        :>> title = "The guard that stops a FABRICATED human acceptance could be switched off with one command";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Found while checking a premise rather than accepting it. The human said they were fine with the process for defining and cementing processes being undisable-able and shipped, and asked if that was what we were discussing - it was not: process-change has been a CORE never-deactivatable guard all along and change-request is a workflow outside activation, so both halves were already true, while what I had reported was a packaging defect. Checking turned up the real gap. decision-authoring CLAIMED five guards, and activation::guard_state returns Core exactly when NO unit claims a guard - so claiming made them switchable, and `keel deactivate decision-authoring` switched off decisionRationale, acceptanceEvents, decisionRequirementLink, attestationSubstance and confirmationAuthenticity together. confirmationAuthenticity is the guard that stops an AI recording a human acceptance THAT NEVER HAPPENED. A single command therefore disarmed the check standing between the engine and a fabricated human judgment, which CLAUDE.md section 4 says must never be possible - the monitor-modifiable-by-monitored class (issue236/D0208) sitting on the most attestation-sensitive path there is. FIX (D0223): the process asserts no guard, so all five are CORE; the definition says why and warns against re-adding an assert; the link is recorded as core = [...] in process-enforcement.toml so a core-enforced process no longer reports guards 0 and read as unenforced.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcDecisionAuthoringIsCore to issue254;


    part issue255 : Issue {
        :>> id = "378acb24-2421-4891-981c-f9da177e5738";
        :>> title = "A shell backtick injected 2000 characters of tool output into a Decision, and only an invalid escape caught it";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Recording D0223 through a shell script, a BACKTICK inside a double-quoted argument was command substitution. The decision text mentioned a keel command in backticks; sh executed it and substituted roughly 2000 characters of hardening JSON into the decision field of a governance record. THE PART THAT MATTERS: it was caught only INCIDENTALLY, because the injected JSON happened to carry a unicode escape the SysML parser rejects. Tool output without an invalid escape would have parsed clean and landed, and the committed record would then assert something no human or AI ever wrote - the same class as a fabricated attestation, and worse than a typo because nothing would ever have flagged it. This is the shell-escaping class that has recurred seven times in this session, but every previous instance broke a BUILD; this one silently corrupted a RECORD. The durable authoring rule (record-decision text goes through a file, never through shell interpolation) is a lesson, and a lesson is not a control - so the resolver adds a write-time refusal of a decision field carrying obvious tool output.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcNoShellInjectionIntoRecords to issue255;

    part issue256 : Issue {
        :>> id = "cde95a59-0c56-4651-a006-80e89c035209";
        :>> title = "An accepted Decision had already shipped with 13,136 characters of tool output injected into its context";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Investigating the D0223 injection (issue255), keel rules flagged D0214 - and its context field held 13,136 characters of console transcript, substituted mid-sentence exactly where a backticked command name stood. The sentence read 'the sharpest answer, measured: <transcript> printed the bare words ALL PASS', with the subject of 'printed' missing: the shell had replaced the command name with what the command printed. So the injection this control was written for had ALREADY happened, in an earlier session, and had been recorded, auto-accepted under standing consent, committed and pushed - undetected, because that output carried no invalid escape and no key-colon pairs. The only reason it ever surfaced is that the pasted text contained verdict words, which tripped a warning-severity rule nobody had read. That makes this the second confirmed instance and the first LANDED one, and it moves the failure from hypothetical to observed: a governance record in this repo asserted 13,000 characters that no author wrote. FIXED HERE: the dump is excised and the sentence restored to name the command it originally named; the acceptance receipt is untouched, since what was agreed did not change. The detector now also refuses the console-transcript shape, calibrated against the whole corpus - no authored field carries more than one guard marker, the landed transcript carried dozens.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcNoShellInjectionIntoRecords to issue256;

    part issue257 : Issue {
        :>> id = "b580c481-4f43-4aa3-8215-541282dbd105";
        :>> title = "A scaffolded project gets 25 active processes and is never asked what it is building";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "keel init lays down a working project with every process active, and nothing ever asks the author what they are building or what must not go wrong with it. The author then faces 25 processes with no basis to judge them and has two bad options: keep all of them and meet friction they never chose - D0054 names adoption friction as the dominant failure mode, above bad architecture - or switch off the ones whose names they do not recognise, which is the worst available selection rule, because the least familiar names are doing the least obvious work. The self-build could never feel this, since keel wants every process on; it surfaced only when adopting onto a second repository. The author raised it directly and made it a precondition on that migration, and named the range it has to serve: hardware, software, both, business processes, safety-related, art, video games, construction plans, whatever. The word whatever is part of the requirement - a closed table of domains cannot serve a domain nobody listed.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcProjectOnboarding to issue257;

    part issue258 : Issue {
        :>> id = "71ad8e6e-2840-4c1f-b6f8-84c8134d0011";
        :>> title = "The decision channel opens one GitHub issue per Decision and is barraging its own owner";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Volunteered by the author while answering a different question: they do not want a second decider on another project enrolled, because of the notification volume they are getting now. Measured rather than assumed: 22 issues exist and 20 were opened in one day, one per proposed Decision, because D0207 gives every non-fork a forever-open override thread of its own; 94 workflow runs landed the same day, 28 CI failures among them, and each failure emails the pusher; the sweeper fires every 30 minutes. This is the D0054 adoption-friction class turned on its author - a control I built imposing a cost its design never counted - and it is the first friction report about one of these controls. It matters beyond this repo: adopting the channel onto a second project would replicate the barrage there, which is precisely the outcome the author said they wanted to avoid for a colleague. The override thread itself is not the problem and must not be weakened: it is the human only route to reverse a Decision that was auto-accepted under standing consent.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcChannelNotificationCost to issue258;

    part issue259 : Issue {
        :>> id = "1940c642-88a5-4c9a-a02b-a432da5025a3";
        :>> title = "A guard landed today broke the commit gate of the first project keel was adopted onto";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Guard 50 requires every declared Process to carry an APPLIES-WHEN condition. Run against penumbra - a real project on a 20-August engine snapshot that has never adopted project-onboarding - it failed 22 times out of 22 and would have broken that project's commit gate on arrival, for a convention it never chose. This is the failure D0164 exists to prevent, and I wrote the guard the same day while citing D0164 in a different context, which is the part worth recording: knowing the rule did not prevent breaking it, because nothing in the self-build could feel the consequence. It is the third time in four sprints that a defect was invisible to keel and obvious to a second project. The measurement was only available because the adoption was actually attempted rather than reasoned about.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcGuardScopedToAdoption to issue259;

    part issue260 : Issue {
        :>> id = "9d9f973c-6f4f-4d9c-88db-b3472dbdabe9";
        :>> title = "The release gate had been red since guard 48 landed, and only cutting a release could reveal it";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Tagging v0.3.0, the release workflow failed - not on anything in the release, but on the test asserting that no guard reports a zero population. The guard reporting zero was claim-ancestry, which skips loudly on a shallow clone because a depth-dependent verdict is the machine dependence K15 forbids, and actions/checkout defaults to depth 1. ci.yml carries fetch-depth 0 with a comment naming issue229 - the exact fix, for the exact reason, already written down in this repo. release.yml never received it. Three of four workflows had it. So the release gate has been red since guard 48 landed weeks ago, undetected because v0.2.3 was tagged on 2026-08-14, before that guard existed, and nothing has cut a release since. Two things make this worse than an ordinary miss: the release path is the one gate where no other run would notice, and the fix had ALREADY SUCCEEDED once in this repo and still did not generalise - so this is not vigilance failing, it is vigilance having worked and not transferred, which is the strongest possible case for D0047.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcGatingWorkflowHistory to issue260;

    part issue261 : Issue {
        :>> id = "21d59181-8670-4b5e-945c-b0caa093038b";
        :>> title = "A second project's commit gate was running this repository's uncommitted working build";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "keel on PATH resolved to target/release/keel in a sibling working directory, reporting version 0.3.0+dirty, and penumbra's scaffolded pre-commit used it. Three consequences, all silent: a broken or half-finished build here would break that project's gate; every guard added here applied immediately to their commits without them adopting anything, which is exactly how a guard landed this session failed 22 of their process definitions on contact (issue259); and their gate's verdicts were reproducible only on this machine, which is the machine-dependence K15 forbids. The scaffolded hook already honoured a KEEL_BIN environment variable, but an environment variable is not a pin - it must be set correctly in every shell by every contributor forever, which is the manual-vigilance shape D0047 rules out. Raised while executing the author's chosen option, pin penumbra to a release, which is what made the coupling visible.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcPinnedFirstBinary to issue261;

    part issue262 : Issue {
        :>> id = "86900de4-62a8-483c-af2a-e61adc064ed0";
        :>> title = "The first live run of the standing thread created it twice and then closed it";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "Two defects, both invisible to the stubbed dry run and both found in the first live run of D0227 OPTION A. FIRST: the thread was created TWICE, issues 28 and 29, seconds apart by the same run - two lookups each missed the other's creation because the lookup used gh --search, which reads the GitHub SEARCH INDEX and lags creation by seconds. The dry run could not have caught it: the stub returned an existing thread instantly, so the create-then-look-again path never executed. FIXED by finding the thread by LABEL, which the API serves directly, plus an in-process cache, plus taking the lowest number if a race ever leaves two. SECOND, and worse: the recorder CLOSED the standing thread after posting the auto-accept receipt, because that step had always closed the per-decision issue it was working on. Under D0227 that one thread is the override surface for EVERY auto-accepted decision, so closing it destroyed the reversal route for all of them at once - the exact property the fork was approved to protect. FIXED by never closing a thread carrying the keel-standing label, while a per-decision fork issue still closes because it is about that decision alone. Both threads reconciled live: 29 reopened and labelled as canonical, 28 closed as the duplicate.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcChannelNotificationCost to issue262;

    part issue263 : Issue {
        :>> id = "b37f60c1-9e28-4ce4-82b7-368ba3a917a5";
        :>> title = "process audit said the unit travels whole; the adoption proved it does not";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> description = "keel process audit - built two sprints ago precisely to answer whether a unit would land green in a project that lacks it - reported decision-channel TRAVELS WHOLE at 7 files. Imported into penumbra it left check-engine with an unresolved type reference: the process asserts constraint judgmentRequestQuality, which is defined in the SOURCE project's schema and is not part of the unit, so the claim arrived pointing at nothing. The audit checks that a deploying skill exists, that the skill-to-process binding travels, that declared extras are present on disk, and that no carried file cites a path the adopter will not have. It does not check that the TYPE and CONSTRAINT names the carried files reference are ones the unit brings with it. This is the THIRD distinct way a unit has failed to travel whole - the mechanism stayed home (issue251), the binding declaration stayed home (issue252), and now a schema symbol stays home - and each was found by attempting a real adoption rather than by any check. The audit's verdict was not merely incomplete, it was CONFIDENT: it printed every unit travels whole, which is worse than printing nothing, because it is the surface someone would trust instead of adopting. The penumbra adoption worked around it by dropping the claim, which is a better outcome anyway - an unclaimed guard is CORE, so the bar applies unconditionally with no activation step.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAuditSeesSchemaReferences to issue263;

    part issue264 : Issue {
        :>> id = "51f523c3-5799-41eb-b752-e9614c4db1c4";
        :>> title = "Verification is strong and points inward; every recent defect arrived at a boundary nothing measures";
        :>> createdAt = "2026-08-25";
        :>> createdBy = "claudeFable5";
        :>> description = "The human asked which of three things is failing: verification not strong enough, the wrong measures, or the reporting from verification being inaccurate. MEASURED. NOT the first: 51 guards with 42 hard-blocking, 99 percent of live requirements EXAMINED and 91 percent EXERCISED, and no recent defect traces to a lax threshold - when a guard is aimed at something it catches it, including refusing its own author. THE SECOND, in one precise place: 39 of 51 guards read only the authored model, 11 cross a boundary, and ZERO read another project's tree, while 11 of the last 20 issues were discoveredInField and cluster at exactly the boundaries nothing measures - another project's tree, an external service, a shell layer, a different binary, the human's attention. THE THIRD, and dominant: 21 Decisions record a verdict overstating what it verified, 9 of them in the last 18, and the instances are the loudest surfaces in the engine - guard ALL PASS printed over 80 unread warnings, process audit printing every unit travels whole when a schema symbol did not travel, validate green never meaning valid SysML, a stubbed dry run reported as verification. A second form of the same failure: a correct signal nobody consumes - decisionNoVerdictProseRule flagged the 13136-character injection in D0214 for weeks at warning level, and D0214 is itself the Decision recorded to fix unread warnings. THE NUMBER THAT SETTLES IT: 5120 recorded TestResults at a 3.9 percent fail rate, 907 of them stamped by one actor in five days at roughly 2 percent. The actor doing the work also authors the verdict AND the sentence describing it, and no step anywhere converts evidence into a claim under any independent check. So verification is not the weak link - ATTESTATION IS UNAUDITED. This diagnosis carries the same structural weakness it describes: I computed it with measures I built.";
        :>> discoveredInField = true;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcForeignTreeIsTheTest to issue264;

    part issue265 : Issue {
        :>> id = "ad126e7f-5b26-43df-9af0-6ac836f6247f";
        :>> title = "The foreign-tree fixture is keel adopting keel, so it cannot see the two defects that actually happened";
        :>> createdAt = "2026-08-25";
        :>> createdBy = "claudeFable5";
        :>> description = "adoption-check gates a fresh scaffold, which is the right shape but the wrong foreign-ness: the fixture is a CURRENT keel scaffold, so it represents keel adopting keel rather than a real adopter. Two defects from this session are invisible to it, and both were verified invisible rather than reasoned about. issue263 - a unit asserting a constraint it does not carry - passes, because keel init ships .engine/rules/guard-constraints.sysml, so the symbol resolves in the fixture; penumbra could not resolve it only because penumbra runs an older engine snapshot. issue259 - a control firing on a project that never adopted it - passes for the same reason: every process in the fixture already carries the current APPLIES-WHEN convention, so a convention-based guard has nothing to fire on. This matters more than the two specific misses: it means the new gate's PASS is bounded in a way that is invisible from its output, which is the same verdict-overstates-scope class the gate was built in response to. I first claimed the check would have caught five of the recent defects; testing the claim showed two, and the difference is exactly this. The fix is a second fixture pinned to a prior RELEASE, obtained from the release channel so the vintage is real rather than synthesised.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcVintageFixture to issue265;

    part issue266 : Issue {
        :>> id = "4802fda8-76c4-461b-a112-46adcbaf3ae4";
        :>> title = "Every pass in this model is a testimony: of 5135 results, one recorded what produced it";
        :>> createdAt = "2026-08-25";
        :>> createdBy = "claudeFable5";
        :>> description = "Raised by the human, who named it plainly: I very often over-claim - one test for one scenario, reported as though the thing were now perfect - and asked how to improve attestation systemically rather than case by case. Measured: a TestResult carries outcome, judgedBy, judgedAt and judgedAgainst, and nothing else. Of 5135 recorded results, exactly ONE recorded what produced it. So no third party can re-derive any verdict in this model, and the actor doing the work authors the evidence, the verdict, and the sentence describing it. The 3.9 percent fail rate over 5120 results is not a quality signal, it is the signature of a system recording that work happened rather than testing whether it worked. Compounding it: I choose the METHOD myself, and I choose method=test when the evidence was one manual observation, which means the engine's best honesty instrument - the EXAMINED versus EXERCISED split, reporting 99 and 91 percent - is fed by my own self-classification. The human also corrected the aim of any fix: governance is here to constrain and check the AI and other work, never them, so a control demanding a receipt from a human judgment would point at the wrong party.";
        :>> discoveredInField = true;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcEvidenceCited to issue266;

    part issue267 : Issue {
        :>> id = "f6ff57c3-9986-4a3b-84fc-c8d6084c5864";
        :>> title = "Sprint ceremony records were filled by a scratchpad script that bypasses the write API and double-stamps every verdict";
        :>> createdAt = "2026-08-25";
        :>> createdBy = "claudeFable5";
        :>> description = "Found by guard 52 on its first real use: it flagged a TestResult that the write API never wrote. Sprint ceremony records have been filled all session by a python script in the scratchpad which emits TestResult lines directly into the model, so it bypasses every check the write path carries - actor validation, the write lock, the method=confirmation AI refusal, and now the evidence receipt - and its R1 result DOUBLE-STAMPS the R2 that append-result writes for the same test. Seven duplicate results in sprint467 alone; sprints 461 through 466 carry the same shape. Two consequences beyond the duplication: the result population that every attestation number is computed over is inflated by roughly half for recent sprints, and a script with no provenance checks has been the de facto write path for the most judgment-bearing records in the model. The author had already given the rule that covers this - if we need something consistently, build it into keel rather than a bespoke script - and I kept using the script because it was there. The new guard is what surfaced it, which is the argument for the guard.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcSprintFillIsAWritePath to issue267;

    part issue268 : Issue {
        :>> id = "787db3bf-21d3-4846-91c1-4acc515911dd";
        :>> title = "The console never said which project it served, and two consoles were indistinguishable in the browser tab bar";
        :>> createdAt = "2026-08-25";
        :>> createdBy = "claudeFable5";
        :>> description = "Reported by the author while using it: keel serve does not indicate what project it is serving. Three places said nothing - the startup banner printed the port and every endpoint but never the root; the page title was the constant Keel console; the header was the constant Keel dot console. The title is the one that matters most, because a browser TAB shows the page title, so two consoles open on one machine were told apart by nothing at all. That is not cosmetic: the console WRITES, so two identical tabs means a Decision accepted into the wrong project - an attested fact in the wrong tree, with no way to notice. WHAT MAKES THIS WORSE THAN A MISSING LABEL: the capability the author suggested as a better feature was ALREADY BUILT server-side. /api/projects discovers every sibling keel project and names the active one, and its own doc comment says it is the half of N-C4 that stops a supervisor wondering which project they are looking at; POST /api/project switches and refuses a non-project rather than falling back. Nothing in the console rendered either endpoint. So this is the accepted-Decision-with-artifact-unbuilt class: a surface was specified, the server half shipped, and the half the human actually sees never did.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcWhichProject to issue268;

    part issue269 : Issue {
        :>> id = "c8f0d8db-2b83-42d7-8083-14b6f175f156";
        :>> title = "keel validate reports a clean tree for a directory that is not a keel project at all";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Found while testing whether several keel projects can share one git repo. Pointed at a directory with no .engine, keel validate prints 0 tracking file(s) validated clean and exits 0. That is a vacuous pass on the loudest surface in the engine - validate is the first line of every gate, including the scaffolded pre-commit hook - so a hook placed at a parent directory would gate NOTHING while reporting success, and both projects underneath it would commit unguarded. The two halves of the gate also disagree in that position: guard all correctly FAILS at the same path, so one half treats an absent project as a usage error and the other treats it as a clean tree. This is the same class as the guard-passing-on-an-empty-population defects already recorded (issue250, and claude-surface-drift over zero skills), except it is reachable by a single command with no unusual setup, and the layout that triggers it - a parent repo holding several small keel projects - is one a user would reasonably try.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcValidateRefusesANonProject to issue269;

    part issue270 : Issue {
        :>> id = "c3c58b60-dd6d-4827-8841-ccd41b8df713";
        :>> title = "Several keel projects cannot share one git repository: numbering collides, the channel confuses them, and one hooksPath cannot gate them";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "The author wants one repo for a handful of small short-term projects and expects the layout to recur. Built and RAN it rather than reasoning about it. Three things already work: root resolution walks up to .engine, drift is file-scoped through each project's own deliverable-manifest, and the write lock sits beside each model root. Four break. DECISION NUMBERING COLLIDES: two fresh projects both get d0001. THE CHANNEL CANNOT TELL THEM APART: its marker is keel-decision d0001 and its GitHub lookup is repo-scoped, so the second project's issue never opens because the search finds the first, and a reject gesture is ambiguous. THE GATE CANNOT COVER THEM: git allows exactly one core.hooksPath per repository, so a per-project hook gates at most one project, and keel validate at the parent printed 0 tracking file(s) validated clean and exited 0 - a hook there would gate nothing while reporting success. AND LAND GATES ONE ROOT WHILE PUSHING THE WHOLE REPO, so the other projects' changes ride out covered by no gate. Each of the four is a property of git or of the channel being repo-scoped, not a preference, so the engine has to know the difference between a project and the repository holding it.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcWorkspace to issue270;

    part issue271 : Issue {
        :>> id = "4bc157a7-44a8-49bf-8052-4c5bb95c6619";
        :>> title = "Every project's guards saw every other project's staged files";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "git diff --cached answers for the whole REPOSITORY no matter which subdirectory asks it, and the guards that read staged files did not re-scope the answer. In a repo holding several keel projects that means each project's guards judge the other projects' changes, and the workspace-level files too. Found live and immediately: a two-project workspace failed BOTH projects' process-change guard over .githooks/pre-commit, a file belonging to neither project. The failure is not merely noisy - it is wrong in the direction that matters, because a project's gate reported a violation about a file it does not own, which is a verdict no reader could act on. Invisible in a single-project repo, where the project IS the repo root and the unscoped answer happens to be correct - so nothing before now could have surfaced it.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcStagedFilesAreProjectScoped to issue271;

    part issue272 : Issue {
        :>> id = "a762e9c6-0cfc-4690-b322-ab00baaa5be3";
        :>> title = "A freshly scaffolded project could not make its first commit";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "keel init lays down a working project and tells the author to commit. That commit FAILED. init stages every .engine/processes and .engine/workflows file, the keystone lock read those as locked surfaces changing without a co-committed signed Decision, and the guard blocked - telling a brand-new user to record a process-change Decision for a scaffold they did not write and had no opinion about. Reproduced on a plain single-project keel init, so this has nothing to do with workspaces and has been true for every new project. It is the worst adoption defect found so far, because it is the FIRST thing anyone does after scaffolding and the message sends them somewhere useless. The distinction the lock was missing: it exists so a control cannot be WEAKENED unsigned, and a locked file being ADDED is a control arriving, not one being weakened. Found only because a workspace experiment made me commit a fresh scaffold - adoption-check runs validate, guard and check-engine but never a COMMIT, so nothing staged means process-change sees nothing and the whole class was outside every existing check.";
        :>> discoveredInField = true;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcKeystoneLockIsAboutChangeNotArrival to issue272;

    part issue273 : Issue {
        :>> id = "d50fbb2f-4791-4b46-9ba6-24c963180b0c";
        :>> title = "A git mv bypassed the keystone lock in every repository, opened by my own fix to it the day before";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Found by an adversarial panel convened on D0234, by PEDANT-3, within the hour. Yesterday I narrowed the keystone lock from --diff-filter=ACMR to MD so that a freshly scaffolded project could make its first commit (issue272). ACMR had included R. MD does not. So a locked file could be edited and MOVED in one staged change and the guard saw nothing: sed blocking to warning into a new filename, remove the original, git add -A, and git records R096 while the MD filter matches nothing. keel guard process-change reports PASS - 0 scanned. Every blocking rule in a project downgraded to a warning with no signed Decision. REPRODUCED ON A PLAIN SINGLE-PROJECT REPO, so this was live in this repository and in penumbra, not confined to the workspace path I had just built - and I had told the human there was no live exposure, which was wrong. The deeper error was reading --name-only, a projection that prints only a rename's destination, so a filter over it hides one half of the very operation being policed. The same function carried a second blindness the same panelist found: git C-quotes a path containing a non-ASCII byte in this output while rev-parse --show-prefix does not, so a project named with a circumflex was invisible to the workspace gate and to its own guards, and a full rules downgrade there passed. The general lesson is narrower than it looks: I weakened a control and never tested the weakening against the class of change it was supposed to keep catching.";
        :>> discoveredInField = true;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcRenameIsAChange to issue273;

    part issue274 : Issue {
        :>> id = "1992d2ac-cd65-4bca-8219-589927dbd1d1";
        :>> title = "Recording an obligation made the tree invalid: the recorder's own file does not validate";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "The Stop hook recorded a red-yield obligation and the file it wrote imports only EngineElement, while Issue is defined in EngineWork and the Resolves edge in EngineRelationships. So validate reported a semantic diagnostic - unresolved type reference Issue - the moment the obligation was staged. That is self-defeating twice over: the record exists to say the tree needs a correction pass, and it is itself the thing making the tree incorrect; and it puts the recorder in the one position a recorder must never occupy, unable to record without breaking what it is reporting on. Found while triaging a genuine obligation, so the defect had shipped and had never been exercised - no obligation had been recorded since the template was written.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcObligationRecorderValidates to issue274;

    part issue275 : Issue {
        :>> id = "06fdbb25-8072-4156-94b1-a004c58c599c";
        :>> title = "Workspace discovery stops at the first project, so nested and deep projects are invisible and ride out ungated";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Three panelists found this independently. The walk returns the moment a directory is a project and is bounded at depth three, and init refuses neither position, so a project nested inside another or deeper than three is invisible to every workspace-scoped mechanism. Verified: keel init dot then keel init sub in one repo reports a single project; a commit adding an unparseable tracking file plus a downgrade of every blocking rule in the invisible project exits 0 while the gate prints clean, and validate inside that project then fails. Same at depth four. This is the most likely layout for the stated goal - an author whose repo already has a project at the root adding small ones beside it - and it silently collapses all four workspace fixes back to one project. Aggravating: my own unit test asserted exactly this shape as correct, so it pinned the bug as intended behaviour and would have blocked anyone from fixing it.";
        :>> discoveredInField = true;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcWorkspaceDiscoveryIsComplete to issue275;

    part issue276 : Issue {
        :>> id = "eb297eac-6d5a-4cb8-aae4-241e3fdd65c2";
        :>> title = "The one hook that gates every project in a workspace can be replaced with exit 0, unsigned, and the gate passes";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Two panelists found this independently and I reproduced it. In a workspace the repo-root enforcement surface belongs to no project, and the keystone lock matches project-relative paths, so the root hook and the root workflows directory are covered by no guard at all. Verified: the single pre-commit hook that gates every project replaced with a two-line exit 0, staged alone with no Decision - gate --workspace printed a line NAMING the unowned file and then reported 2 projects gated clean, exit 0. The identical edit in a single-project repo is BLOCKED. The tell is my own output: a gate that reports what it is not checking and passes anyway is the false-green class this session spent its length closing. Also verified: staging the deletion of an entire project, 445 paths including every process definition and its guards, passes the gate silently.";
        :>> discoveredInField = true;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcWorkspaceSurfaceIsOwned to issue276;

    part issue277 : Issue {
        :>> id = "436d289f-80ea-4a7d-aa77-e51cf29f9022";
        :>> title = "The compensating control cited for relaxing the keystone lock is blind in the arrangement the same Decision introduced";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "issue272 relaxed the keystone lock to exempt ADDED locked files, and D0234 justified the residual by naming audit-adherence's guard-state monotonicity as the check that catches an added process weakening a core guard. audit-adherence derives its enforcement signature from repo-root-relative pathspecs, which match nothing when projects live one directory down. Verified by two panelists with an identical unsigned rule weakening: single-project repo FAIL exit 1, naming the blocking-to-warning transition; two-project workspace PASS exit 0, both from the repo root and from inside the project. So the relaxation is unconditional while its stated compensation exists only in the single-project case. I weakened one control and justified it by pointing at another without checking that the second worked in the new case - which is the same reasoning error as citing a stub dry run as verification.";
        :>> discoveredInField = true;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcAdherenceIsWorkspaceAware to issue277;

    part issue278 : Issue {
        :>> id = "c43e2eee-88fc-41f7-b30e-47e9df4904c8";
        :>> title = "A freshly created workspace is completely ungated, and the remediation message installs a gate that checks nothing";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Three panelists found this. init writes a hook per project and sets core.hooksPath only when its target itself holds a git directory, so git init followed by two keel inits leaves no repo-root hook and no hooksPath - verified, the subsequent commit ran with zero pre-commit lines. init then prints instructions which, followed literally, create a nested repository and destroy the workspace. Worse, the refusal message I added to fix the issue269 vacuous pass advises keel hook pre-commit: that is not a hook event, and at a workspace root it prints nothing and exits 0. Anyone who wires it into a hook installs a gate that passes while checking nothing - issue269 reintroduced inside its own fix, by its own remediation text.";
        :>> discoveredInField = true;
        :>> severity = Severity::Critical;
    }
    #Resolves dependency from dcWorkspaceIsWireable to issue278;

    part issue279 : Issue {
        :>> id = "6890b896-6150-4179-b4dd-73131e5fb4de";
        :>> title = "The decision qualifier does not round-trip, so a human's judgment still lands in the wrong project";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Four verified gaps, found across three panelists. The DURABLE marker written into an issue body uses the bare name while every lookup uses the qualified id, so in a workspace no lookup can match what the opener wrote: the standing thread re-posts on every run, the backfill posts a duplicate, and two projects write byte-identical markers - the collision D0234 claims to have removed. The gesture parser drops the qualifier, so a reject naming a qualified id resolves to the bare name and retargets the human's rejection to the ROOT project, while the opener instructs them to type exactly that form. The human-gesture branch of the recorder never calls the splitter, so a fork acceptance - the one decision class that genuinely needs a human - is recorded against the wrong tree or refused outright. And the decider table is read from the repo root, absent in a workspace, so every gesture is answered not-a-declared-decider. My verification tested the join function in isolation and never checked what actually gets written into an issue body.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcChannelQualifierRoundTrips to issue279;

    part issue280 : Issue {
        :>> id = "9ded8084-48f6-48b9-8f66-9c03df6ce809";
        :>> title = "keel land gates nothing before the first push, so a broken project leaves the machine";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "D0234's fourth clause claims land gates every project before a push. It does not: land pushes first and only reaches the gate after a REJECTION and merge, so on the success path nothing is gated at all. Verified against a bare remote - a project carrying an unparseable tracking file, committed with the skip flag as any contributor without a configured hook would, then land run from a different project in the same workspace: pushed, exit 0, and the broken file is present in the remote. sync does gate the whole workspace unconditionally; only land is holed. So the Decision asserts a property the code does not have, and the clause I wrote to fix the original complaint about push reconciliation is the one clause that was never implemented on the path that matters.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcLandGatesBeforeThePush to issue280;

    part issue281 : Issue {
        :>> id = "d02d8a2d-a5ce-44ab-b2ea-bf38d6b71ec5";
        :>> title = "orient reports an all-green empty state at a workspace root, and validate can answer for another repository";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "The issue269 refusal was applied to validate alone; the project predicate has exactly one non-test call site. So at a workspace root orient exits 0 with an empty ready list, zero done, zero outstanding, 100 percent decomposed and answerStatus COMPUTED - and orient is the surface CLAUDE.md makes the AI's only legitimate state read, explicitly forbidding prose substitutes. whats-next prints COMPUTED-EMPTY over zero items, calling it an answer rather than a failure. check-engine prints validated clean over zero files and is a blocking step in this repo's own commit gate. A separate finding on the same theme: find_repo_root has no git boundary while workspace discovery does, so standing in a directory nested under an unrelated keel project, validate with no argument validated the OUTER repository's project and reported it clean.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcNoCommandReportsGreenOverNothing to issue281;

    part issue282 : Issue {
        :>> id = "63a56a6e-4ae4-413d-97ab-6f19be9d7d16";
        :>> title = "Three bodies of the gate with three different bars, and the push gate is the weakest";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "The workspace per-project gate runs validate plus guards plus declared rules; the scaffolded pre-commit hook runs validate plus guard plus rules enforce; the sync and land gate runs validate plus guards and no rules at all - that file contains zero occurrences of the word. The declared-rule layer is precisely the one a downstream project uses to add a blocking control without writing Rust, so such a project has its only control enforced at commit and unenforced on the merged tree, which is the class the merged-tree gate exists to catch since a merge can violate an edge rule neither side violated. The sync gate's own doc comment still reads deliberately the SAME entry point the commit hook uses, rather than a parallel definition, and D0234 edited the body underneath that comment while writing a third, fuller definition four hundred lines away.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcOneGateDefinition to issue282;

    part issue283 : Issue {
        :>> id = "7f257082-8fe5-47f3-813e-6eaacf3d145a";
        :>> title = "Six claims in the workspace record are wrong, including evidence references that do not resolve";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "The facts pedant verified most of D0234 and refuted six claims. byte-identical for a single-project repo is an over-claim: the multi-project predicate gates only message wording, and three changes alter single-project behaviour - the keystone filter, validate's refusal, and staged-path re-basing for a project nested in a larger git repo. verified against both real repositories undercounts by six; there are eight keel repos here. no nested false positives was VACUOUS, because the walk short-circuits at a root project and therefore examined no subdirectory in any of them - the check could not have found a false positive if one existed. The two headline RAN evidence references in the sprint record do not resolve, being commits in scratch repos I deleted, which violates D0129 inside the very receipt mechanism built one day earlier to make a pass re-derivable by someone else. The refusal message names the wrong missing directory. And an unexplained installed-units counter bump rode the commit.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcPanelRecordCorrections to issue283;

    part issue284 : Issue {
        :>> id = "1cb2a9ed-f269-4102-bb0e-e6b55023caf1";
        :>> title = "keel projects cannot tell you which project you are in on this platform";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Two panelists found this independently. The command canonicalizes the working directory, which on Windows returns the extended-length prefix with backslashes, while discovery takes the repository root from git in forward-slash form, so the prefix comparison never matches. Verified: run from inside a project, no you-are-here marker is printed and the JSON reports current false for every project - including in all eight real keel repos on this machine. The unit test passes only because it constructs the structure by hand from POSIX literals and never goes through discovery, which is the same hollow-verification shape as the vacuous nested-false-positive check in the same module.";
        :>> discoveredInField = true;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcOwnerOfMatchesOnWindows to issue284;

    part issue285 : Issue {
        :>> id = "45d82ef0-ff7b-454a-a146-e821a4e19d29";
        :>> title = "The console advertised six computed views it could not serve, and the human found the first by clicking it";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Reported by the human from the console: clicking attestation-coverage returned no computed view is bound to the command, while the same command works on the CLI. Diffing the viewpoint registry's renderer prose against the server's binding table found SIX unbound - attestation-coverage, critique-policy, decisions, reprocess-candidates, verification and whats-next - so the human happened to click the first of six dead panels. This is the accepted-Decision-with-artifact-unbuilt class (issue174) landing on the human's own oversight surface: a viewpoint that names a renderer is a promise the console can serve it, and the model kept declaring surfaces the server did not implement. whats-next is among them, which is the ranked frontier - the single most load-bearing view in the engine. Found from USE, again: nothing internal compared the two sets, and both were readable from source the whole time.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcEveryAdvertisedViewIsBound to issue285;

    part issue286 : Issue {
        :>> id = "b3b12e28-d029-4044-83ab-fda17243dcc5";
        :>> title = "Every generic viewpoint panel dumped raw JSON while the console's own item-linking sat unused";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Reported by the human: tools give a vanilla json payload, needs to be displayed properly and link to items properly and dynamically. The generic viewpoint panel rendered its payload as pre JSON.stringify with one-space indentation - a blob the reader has to parse by eye - and every item name in it was dead text. The console already had itemLink, an openItem panel and an /api/item/:name endpoint, used by the bespoke panels and by orient; the generic path simply never called them. So this was not a missing capability but an unused one, which is why it survived: each new viewpoint got a reachable panel and nobody asked whether the panel was readable. The consequence compounds with issue285 - as more viewpoints get bound, more of the human's oversight surface becomes JSON blobs unless the generic renderer is the one that improves.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcComputedViewsRenderAndLink to issue286;

    part issue287 : Issue {
        :>> id = "c2133e83-c201-4ca0-a261-e9f352ae266f";
        :>> title = "A guard demanded that the human quote themselves, because nothing records who recorded an acceptance";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "The human accepted d0235 through a human surface while the AI was working, noting agreed with panel's decisions. delegatedAcceptanceSubstanceRule then failed the commit gate, requiring a verbatim quoted span or a cited surface gesture - which for a record the human wrote themselves means demanding that they quote themselves. That is governance aimed at the one party it must never bind, and the human had already stated the principle: the AI is here to constrain and check other work, not them. The rule's intent is correct and load-bearing: an acceptance an AI records ON THE HUMAN'S BEHALF must quote their words, because that quote is the only thing between a real sign-off and a fabricated one. The defect is a missing fact - a TestResult records who JUDGED it but nothing records who RECORDED it, so the rule cannot distinguish a delegated record from a self-recorded one and applies the delegated demand to both. Two further notes on handling: the AI must NOT edit the human's note to satisfy the guard, because altering an attestation to pass a check is the exact failure the check exists to prevent; and it must not widen the rule to unblock itself, because that is weakening a control for convenience. Both were declined; the forward fix stamps the channel at the console write path, and the past record is left for the human.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcRecorderIsDistinctFromJudge to issue287;

    part issue288 : Issue {
        :>> id = "8179e3e8-20ec-4114-9508-f91bbb691eab";
        :>> title = "The console tab counted viewpoints instead of flagging what waits on the human";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Reported by the human: putting the number of views in the tab heading is bad, each tab should have actionable items in the heading, because when they open the console they are trying to resolve items. The tab rendered the surface's VIEWPOINT count - assurance (11), act (4) - so the most-read navigation in the engine showed how many lenses a surface holds, which is never the question a supervisor arrives with. What makes this the unused-capability class rather than a missing feature: /api/obligations already computed what awaits the human per viewpoint, derived from viewpoints declaring surface=act, complete with a countable flag and its own note that a class with no bound counter must never be reported as zero. The console had the number and rendered a different one. Two honesty properties had to be preserved while changing it: a class with no counter must render as unknown rather than zero, and a FAILED obligations fetch must render unknown on every tab rather than showing no badge - because an absent badge means nothing is waiting, and that would be a false all-clear on the surface whose entire purpose is telling the human what they owe.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcTabsFlagWhatNeedsTheHuman to issue288;

    part issue289 : Issue {
        :>> id = "3615d275-41bd-46e7-861e-6a9556b4a65f";
        :>> title = "Intake had no write API, so every recorded human utterance was hand-edited into a file";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "The human asked whether the claim was accurate: is there really no write api surface for statements or user stories. It was. keel record covered decision and issue only. That is the record type where the gap matters most, because D0216 makes the verbatim Statement the precondition for every Need - a Need with no cited utterance cannot be checked against what the human actually said, which is exactly how one comes to be written wider than the demand. So the record with the strictest rule had the weakest path: hand-edited into a file with no actor validation, no write lock, no refusal on a defaulted date, and nothing stopping a paraphrase typed from memory being stored in a field labelled VERBATIM. Every Statement in this repository was written that way, including five from this session. A second-order defect the fix had to avoid: the existing field sanitiser collapses runs of whitespace, which is correct for a title and destroys a quote, so reusing it for text would have silently altered what the human said while appearing to store it faithfully.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcIntakeHasAWritePath to issue289;

    part issue290 : Issue {
        :>> id = "9241ce15-f106-4223-947b-3eb88fbe715e";
        :>> title = "The adopted channel's mechanism never landed downstream, and nothing checks that a process's mechanism exists";
        :>> createdAt = "2026-08-26";
        :>> createdBy = "claudeFable5";
        :>> description = "Adopting the decision channel onto penumbra, the merged PR carried the process definition, the skill, its registry entry and two contracts - and NOT the mechanism. Both workflows and both scripts were missing, so that project held a process definition and a skill referencing scripts that do not exist. The cause was mine and it was hand-staging: to avoid committing that repo's untracked logs and screenshots I staged explicit paths rather than add-all, omitted .github from the list, and then deleted .github from the working tree so a later add-all could not land it out of band - which destroyed the only copy. This is issue251 verbatim, the channel leaving its mechanism behind, reproduced by hand downstream AFTER being fixed in the export. Two checks that should have caught it did not. adoption-check gates a fresh scaffolded fixture, so it verifies what EXPORT produces and never what a TARGET actually received - the gap named in its own stated limits, now with a concrete instance. And no guard flags a process whose declared mechanism files are absent, even though the tool-reference guard already performs exactly that check for .engine/tools paths. Fixed downstream by a follow-up import, which also correctly preserved that project's two documented local divergences.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcMechanismMustBePresent to issue290;
}
