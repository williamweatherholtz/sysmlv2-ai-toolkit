// ProjectActors — this project's actor registry (INSTANCE data). EDIT to your real actors.
// The declared part name is the id that createdBy/judgedBy reference (enforced by `keel guard actors`).
package ProjectActors {
    private import EngineElement::*;

    part you : Person { :>> name = "Your Name"; :>> email = "you@example.com"; }
    part ai : Actor { :>> name = "AI assistant"; :>> kind = ActorKind::ai; }
}
