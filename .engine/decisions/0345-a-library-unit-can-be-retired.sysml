// D0345 (PROPOSED — NOT YET ACCEPTED) — libraryUnitRetirement: keel process retire marks a library unit RETIRED with its reason and replacement, import refuses it, list and status mark it - and decision-channel v6 is retired for GH#57 and GH#58
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0345Accept event on sign-off.
package Decision0345 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0345 : Decision {
        :>> id = "e7a4e4f0-6085-4a2d-9649-acb9b5cd05e5";
        :>> title = "libraryUnitRetirement: keel process retire marks a library unit RETIRED with its reason and replacement, import refuses it, list and status mark it - and decision-channel v6 is retired for GH#57 and GH#58";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "GH#58 (st093, us092, issue381) and GH#57's library half (st094, us093). keel-lib's decision-channel v6 ships .github/scripts/record_decision.sh - which records an acceptance as a hardcoded actor quoting one specific human's standing consent, a fabricated-attestation path - and decision-issue.yml, which never creates the keel-standing label its own script requires, so the channel's first run in a new repository dies. This repository disconnected the channel in sprint 527 (D0291) and deleted those scripts, so library-stewardship rule 1 - publish from landed trees only - means the unit cannot be republished from here; and the library had no way to say a unit is abandoned, so it stayed on offer with both defects.";
        :>> decision = "keel process retire <name> --why TEXT [--replaced-by TEXT] [--at DATE] writes retired = true, retiredAt, retiredWhy and optionally replacedBy into the unit's unit.toml in the library clone and commits there, not pushed - the push is the deliberate act, as for publish; --why is required with at least 20 characters and a second retirement is refused; import --from-library refuses a retired unit naming the why and the replacement; library list marks it; keel status marks an INSTALLED retired unit and never offers a retired one, so the currency pass's drift lines carry the mark; library-stewardship gains rule 6 for when rule 1 cannot apply. Then decision-channel is retired in keel-lib with the two reasons and record-time standing consent (D0291, with the project's own words per D0340) as its replacement. NOT A FORK: patching the unit in place was ruled out by rule 1, and deleting it from the library would erase the history its consumers installed against.";
        :>> rationale = "Why retire and not delete: a consumer that installed v6 must still be able to read what it installed; a retirement is a fact added, not history removed. Why refuse import rather than warn: importing a retired unit installs the defect it was retired for, and a warning on an install path is the class that produced GH#57 downstream. Why the unit stays in status as attention: the project that installed it is the one that must decide, and status is where it reads what changed upstream. Arming (D0298): three binary tests on an isolated library - retire writes the record, commits once, list marks it, import refuses naming the why and the replacement, a second retirement is refused; a retirement under 20 characters is refused and writes nothing; status marks an installed retired unit and never offers it.";
        :>> consequences = "decision-channel is retired in the shared library; jama-rereviewer and any other consumer see RETIRED in keel status after their next library sync and can remove the unit (dcProcessRemoveCommand, when built, is the one-act path). issue381 resolves on dcLibraryUnitRetirement's pass. Marked process-change: library-stewardship's skill text changes, so under D0337 this waits for the human's acceptance; the retirement itself is pushed now because the unit's defects are live for every consumer and the record is reversible by a later edit.";
    }
}
