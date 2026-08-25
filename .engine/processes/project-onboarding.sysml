// Project onboarding — work out what this project IS, then choose its process set on that basis and
// record why. Governed by D0225. The procedure the project-onboarding skill deploys.
//
// WHY THIS EXISTS (issue257). `keel init` lays down a working project with every process active and
// never asks the author what they are building. The author then faces 24 processes with no basis to
// judge them, and has exactly two bad options: keep all of them and meet friction they never chose
// (D0054 names adoption friction as the top risk, above bad architecture), or switch off the ones
// whose names they do not recognise — which is the worst available selection rule, because the units
// with the least familiar names are the ones doing the least obvious work.
//
// THE DESIGN TENSION, and it is the whole problem. D0216 forbids offering a menu, because a chosen
// option is evidence about the menu rather than about the chooser's work. But picking a process set
// from a catalogue IS a menu. The resolution: onboarding never asks "which processes do you want".
// It asks about the WORK — what is being built, who is hurt when it goes wrong, what must be
// provable and to whom — records those answers verbatim as Statements, and only then RECOMMENDS,
// with the reasoning shown per process so the author can reject any line of it. The recommendation
// is an argument, not a form.
//
// WHY THE CONDITION LIVES BESIDE EACH PROCESS. Every process definition declares an `// APPLIES-WHEN:`
// line stating the SITUATION in which a project needs it — never a domain label. The author named
// "HW, SW, all, business processes, safety-related, art, video games, construction plans, whatever",
// and "whatever" is part of the requirement: a closed domain table cannot serve a domain nobody
// listed, but a condition can be matched against any project's elicited facts. Beside the process
// rather than in a central table because a central table cannot travel with a unit — the defect that
// made 23 of 24 units land red in a project that lacked them (issue253/D0222).
package ProcessProjectOnboarding {
    private import EngineElement::*;
    private import EngineProcess::*;

    action projectOnboarding : Process {
        :>> id = "b4f19a27-6c53-4e18-9d70-2a815f63c904";
        :>> title = "Project onboarding (choose the process set, and record why)";
        :>> purpose = "Establish what a project is building and what must not go wrong with it, then charter its process set from those facts with per-process reasoning the author can reject — so no project runs a discipline nobody chose, and none silently lacks one it needed.";
        :>> cadence = Cadence::perItem;
        // APPLIES-WHEN: this project's process set has never been chartered - `keel onboard` reports NOT CHARTERED - which is true of every freshly scaffolded project and of any project that adopted the engine without deciding what to adopt
    }

    action poElicit : ProcessStep {
        :>> id = "b4f19a27-6c53-4e18-9d70-000000000c01";
        :>> title = "Ask about the work, never about the processes";
        :>> triggerCondition = "`keel onboard` reports NOT CHARTERED, or the author asks which processes their project should run.";
        :>> actionText = "Deploy `business-elicitation` (D0216) against the PROJECT rather than a product need. Ask what is being built and for whom; what has gone wrong before or would be expensive to get wrong; who has to be convinced and with what evidence; who else touches the work; what has to be provable later and to whom. NEVER present the process catalogue as options: a chosen option is evidence about the menu, not about their work. Record each answer VERBATIM as a `Statement` before interpreting any of it — the reading and the words stay separate, separately auditable facts.";
        :>> producedArtifact = "One `Statement` per answer, in the author's own words, recorded before any recommendation exists.";
        :>> owner = Owner::either;
    }

    action poMatch : ProcessStep {
        :>> id = "b4f19a27-6c53-4e18-9d70-000000000c02";
        :>> title = "Match the elicited situation against each process's declared condition";
        :>> triggerCondition = "Statements have been recorded and the project's situation is stated.";
        :>> actionText = "Read every process's `// APPLIES-WHEN:` condition (`keel onboard`) and decide, per process, whether the elicited facts satisfy it. Match on the SITUATION, not on the domain word: 'safety-related' is not itself a reason to adopt anything — the reason is that an element being wrong is expensive, which is what element-critique's condition states. A process whose condition is NOT met is a recommendation AGAINST, recorded as such; silence is not a decision.";
        :>> producedArtifact = "A per-process verdict — adopt or not — each tied to the elicited fact that decides it.";
        :>> owner = Owner::ai;
    }

    action poResearch : ProcessStep {
        :>> id = "b4f19a27-6c53-4e18-9d70-000000000c03";
        :>> title = "Where the engine has no process for the domain's practice, RESEARCH it";
        :>> triggerCondition = "The elicited work implies a discipline no declared process covers — a domain practice, a regulatory regime, a review or approval step the engine has never modelled.";
        :>> actionText = "Research what practitioners in that domain actually do, and record what you found WITH ITS SOURCE — never invent a standard and never attribute a practice to a body that did not publish it. If research is unavailable, say so plainly rather than guessing; an unresearched gap recorded as a gap is honest, and a fabricated best practice is not. Then either map the practice onto an existing process, or DEFINE a new one — section 3 says defining the process is the creative output when a request maps to none.";
        :>> producedArtifact = "A recorded finding with provenance, and either a mapping to an existing process or a new process definition.";
        :>> owner = Owner::ai;
    }

    action poRecommend : ProcessStep {
        :>> id = "b4f19a27-6c53-4e18-9d70-000000000c04";
        :>> title = "Put the recommendation to the author as an argument they can reject";
        :>> triggerCondition = "Every declared process has a verdict and the gaps have been researched.";
        :>> actionText = "Present the recommended set with, for each process, the elicited fact that implicates it and the cost of adopting it — and present what is NOT recommended with the same reasoning. State the friction honestly: every adopted process is a discipline someone has to carry out. Ask what is WIDER than they meant rather than whether the set is good (D0157: the finding there was that a Need had been written wider than the demand, not that it was wrong).";
        :>> producedArtifact = "A recommendation the author has seen the reasoning for, and can strike any line from.";
        :>> owner = Owner::either;
    }

    action poCharter : ProcessStep {
        :>> id = "b4f19a27-6c53-4e18-9d70-000000000c05";
        :>> title = "Record the choice as a Decision and charter the activation set";
        :>> triggerCondition = "The author has settled the set — including any line they struck.";
        :>> actionText = "Record a `Decision` stating the chosen set, the elicited facts behind it, and what was deliberately NOT adopted and why — a rejection is a first-class Decision even though it causes no action (invariant 4). Then write `.engine/contracts/activation.toml` to match, declaring `charteredBy = \"dNNNN\"` under `[processes]`. The charter is what makes the set explicable later: an activation file without one is a set of switches nobody can account for.";
        :>> producedArtifact = "A Decision recording the chosen set and the rejections, and an activation.toml naming it as its charter.";
        :>> owner = Owner::ai;
    }

    action poVerify : ProcessStep {
        :>> id = "b4f19a27-6c53-4e18-9d70-000000000c06";
        :>> title = "Read the result back, including what the chosen set does not cover";
        :>> triggerCondition = "The charter has been recorded.";
        :>> actionText = "`keel onboard` reports CHARTERED by that Decision; `keel activation` reports the declared set; `keel guard` is green. Then state the RESIDUAL out loud: the conditions that were met but whose process the author declined, and any researched practice the engine still has no process for. A project should know what it chose not to do — that is the difference between a scoped adoption and an unexamined one.";
        :>> producedArtifact = "A read-back confirming the charter, and a stated list of what the chosen set does not cover.";
        :>> owner = Owner::ai;
    }

    // projectOnboarding: step order, native succession (D0143)
    first poElicit then poMatch;
    first poMatch then poResearch;
    first poResearch then poRecommend;
    first poRecommend then poCharter;
    first poCharter then poVerify;
}
