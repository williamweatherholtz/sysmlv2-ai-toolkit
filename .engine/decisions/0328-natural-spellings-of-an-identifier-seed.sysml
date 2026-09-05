// D0328 (status: the `status` field below is the truth) — naturalIdentifierSpellings: the spellings a person types for an identifier join into the compact form the corpus mints before seeding
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0328Accept event on sign-off.
package Decision0328 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0328 : Decision {
        :>> id = "fe97b875-c563-478e-8c69-2dccc699eafe";
        :>> title = "naturalIdentifierSpellings: the spellings a person types for an identifier join into the compact form the corpus mints before seeding";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue297 / identifierMatchingAcceptsNaturalSpellings: identifier lookup is the one query-sensitive recall path both the cost-benefit and IR critics named as the feature's real value, and it was brittle - `d0242`, `issue294` and `what does D0242 say` seeded, while `issue 294`, `D-0242` and `decision 242` pushed nothing, because is_identifier_token wanted the digits abutting the prefix with no separator. Reconfirmed on the binary before the change: `issue 294` seeded the words issue(1047) and 294(16) and surfaced the item whose DoD text mentions 294, not issue294. A sprint had no identifier at all: `sprint480` names nothing in the corpus, whose sprint element is the delivery run `DeliveryRunS480`.";
        :>> decision = "query_terms joins natural spellings into the compact identifier before tokenising - a prefix token followed by an all-digit token (`issue 294` -> issue294, `decision 242` -> d0242 zero-padded to four, `sprint 480` -> DeliveryRunS480) and a hyphen between prefix and digits (`D-0242` -> d0242) - for the four prefixes issue, sprint, decision and d only, so a bare number or a year is never joined; `sprintNNN` in both spellings maps to the delivery run `DeliveryRunSNNN`, which is_identifier_token now recognises as an identifier. Pinned by unit test: the six spellings, the year, and a hyphenated ordinary word that is not a prefix-digit pair. NOT A FORK: one course, the join; the spellings named are inputs, not alternatives.";
        :>> rationale = "Why join before tokenising rather than widen the identifier regex: the corpus mints one shape per kind and the reader's job is to reach it from whatever the person typed; joining at the boundary keeps is_identifier_token the single definition of what an identifier IS. Why the delivery run for sprints: measured - the compact form seeded nothing, so parity between spellings would have been parity at zero; the run is the element the scaffold names for a sprint. Why only four prefixes: `in 2026 we shipped 294 things` must stay words, and it does. Read back on the binary: all six spellings seed the same element with top score 27.8 and the named marker; the 50-case set is unchanged at 48/50 in 13 rows.";
        :>> consequences = "`keel recall`, the prompt-path push and `keel why` accept the natural spellings. issue297 is resolved by identifierMatchingAcceptsNaturalSpellings. STATED: a prefix followed by a number that names nothing (`issue 2026`) is joined and finds nothing, which the payload says; it is not treated as a year.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0328Accept : Test { :>> id = "d00201f3-ba8c-486d-a363-3b622c1d6b2b"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0328AcceptR1 : TestResult { :>> id = "49e0a631-3774-4654-8edc-a6d9cd0402af"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "73ae2a2"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
