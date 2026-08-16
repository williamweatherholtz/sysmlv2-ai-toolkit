// =====================================================================
// ProjectConsoleBusiness — Business-layer facts for the keel OPERATOR CONSOLE (`keel serve`).
//
// WHY THIS FILE EXISTS AT ALL: the console had NO authored Business layer. 56 UseCases exist in this
// model and not one describes the console; its 12 tabs are a hardcoded array (console.html:41)
// organised by DATA TYPE — Business, Decisions, Processes, Reports, Section, Boundary, History —
// rather than by anything a person is trying to DO. It accreted one tab per API endpoint.
//
// THE NEEDS WERE REWRITTEN ONCE, on the human's correction: "these needs are pretty close to use
// cases. try and identify things that are more fundamental. for example n-c1 is really two distinct
// needs. one to know what is pending, and one to not be misled." That split is the reason this set
// looks different from a list of console features — N-C2 (never misleads) is not about obligations
// at all, it is the engine's own honest-state doctrine pointed at a HUMAN rather than at a gate, and
// it constrains every surface the console will ever grow. The human also directed the human-factors
// angle explicitly: "'fewer tabs' I agree isn't a need, but UX and good human factors design is."
// N-C5 carries that, and it is what "twelve tabs" actually violates — not the count, but that using
// the surface requires remembering a map of it.
//
// DELIBERATELY SMALL. Merging the console with the planned viewer means the viewer's Needs BECOME
// the console's: N-17 already forbids hardcoded views, N-1 already defines a viewpoint as a query
// AND a form targeting an audience, N-2 covers the explorable graph, N-6 keeps authority with keel.
// Only what those do not cover is authored here.
//
// NOT AUTHORED, deliberately: an audience/role filter, drafted as "N-D" and DEFERRED by
// wweatherholtz 2026-08-16 — "I won't prevent access or use role-specific views for a bit." Recorded
// so a later reader finds a considered deferral rather than an unnoticed gap. Consequence recorded
// honestly: the merge argument no longer leans on "the audience split is a filter rather than a
// deployment boundary", because that filter is not being built yet.
// =====================================================================
package ProjectConsoleBusiness {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    part consoleBrief : Brief {
        :>> id = "3c7f1a92-6b04-4e58-9d31-8a2c05e7f4b6";
        :>> title = "keel operator console — the surface was never designed";
        :>> createdAt = "2026-08-16";
        :>> createdBy = "claudeOpus5";
        :>> problem = "The console presents twelve hardcoded tabs organised by data type, with no authored use case behind any of them. A supervisor cannot tell what is owed by them from what is merely available to read; cannot tell a surface with nothing in it from a surface that failed to load; and cannot tell the engine's own tracked work from the work of the project the engine is tracking — a distinction the engine treats as foundational and which coincides only in this repository. It binds one project root per process. Using it at all requires remembering which of twelve tabs holds what, and that cost grows with every capability added.";
        :>> opportunity = "The model already declares Viewpoints carrying concern, audience, sources and renderer, and already requires (N-17) that the viewing surface generate its UI from the declared model rather than hardcode it. Honouring what is already authored turns 'how should this be organised' from a design argument into declared data. Merging the console with the planned separate viewer lets one surface inherit that whole Need set instead of maintaining two.";
        :>> constraintsNote = "keel's write API and guards remain the ONLY write path (N-6): the surface consumes the engine, never duplicates its management. CLI/JSON stays the automation authority; the surface is the human's oversight lens (D0093). Audience/role filtering is DEFERRED by the human as of 2026-08-16 and is out of scope. The supervisor is asynchronous and intermittent (asyncSupervisor) — the surface is used after long absences, not continuously.";
    }

    // ---- Needs. Fundamental properties of the supervisor's situation, not console features. ------
    // Each is solution-free: none names a tab, a layout, or a count of anything.

    requirement nConsoleObligationDiscoverable : Need {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000n01";
        :>> title = "N-C1: what is owed by me is discoverable without knowing to look for it";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A supervisor can discover every obligation that requires their own judgment WITHOUT knowing in advance that one exists or where it would appear. Discovery does not depend on remembering which part of the surface holds which class of obligation, nor on having been told that a new class of obligation now exists.";
        :>> marketJustification = "The supervisor is intermittent and the agents are not: obligations accumulate while nobody is watching for them. An obligation a supervisor must already know about in order to find is not discoverable — it is merely retrievable, and it will be missed exactly when the supervisor has been away longest, which is precisely when the most has accumulated. The cost of missing one is an agent blocked, or an agent proceeding on a judgment only a human may supply.";
        :>> priority = Priority::must;
    }

    requirement nConsoleNeverMisleads : Need {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000n05";
        :>> title = "N-C2: the surface never misleads — absence and failure are distinguishable";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "Everything the surface presents is true of the model at the moment shown. The ABSENCE of information is always distinguishable from information ABOUT absence: a view that could not be computed never renders as a negative result, an empty list never looks like a failed load, and stale content is never shown as current. Where the surface does not know, it says it does not know.";
        :>> marketJustification = "This is the engine's own honest-state doctrine (§1.6, D0098) applied to the human lens rather than to a gate, and it is the deeper half of what was first written as one Need about obligations. A supervisor who cannot trust a quiet surface must verify it elsewhere, at which point the surface has negative value — it costs attention and returns nothing. The failure mode is specific and has recurred repeatedly in this engine's own tooling: a confident wrong answer rather than an error — an empty registry for a path that does not exist, a PASS over a corpus never read, zero edges from a case mismatch. A human surface fails the same way and worse, because a human cannot re-run it to check.";
        :>> priority = Priority::must;
    }

    requirement nConsoleContextCertain : Need {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000n02";
        :>> title = "N-C3: which model I am looking at is never in doubt";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A reader is never uncertain whether what they are looking at belongs to the engine that tracks the work or to the work being tracked. Where the two are distinct models the surface presents one at a time and names it; where they genuinely coincide the surface says so, rather than leaving the reader to infer either.";
        :>> marketJustification = "'Two models, never conflated — the engine model tracks the work; the deliverable is what the work produces' is the first invariant this engine states about itself. A surface that mixes them teaches the opposite of the discipline the engine exists to enforce, and does so to the one person whose judgment the whole loop depends on. In THIS repository the two coincide, which is why the defect is invisible here and would be immediate in any downstream project.";
        :>> priority = Priority::must;
    }

    requirement nConsoleMultiProject : Need {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000n03";
        :>> title = "N-C4: overseeing several projects does not require several surfaces";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A supervisor responsible for more than one project reaches all of them through one surface, and is never uncertain which project is in view. The mapping from project to where it can be seen is held by the surface, not by the supervisor.";
        :>> marketJustification = "The alternative is one server per project and the port-to-project mapping kept in the supervisor's head — unrecorded state of exactly the kind the engine forbids everywhere else, and the first thing lost after an absence. The engine is built to be adopted by downstream projects, so single-root is a limit on the product rather than on this repository's convenience.";
        :>> priority = Priority::should;
    }

    requirement nConsoleFitsIntermittentAttention : Need {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000n04";
        :>> title = "N-C5: usable by someone returning cold, and it does not degrade as the model grows";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "The surface is usable by a supervisor returning after a long absence with no memory of its structure: re-orienting does not require recalling where anything lives. The effort to re-orient does not grow as the model grows or as capabilities are added — what competes for attention on arrival stays bounded even as what is available expands.";
        :>> marketJustification = "Directed by the human: 'fewer tabs' is not a need, but human factors is. This is what twelve tabs actually violates — not the count, but that the surface must be LEARNED and re-learned, and that every capability added made it worse. A surface whose orientation cost scales with the model punishes exactly the growth the engine is designed to sustain, and it punishes the intermittent supervisor hardest, since they pay the re-learning cost every visit while a continuous user pays it once.";
        :>> priority = Priority::must;
    }

    // ---- UseCases. Concrete usage, traced to the Needs above. -------------------------------------

    use case ucConsoleCheckObligations : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u01";
        :>> title = "UC-C1: Return after an absence and find out whether anything is owed";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As an asynchronous supervisor returning after hours or days away, I want to learn whether anything requires my judgment without knowing what kinds of obligation exist, so that I can discharge what is owed or stop, confident that nothing is blocked on me. (Serves N-C1, N-C5.)";
        :>> mainSuccess = "Arrive at the surface; what is owed by me is presented without my having navigated to it, each item naming what it is waiting for and why it needs a human.";
        :>> extensions = "NOTHING OWED: the surface states that no obligation is outstanding, distinguishably from a surface that has not loaded (N-C2). CANNOT COMPUTE: a class that failed to compute says so and is not rendered as zero — an empty obligation list must never be how an error looks. NEW CLASS: an obligation class introduced after my last visit appears without my having been told it exists.";
    }

    use case ucConsoleDischargeObligation : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u02";
        :>> title = "UC-C2: Discharge an obligation where it was found";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As an asynchronous supervisor, I want to act on an obligation at the point I encounter it — accept a Decision, disposition a finding, complete a sitting review — so that discharging my queue is not a second task of translating it into commands. (Serves N-C1.)";
        :>> mainSuccess = "Select an obligation; see the context needed to judge it — a Decision's rationale, alternatives and consequences; a finding's severity and the critique that raised it; record the judgment; the obligation leaves the surface because the computed state changed, not because the surface marked it done.";
        :>> extensions = "REFUSED: an obligation an AI actor may not discharge is never offered to one — human-only attestation classes stay human-only, checked by kind before role. THIN CONTEXT: the surface links to the full item rather than inviting judgment on a summary.";
    }

    use case ucConsoleTrustAQuietSurface : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u06";
        :>> title = "UC-C3: Tell 'nothing to report' from 'failed to report'";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As a supervisor, I want to know whether a quiet surface means there is nothing to see or that something did not load, so that I can trust silence and stop verifying it elsewhere. (Serves N-C2.)";
        :>> mainSuccess = "A view with no content states that it computed successfully and found nothing; a view that failed states what failed and against which model it was attempted.";
        :>> extensions = "STALE: content that could not be recomputed is shown with the moment it was true, not as current. PARTIAL: a surface where some sources answered and others did not reports which — a partial answer presented as whole is the same defect as an error presented as zero.";
    }

    use case ucConsoleScopeToModel : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u03";
        :>> title = "UC-C4: Look at the deliverable's work without the engine's own";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As a supervisor of a project that USES keel, I want to see the work my project is tracking without the engine's own backlog, decisions and sprints mixed in, so that what I read is about my system rather than about the tool. (Serves N-C3.)";
        :>> mainSuccess = "Choose the model scope; the surface shows only that model's items and names the scope in view; every item displayed belongs unambiguously to it.";
        :>> extensions = "COINCIDENT: where the engine builds itself, the surface states that the two models coincide here rather than offering a switch that appears to do nothing. UNSCOPED: an item whose model cannot be determined is shown as unscoped rather than assigned to a default — a wrong scope is worse than an admitted unknown (N-C2).";
    }

    use case ucConsoleSwitchProject : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u04";
        :>> title = "UC-C5: Move between projects in one surface";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As a supervisor overseeing several projects, I want to switch the project in view without restarting anything, so that my obligations across projects are reachable from one place. (Serves N-C4.)";
        :>> mainSuccess = "Select another project; the surface rebinds, states which project is in view, and recomputes every view against that project's model.";
        :>> extensions = "UNREADABLE: a project whose model fails to parse reports that plainly and never silently falls back to the previous one. STALE: a project changed on disk since last read is recomputed rather than served from cache.";
    }

    use case ucConsoleAddCapabilityByDeclaration : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u05";
        :>> title = "UC-C6: Add a surface by declaring a viewpoint, not by editing the console";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As a technical modeler, I want a newly declared viewpoint to appear without anyone editing console code, so that the surface tracks the model instead of lagging it — and so that adding capability does not add orientation cost. (Serves N-17 and N-C5.)";
        :>> mainSuccess = "Declare a Viewpoint with its concern, sources and renderer; it appears in the surface with no code change.";
        :>> extensions = "UNRENDERABLE: a viewpoint naming a renderer that does not exist is reported as such — the viewpoint-renderer guard refuses this at commit time and the surface must not present a viewpoint it cannot draw. RETIRED: a superseded viewpoint stops appearing without being deleted.";
    }

    // ---- traces: UseCase -> Need --------------------------------------------------------------
    #DerivedFrom dependency from ucConsoleCheckObligations to nConsoleObligationDiscoverable;
    #DerivedFrom dependency from ucConsoleDischargeObligation to nConsoleObligationDiscoverable;
    #DerivedFrom dependency from ucConsoleTrustAQuietSurface to nConsoleNeverMisleads;
    #DerivedFrom dependency from ucConsoleScopeToModel to nConsoleContextCertain;
    #DerivedFrom dependency from ucConsoleSwitchProject to nConsoleMultiProject;
    #DerivedFrom dependency from ucConsoleAddCapabilityByDeclaration to nConsoleFitsIntermittentAttention;

    // ---- the Brief is the origin of every Need here ----------------------------------------------
    #DerivedFrom dependency from nConsoleObligationDiscoverable to consoleBrief;
    #DerivedFrom dependency from nConsoleNeverMisleads to consoleBrief;
    #DerivedFrom dependency from nConsoleContextCertain to consoleBrief;
    #DerivedFrom dependency from nConsoleMultiProject to consoleBrief;
    #DerivedFrom dependency from nConsoleFitsIntermittentAttention to consoleBrief;
}
