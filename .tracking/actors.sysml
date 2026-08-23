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

    part wweatherholtz : Person { :>> name = "William Weatherholtz"; :>> email = "0612@asirobots.com"; :>> role = "supervisor"; }
    part claudeOpus : Actor { :>> name = "Claude Opus 4.8"; :>> kind = ActorKind::ai; :>> role = "contributor"; }
    part claudeSonnet46 : Actor { :>> name = "Claude Sonnet 4.6"; :>> kind = ActorKind::ai; :>> role = "contributor"; }
    part claudeOpus5 : Actor { :>> name = "Claude Opus 5"; :>> kind = ActorKind::ai; :>> role = "contributor"; }

    // The three-critic panel the human commissioned on 2026-08-14 (commit ec7d548). Registered as
    // actors so the panel's findings can be attributed rather than left in chat history (D0018).
    //
    // WHAT THEY ACTUALLY WERE, stated so a later reader cannot mistake them for independent
    // reviewers: three separate subagent CONTEXTS of the SAME model as the author (Claude Opus 5),
    // each given a different brief and launched BEFORE the author wrote their own analysis, so the
    // findings were not anchored to the author's framing. Distinct context and distinct brief —
    // NOT a distinct model, and not a distinct judgement substrate. `name` says so on each record
    // rather than only in this comment, because the name is what a viewer renders.
    part panelAntagonist : Actor { :>> name = "Panel antagonist (Opus 5 subagent context, not an independent model)"; :>> kind = ActorKind::ai; :>> role = "reviewer"; }
    part panelProtagonist : Actor { :>> name = "Panel protagonist (Opus 5 subagent context, not an independent model)"; :>> kind = ActorKind::ai; :>> role = "reviewer"; }
    part panelCorrectness : Actor { :>> name = "Panel technical-correctness critic (Opus 5 subagent context, not an independent model)"; :>> kind = ActorKind::ai; :>> role = "reviewer"; }

    // The OMG SysML v2 pilot kernel is NOT registered here, and that is the schema working rather
    // than a gap: `ActorKind` is human|ai deliberately, and tool-ness belongs on the CRITIQUE
    // (`critiquedBy = CriticKind::tool`) while the identity in `judgedBy` stays the actor who ran
    // it. A kernel-adjudicated critique is recorded that way — see .tracking/panel-critiques.sysml.
    part claudeFable5 : Actor { :>> name = "Claude Fable 5"; :>> kind = ActorKind::ai; :>> role = "contributor"; }

    // D0205 (githubChannel): the GitHub Actions recorder - durable compute that transcribes the
    // human's authenticated issue-comment gesture into the tree while the local machine is off.
    // kind=ai (it is automation, never a Person); it RECORDS human judgment under the D0192-family
    // delegation and never supplies it - the acceptance's judgedBy stays the mapped human, and the
    // ownership guard admits only the exact accept transform from a non-owner.
    part githubRecorder : Actor { :>> name = "GitHub Actions recorder (keel binary in CI, D0205)"; :>> kind = ActorKind::ai; :>> role = "contributor"; }
}
