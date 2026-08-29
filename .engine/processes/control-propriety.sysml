// controlPropriety — the propriety panel over controls, guards, probes and conventions (D0257).
//
// Chartered by st032: "for each existing and proposed solution, hold a panel to assess solution
// propriety" — five named questions. Distinct from element-critique (requirement-quality lenses over
// model elements) and from adversarial-panel-review (persona panels over design artifacts): this
// process assesses ENFORCEMENT MECHANISMS, and its grounding is the control-validation literature —
// mutation variants over specimens, defeaters over affirmations, attack over inventory.
package ProcessControlPropriety {
    private import EngineElement::*;
    private import EngineProcess::*;

    action controlPropriety : Process {
        :>> id = "b3a1e6c4-77d2-4c1e-9f10-5a2b8c9d0e11";
        :>> title = "Control-propriety panel";
        :>> purpose = "Assess each enforcement mechanism against five adversarial lenses - overfit, class coverage, change robustness, circumventability, measurability - with a named defeater per verdict, so a control's soundness is an argued claim rather than an assumed one.";
        :>> cadence = Cadence::eventDriven;
        // APPLIES-WHEN: the project declares controls, guards or probes whose soundness is asserted
        // rather than argued - any project past its first guard qualifies
    }

    action cpInventory : ProcessStep {
        :>> id = "b3a1e6c4-77d2-4c1e-9f10-5a2b8c9d0e12";
        :>> title = "Inventory the targets, in tranches sized to be done WELL";
        :>> triggerCondition = "A new or changed enforcement mechanism lands, or a panel round over the existing set is called.";
        :>> actionText = "Enumerate the mechanisms to assess: declared controls (the control map), guards, arming probes, and load-bearing conventions (a contract field, a naming rule). A PROPOSED solution is assessable before it is built - its design is the target. Tranche the list rather than sweeping it: a panel run that covers everything shallowly certifies nothing.";
        :>> producedArtifact = "A named tranche of targets.";
        :>> owner = Owner::ai;
    }

    action cpAssess : ProcessStep {
        :>> id = "b3a1e6c4-77d2-4c1e-9f10-5a2b8c9d0e13";
        :>> title = "Assess one target under all five lenses, adversarially";
        :>> triggerCondition = "A tranche exists.";
        :>> actionText = "Per lens, argue AGAINST the mechanism before crediting it. OVERFIT: mutate the motivating defect - would the variant slip past? CLASSCOVERAGE: name the class, then name what is IN the class but OUTSIDE the mechanism's reach. CHANGEROBUSTNESS: name the incidental facts (a file format, a naming convention, a prefix) the mechanism silently depends on, and what happens when each moves - a check that scans 0 and passes has died without a sound. CIRCUMVENTABILITY: how does an actor route around it (a bypass flag, a plausible 'none' note, an unverified pointer), and what backstops the route? MEASURABILITY: is the verdict re-derivable by a third party from what is recorded, or does it rest on the assessor's word (D0232 receipt vs testimony)?";
        :>> producedArtifact = "Argued positions per lens, ready to become findings.";
        :>> owner = Owner::ai;
    }

    action cpVerdict : ProcessStep {
        :>> id = "b3a1e6c4-77d2-4c1e-9f10-5a2b8c9d0e14";
        :>> title = "Record one ProprietyFinding per lens, each with its DEFEATER";
        :>> triggerCondition = "An assessment is argued.";
        :>> actionText = "One atomic ProprietyFinding per target x lens: verdict (sound/conditional/unsound), rationale citing the mechanism, and the DEFEATER - for sound, the refutations that were TRIED and failed; for conditional, the concrete condition that stands; for unsound, the demonstration. Eliminative induction: confidence comes from defeaters sought and eliminated, never from affirmation. A finding whose defeater cannot be named is returned to cpAssess, not recorded.";
        :>> producedArtifact = "ProprietyFinding items, five per target.";
        :>> owner = Owner::ai;
    }

    action cpRoute : ProcessStep {
        :>> id = "b3a1e6c4-77d2-4c1e-9f10-5a2b8c9d0e15";
        :>> title = "Route every unsound - and every actionable conditional - to tracked work";
        :>> triggerCondition = "Findings are recorded.";
        :>> actionText = "An unsound verdict is a defect in a CONTROL, which outranks a defect in code: it becomes an Issue with a resolver, never a note (D0047). A conditional whose defeater is cheap to eliminate becomes work; one whose defeater is accepted is recorded via the disposition lifecycle (D0092) so the acceptance is a human-visible verdict rather than a silence. Coverage - which targets carry a complete five-lens assessment - is computable from the findings.";
        :>> producedArtifact = "Issues with resolvers, or dispositions, for every non-sound finding.";
        :>> owner = Owner::ai;
    }

    action cpIndependence : ProcessStep {
        :>> id = "b3a1e6c4-77d2-4c1e-9f10-5a2b8c9d0e16";
        :>> title = "State the independence of the panel, honestly";
        :>> triggerCondition = "A round is complete.";
        :>> actionText = "The assessor is recorded on every finding. Where the assessor AUTHORED the mechanism under assessment, the round summary says so - an author auditing their own control is a real bias, and this project's evidence is that self-audit still finds real defects provided the stance is eliminative. An unsound verdict never requires independence to record (an author may always convict their own work); crediting sound on a Critical-severity control follows the critic-independence rule (D0080): an aiModel author's sound verdict on their own mechanism is a testimony, and the round summary names it as one.";
        :>> producedArtifact = "A round summary stating who assessed what, and which sound verdicts are self-judged.";
        :>> owner = Owner::ai;
    }

    first cpInventory then cpAssess;
    first cpAssess then cpVerdict;
    first cpVerdict then cpRoute;
    first cpRoute then cpIndependence;
}
