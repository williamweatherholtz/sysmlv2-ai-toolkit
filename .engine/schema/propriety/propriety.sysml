// =====================================================================
// EnginePropriety — the control-propriety panel's vocabulary (D0257, st032).
//
// WHY A NEW PACKAGE AND NOT NEW CritiqueLens MEMBERS: schema/core is frozen (invariant 5), and the
// five propriety questions do not map onto the requirement-quality canon without force-fitting —
// overfit is not necessity, circumvention is not correctness. The intake rule generalises: a kind
// the vocabulary cannot express is a CHANGE to the vocabulary, never a forced fit. A non-core
// package (the schema/safety precedent) adds the vocabulary without touching the frozen surface.
//
// THE FIVE LENSES ARE THE HUMAN'S OWN QUESTIONS, verbatim from st032, grounded in the practices the
// research pass found: OVERFIT is mutation-testing's question (does the control kill VARIANTS of the
// defect, or only the specimen that motivated it); CLASSCOVERAGE is variant analysis (one instance
// -> the class); CHANGEROBUSTNESS is the change-detector-test antipattern (does it verify a
// property, or a snapshot of today's implementation); CIRCUMVENTABILITY is control validation à la
// breach-and-attack simulation (attack the control, don't inventory it); MEASURABILITY is the
// objective/qualitative split that decides whether a verdict is a receipt or a testimony (D0232).
// =====================================================================
package EnginePropriety {
    private import ScalarValues::*;
    private import EngineElement::*;

    enum def ProprietyLens {
        overfit;            // fitted to the motivating example, or to the defect class?
        classCoverage;      // does it prevent the CLASS of bad behaviour, or this one instance?
        changeRobustness;   // does it survive ordinary code evolution, or depend on incidentals?
        circumventability;  // can an actor (or accident) route around it?
        measurability;      // is the verdict objectively re-derivable, or a qualitative judgment?
    }

    // ELIMINATIVE, not affirmative (Goodenough/Weinstock/Klein): `sound` is not praise, it is
    // "credible defeaters were sought and not found". `conditional` names the defeater that stands.
    // `unsound` means the lens's question is answered against the control today.
    enum def ProprietyVerdict { sound; conditional; unsound; }

    // One finding = one target x one lens (atomic, invariant 2 — never a five-row checklist blob).
    part def ProprietyFinding :> Element {
        attribute target : String;              // the control/guard/probe/convention assessed, by name or path
        attribute lens : ProprietyLens;
        attribute verdict : ProprietyVerdict;
        attribute rationale : String;           // WHY this verdict — cites the mechanism, not vibes
        // The eliminative-induction core: the CONCRETE condition that would flip this verdict.
        // A finding with no nameable defeater is either trivially true or not yet thought through —
        // both worth seeing. `sound` verdicts name what was TRIED; `conditional` names what STANDS.
        attribute defeater : String;
    }
}
