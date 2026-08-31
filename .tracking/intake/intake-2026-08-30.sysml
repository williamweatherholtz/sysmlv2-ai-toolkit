// Intake for 2026-08-30 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260830 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st037 : Statement {
        :>> id = "09a73dab-09fe-4593-9e18-1b710e94056e";
        :>> title = "Why is finished work read as open work - is it the dropped unenforceable checks, or unverifiable work flagged unverified?";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "why is finished work being confused for open work now? we should be preventing this for sure. we just dropped some unenforceable checks right? seems like we need a strong process here. or is the issue that unverifiable work is being flagged as unverified?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us053 : UserStory {
        :>> id = "61d9ae42-5ef0-431b-a049-8b9db84c366f";
        :>> title = "The finished-vs-open confusion is measured, not assumed, and its one real instance gets a ratchet guard";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the cause of finished work reading as open work established from the data and prevented";
        :>> soThat = "the frontier's ready list can be trusted without me spot-checking it";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug and not process: their question named two candidate causes and both are refuted by measurement (the dropped gate was about DoD WORDING, never stamping; and the inverse of their second hypothesis is what happened). The residue is one real defect in 496 sprints with a mechanical cause, so it routes as a bug with a ratchet control - D0260 - rather than the new ceremony a process implication would imply. The correction to my own mis-report rides in the same Decision.";
    }
    #DerivedFrom dependency from us053 to st037;

    #Implicates dependency from us053 to d0260;

    part st038 : Statement {
        :>> id = "552fde3c-fdaa-437f-868a-6bd77ed63ed7";
        :>> title = "What control prevents the regenerate-from-model class in future?";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "solid what to prevent this in the future?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us054 : UserStory {
        :>> id = "48088f73-5b26-47de-b8c2-c3aac0813fc8";
        :>> title = "No-op invariance becomes the control for regenerate-from-model writers";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "a control that prevents a writer silently dropping facts it cannot represent";
        :>> soThat = "the next such defect fails a test instead of waiting for a locked file to change";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not bug: the defect itself (issue293) is already resolved; what they are asking for is the standing prevention, which is a control plus its coverage floor. Routed to issue310, the generalisation filed from issue293's retro, rather than opening a new item - one governance home per defect class.";
    }
    #DerivedFrom dependency from us054 to st038;

    #Implicates dependency from us054 to issue310;

    part st039 : Statement {
        :>> id = "f5a54604-ce0f-428a-9c5e-3c0d6f5e961d";
        :>> title = "Continue the frontier, and find where rigor has been lost in the processes";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "continue through all of these. I think we've lost some rigor in our processes. let's do this work and figure out where any gaps are";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us055 : UserStory {
        :>> id = "24649a02-b212-47e2-afae-11c61aa23828";
        :>> title = "The rigor audit: measure the hypothesis, clean the channel, discharge the debt it hid";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "the process gaps found and closed rather than suspected";
        :>> soThat = "the signals I rely on mean something when I read them";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not question: they asked where the gaps are AND told me to do the work, so this routes to execution, not an answer. The measured finding is narrower than the hypothesis - three of four hardening dimensions were perfect - but the warning channel was 64 percent undischargeable noise and it hid real debt including a direction the human chose a week ago. Routed to D0261 and the three chartered tasks.";
    }
    #DerivedFrom dependency from us055 to st039;

    #Implicates dependency from us055 to d0261;

    part st040 : Statement {
        :>> id = "ddd1b3c3-62b8-45b5-a43e-aec68f699dcb";
        :>> title = "Build a comprehensive linked-project test platform: seven claims, designed, panel-critiqued, implemented, re-panelled";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "make a comprehensive test platform around linked projects. if another keel project is made, does everything work properly? pushing issues back to keel itself? picking up processes? if it publishes a new process, is that linked back upstream to keel? is the numbering scheme properly reset? other proper initialization steps followed? is the keel instantiated project confused on what keels purpose vs its own purpose is?\n\ndesign a test suite, hold a panel to critique, implement and test, then reaporoach panel with findings. strong empirical evidence recorded at every step.";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part st041 : Statement {
        :>> id = "7b9ae63e-3afc-4d06-922d-c3276ac0f2cc";
        :>> title = "Add a process and AI skill to ingest GitHub issues";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "and add a new process and AI skill to ingest GitHub issues";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us056 : UserStory {
        :>> id = "bf53cae5-12d9-4035-986c-9c641591d819";
        :>> title = "A federation test platform: the seven linked-project claims, held empirically";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "a test suite that proves a downstream keel project actually works in a federation";
        :>> soThat = "I can start new projects without discovering the failures myself";
        :>> implication = ImplicationKind::verifiedRequirement;
        :>> triageNote = "verifiedRequirement and not bug: they named seven CLAIMS and asked for them to be held by tests, which is a verification obligation rather than a defect report - though reconnaissance turned two of the seven into defects before the suite existed. The design-panel-implement-repanel sequence they specified IS the control-propriety process (D0257) applied to a test design.";
    }
    #DerivedFrom dependency from us056 to st040;

    part us057 : UserStory {
        :>> id = "2e48cf4d-6a61-404a-8846-8e29258b3b3d";
        :>> title = "GitHub issue ingestion becomes a process with a deploying skill";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "GitHub issues ingested into the model as tracked Issues with provenance";
        :>> soThat = "a defect reported from a downstream project or a collaborator becomes tracked work rather than a browser tab";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not implementationChange: they asked for a PROCESS and a SKILL by name, and the mechanism is the inbound half of the upstream loop st040 asks about - a downstream project filing against keel, and keel taking it in. Routed together with st040 for that reason.";
    }
    #DerivedFrom dependency from us057 to st041;

    part st042 : Statement {
        :>> id = "a2989034-b345-4401-a81b-b5a7b791b8b6";
        :>> title = "GH#38 The decision-channel workflows use third-party actions, so under an org Actions allowlist they startup_failure forever and the cron emails on every run";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "Verified: `.github/workflows/decision-issue.yml` and `.github/workflows/decision-record.yml` both reference `dtolnay/rust-toolchain` and `Swatinem/rust-cache`.\n\n## What happens in a project that adopts the channel\n\nMany GitHub organisations restrict Actions to GitHub-owned plus a named allowlist. Where neither third-party action is allowlisted, the workflow never parses into a run — every trigger is a `startup_failure`, not a failing step.\n\nIn the deployment we hit this in: **57 of 57 runs startup_failed, zero successes ever.** The channel had therefore never worked in that repo at all — including the `issue_comment` path, so a one-letter acceptance comment recorded nothing, silently.\n\nThe second cost is attention. The sweeper cron (`*/30`) retries twice an hour indefinitely, and GitHub emails the workflow actor on every failed scheduled run — roughly 48 mails a day. That is what surfaced it: the human asked why they were getting so much mail from a repo, not why decisions weren't recording.\n\n## Why this is worth fixing rather than documenting\n\nThe failure is invisible in the direction that matters. A channel that has never once run looks, from inside the project, exactly like a channel with nothing to say — there is no decision sitting unrecorded that anyone can see. The only signal is the mail volume, which reads as noise rather than as a broken mechanism.\n\n## Direction (yours)\n\n`ubuntu-latest` ships a Rust toolchain and `rustup` is preinstalled, so the third-party actions may not be needed at all for these two workflows — dropping them would make the channel work under any allowlist without asking every adopting org to change policy. Vendoring a release binary would also do it.\n\nWorth considering separately: a startup_failure is not observable from the project side, so if the channel is meant to be relied on, it may want a way to say ''I have never successfully run here''.\n\nFor context on the reporting project: we retired the channel locally rather than pursue an org allowlist change, so this is filed for other adopters, not as a blocker for us.";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/38";
    }

    part us058 : UserStory {
        :>> id = "136fa270-2398-434a-9f34-56996ce1bec6";
        :>> title = "Decision-channel workflows fail closed under an org Actions allowlist and email on every cron run";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "maintainer";
        :>> iWant = "the decision-channel workflows to run without third-party actions";
        :>> soThat = "the channel works under an org allowlist instead of startup_failing forever and emailing on every cron";
        :>> implication = ImplicationKind::bug;
        :>> triageNote = "bug and not process: the process is sound and its IMPLEMENTATION fails in an environment the design never considered - an org-level Actions allowlist that blocks third-party actions, which makes every run a startup_failure and turns the cron into a mail generator. INGESTED FROM GITHUB (issue 38), the first utterance recorded through the new github-ingest path, which is why it carries a sourceUrl a reader can check the transcription against.";
    }
    #DerivedFrom dependency from us058 to st042;

    part st043 : Statement {
        :>> id = "25f65ed8-5055-4ebc-81de-dd03bdf92a21";
        :>> title = "GH#37 A project cannot decline an engine-shipped skill: removing the deployed copy reds guard:activation-manifest, and the registry is engine-owned";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "Verified against `keel-cli/src/claude_surface.rs` at 6146f43 (0.3.1).\n\n## What happens\n\n`claude_surface` iterates `registry_skills(root)` and, for every entry, requires a byte-identical `.claude/skills/<name>/SKILL.md`:\n\n```rust\nfor (title, loc) in &registry {\n    let dst = claude.join(''skills'').join(title).join(''SKILL.md'');\n    ...\n    if std::fs::read_to_string(&dst).unwrap_or_default() != src_text { missing_skills += 1; }\n}\nif missing_skills > 0 { drift.push(format!(''{missing_skills} skill(s) missing or stale under .claude/skills/'')); }\n```\n\nSo a project that deliberately retires a workflow has no way to stop the skill being offered. Deleting the deployed copy gives:\n\n```\nERROR 1 skill(s) missing or stale under .claude/skills/\n[guard:activation-manifest] FAIL\n```\n\nand the registry that drives it lives under `.engine/`, which a project must not edit.\n\n## Why it matters\n\nWe retired a workflow with an accepted Decision recording exactly why. We could record the retirement but not enforce it: the skill still appears in the agent's skill listing, and the strongest signal available is a paragraph inside a decision file that the next session may never open. A listed skill reads as an available one.\n\nThis is the same shape as an inert contract left in place — except a contract sitting unused is harmless, while a deployed skill actively advertises itself to the agent.\n\n## What would help\n\nA project-scoped opt-out the guard honours, so ''we do not use this here'' is a computable fact rather than prose. Shapes that would work for us, in rough order of preference:\n\n1. A declined-skills list in `.tracking/` (or wherever project activation already lives), consulted by `claude_surface` before requiring deployment.\n2. A `retired`/`declined` flag the registry can carry per project.\n3. Failing both: have the guard treat an absent skill as satisfied when the project records the decline somewhere the engine already reads.\n\nHappy to be told this belongs in the activation manifest rather than the registry — the ask is only that declining is expressible without editing `.engine/`.";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/37";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st044 : Statement {
        :>> id = "e780c2e1-b9ad-4a2f-9c47-1d469f44ce2d";
        :>> title = "GH#36 keel migrate: the removed-type scan reads prose, so a description mentioning `ureq::Agent` blocks migration with no way through";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "Reported from a downstream project that is pinned to 0.3.0 because of this. Verified against `keel-cli/src/migrate.rs` at 6146f43 (0.3.1).\n\n## What happens\n\n`step_removed_types` scans every authored file **line by line** and applies `types_as(line, ty)` with no awareness of string literals or comments. `types_as` accepts a hit when the text before it ends with `:` or `:>`.\n\nThat makes ordinary prose look like an instance of a removed type. The case that blocked us:\n\n```\n:>> description = ''... the HTTP client is a ureq::Agent held by the adapter ...'';\n```\n\n`ureq::Agent` → the text before `Agent` ends with `:`, the left char `:` is not an ident char, nothing follows → **matches the removed `Agent` type**. The line is a description string. There is no `Agent` item anywhere in the model.\n\nAny `foo: Agent`, `Risk`, `Task`, `Epic`, `Role` etc. inside a `description`, `procedureText`, `notes` or `rationale` will do the same — and those fields are exactly where a project explains *why it removed something*, so writing the migration note can itself block the migration.\n\n## Why there's no way through\n\n- The blocker is prose, so there is nothing to ''migrate''.\n- The advice on `REMOVED_TYPES` says to re-add the def in your own `.engine/schema/`. That doesn't clear it: `REMOVED_TYPES` is a static list and the scan never consults the project's schema, so a project that *has* re-added the type is still blocked, on prose, forever.\n- Rewording the prose works, but it means editing authored history to satisfy a scanner — and on a large model you find them one migration run at a time.\n\n## Minimal repro\n\nIn any project on 0.3.0, add to an authored `.sysml`:\n\n```\n:>> description = ''the client is a ureq::Agent'';\n```\n\nthen `keel migrate .` — it reports a `removed-types` blocker for `Agent` on that line.\n\n## Direction (yours to choose)\n\nThe general fix is for the scan to understand what it's reading rather than pattern-match raw lines: skip string literals and comments, or scan only *declaration* positions rather than any `:`-adjacent occurrence. A narrower fix that would have unblocked us: consult the project's own `.engine/schema/` before reporting a removed type, so the documented ''re-add it yourself'' advice actually works.\n\nRelated, same class: #33 and #34 are also scans whose coverage or precision differs from what they report.";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/36";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st045 : Statement {
        :>> id = "cab2b354-a27c-4df9-b585-cc81e8a923ce";
        :>> title = "GH#35 status-report skill is inert: it invokes .engine/tools/report.py, which the binary does not ship";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "## Summary\n\nThe `status-report` skill that `keel init` deploys instructs the operator to run `python .engine/tools/report.py`. That file does not exist in a scaffolded project, and the binary does not ship it. The skill is discoverable and inert.\n\n## Repro\n\n1. `keel init myproject --profile strict`\n2. Invoke the `status-report` skill (or read `.claude/skills/status-report/SKILL.md`)\n3. Follow its ''How to Invoke'' section\n\n## Observed\n\n`.claude/skills/status-report/SKILL.md` says:\n\n```powershell\n# From repo root — generates and opens in browser:\npython .engine/tools/report.py\n\n# With explicit output path:\npython .engine/tools/report.py --output docs/status.html\n```\n\nand adds:\n\n> The script requires only Python standard library + the `target/release/keel.exe` binary for orient JSON.\n\nBut:\n\n```\n$ ls .engine/tools/\ndeck_inbox_record.py\ntest_deck_e2e.py\n\n$ find . -name 'report.py'\n(no results)\n```\n\nTwo separate problems in that quote:\n\n1. **`report.py` is absent.** Nothing in the scaffolded tree provides it.\n2. **`target/release/keel.exe` is a path from the engine's own source tree.** No scaffolded project has a `target/release/`; a downstream user installs the released binary onto PATH. Even if `report.py` shipped, that reference would not resolve.\n\n## Working equivalents that DO ship\n\nThe CLI already covers the skill's stated output:\n\n```\nkeel orient . --html                          # the dashboard\nkeel report governance . --html               # and assurance | traceability |\n                                              # quality-debt | flow | friction\n```\n\nBoth produce self-contained HTML carrying the right provenance meta tag:\n\n```html\n<meta name=''generator'' content=''keel report (computed #View; regenerate, do not commit as truth)''>\n```\n\nThe scaffolded `.gitignore` also already reserves `*.keel.html` for these outputs, so the convention exists — only the skill is out of date.\n\n## Expected\n\nEither ship `.engine/tools/report.py`, or rewrite the skill onto `keel orient --html` / `keel report <r> --html` and drop the `target/release/keel.exe` reference. The second looks correct given the CLI already does the job and needs no Python at all.\n\n## Sharper point\n\nA skill that is deployed, listed and inert is worse than an absent one. It is discoverable, so an operator finds it and follows it, and the failure only surfaces at the shell. `keel sync-claude --check` reports drift in the keel-owned `.claude/` surface but did not catch this, because the skill file is present and internally consistent — it is only its *referent* that is missing. A check that a skill's named tool paths resolve would have caught it.\n\n## Environment\n\n```\nkeel 0.3.0\nbuild commit: c444ca5\napi contract: 2.0.0 (derived)\nguards: 51 (42 hard-blocking, 9 warning-only)\nclaude surface: 0.3.0 (generated from this binary)\n```\nWindows 11. Project scaffolded 2026-08-29 via `keel init . --profile strict`. 38 skills deployed.\n";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/35";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st046 : Statement {
        :>> id = "8e48b0ff-9dac-4952-8a5e-55279b2af3ec";
        :>> title = "GH#34 keel trace reports no edges for marker-carried dependencies while marker-census counts 111 in the same model";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "## Summary\n\n`keel trace <item>` reports `upstream: []` / `downstream: []` for every item tested, in a model where `keel marker-census` AST-counts **111 marker edges**. Two computed views disagree about the same tree.\n\n`keel --help` describes trace as *''every typed edge reaching an item, both directions''*, which promises the marker-authored edges are included.\n\n## Observed\n\nAuthored in `.tracking/intake/`, both endpoints declared as parts in the same model:\n\n```sysml\n#DerivedFrom dependency from us18 to s8;\n```\n\n`keel validate .` — clean, 7 files. Then:\n\n```\n$ keel trace us18 .\n{''name'':''us18'',''upstream'':[],''downstream'':[]}\n\n$ keel trace s8 .\n{''name'':''s8'',''upstream'':[],''downstream'':[]}\n```\n\nSame empty result for 7 items spanning both trees and four item kinds — `s1`, `s8`, `us18`, `us24`, `us31`, `supplyFlowBucketTest`, `d0230`, plus `d0226` from `.engine/reference/`.\n\nMeanwhile, on the same tree:\n\n```\n$ keel marker-census .\nDerivedFrom 37 | Supersede 32 | Resolves 17 | DependsOn 11 | JustifiedBy 10\nImplicates 2  | Verify 1     | Dispositions 1        (edges, AST-counted)\n```\n\n## Name resolution is not the cause\n\nTrace distinguishes an unknown name from an empty result, and the bare names resolve:\n\n```\n$ keel trace IntakeReadback::us18 .\nkeel trace: no item named `IntakeReadback::us18` is declared in this model. An unknown name\nexits non-zero rather than answering with an empty result, because an empty result reads as\n`this item has no edges` ...\n```\n\nSo a qualified name errors, a bare name returns the empty result — trace is finding the item and reporting that it has no edges.\n\n## Question rather than a claim\n\nTrace may be scoped to **native** typed edges (`satisfy`, `allocate`, `:>`) by design, with metadata-marker dependencies deliberately excluded. If that is the intent, this is a documentation defect rather than a logic one — but it is still a defect, because the help text says ''every typed edge'' and the engine's own `relationships.sysml` header documents markers as the edge algebra's carrier wherever no native form exists:\n\n> `verify` -> NO BASE FORM EXISTS for this edge. `#Verify` is a JUSTIFIED last resort (D0140).\n> `#DerivedFrom` is a JUSTIFIED last resort (D0140): all 37 of its edges are CROSS-metaclass ...\n\nIf the majority of a project's real graph is marker-carried by design, a trace that omits markers shows almost nothing.\n\n## Why it matters\n\n`relationships.sysml:28-30` documents suspicion propagating along `satisfy`, `verify`, `:>`, `allocate` and semantic `dependsOn`. Trace is the natural view for checking that a change actually reaches its dependents before relying on it. In this project the author's stated need is precisely that — every layered assumption verified, with invalidation propagating upward when a layer is overturned. Today the propagation can be authored but not inspected.\n\n## Environment\n\n```\nkeel 0.3.0\nbuild commit: c444ca5\napi contract: 2.0.0 (derived)\nguards: 51 (42 hard-blocking, 9 warning-only)\n```\nWindows 11. Project scaffolded 2026-08-29 via `keel init . --profile strict`. `keel validate` and `keel guard` both green at time of observation.\n";
        :>> saidBy = "williamweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::github;
        :>> sourceUrl = "https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/34";
        :>> sourceTrust = SourceTrust::untrusted;
    }

    part st047 : Statement {
        :>> id = "6eff6a82-443a-426f-a868-c6cf1f756079";
        :>> title = "An issues pull+ingest pipeline with autonomy split by repo visibility; and how other projects get the skill";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "I want an issues pull + ingest pipeline for projects generally. for private repos, auto act on issues, for public ones make plans but dont auto implement, propose decisions to implement. can this be deployed globally? how would other project pull this skill after its defined? the library?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us059 : UserStory {
        :>> id = "78090c70-8142-4d9b-97f4-baac273ab588";
        :>> title = "Autonomy follows repository visibility, and the unit travels through the library";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "issues pulled and ingested across projects, with public-repo input planning rather than acting";
        :>> soThat = "a project can take field reports continuously without executing a stranger's instructions";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "process and not architecture: the mechanism is small (a pull loop over the existing ingest path) and the substance is a POLICY about how much autonomy an input carries - which is a process rule with a guard behind it. Their two deployment questions are answered by existing mechanisms (engine-shipped unit plus library import), so they route as VIEW, not as new work.";
    }
    #DerivedFrom dependency from us059 to st047;

    part st048 : Statement {
        :>> id = "8ca9e8bb-90d9-4531-addc-9e853ac81774";
        :>> title = "Is there a Django-migrations equivalent, and can antiquated data's authoring process be known? Panel it, then propose";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "do we have a means up pulling data up to the current schema? like schema files for Django? or is that not a thing? I feel like there is a lot of antiquated data sitting out there with no indication as to what process it was authored under, thus it cannot be cleanly grandfathered in or updated or baselined or tracked. no? hold a panel to determine best implementations, then propose top ideas to me";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us060 : UserStory {
        :>> id = "20b5cbd7-5169-4b88-9c97-1dddd6b5c632";
        :>> title = "Vintage resolution: the derivation is right, its coverage is one item type and its corpus lens does not finish";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "to know what process governed any item, and which antiquated items need revisiting";
        :>> soThat = "old data can be grandfathered, updated or baselined deliberately rather than by assumption";
        :>> implication = ImplicationKind::question;
        :>> triageNote = "question and not bug: they asked whether the capability exists and asked for PROPOSALS, not implementation - so the deliverable is the panel and a ranked set of options, and building would exceed what was asked. The panel nevertheless convicted twice on measured evidence, so two Issues are filed from it: the corpus lens cannot finish, and the resolver answers confidently for item types it does not cover.";
    }
    #DerivedFrom dependency from us060 to st048;

    part st049 : Statement {
        :>> id = "29358da2-60bf-4b67-923d-3f29e6530989";
        :>> title = "Auto-update consent-gated is the choice, conditional on correctness actually being guaranteeable";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> text = "i like auto-update consent gated, if you really can guarantee correctness";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-08-30"; :>> channel = StatementChannel::chat;
    }

    part us061 : UserStory {
        :>> id = "dd25b52f-68e6-41ed-8bb6-d810ec12c322";
        :>> title = "Auto-update needs reversible-and-verified, not empty-plan; the rollback is the prerequisite";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> asA = "supervisor";
        :>> iWant = "engine updates applied automatically where correctness can be guaranteed, and consent asked where it cannot";
        :>> soThat = "projects stay current without me approving every bump, and without a bad bump landing unnoticed";
        :>> implication = ImplicationKind::question;
        :>> triageNote = "question and not designChange: their choice was CONDITIONAL - 'if you really can guarantee correctness' - so the deliverable is an honest answer to the condition, not a build. Tested: the empty-plan gate is VACUOUS (an empty plan makes migrate return before it re-stamps the pin, so auto-applying one achieves nothing), and no rollback exists - demonstrated accidentally by killing migrate with a truncating pipe, leaving unit-ids.toml written and the pin unstamped. Routed as the answer plus a named prerequisite (dcMigrationRollsBack), not as work to start.";
    }
    #DerivedFrom dependency from us061 to st049;
}
