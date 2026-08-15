// =====================================================================
// EngineDistributedCollaboration — the per-session working loop for an ASYNCHRONOUS, REMOTE,
// MOSTLY-AI team where the git remote is the ONLY coordination medium (D0129). Runs once per work
// session, around the existing per-item sprint ceremony:
//
//     actor-enrollment (once per actor+machine)
//        -> distributed-collaboration (once per session)   <-- THIS
//              -> agile-workflow sprint ceremony (once per work item)
//
// Two integration LANES, routed by change class, never by contributor discretion (D0129-A):
//   FACTS  (RECORD / EXECUTE)  -> land on the trunk via the gate-on-merged-tree loop
//   CHANGE (schema/process/gate/rule/view-meaning) + deliverable source
//                              -> reviewable proposal, independent reviewer, human acceptance
// Integration is ALWAYS ancestry-preserving: fast-forward or merge commit, never squash, never
// rebase, never force — because recorded evidence is anchored to commit identity (issue071).
// Deployed by the distributed-collaboration skill.
// =====================================================================
package EngineDistributedCollaboration {
    private import EngineElement::*;     // Cadence / Owner
    private import EngineProcess::*;

    action distributedCollaboration : Process {
        :>> id = "0d600001-0812-4001-9b3e-000000010001";
        :>> title = "Distributed collaboration session loop (sync, claim, work, land, escalate, close)";
        :>> purpose = "Let several contributors — nearly all AI, on separate machines, supervised asynchronously across timezones — work one model concurrently without duplicating each other's work, silently overwriting each other's facts, invalidating each other's evidence, or needing anyone to be awake. Coordination uses only what a git remote provides: an atomic ref update (a compare-and-swap) and an ancestry-preserving history. Every state it depends on is COMPUTED from authored facts plus git; the only things authored are irreducible facts, including the claim.";
        :>> cadence = Cadence::perItem;
    }

    action dcSync : ProcessStep {
        :>> id = "0d600002-0812-4002-9b3e-000000010002";
        :>> title = "Synchronize, then orient — never orient from a stale tree";
        :>> triggerCondition = "A work session begins, or the contributor is about to write to a shared region, or a land attempt was rejected.";
        :>> actionText = "Fetch, report divergence from the trunk (behind / ahead / diverged, and by how much), and integrate remote work ANCESTRY-PRESERVINGLY — fast-forward or merge, NEVER rebase, because a rebase rewrites local commit identity and orphans the judgedAgainst anchors of evidence already recorded, which makes computed state differ per machine (issue071). Confirm gate parity (binary present, enforced check set and engine version matching what the project declares) — a contributor on a different engine version can legitimately disagree about whether the same model is valid. THEN compute orientation. Treat evidence whose anchor is absent from this clone as UNSYNCHRONIZED, not invalid, and never as work-not-done. A computed answer is only as true as the tree it was computed from, and the tree is stale by default in a distributed team.";
        :>> producedArtifact = "A synchronized working copy, a stated divergence + gate-parity result, and an orientation computed against them.";
        :>> owner = Owner::ai;
    }
    action dcClaim : ProcessStep {
        :>> id = "0d600003-0812-4003-9b3e-000000010003";
        :>> title = "Claim the work before starting it — first to land wins";
        :>> triggerCondition = "Orientation is computed on a synchronized tree and the contributor is choosing what to do.";
        :>> actionText = "Choose from the ready frontier MINUS items already claimed by others. Record the claim as an authored fact (claiming actor, item, time, and the commit claimed against) and land it IMMEDIATELY so it is visible remotely before work begins. Mutual exclusion needs no lock server, but it does NOT come from the push (D0149): claims are written to per-actor files precisely so concurrent appends do not collide, so if two contributors claim the same item both claims LAND — the loser's push is rejected, they merge, they retry, and their claim is then as valid as the winner's. A two-clone test proved this; the earlier text asserting the remote accepts exactly one was wrong. WHO HOLDS the item is COMPUTED: the earliest un-expired claimedAt, tie-broken by the lower claim UUID, so every clone reaches the same holder without coordinating. Do not try to read who landed first from git — parallel claims are siblings of equal ancestry depth and their commit timestamps tie within a second. After re-synchronizing, read the COMPUTED holder (`keel claim --list`) rather than assuming the rejection decided it, and choose different work if someone else holds it. Never take an item another contributor holds live; a claim held past expiry without progress is computed STALE and is fair to take. If nothing unclaimed is ready, say so rather than contending.";
        :>> producedArtifact = "A landed claim fact making this contributor's intent visible to every other contributor, or a re-selection after losing the race.";
        :>> owner = Owner::ai;
    }
    action dcWork : ProcessStep {
        :>> id = "0d600004-0812-4004-9b3e-000000010004";
        :>> title = "Do the work under the ownership contract";
        :>> triggerCondition = "A claim on the item has landed.";
        :>> actionText = "Run the normal per-item sprint ceremony (agile-workflow) and author facts through the write API, under D0108's ownership contract: edit fields only on items this actor owns; for anything else ADD items and typed edges, or SUPERSEDE — never overwrite another actor's item in place. Re-synchronize before writing to a shared region, because the model may have moved since orientation. Every fact carries true provenance: the actor that actually produced it (never a default, never a human's name from an AI's session), the attestation time, and the commit it was made against. Do not record a human confirmation the human did not give — an AI actor cannot supply human acceptance regardless of instruction.";
        :>> producedArtifact = "The item's defined artifacts plus recorded facts with true provenance, authored within the ownership contract.";
        :>> owner = Owner::ai;
    }
    action dcLand : ProcessStep {
        :>> id = "0d600005-0812-4005-9b3e-000000010005";
        :>> title = "Land it — gate the tree that will actually land, by lane";
        :>> triggerCondition = "A coherent unit of work is complete and recorded, and the local gate is green.";
        :>> actionText = "Route by CHANGE CLASS, not by preference. FACTS LANE (RECORD / EXECUTE — verification results, claims, ceremony gates, new items and typed edges): loop { fetch; integrate the trunk ancestry-preservingly; run the FULL enforced gate against the RESULTING tree; push } and repeat on rejection, with bounded retries and back-off. Gating the merged tree rather than the isolated contribution is the point: two contributions that each pass alone can fail together, and this is what a merge queue exists to prevent — executed client-side, since there is no server. CHANGE LANE (schema, process, workflow, gate, rule, or the MEANING of a computed view; and deliverable source): propose it as a separately reviewable unit, let CI gate it, have it reviewed by an actor OTHER than its author, and carry the recorded human acceptance the change path already requires. Never squash, never rebase-merge, never force-push a shared ref. Never exhaust retries into a force.";
        :>> producedArtifact = "Landed work — trunk for facts, a reviewed and accepted proposal for change — with every evidence anchor still resolvable in every clone.";
        :>> owner = Owner::ai;
    }
    action dcEscalate : ProcessStep {
        :>> id = "0d600006-0812-4006-9b3e-000000010006";
        :>> title = "Escalate what must not be resolved by precedence";
        :>> triggerCondition = "A conflict, contention, or contradiction is encountered at any point in the session.";
        :>> actionText = "Distinguish the two kinds. A TEXTUAL conflict (both contributors appended to the same region) is resolved so that BOTH facts survive, and the resolution is then subjected to the FULL gate before it can land — a textually plausible resolution can still produce duplicate identity or a broken reference, and merge commits are exactly where that slips through. A SEMANTIC conflict (two conclusions that cannot both be true; two decisions governing one subject; contention for one item) is NOT the contributor's to settle: record it as a tracked Issue naming both positions and their bases, so it enters what awaits human authority, and let the HUMAN adjudicate (D0108). Never resolve such a conflict in favour of your own conclusion, and never concede silently — either way the disagreement, which is usually information about an upstream ambiguity, is lost. If conflicts recur on the same file, record that as an observation: it means the write targets should be separated, not that friction is normal.";
        :>> producedArtifact = "A gated resolution for textual conflicts; a tracked Issue awaiting human adjudication for semantic ones.";
        :>> owner = Owner::either;
    }
    action dcClose : ProcessStep {
        :>> id = "0d600007-0812-4007-9b3e-000000010007";
        :>> title = "Close the session — release the claim, leave nothing in conversation";
        :>> triggerCondition = "The claimed work is landed, or the session is ending with the work incomplete.";
        :>> actionText = "Release the claim (or leave it to expire, which releases the CLAIM but never the recorded facts — partial work stays evidence). Confirm everything needed to continue is IN THE MODEL: there is no prose handoff, no status document, no worklist (D0018) — the next contributor, who may be a different actor on a different machine hours later, orients from computed state. Anything a successor would need that is not recorded is a defect: record it as a fact, a Decision, or an Issue, not as a note. Items genuinely requiring human authority accumulate in the batched queue for the asynchronous supervisor (D0049) — do not block on them, and do not pre-empt them by recording a judgment the human did not make.";
        :>> producedArtifact = "A released claim, a fully-recorded session state, and any human-authority items queued for batched review.";
        :>> owner = Owner::ai;
    }

    // distributedCollaboration: step order, native succession (D0143 — was `order : Integer`)
    first dcSync then dcClaim;
    first dcClaim then dcWork;
    first dcWork then dcLand;
    first dcLand then dcEscalate;
    first dcEscalate then dcClose;
}
