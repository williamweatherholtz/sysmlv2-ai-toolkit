// =====================================================================
// PortableWorkingMaterialNeeds — keel as the full-service AI manager: the working material travels.
//
// SOURCE. st024, st025 and st026, recorded verbatim, translated as us033, us034, us037, us038, us041
// and us042. This is elicited direction, not derivation: every Need here cites a UserStory that cites
// an utterance.
//
// CONSOLIDATED ON PURPOSE, and this is the human's own stated principle applied to their own words:
// "Having a use case for typing A, another for typing B, etc. could be consolidated into the need for
// having a keyboard. That would be a tight, concise paring down of real functional needs." Six stories
// produced TWO Needs, not six. The consolidation is recorded per Need so it can be argued with:
//
//   nWorkingMaterialTravels   <- us034 (templates + skills travel), us037 (full-service AI manager),
//                                us041 (universal transfer is not activation), us042 (silent sync)
//   nPortableContentSurvivesEngineChange <- us038 (survives a breaking keel change)
//
// THREE STORIES DELIBERATELY PRODUCED NO NEED HERE, because a Need is not the only right answer:
//   us033 (keel ships per-project, versioned, not from PATH) is already the outcome
//         nEnginePinAndEvolution states; it gains a second #DerivedFrom edge rather than a duplicate
//         Need, since two Needs over one outcome split satisfaction and hide a gap in both.
//   us039 (GitHub issues become an intake channel) is a USE CASE of nIntakeBeforeInterpretation - a
//         channel is a way of using a capability, not an outcome. Authored as a UseCase, not a Need.
//   us040 (an autonomous polling daemon) is DEFERRED by st026 and gets no Need until the daemon
//         actor and its authority ceiling are decided. Recording a Need for deferred scope is how
//         retired work later gets credited against live direction (the D0126/issue088 class).
//
// Chartered by D0246 clause A: keel's subject is the management of AI work.
// =====================================================================
package PortableWorkingMaterialNeeds {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    requirement nWorkingMaterialTravels : Need {
        :>> id = "7ac30aa1-051b-4f8e-bfb4-43c8bcd2e878";
        :>> title = "The working material is available in every project without being managed, and availability is never activation";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "Processes, skills, templates and output formats are available in every project without the author installing, copying or approving them project by project, and that content keeps itself current without asking permission for each update. AVAILABILITY IS NOT ACTIVATION: content being present lets it be read, searched and launched; it governs nothing until the project activates it, and an activated unit's version is pinned in that project's tree so its gate cannot change without its tree changing. Content the engine carries is opaque to the engine - it is versioned, moved and merged, never interpreted - so a project's own domain vocabulary cannot enter the engine through it.";
        :>> priority = Priority::must;
    }

    requirement nPortableContentSurvivesEngineChange : Need {
        :>> id = "19f997ae-a987-49ae-9796-484877f7fecd";
        :>> title = "Portable content states the engine it works against and survives an engine that has moved on";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A unit of portable content declares the range of engine versions it is defined against, so installing it into a project outside that range produces a stated refusal at install time rather than a failure discovered later at a gate. Content that has fallen behind a moved engine has a path forward that does not require rewriting it by hand, and a unit whose declaration cannot be satisfied says so plainly instead of installing and misbehaving.";
        :>> priority = Priority::must;
    }

    // ── Provenance: each Need names every utterance it consolidates (D0216) ──────────────────────

    #DerivedFrom dependency from nWorkingMaterialTravels to us034;
    #DerivedFrom dependency from nWorkingMaterialTravels to us037;
    #DerivedFrom dependency from nWorkingMaterialTravels to us041;
    #DerivedFrom dependency from nWorkingMaterialTravels to us042;
    #DerivedFrom dependency from nPortableContentSurvivesEngineChange to us038;

    // us033 is served by an EXISTING Need rather than a new one - the second source edge, not a
    // second Need. nEnginePinAndEvolution was derived from the code (us029); this records that the
    // human independently asked for the same outcome in st024, which is corroboration, not duplication.
    #DerivedFrom dependency from nEnginePinAndEvolution to us033;
}
