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

}
