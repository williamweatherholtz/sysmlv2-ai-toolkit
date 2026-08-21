// Review Critique — sprint reviews RUN the critique passes, and retro findings become TRACKED ITEMS.
// Governed by D0172. Exists because the evidence said the current ceremony does neither: the heredoc
// failure appears in FIVE retros of one session and zero backlog items, and the retro-backlog guard is
// satisfied by ANY co-staged tracked item, so a retro finding never needs its own.
package ProcessReviewCritique {
    private import EngineElement::*;
    private import EngineProcess::*;

    action reviewCritique : Process {
        :>> id = "9d43b0e6-2f75-4a18-8c50-64ba21ce7f03";
        :>> title = "Review Critique";
        :>> purpose = "Make the sprint review an adversarial act and the retro a producer of tracked work: three critique passes per review, and every recurring retro finding becomes a backlog item or carries a recorded reason it should not.";
        :>> cadence = Cadence::perSprint;
    }

    action rcPasses : ProcessStep {
        :>> id = "9d43b0e6-2f75-4a18-8c50-000000000r01";
        :>> title = "The review runs THREE critique passes, not a reading";
        :>> triggerCondition = "A sprint's implement gate has passed and the review gate is being formed.";
        :>> actionText = "Three passes over the sprint's deliverable, each a different attack: (1) CLAIMS - every number and done-statement in the DoD checked against the computed view or the emitted artifact, never against the code that was meant to produce it; (2) BOUNDARY - the deliverable probed at its edges (empty input, absent file, concurrent caller, flag-shaped argument), on a COPY when the probe can write; (3) SELF - the diff searched for defects the sprint itself introduced, because five of the sixteen hardening findings were in code written during that same series. A pass that finds nothing states what it checked; a pass that is skipped states why.";
        :>> producedArtifact = "Three named passes inside the review gate's procedureText, each with its findings or its checked-clean statement.";
        :>> owner = Owner::ai;
    }

    action rcFindingsAreItems : ProcessStep {
        :>> id = "9d43b0e6-2f75-4a18-8c50-000000000r02";
        :>> title = "A review finding becomes a tracked item before the gate closes";
        :>> triggerCondition = "Any critique pass surfaces a defect, gap, or risk.";
        :>> actionText = "Record it as an Issue with a resolver, or a backlog action with a DoD, IN THE SAME SPRINT - never as prose inside the gate text. A finding that lives only in a review gate cannot be resolved, re-checked, or counted, which is D0046's founding rule applied to the ceremony that was exempting itself.";
        :>> producedArtifact = "A tracked item per finding, linked before closeOut.";
        :>> owner = Owner::ai;
    }

    action rcRecurrence : ProcessStep {
        :>> id = "9d43b0e6-2f75-4a18-8c50-000000000r03";
        :>> title = "The retro asks: has this AVOIDABLE-ISSUE appeared before?";
        :>> triggerCondition = "A retro records an AVOIDABLE-ISSUE.";
        :>> actionText = "Search the prior retros for the same failure class. On the SECOND occurrence it is no longer an avoidable issue - it is a recurring defect in how the work is done, and it becomes a tracked item with a preventing change, or carries a recorded reason none is possible. The evidence for this step: one failure class (shell-mangled authoring) appeared in five retros of a single session, each time as prose, each time relearned. Manual vigilance is not a control (D0047), and a retro that only WRITES lessons is manual vigilance with better formatting.";
        :>> producedArtifact = "A backlog item or Issue for every second-occurrence finding, or its recorded justification.";
        :>> owner = Owner::ai;
    }

    action rcGuardHonesty : ProcessStep {
        :>> id = "9d43b0e6-2f75-4a18-8c50-000000000r04";
        :>> title = "The retro-backlog control must not be satisfiable by accident";
        :>> triggerCondition = "The ceremony's guards are evaluated at commit.";
        :>> actionText = "The retro-backlog guard must tie the co-staged item to THE FINDING, not merely to the commit: a commit that stages unrelated issues while its retro's finding goes untracked is the exact hole this process exists to close (issue189). Until the guard is tightened, this step is discipline - and it says so rather than claiming enforcement it does not have.";
        :>> producedArtifact = "A guard whose PASS means what it says.";
        :>> owner = Owner::ai;
    }
}
