package Conventions {

    // ---------------------------------------------------------------
    // Status vocabulary — every WorkItem carries one of these.
    // Kept as String for now; promote to an enumeration once the
    // pilot implementation's enum syntax is validated.
    // Valid values: "backlog" | "ready" | "in_progress" |
    //               "in_review" | "done" | "blocked"
    // ---------------------------------------------------------------

    // ---------------------------------------------------------------
    // Core work-tracking types
    // ---------------------------------------------------------------

    part def WorkItem {
        doc /* Anything trackable that has a status and an owner.
               Parent type for Epic, Story, Task. */
        attribute title : String;
        attribute status : String;
        attribute owner : String;
        attribute priority : String;     // "p0" | "p1" | "p2" | "p3"
        attribute created : String;      // ISO 8601 date
        attribute updated : String;
    }

    part def Epic :> WorkItem {
        doc /* A large body of work decomposed into Stories.
               Status is derived: done when all child Stories done. */
        attribute themeArea : String;
    }

    part def Story :> WorkItem {
        doc /* A sized, acceptance-criteria-bearing work item.
               The unit of value delivery. */
        attribute acceptanceCriteria : String;
        attribute estimatedPoints : Integer;
        attribute actualPoints : Integer;
    }

    part def Task :> WorkItem {
        doc /* A subtask of a Story. Sized in hours, not points. */
        attribute estimatedHours : Real;
    }

    // ---------------------------------------------------------------
    // Decision records (ADRs in typed form)
    // ---------------------------------------------------------------

    part def Decision {
        doc /* An architectural or process decision. Immutable once
               accepted; superseded by linking to a successor. */
        attribute title : String;
        attribute status : String;        // "proposed" | "accepted" | "superseded"
        attribute date : String;
        attribute context : String;
        attribute decision : String;
        attribute consequences : String;
        attribute supersededBy : String;  // qualified name of successor, or empty
    }

    // ---------------------------------------------------------------
    // Process modeling
    // ---------------------------------------------------------------

    part def Process {
        doc /* A named workflow procedure. Composed of ProcessSteps. */
        attribute name : String;
        attribute purpose : String;
        attribute cadence : String;       // "continuous" | "per-story" | "per-sprint" | "ad-hoc"
    }

    part def ProcessStep {
        attribute name : String;
        attribute order : Integer;
        attribute trigger : String;
        attribute action : String;
        attribute output : String;
        attribute owner : String;         // "human" | "ai" | "either"
    }

    // ---------------------------------------------------------------
    // AI skill / agent registrations
    // Pointers to executable artifacts on disk. The .sysml side
    // carries workflow context; the actual prompt/code lives at
    // the `location` path.
    // ---------------------------------------------------------------

    part def AISkill {
        attribute name : String;
        attribute purpose : String;
        attribute location : String;      // path on disk, e.g. "skills/triage/SKILL.md"
        attribute trigger : String;       // when it fires
        attribute writePolicy : String;   // "direct" | "pr-only" | "read-only"
    }

    part def Agent :> AISkill {
        doc /* A skill that can invoke other skills or sub-agents. */
        attribute subagents : String[*]; // qualified names of child AISkills
    }

    // ---------------------------------------------------------------
    // Use cases and architecture
    // ---------------------------------------------------------------

    part def UseCase {
        attribute name : String;
        attribute actor : String;
        attribute goal : String;
        attribute steps : String;
    }

    part def Component {
        doc /* An architectural unit — a service, library, CLI, etc. */
        attribute name : String;
        attribute kind : String;          // "service" | "cli" | "library" | "ui" | "infra"
        attribute language : String;
        attribute notes : String;
    }

    // ---------------------------------------------------------------
    // Tests
    // ---------------------------------------------------------------

    part def TestCase {
        attribute name : String;
        attribute kind : String;          // "unit" | "integration" | "e2e" | "acceptance"
        attribute location : String;      // path to the test file
        attribute lastRun : String;
        attribute lastResult : String;    // "pass" | "fail" | "skip" | "never"
    }

    // ---------------------------------------------------------------
    // External references (NOT mirrors — pointers only)
    // ---------------------------------------------------------------

    part def ExternalRef {
        doc /* A pointer to something that lives outside this model:
               a GitHub workflow, an OCI image, a dashboard, a doc.
               The external system is the source of truth for its
               own data. We only record the pointer. */
        attribute name : String;
        attribute kind : String;          // "workflow" | "image" | "dashboard" | "doc" | "repo"
        attribute uri : String;
    }

    // ---------------------------------------------------------------
    // Sprint container
    // ---------------------------------------------------------------

    part def Sprint {
        attribute name : String;          // e.g. "2026-S22"
        attribute startDate : String;
        attribute endDate : String;
        attribute goal : String;
        attribute items : String[*];      // qualified names of Stories in sprint
    }
}
