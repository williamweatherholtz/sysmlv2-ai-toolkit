// =====================================================================
// ProjectConsoleBusiness — Business-layer facts for the keel OPERATOR CONSOLE (`keel serve`).
//
// WHY THIS FILE EXISTS AT ALL: the console had NO authored Business layer. 56 UseCases exist in this
// model and not one describes the console; its 12 tabs are a hardcoded array (console.html:41)
// organised by DATA TYPE — Business, Decisions, Processes, Reports, Section, Boundary, History —
// rather than by anything a person is trying to DO. It accreted one tab per API endpoint. This file
// is the missing upstream, authored per the business-architecture skill (Brief -> Personas -> Needs
// -> UseCases), with the human's Needs gate before any architecture is recorded.
//
// DELIBERATELY SMALL. Merging the console with the planned viewer app means the viewer's Needs
// BECOME the console's: N-17 already forbids hardcoded views ("the viewer must DYNAMICALLY generate
// its UI from keel's DECLARED model — NOT hardcode views or actions"), N-1 already defines a
// viewpoint as a query AND a form targeting an audience, N-2 covers the explorable graph, and N-6
// keeps authority with keel. Only what those do not cover is authored here.
//
// NOT AUTHORED, deliberately: an audience/role filter on the surface. It was drafted (as "N-D") and
// DEFERRED by wweatherholtz 2026-08-16 — "I won't prevent access or use role-specific views for a
// bit." Recorded here so a later reader finds a considered deferral rather than an unnoticed gap.
// Note the consequence honestly: the merge argument no longer leans on "the audience split is a
// filter rather than a deployment boundary", because that filter is not being built yet.
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
        :>> problem = "The console presents twelve hardcoded tabs organised by data type, with no authored use case behind any of them, so a supervisor cannot tell at a glance what is owed by them versus what is merely available to read. It shows the engine's own tracked work indistinguishably from the work of the project the engine is tracking — a distinction the engine treats as foundational and which coincides only in this repository. It binds one project root per process. The surface grew one tab per API endpoint as endpoints were added; nothing declares who it serves.";
        :>> opportunity = "The model already declares Viewpoints carrying concern, audience, sources and renderer, and already requires (N-17) that the viewing surface generate its UI from the declared model rather than hardcode it. Honouring what is already authored turns 'how many tabs' from a design argument into declared data: a capability is added by declaring a viewpoint, not by editing a tab array. Merging the console with the planned separate viewer lets one surface inherit that whole Need set instead of maintaining two.";
        :>> constraintsNote = "keel's write API and guards remain the ONLY write path (N-6): the surface consumes the engine, never duplicates its management. CLI/JSON stays the automation authority and the surface is the human's oversight lens (D0093). Audience/role filtering is deferred by the human as of 2026-08-16 and is out of scope here.";
    }

    // ---- Needs. Solution-free and measurable; the surface's SHAPE is deliberately absent. --------

    requirement nConsoleObligationsInOnePlace : Need {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000n01";
        :>> title = "N-C1: one place answers 'is anything waiting on me?'";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "An asynchronous supervisor can determine, from a single surface and without cross-referencing others, every obligation that requires their own judgment — and nothing that does not. When no obligation is outstanding the surface states that affirmatively rather than presenting as empty or unloaded.";
        :>> marketJustification = "The supervisor is asynchronous and intermittent (asyncSupervisor): the cost of a missed obligation is an AI agent blocked, or worse, an AI proceeding without a judgment only a human may supply. Today the four obligation classes are scattered across separate tabs and computed views, so answering 'does anything need me?' requires knowing where to look and looking in four places.";
        :>> priority = Priority::must;
    }

    requirement nConsoleModelScopeExplicit : Need {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000n02";
        :>> title = "N-C2: the engine's own model is never mixed with the deliverable's";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "Where a project's tracked work and the engine that tracks it are distinct models, the surface presents one at a time and names which is in view; a reader is never required to infer which model an item belongs to. Where the two genuinely coincide, the surface says so rather than silently conflating them.";
        :>> marketJustification = "'Two models, never conflated — the engine model tracks the work; the deliverable is what the work produces' is the first invariant this engine states about itself. A surface that mixes them teaches the opposite of the discipline the engine exists to enforce. In THIS repository the two coincide, which is exactly why the defect is invisible here and would be immediate in any downstream project.";
        :>> priority = Priority::must;
    }

    requirement nConsoleMultiProject : Need {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000n03";
        :>> title = "N-C3: one surface oversees several projects";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A supervisor overseeing more than one project can move between them within the surface, and the surface makes clear at all times which project is in view. Switching requires no process restart and no configuration edit.";
        :>> marketJustification = "A supervisor of several projects otherwise runs one server per project and keeps the mapping between port and project in their head — which is the same unrecorded-state failure the engine forbids everywhere else. The engine is explicitly built to be adopted by downstream projects, so single-root is a limit on the product, not just on this repo's convenience.";
        :>> priority = Priority::should;
    }

    // ---- UseCases. Concrete usage, traced to the Needs above. -------------------------------------

    use case ucConsoleCheckObligations : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u01";
        :>> title = "UC-C1: Open the console and find out whether anything is owed";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As an asynchronous supervisor returning after hours away, I want one surface to tell me whether anything requires my judgment, so that I can discharge it or close the tab knowing nothing is blocked on me. (Serves N-C1.)";
        :>> mainSuccess = "Open the console; the obligation surface lists every item awaiting my judgment across all four classes — a Decision awaiting acceptance, a finding at or above Medium awaiting disposition, a sitting awaiting review, and work where an AI is blocked on a human — each naming what it is waiting for.";
        :>> extensions = "NOTHING OWED: the surface states that no obligation is outstanding, distinguishably from a surface that failed to load. PARTIAL: a class that cannot be computed reports its own failure rather than silently showing zero — an empty obligation list must never be the way an error looks.";
    }

    use case ucConsoleDischargeObligation : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u02";
        :>> title = "UC-C2: Discharge an obligation without leaving the surface";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As an asynchronous supervisor, I want to act on an obligation where I find it — accept a Decision, disposition a finding, complete a sitting review — so that discharging my queue is not a second task of translating the queue into commands. (Serves N-C1.)";
        :>> mainSuccess = "Select an obligation; see the context needed to judge it — for a Decision its rationale, alternatives and consequences; for a finding its severity and the critique that raised it; record the judgment; the obligation leaves the surface because the underlying computed state changed, not because the surface marked it done.";
        :>> extensions = "REFUSED: an obligation an AI actor may not discharge is not offered to one — the human-only attestation classes stay human-only, checked by kind before role. INSUFFICIENT CONTEXT: the surface links to the full item rather than inviting a judgment on a summary.";
    }

    use case ucConsoleScopeToModel : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u03";
        :>> title = "UC-C3: Look at the deliverable's work without the engine's own";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As a supervisor of a project that USES keel, I want to see the work my project is tracking without the engine's own backlog, decisions and sprints mixed in, so that what I read is about my system rather than about the tool. (Serves N-C2.)";
        :>> mainSuccess = "Choose the model scope; the surface shows only that model's items and names the scope in view; every item displayed belongs unambiguously to it.";
        :>> extensions = "COINCIDENT MODELS: in a repository where the engine builds itself, the surface states that the two models coincide here rather than offering a switch that appears to do nothing. UNSCOPED ITEM: an item whose model cannot be determined is shown as unscoped rather than assigned to a default — a wrong scope is worse than an admitted unknown.";
    }

    use case ucConsoleSwitchProject : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u04";
        :>> title = "UC-C4: Move between projects in one surface";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As a supervisor overseeing several projects, I want to switch the project in view without restarting anything, so that my obligations across projects are reachable from one place. (Serves N-C3.)";
        :>> mainSuccess = "Select another project; the surface rebinds to it, states which project is in view, and recomputes every view against that project's model.";
        :>> extensions = "UNREADABLE PROJECT: a project whose model fails to parse reports that plainly and does not silently fall back to the previous one. STALE: a project changed on disk since it was last read is recomputed rather than served from cache.";
    }

    use case ucConsoleAddCapabilityByDeclaration : UseCase {
        :>> id = "3c7f1a92-6b04-4e58-9d31-000000000u05";
        :>> title = "UC-C5: Add a new surface by declaring a viewpoint, not by editing the console";
        :>> createdAt = "2026-08-16"; :>> createdBy = "claudeOpus5";
        :>> description = "As a technical modeler, I want a newly declared viewpoint to appear in the surface without anyone editing console code, so that the surface tracks the model instead of lagging it. (Serves N-17, the existing generative-UI Need; recorded here because the console is where it must now hold.)";
        :>> mainSuccess = "Declare a Viewpoint with its concern, sources and renderer; it appears in the surface with no code change and no restart beyond re-reading the model.";
        :>> extensions = "UNRENDERABLE: a viewpoint naming a renderer that does not exist is reported as such — the viewpoint-renderer guard already refuses this at commit time, and the surface must not present a viewpoint it cannot draw. RETIRED: a superseded viewpoint stops appearing without being deleted.";
    }

    // ---- traces: UseCase -> Need --------------------------------------------------------------
    #DerivedFrom dependency from ucConsoleCheckObligations to nConsoleObligationsInOnePlace;
    #DerivedFrom dependency from ucConsoleDischargeObligation to nConsoleObligationsInOnePlace;
    #DerivedFrom dependency from ucConsoleScopeToModel to nConsoleModelScopeExplicit;
    #DerivedFrom dependency from ucConsoleSwitchProject to nConsoleMultiProject;
    #DerivedFrom dependency from ucConsoleAddCapabilityByDeclaration to nViewerGenerativeUi;

    // The Brief is the origin of the three new Needs.
    #DerivedFrom dependency from nConsoleObligationsInOnePlace to consoleBrief;
    #DerivedFrom dependency from nConsoleModelScopeExplicit to consoleBrief;
    #DerivedFrom dependency from nConsoleMultiProject to consoleBrief;
}
