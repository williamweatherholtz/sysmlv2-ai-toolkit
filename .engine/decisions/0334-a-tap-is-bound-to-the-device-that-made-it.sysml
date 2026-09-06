// D0334 (status: the `status` field below is the truth) — tapBindsToDevice: a console or deck tap carries a paired device's HMAC and is refused without it - the device half of D0201 option B
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0334Accept event on sign-off.
package Decision0334 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0334 : Decision {
        :>> id = "27feef5f-baad-4df8-8b7e-8f615663dec3";
        :>> title = "tapBindsToDevice: a console or deck tap carries a paired device's HMAC and is refused without it - the device half of D0201 option B";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0201 OPTION B was the human's choice on 2026-08-23 (deck-tap signature binding: device HMAC on taps, read-back ratification for chat approvals) and no tracked work carried it until the rigor audit found the gap seven days later (dcAttestationBindsToDevice). Until today a tap on the console or deck was whatever POST reached keel serve on localhost - any process on the machine, the agent included, could record a human's judgment because a request said so.";
        :>> decision = "keel serve prints a six-digit pairing code to its terminal at start; a browser generates a device key once, pairs by POSTing the code and its key to /api/device/enroll (stored machine-locally in .keel/devices.toml, gitignored), and signs every human tap - accept, reject, gate result, disposition, test verdict, sitting review - with HMAC-SHA256 over kind|target|date|judge|note; the server verifies against the paired keys in constant time and refuses a tap with no device, an unknown device or a non-matching signature with 401 and nothing written; a recorded tap carries `[device <id> HMAC-verified]`. SCOPE, stated so it is not over-claimed: this proves a tap came from the browser that paired, not who was at it; the trust that the paired browser is the human's rests on the pairing code reaching them through the serving terminal, the same assumption keel accept at a TTY makes (D0315).";
        :>> rationale = "Why a device key and not a password: the server and browser share localhost; the one thing a request cannot forge is a key the browser holds and the server learned only through the human typing the terminal's code, and HMAC over the canonical text stops a signature over one tap being replayed as another. Why HMAC-SHA256 by hand on sha2: RFC 2104 is eleven lines and the tree already carries sha2; the implementation is pinned to RFC 4231 test vector 2. Why the tap's note is tagged: the record says which device signed, so a later reader can tell a paired-device attestation from the pre-D0334 records that carry none. Arming (D0298): device unit tests (RFC vector, both directions, store round-trip); keel-cli/tests/device_bound_taps.rs on a live serve - unsigned refused and nothing written, wrong pairing code refused, paired device's wrong signature refused and nothing written, correct signature recorded and device-tagged; the deck e2e tool pairs before it taps.";
        :>> consequences = "The console and deck pair once per browser (a prompt on the first tap); .engine/tools/test_deck_e2e.py pairs programmatically. Every tap endpoint's request gains device_id and hmac, required in effect. CLAUDE.md §4 states the binding and its scope. The chat half of D0201 B is D0335. STATED RESIDUAL: a process on the machine that reads the serving terminal's stderr can pair a device of its own - the binding is to the terminal's reader, which in this repository's own sessions is the agent that launched serve; the human's own terminal, not the agent's, is where a serve meant to bind the human belongs.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0334Accept : Test { :>> id = "92ea61ee-920c-4421-8e91-cbc8beb7da81"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0334AcceptR1 : TestResult { :>> id = "c85b6b7f-ef29-4f06-bcd3-f36b5dd574d9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c850aff"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
