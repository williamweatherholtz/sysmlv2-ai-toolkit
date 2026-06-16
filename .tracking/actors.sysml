// =====================================================================
// ProjectActors — the actor registry (INSTANCE data; per-project).
//
// CR-5 (2026-06-11): actors are typed Person/Actor PART INSTANCES — name, email,
// and kind are MODEL data (queryable), not comments (critique S2). Populated by
// .engine/tools/capture_user.py from git effective identity; the declared part
// name is the actor id that authoredBy / judgedBy reference. One line per actor.
// =====================================================================
package ProjectActors {
    private import EngineElement::*;

    part wweatherholtz : Person { :>> name = "William Weatherholtz"; :>> email = "0612@asirobots.com"; }
    part claudeOpus : Actor { :>> name = "Claude Opus 4.8"; :>> kind = ActorKind::ai; }
    part claudeSonnet46 : Actor { :>> name = "Claude Sonnet 4.6"; :>> kind = ActorKind::ai; }
}
