// =====================================================================
// EngineGuardConstraints — each PROCESS-BOUND guard as a base SysML v2 `constraint def` (D0139(D)).
//
// WHY THIS EXISTS. CLAUDE.md §1.7 already holds that "the guards ARE the constraint layer", so a
// Process asserting the constraints it enforces is base SysML v2 stating exactly what the engine means.
// Until now that mapping lived in `.engine/contracts/process-units.toml` — a table no `keel trace`,
// declared view or viewpoint could reach, so an ENGINE FACT sat outside the model that is supposed to
// hold every engine fact.
//
// HOW IT IS READ. A process unit is a FILE in `.engine/processes/`. Its guards are the union of
// `assert constraint <member> : <ConstraintDef>;` across every `Process` part in that file — so an
// assert lands on the specific phase or step that enforces it (ceremony on the sprint phases, retro
// backlog on the retrospective) rather than on a file-level list. That is strictly more informative
// than the TOML it replaces.
//
// NAMING. Each constraint def is the camelCase form of its guard name; the binary lowercases and
// hyphenates to recover `sprint-coverage` from `sprintCoverage`. One mechanical rule, no lookup table
// to drift.
//
// CORE guards are deliberately ABSENT — identity, provenance, vocabulary, rootedness, well-formedness
// always run and belong to no process (D0138). Adding one here would make it deactivatable, which is
// the sanctioned-bypass failure D0138 exists to prevent.
// =====================================================================
package EngineGuardConstraints {

    // agile-workflow
    constraint def ceremony;
    constraint def sprintCoverage;
    constraint def retroBacklog;
    constraint def priorityInversion;
    constraint def verificationTrace;

    // decision-authoring
    constraint def decisionRationale;
    constraint def acceptanceEvents;
    constraint def decisionRequirementLink;
    constraint def attestationSubstance;
    constraint def confirmationAuthenticity;

    // issue-resolution
    constraint def issues;

    // element-critique
    constraint def criticIndependence;

    // render
    constraint def viewpointRenderer;

    // doc-sync
    constraint def docSync;

    // D0191: the admitted-gap guards, unit-owned by deploy and actor-enrollment.
    constraint def releaseRecorded;
    constraint def enrollmentBinding;
    // D0195 clause 1 (panel R1, aerospace flip 2): EVERY enforced guard has a constraint-def
    // identity, not only the unit-owned ones - the model identity the control map and any
    // future unit-ownership declaration binds to. Names are the guards' camelCased kebab names.
    constraint def actors;
    constraint def charter;
    constraint def processChange;
    constraint def manifestCoverage;
    constraint def processSkill;
    constraint def requirementRootedness;
    constraint def markerVocabulary;
    constraint def duplicateIdentity;
    constraint def engineLint;
    constraint def hookConfigIntegrity;
    constraint def activationManifest;
    constraint def sequenceMultiplicity;
    constraint def parserCoverage;
    constraint def baseFirstJustification;
    constraint def edgeEndpoints;
    constraint def ownership;
    constraint def attestationAuthority;
    constraint def typeCollision;
    constraint def attributeVocabulary;
    constraint def resolverKind;
    constraint def staleGateProse;
    constraint def impossibleEvidenceDate;
    constraint def identityPresent;
    constraint def identityWellFormed;
    constraint def toolReference;
    constraint def scaffoldPlaceholder;
    constraint def claudeSurfaceDrift;
    constraint def decisionScaffolding;
    constraint def controlEventCoverage;

}
