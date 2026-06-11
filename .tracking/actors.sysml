// =====================================================================
// ProjectActors — the actor registry (INSTANCE data; per-project, like backlog.sysml).
//
// WHO did tracked work. Populated by .engine/tools/capture_user.py from git
// identity (piggyback) — git is REQUIRED by this workflow, so it is the canonical
// source of the acting human. The set is small by nature (solo + AI). Provenance
// fields (authoredBy / verifiedBy) reference these ids.
//
// `ActorId` is an enum (a closed, %show-readable set the capture process appends
// to). Human ids are git-derived slugs; the AI actor is the model.
// =====================================================================
package ProjectActors {
    enum def ActorId {
        wweatherholtz;   // William Weatherholtz <0612@asirobots.com> — human (from git)
        claudeOpus;      // Claude Opus 4.8 — ai
    }
}
