// D0354 (status: the `status` field below is the truth) — standingSummaryUsesTheKit: the standing executive summary is rendered from the asi-templates exec-summary surface, and the unit's contract checker is this repository's gate on it
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0354Accept event on sign-off.
package Decision0354 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0354 : Decision {
        :>> id = "69f0ba9c-af65-423b-8efd-a68a39d58fa0";
        :>> title = "standingSummaryUsesTheKit: the standing executive summary is rendered from the asi-templates exec-summary surface, and the unit's contract checker is this repository's gate on it";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human's direction, verbatim (st098, us097): 'use the asi template skill for this standing executive summary process.' The standing summary - the one published page carrying what waits on them - was hand-authored HTML: sections of prose, no steel-manned alternatives, no reasoning chain, no way to copy a decision back into a session. The asi-templates unit v3 sits in the library, uninstalled here, and carries the surface their own standing direction describes: a decision page that is extremely concise, recommendation-first, tabbed, diagrammed and one-click copyable. The unit declares no guard, so importing the files alone would land the skill and leave the teeth behind - the failure its manifest exists to prevent.";
        :>> decision = "The asi-templates unit v3 is imported into this project and its exec-summary surface renders the standing executive summary: one tab per decision with steel-manned choices carrying their real costs, the reasoning chain naming the criterion before the comparison, a mechanism diagram where the subject has structure, and the Copy-for-AI digest that carries the reasoning rather than the verdict; the source project is named in the title, the badge and the digest's first line; the page is published to the one standing artifact URL so the queue keeps one link; the brand baseline is re-verified before a build and never refreshed to bump its date; and because the unit ships no guard, this repository's pre-commit hook runs scripts/check_templates.py whenever templates/ or the checker itself is staged, blocking loudly when python is absent rather than skipping. NOT A FORK: the human named the surface; what is recorded here is the adoption and the gate that makes it enforceable.";
        :>> rationale = "Why import rather than copy the template: the unit carries the brand tokens, the contract checker and the harness together, and a copied template drifts from the checker that defines it - which is the drift the unit's manifest names. Why wire the checker into pre-commit: the unit declares no guard, so without this the only enforcement is my own discipline, and a discipline is not a control (D0047). Why block when python is missing: a template change landing unchecked is the silent-pass class the hook's own K2 comment refuses; the rust step already has this shape. Arming (D0298): the checker over the rendered page and over templates/ (clean), the harness at 8/8 against the rendered page including the mobile one-tap copy, the brand comparator reporting no drift on every observed field, and the hook step itself, which refuses a staged templates/ change that breaks the contract.";
        :>> consequences = "The standing summary is a document with a contract instead of prose I compose each time; a page that breaks the contract cannot be delivered, and a template edit cannot be committed. The repository gains 39 files (template, brand tokens, vendored fonts, checker, harness) and a python dependency on the templates path. The decision-surfacing process keeps its rule of one published page at one URL; only the surface changes.";
    }
    #DerivedFrom dependency from d0354 to st098;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0354Accept : Test { :>> id = "67d39b3b-b32c-44bc-9e2d-68fbf6959bf3"; :>> method = VerificationMethod::confirmation; :>> procedureText = "their direction in chat, verbatim: 'use the asi template skill for this standing executive summary process.'"; }
    part d0354AcceptR1 : TestResult { :>> id = "b1de4f92-48ac-47cc-b938-9df0b0d2b0e9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "24911a2"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
