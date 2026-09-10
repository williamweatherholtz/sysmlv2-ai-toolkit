// D0423 (PROPOSED — NOT YET ACCEPTED) — delegatedWordsAreRecordedNotRefused: keel accept --words records the human's words as they gave them - a bare letter included - and the ten-character, read-back and gesture-word checks report in the note, never refuse
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0423Accept event on sign-off.
package Decision0423 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0423 : Decision {
        :>> id = "1ab53d80-ab65-4c64-97b1-ff0319de345f";
        :>> title = "delegatedWordsAreRecordedNotRefused: keel accept --words records the human's words as they gave them - a bare letter included - and the ten-character, read-back and gesture-word checks report in the note, never refuse";
        :>> createdAt = "2026-09-10";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "On 2026-09-10 the human answered an acceptance ask with 'A' and keel accept refused it (D0375: at least ten characters). Asked whether any agent had ever attempted to accept a Decision under their authority, the record could not say: a refused accept writes nothing (issue445), the fire-ledger records every in-loop block as an allow (issue446), and the three Decisions behind the refusal rest on issue059 (a vigilance gap, not an acceptance), issue426 (a UCA found by code read in stpa-self run 2, no act observed) and issue397 - the same gate refusing EIGHT of the human's own correct acceptances in one note. 453 acceptances recorded by AI actors stand in .engine/decisions, every one of which passed the gate of its day. Their words, st115: 'I thought we got rid of these guards that handcuff me and do very little to AI agents ... if not, then this guard is moot and should be removed along with others like it ... users like me shouldn't be the entities strictly regulated here.'";
        :>> decision = "NOT A FORK: the human has decided the direction in st115; this is its first instance. When an agent session records a human's acceptance under delegation (attestation-policy.toml delegatedRecording), keel accept --words takes the words as given and writes the record: the recorder is stamped createdBy, the judge judgedBy, the words are folded into the note inside typographic quotes as today. The three checks that refused - fewer than ten characters (D0375), no read-back of the id, an option letter or three title words (D0289 / D0201 B), and a gesture word as the only evidence (D0411's delegated clause) - are computed the same way and, when they would have refused, are written into the note as a WARN line naming which check, so the record says what kind of receipt it is. Nothing about a console or deck record, a TTY record, or the requirement that the words be the human's VERBATIM changes; a paraphrase is still the fabrication D0198 names.";
        :>> rationale = "A control is kept on evidence that it prevents something (nControlBindsTheSubvertingActor, us105). This one's whole record is: zero observed attempts, an instrument that could not have counted one, and nine refusals of a human's correct word. The theory behind it - that a weak word could be attached to any Decision - stays visible as the WARN line, which is what a reviewer of an acceptance actually needs; the refusal added nothing to that visibility and cost the human their channel. What would change it: once refusals are ledger facts (dcRefusalIsALedgerFact) and the census (dcControlCensusByActorKind) shows a WARN line on a record whose words the human disputes, a refusal is restored for that check by a superseding Decision with the incident named.";
        :>> consequences = "keel-cli/src/main.rs verdict_channel_refusal: the three delegated-word checks return a warning string instead of Some(exit), appended to the note as WARN: <check>; the TTY, console and deck paths are untouched. keel-cli/tests covering D0375 / D0289 / D0411 delegated refusals are rewritten to assert the WARN line and the successful write; the acceptance recorded 2026-09-10 for D0421/D0422 needed a second attempt only because of this and is otherwise unaffected. CLAUDE.md sec 4 acceptance paragraph and .engine/docs/guards.md say the checks report. Safety-change (D0209 clause 2): a control's refusal is weakened, so this stays proposed under D0337 for the human's word and audit-adherence reads it as the signed weakening. What accepting binds: an agent session can record any words the human gave. What it forecloses: the engine refusing on the shape of a human's word.";
    }
    #SupersedeClause dependency from d0423 to d0375;
    #SupersedeClause dependency from d0423 to d0289;
    #SupersedeClause dependency from d0423 to d0411;
}
