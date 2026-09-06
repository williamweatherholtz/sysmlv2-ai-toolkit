// D0340 (PROPOSED — NOT YET ACCEPTED) — standingWordsAreProjectLocal: a standing-consent acceptance quotes the adopting project's own standingWords or stays proposed, and init ships the attestation policy with no grant
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0340Accept event on sign-off.
package Decision0340 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0340 : Decision {
        :>> id = "29c570d4-2620-4a62-80de-c0fcdadad4db";
        :>> title = "standingWordsAreProjectLocal: a standing-consent acceptance quotes the adopting project's own standingWords or stays proposed, and init ships the attestation policy with no grant";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "GH#57 (st094, us093, issue376 - Critical). The reporter found the library unit's record_decision.sh recording acceptances as a hardcoded actor quoting one human's standing consent. Verified wider on main: keel record decision built the AUTO-ACCEPTED note from a string literal of this repository's human's words; keel init shipped attestation-policy.toml verbatim, standingConsent = d0207 and delegatedRecording = d0192 live; keel migrate resynced the file wholesale. A fresh project with one declared decider therefore auto-accepted every non-fork Decision in that decider's name, quoting consent they never gave - the class D0106 and confirmation-authenticity exist to make impossible, produced by the engine itself.";
        :>> decision = "The words a record-time acceptance quotes are read from the project's attestation-policy.toml as standingWords beside standingConsent; with consent declared and no words recorded, keel record decision leaves the Decision proposed and prints what is missing; keel init writes the policy with its three grant lines (delegatedRecording, standingConsent, standingWords) commented out under a note that grants are not inherited, so the policy arrives and no consent does; keel migrate treats attestation-policy.toml as project-owned, so an adopter's grants survive a resync byte-identically. This repository records its human's words of 2026-08-23 as its standingWords. NOT A FORK: there is one honest shape - the human's words are the project's fact, and a value the engine cannot supply must be declared or absent.";
        :>> rationale = "Why a declared value and not a per-project literal: a value in the binary is the same defect one build later. Why proposed rather than refused when the words are absent: the record is honest either way - a proposed Decision is a true state, and the output names the one line that would change it. Why the grant lines are commented rather than deleted at init: the reader sees the shape and the warning where they would otherwise search for it, and the rest of the policy - kinds, roles, the disposition threshold - is engine definition that must still ship. Why project-owned in migrate: the same argument that made activation.toml and github-actors.toml project-owned (issue314, D0219) - a recorded human grant is instance data. Arming (D0298): three binary tests on a fresh scaffold (no live grant; consent without words stays proposed naming standingWords; consent with words quotes exactly them and never this repository's), the record-time consent test now asserting the fixture's own words, and federation F9 asserting the policy survives migrate byte-identically.";
        :>> consequences = "A fresh keel project gives no standing consent until its human writes their words into the policy; this repository's auto-accept continues on its recorded words. The library unit's script half of GH#57 is issue381 (dcLibraryUnitRetirement). Marked safety-change: it changes a contract under the keystone lock and closes a fabricated-attestation hazard, so under D0337 it waits for the human's acceptance; the code ships now because the state it produces is honest in every arm.";
    }
}
