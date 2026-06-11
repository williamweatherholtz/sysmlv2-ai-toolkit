// =====================================================================
// BusinessWorkflow — onboarding / "what-why", as a native action behavior.
// Brief -> Personas -> Needs -> Use Cases (sequential succession + item flow).
// Each action's in/out is the artifact it consumes/produces. See spec §5.
// =====================================================================
package BusinessWorkflow {
    private import EngineNeeds::*;          // Brief, Persona, Need, UseCase (canonical schema types)

    action def Business {
        action brief { out o : Brief; }
        action personas { in i : Brief; out o : Persona; }
        action needs { in i : Persona; out o : Need; }
        action useCases { in i : Need; out o : UseCase; }

        first brief then personas;
        first personas then needs;
        first needs then useCases;

        flow from brief.o to personas.i;
        flow from personas.o to needs.i;
        flow from needs.o to useCases.i;
    }
}
