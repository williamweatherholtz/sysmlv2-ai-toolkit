// =====================================================================
// Sitting 03 review — the per-sitting human gate (D0049/D0073), plus the §2.5 frozen-schema sign-off
// that unblocks dcDeclaredMarkers. Confirmation TestResults written ONLY on the human's explicit
// attestation (D0016/D0051 — never inferred), and substantive per issue083/dcAttestationSubstance.
// =====================================================================
package ProjectSitting03 {
    private import EngineElement::*;
    private import EngineVerification::*;

    verification sitting03Review : Test {
        :>> id = "2dfadd37-5a74-41f4-8964-dd78c435b4d8";
        :>> title = "Sitting 03 review — sprints 247-255 (distributed collaboration + the tracking-mechanism controls) + D0129-D0132 (2026-08-12/13)";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Per-sitting sprint review (D0049 single human gate; D0073 mechanism; batched per D0019). wweatherholtz reviewed and accepts the content of this sitting: sprint 247 (distributed-collaboration SE chain + five day-one blockers: ancestry-preserving integration, duplicate-identity guard, CSPRNG ids, actor identity binding, portable gate), 248 (supersede D0014, reprioritize the backlog per D0052, declare four tracking-mechanism controls), 249 (response contract into a system-prompt output style + the in-loop Stop gate), 250 (verification-trace guard + verifiedByMethod), 251 (attestation-substance guard), 252 (priority-inversion guard), 253 (retroTrack process step + retro-backlog guard), 254 (kernel instance validator demoted to opt-in), 255 (decompose the three D0128 Needs into 10 SRs, root D0130-D0132, fix a dual-truth). Decisions accepted across the sitting: D0128 and D0129 (attested via keel serve on 2026-08-13), D0130 (attested 'accepted proceed' after requesting and reading its full text), and D0131 + D0132 (this attestation). Guard count 17 -> 22. Issues 069-088 recorded; issues 071-076, 079, 081-085 resolved. The sitting's own defects are recorded rather than smoothed over: eight SKIP_VALIDATE bypasses before issue081 was fixed, a live reproduction of the issue074 number-collision on the author's own work, a priority inversion in the author's own reordering, and a dual-truth verdict the author introduced into d0130. Explicit attestation: 'accept on all counts' (2026-08-13), in direct response to an enumerated list naming the frozen-schema sign-off, D0131/D0132 acceptance, and this per-sitting review.";
    }
    part sitting03ReviewR1 : TestResult {
        :>> id = "a579941b-1c74-44f5-971e-da9143e24b72";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "8df2822";
        :>> judgedAt = "2026-08-13";
        :>> judgedBy = "wweatherholtz";
    }

    // ── §2.5 frozen-schema sign-off: the gate that unblocks dcDeclaredMarkers ──
    // schema/core is FROZEN; a change to it requires explicit human sign-off (CLAUDE.md §2.5/§4).
    // This is a SEPARATE attestation from the sitting review because it authorizes a specific
    // structural change, not a body of completed work.
    verification declaredMarkersSchemaSignOff : Test {
        :>> id = "4811f6b8-cedf-4689-b91f-35e25b187129";
        :>> title = "Frozen-schema sign-off (§2.5) — declare the guard-consumed metadata markers";
        :>> createdAt = "2026-08-13";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz authorizes the FROZEN-SCHEMA change (§2.5/§4) required by dcDeclaredMarkers / issue077: declaring the metadata markers that guards and computed views consume in .engine/schema/core/relationships.sysml, and making validation REJECT an undeclared or misspelled marker. Scope authorized: #Verify (used in 31 files, the most-used of all and what tier-satisfaction/sr_verified_pct and the verification-trace guard both key on), #DerivedFrom (22 files, load-bearing for the hard requirement-rootedness guard), #Capability (15), #Dispositions (10), #Covers (6), #ProcessDefect (5), plus #Satisfy and #Allocate. RISK ACCEPTED: markers are currently NOT type-checked, so a misspelling validates clean and silently removes that item from the depending guard's view — a blind guard reports PASS, and a typo in #Verify would report a DELIVERED requirement as unverified. Attested 2026-08-13 with the words 'accept on all counts', in direct response to an enumerated list in which this frozen-schema sign-off was the first named item.";
    }
    part declaredMarkersSchemaSignOffR1 : TestResult {
        :>> id = "02c38c5e-8590-4e33-baaf-95f5d51ca5f9";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "8df2822";
        :>> judgedAt = "2026-08-13";
        :>> judgedBy = "wweatherholtz";
    }
}
