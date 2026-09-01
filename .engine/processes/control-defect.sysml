// controlDefect — what to do the moment a CONTROL is found to be wrong (D0278).
//
// WHY THIS IS ITS OWN PROCESS AND NOT A SEVERITY LABEL ON issue-resolution. An ordinary defect
// affects what it touched. A defect in a control INVALIDATES ITS OWN HISTORY: every verdict it gave
// while broken is unverified, and nobody knows which ones. That blast radius is retroactive and
// unbounded until somebody assesses it — a question no ordinary bug raises and no existing process
// asks. The human's instruction was "when you encounter a huge issue, like guards over/under
// reporting, they need to be fixed ASAP", and the ASAP follows from the retroactivity: every hour a
// broken control keeps running, it adds verdicts nobody can trust.
//
// THE EVIDENCE THAT THE GAP IS REAL, counted rather than felt. On 2026-09-01 I found three control
// defects — `issues` under-reporting a mis-triage, `decision-scaffolding` under-reporting a
// half-built compound Decision, `ownership` over-reporting on actor attribution — and recorded
// ZERO of them as Issues. All three went into sprint-retro prose and a commit message, which is the
// one place D0018 says state must never live. They were true, they were written down, and they were
// unfindable and unactioned. This process exists because noticing was never the problem.
//
// WHAT CANNOT BE ENFORCED, said plainly so nobody mistakes this for a closed loop. NOTICING is
// conversational: you find a control defect by being SURPRISED at a verdict, and no gate can read
// surprise. That is the same class as the response contract's verify-before-asserting clause
// (D0151). What IS enforced is everything after the noticing — the registry must resolve, and a
// control listed in it announces itself beside its own verdict, so a known-broken control cannot
// hand out a quiet green.
package ProcessControlDefect {
    private import EngineElement::*;
    private import EngineProcess::*;

    action controlDefect : Process {
        :>> id = "d0278a01-0901-4b2c-8e11-1f2a3b4c5d60";
        :>> title = "Control defect";
        :>> purpose = "Turn a wrong verdict from a guard, gate, hook or lens into a tracked, classified, blast-radius-assessed Issue on the spot — and make the defective control declare itself every time it speaks, so its history is never read as sound.";
        :>> cadence = Cadence::eventDriven;
        // APPLIES-WHEN: any control's verdict is found to be wrong in either direction. NOT for a
        // control that is merely noisy-but-correct (that is a tuning question) nor for a control
        // whose finding you dislike.
    }

    action cd0Notice : ProcessStep {
        :>> id = "d0278a01-0901-4b2c-8e11-1f2a3b4c5d61";
        :>> title = "Catch yourself explaining a verdict away";
        :>> triggerCondition = "A control's verdict surprises you - it flagged something you believe is fine, or it passed something you know is not.";
        :>> actionText = "The tell is linguistic and worth learning, because it is the only detector there is: you have just written or thought 'the guard is wrong here', 'that is a false positive', 'it passed but it should not have', or - the most dangerous one - 'that is the guard working correctly' about a verdict you had to think about. STOP THERE. The cost of being wrong in each direction is asymmetric: an over-report you dismiss costs you a minute; an under-report you dismiss costs every green the control has ever given. And if you find an OVER-report, go looking for the matching UNDER-report before you move on - one wrong predicate usually does both, on different inputs.";
        :>> producedArtifact = "A stated suspicion, not yet a finding: the control named, and what you expected it to say.";
        :>> owner = Owner::either;
    }

    action cd1Establish : ProcessStep {
        :>> id = "d0278a01-0901-4b2c-8e11-1f2a3b4c5d62";
        :>> title = "Establish the defect against the control's SOURCE, not its output";
        :>> triggerCondition = "A suspicion exists.";
        :>> actionText = "Read the predicate. A verdict that looks wrong is often a control being right about something you had not considered, and recording a defect against a working control is worse than recording nothing - it discredits the control and wastes the next reader. Name the file:line and say what the predicate ACTUALLY tests versus what it is relied on to test. That gap, stated in one sentence, IS the defect. If you cannot state it, you have a disagreement rather than a finding.";
        :>> producedArtifact = "The predicate at file:line, and one sentence naming the gap between what it tests and what it is trusted to test.";
        :>> owner = Owner::either;
    }

    action cd2Classify : ProcessStep {
        :>> id = "d0278a01-0901-4b2c-8e11-1f2a3b4c5d63";
        :>> title = "Say which direction, because the two are not the same defect";
        :>> triggerCondition = "The defect is established.";
        :>> actionText = "OVER-reporting flags what is fine. It is loud, someone hits it, and its damage is to trust: an over-reporting gate is one people learn to route around, which is the D0214 unread-warning failure arriving through a different door. UNDER-reporting passes what is broken. It is SILENT - nothing surfaces it, and it is found only by accident or by auditing the control itself. An under-reporting defect is therefore never below High severity regardless of how small the case looks, because its severity is not the case, it is the unknown population behind it.";
        :>> producedArtifact = "A direction - over or under - and a severity floored at High for under-reporting.";
        :>> owner = Owner::either;
    }

    action cd3Invalidate : ProcessStep {
        :>> id = "d0278a01-0901-4b2c-8e11-1f2a3b4c5d64";
        :>> title = "Ask what it PASSED while broken - the step no ordinary defect has";
        :>> triggerCondition = "The direction is known.";
        :>> actionText = "This is the whole reason control defects are their own class. Write down what this control let through while it was wrong, and whether any of it was re-checked. Three honest answers are available and only these three: RE-CHECKED (say how, and `keel reverify` is usually the tool); ACCEPTED AS-IS (say why the exposure is tolerable, which is a judgment and must read like one); or NOT RE-CHECKED (say so). The unacceptable answer is silence, because silence reads as re-checked to everyone who comes later. For an over-reporting defect this section is usually short - its errors are refusals, not false greens - but write it anyway, since the reader cannot know that without being told.";
        :>> producedArtifact = "An invalidation statement naming the exposed population and its disposition.";
        :>> owner = Owner::either;
    }

    action cd4Register : ProcessStep {
        :>> id = "d0278a01-0901-4b2c-8e11-1f2a3b4c5d65";
        :>> title = "Record the Issue AND register it against the control, in the same act";
        :>> triggerCondition = "Direction and invalidation are known.";
        :>> actionText = "Record the Issue with a resolver, exactly as any Issue - and then add a section to `.engine/contracts/control-defects.toml` keyed by the CONTROL's name, carrying the issue id, the direction, the effect and the invalidation. The registry is not bookkeeping: it is what makes the next step work. Do both in the same act. The failure this prevents is the one that produced this process - three true findings written into retro prose, where they were unfindable, uncounted and unresolved.";
        :>> producedArtifact = "A tracked Issue with a resolver, plus its registry entry.";
        :>> owner = Owner::ai;
    }

    action cd5Announce : ProcessStep {
        :>> id = "d0278a01-0901-4b2c-8e11-1f2a3b4c5d66";
        :>> title = "The defective control now says so beside its own verdict";
        :>> triggerCondition = "The registry entry exists.";
        :>> actionText = "Nothing to do - this is what the registry BUYS, and it is stated as a step so the reader knows to expect it. `keel guard` prints each registered defect beside the verdict of the control it belongs to, so a green from a control known to under-report arrives already qualified. That is the hook: it fires at the moment of reading rather than depending on anyone remembering, and it keeps firing until the Issue is resolved and the entry removed. A control-defect entry whose Issue does not exist or is already closed fails `control-defect-registry`, so the announcement cannot rot into noise.";
        :>> producedArtifact = "Every subsequent verdict from that control, qualified.";
        :>> owner = Owner::ai;
    }

    action cd6FixOrCarry : ProcessStep {
        :>> id = "d0278a01-0901-4b2c-8e11-1f2a3b4c5d67";
        :>> title = "Fix it now, or carry it deliberately - and say which";
        :>> triggerCondition = "The defect is registered.";
        :>> actionText = "ASAP means the DEFAULT is to fix it in the current turn, because a broken control keeps issuing untrustworthy verdicts for as long as it runs. Two reasons are good enough to carry it instead, and both must be written into the Issue: the fix is a keystone change needing a signed Decision, or fixing it now would bury a change already in flight. 'I was busy' is not one of them. When the fix lands it must be ARMED (D0253) against the broken predicate - a control-defect fix that was never observed to change a verdict has not been shown to fix anything - and the registry entry is removed in the same commit as the Issue's resolution, never before.";
        :>> producedArtifact = "Either a landed, armed fix with its registry entry removed, or a written reason for carrying it.";
        :>> owner = Owner::either;
    }
}
