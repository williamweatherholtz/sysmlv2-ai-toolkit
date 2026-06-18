// =====================================================================
// D0070 — process-change recording = a PREFIX MARKER on the Decision (not a stored edge).
// Supersedes D0069's edge-carrier sub-mechanism; restates D0069's surviving architecture.
// A process change is recorded by #ProspectiveChange / #SafetyChange ON the Decision part;
// which-process + when are COMPUTED from git (the process-def file(s) in that Decision's
// commit), made reliable by a hard-lock keystone guard. The floating .tracking edge file is
// deleted; issue021 (cross-package kernel limit) dissolves — no cross-reference remains.
// Accepted 2026-06-18 by wweatherholtz ("proceed on the basis you're presenting ... confirmed";
// "i want the truth stored in the data, not separate metadata files floating around";
// "i'm fine with a hard lock on processes without explicit change confirmation, including typos").
// =====================================================================
package Decision0070 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // dogfood: this Decision changes process-GOVERNANCE (how process changes are recorded) — a prospective process change (D0062 default).
    #ProspectiveChange part d0070 : Decision {
        :>> id = "63940516-2c7d-4e8f-a070-516273849600";
        :>> title = "D0070: record a process change by a #ProspectiveChange/#SafetyChange PREFIX MARKER on the Decision + git-derived linkage under a hard lock; supersedes D0069's stored-edge carrier";
        :>> createdAt = "2026-06-18";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0069 specified the process-change record as a #Changes dependency EDGE from the Decision to the Process it changed, carrying the retroactivity class. Implementing it (sprint39, pglProcessRef) hit issue021: validate_instances loads each .engine file in kernel isolation (schema/core only), so a Decision file cannot reference a process/workflow element — the edge had to be exiled to a floating .tracking/process-governance.sysml. On review the user rejected that: 'i want the truth stored in the data, not separate metadata files floating around', and authorized a HARD LOCK on process changes 'without explicit change confirmation, including typos ... No need to interpret intent.' Under that hard lock the edge's process-linkage and timing become GIT-DERIVABLE, so the separate file is redundant duplication of git.";
        :>> decision = "RECORD a process change by a metadata PREFIX MARKER on the Decision part itself — `#ProspectiveChange part dNNNN : Decision { ... }` (or #SafetyChange) — in the Decision's own file. The prefix-marker form is validated in BOTH the rust authority (D0048) and the kernel; note the `{ @Marker; }` MEMBER form parses in the kernel but the rust parser rejects it ('unexpected character @'), so the prefix form is the portable one. NO cross-package reference exists and issue021 DISSOLVES. The marker's PRESENCE = 'this Decision is a process change'; its KIND = retroactivity class: prospective (default doctrine, D0062 — outputs chartered under the prior version stay valid) or safety (downstream items are mandatory reprocess candidates). COMPUTE the rest (no stored linkage): which process(es) a Decision governs + the effective commit = the process-def file(s) changed in that Decision's commit, read by git — reliable BECAUSE the keystone hard-lock guarantees the co-commit. HARD LOCK (keystone guard, D0047, no intent interpretation): any staged change to a process-def file (.engine/processes/* or .engine/workflows/*) MUST be co-committed with a Decision bearing #ProspectiveChange/#SafetyChange, or the commit FAILS — typos included; pre-guard process files are grandfathered by a recorded cutoff. DELETE .tracking/process-governance.sysml. Everything else in D0069 STANDS and is restated: guarded recording + a git-traversal RESOLVER (governing-version / process-as-it-was / valid-then? / reprocess-candidates, reusing _is_ancestor + git show); REJECT per-item process-version stamps and a separate content-hash; work->process by CONVENTION/kind; the charter guard (a Story without #CharteredBy is flagged).";
        :>> rationale = "The .tracking edge stored two things: (a) which process, (b) the retroactivity class. (a) DUPLICATES git once the hard lock guarantees the Decision co-commits with the process-def change (D0013 git-only temporal) — storing it is exactly the floating metadata D0069's own output #3 set out to avoid. (b) is irreducible JUDGMENT and belongs ON the Decision (the explicit change confirmation), not a parallel file. The prefix-marker form is valid in-package in both rust and the kernel, so it sidesteps issue021 entirely — the cross-package problem was an artifact of storing a reference we don't need. PRINCIPLED LINE (so this is not ad hoc): store only what is NOT derivable — process-change linkage IS git-derivable (remove it); decision SUPERSESSION is NOT git-derivable (keep it, as the typed status=superseded field). ALTERNATIVES: (A) keep the .tracking edge — rejected: floating duplication of git, against minimal-authored-metadata; (B) add a scalar processChange enum FIELD to the frozen Decision schema — rejected: a schema change + migration for what a prefix marker expresses with zero schema churn; (C CHOSEN) prefix marker on the Decision + git-derived linkage under a hard lock. CRITERIA: minimal-authored x truth-on-the-item x derivable-stays-derived x kernel-valid-without-workaround.";
        :>> consequences = "Supersedes D0069 (status -> superseded; D0070 is the current authority on process-governance resolution, citing D0068 for direction). Reshapes the increments: pglProcessRef (sprint39 — the .tracking edge) is superseded by pglProcessMarkerCarrier (sprint40, this Decision): apply #ProspectiveChange to the historical process-change Decisions (D0039/D0049/D0064/D0067 — grandfathered for resolution, NO fabricated linkage per D0067), delete the edge file, retarget query.py to read the marker + derive process linkage from git, update relationships.sysml comment + usage/syntax docs, mark issue021 superseded-by-design. pglProcessChangeGuard (sprint41) = the keystone hard lock. pglViews (sprint42) = the git-traversal resolver (intent unchanged). SUPERSESSION is recorded FORMALLY: DecisionStatus::superseded on D0069 (a typed schema field, not internal metadata) + this Decision naming it; the back-reference (what supersedes X) is computed. A cross-decision #Supersede edge is intentionally NOT authored — it would re-hit issue021 and we do not reintroduce a floating file for a relation the status field already records. Pairs with D0013 (git temporal), D0001/D0005 (compute-don't-store), D0062 (prospective default), D0047 (corrections->guards). This Decision is resolvable by the very mechanism it defines (it is itself #ProspectiveChange-marked).";
    }

    verification d0070Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a070-516273849601";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of decision d0070 (process change recorded by a prefix marker on the Decision + git-derived linkage under a hard lock; supersedes D0069's stored edge; .tracking edge file deleted; historical decisions grandfathered) via the explicit dialogue + 'proceed on the basis you're presenting ... confirmed' on 2026-06-18.";
    }
    part d0070AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a070-516273849602";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "683b5ef";
        :>> judgedAt = "2026-06-18";
        :>> judgedBy = "wweatherholtz";
    }
}
