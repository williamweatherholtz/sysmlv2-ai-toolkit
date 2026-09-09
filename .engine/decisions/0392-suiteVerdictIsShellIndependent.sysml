// D0392 (status: the `status` field below is the truth) — suiteVerdictIsShellIndependent: a test's outcome never turns on the launching shell's PATH - arithmetic is done in-process, and a test that must run a shell script skips by name when the shell is absent
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0392Accept event on sign-off.
package Decision0392 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: issue412, 2026-09-08: keel suite from PowerShell wrote a fail receipt (fingerprint 37f6d8356120) over a deliverable the same suite passed 589/0 from git-bash, because every test in keel-cli/tests/wrapper_resolves_only_the_pin.rs computed its fixture checksum by spawning sh -c sha256sum and a PowerShell process on this host has no sh on PATH. A grep of keel-cli/tests on 2026-09-08 finds sh spawned in that one file only: once for the checksum (arithmetic) and once to run keelw (the wrapper under test, a POSIX shell script). sha2 0.10 is already a dependency of keel-cli.
    part d0392 : Decision {
        :>> id = "a326a30a-19f9-4336-b81b-eda93fb24837";
        :>> title = "suiteVerdictIsShellIndependent: a test's outcome never turns on the launching shell's PATH - arithmetic is done in-process, and a test that must run a shell script skips by name when the shell is absent";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "dcSuiteReceiptDoesNotTurnOnPath: the suite's verdict is the receipt keel land and the facts read; a verdict that depends on which shell launched cargo is the issue065 friction class reaching the one artefact that is supposed to be the same on every machine. The item distinguishes a test whose OUTCOME turns on a shell from a test whose SUBJECT is a shell script.";
        :>> decision = "NOT A FORK: the two cases the item names each have one honest shape. A test computes what is arithmetic in-process - the wrapper fixtures hash their bytes with sha2, never by spawning a tool. A test whose subject is a shell script may spawn the shell, and checks for it first: when sh is absent the test prints a SKIP naming the missing tool and returns, so the run is green with that scenario unexercised and says so, never red over a tool the deliverable does not need. The rule holds for every test under keel-cli/tests: no test's outcome turns on the launching shell's PATH.";
        :>> rationale = "SHA-256 is SHA-256; spawning a coreutil to compute it bought nothing and cost the verdict its independence from the shell. A wrapper written in POSIX shell cannot be exercised without one, and pretending otherwise would either fake the run or fail it; a named skip is the truthful third state, and it is visible in the output rather than silent. A green suite from a shell that could not run the wrapper is then a true statement about what ran.";
        :>> consequences = "wrapper_resolves_only_the_pin.rs hashes with sha2 and every scenario begins with the sh check; the suite is run once from PowerShell and once from git-bash on this host and both receipts read pass with equal counts, which is the item's verification. A future test that needs a shell for its subject follows the same shape, and one that spawns a shell for arithmetic is the defect this Decision names.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0392Accept : Test { :>> id = "95aa3bd4-ad45-4623-a219-12ba25b5a226"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0392AcceptR1 : TestResult { :>> id = "68e69ae3-31d1-4a3e-99f6-aadbfebfb203"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4b75101"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
