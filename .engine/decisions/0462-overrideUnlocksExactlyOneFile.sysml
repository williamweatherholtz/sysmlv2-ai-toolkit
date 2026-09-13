// D0462 (status: the `status` field below is the truth) — overrideUnlocksExactlyOneFile: an override unlock covers exactly one file, matched by canonical path - keel override refuses a directory or a prefix
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0462Accept event on sign-off.
package Decision0462 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0462 : Decision {
        :>> id = "1a7d8862-6c29-4b5b-9de1-12fae130285e";
        :>> title = "overrideUnlocksExactlyOneFile: an override unlock covers exactly one file, matched by canonical path - keel override refuses a directory or a prefix";
        :>> createdAt = "2026-09-13";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The D0176 tier-3 unlock is documented as single-use, target-path-bound and expiring, and consume_override's own comment says 'an unlock for one file covers no other'. The code did not say that: it matched the written path against the armed target by substring in EITHER direction, so keel override .tracking armed an unlock the next write to ANY path containing .tracking consumed, and a one-character target covered every path holding that character. stpa-self run 2 found it by code read as UCA-O1 (cmdOverride provided with a wide target, hazard EHZ2: a write lands outside a validated channel) and recorded it as issue427 with the constraint ccOverrideNamesOneFile bound to the Issue because no control did this. The exposure was one write of the agent's choosing under a target that named no file, recorded as an obligation naming only the path actually written.";
        :>> decision = "NOT A FORK: issue427's resolver names the change and the constraint CC-O1 states it. NOT A PROCESS CHANGE: the D0176 tier-3 procedure - arm with a reason, one write, recorded obligation, 900 s expiry - is untouched; this makes the code hold the path-bound rule its own comment and D0176 already state. keel override accepts exactly one file: an existing regular file, or one new file under an API-owned surface (its parent exists and the path carries a PROTECTED_PATHS prefix - the one place the API owns a file's creation). A directory, a prefix or a file nowhere is refused, and the refusal names the file-not-directory rule. The unlock stores the target's canonical key (absolute, links and short names resolved, forward slashes, no UNC prefix; a file not yet created is keyed by its parent's canonical form plus its name) and consume_override matches the written path's canonical key by EQUALITY only. Single use, expiry and the recorded obligation stand as today.";
        :>> rationale = "A control whose unlock can be armed wider than one file is a control the agent can widen by choosing the target, which is the self-modification shape D0209 exists to close; binding by canonical equality makes the unlock say exactly one file the way the filesystem names it, so neither a short-name spelling nor a relative path nor a lookalike suffix opens a different write. Allowing one new file under an API-owned surface keeps the sanctioned repair case - a corrupted per-actor issues file the API cannot yet express - without re-opening the prefix.";
        :>> consequences = "Resolves issue427 through dcOverrideBindsToOneFile (sprint 689); ccOverrideNamesOneFile's binding moves from the Issue to the control ctlOverrideObligation in the STPA record. main.rs is not on the keystone lock's enforcement surface (is_enforcement_surface excludes it), so no marker rides this commit; the pre-write hook's behaviour narrows and the existing integration test override_unlock_is_single_use_path_bound_and_recorded must still pass unchanged. STATED: an unlock armed before this build with a directory target is not migrated - it expires within 900 s and its file holds a key the new reader canonicalises like any other; a directory key equals no file key, so it opens nothing.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0462Accept : Test { :>> id = "814109d6-a362-460b-8e3b-0016b4f5c1f9"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0462AcceptR1 : TestResult { :>> id = "d5baf027-ec99-4b97-b15d-271811a5f4e4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c364492"; :>> judgedAt = "2026-09-13"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeFable5"; }
    #DependsOn dependency from d0462 to d0176;
}
