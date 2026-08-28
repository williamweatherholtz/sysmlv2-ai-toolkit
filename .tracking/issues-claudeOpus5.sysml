// Per-actor issues records for claudeOpus5 (issue210/srDcPerActorWriteTargets): new records
// route here so concurrent contributors never conflict at a shared append anchor.
package IssuesClaudeOpus5 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part issue292 : Issue {
        :>> id = "cea7937f-97e0-4482-8a06-4cbfddb8aa79";
        :>> title = "the shipped reference decision copy is parsed as instance data, importing 235 foreign attestations and 236 unresolvable evidence SHAs";
        :>> createdAt = "2026-08-27";
        :>> createdBy = "claudeOpus5";
        :>> description = "keel init remaps the engine's 236 decisions to .engine/reference/decisions/ and Sprint 104's record states they are NOT scanned as instance - the rust views scan .engine/decisions. That was never true: collect_sysml (lib.rs:75) has no exclusions, so the reference copy has always been parsed and registered into the downstream project's model. Sprint 104 implemented the PATH remap and not the scanning boundary it declared. Measured in a fresh init fixture: the downstream model contains 235 TestResults judged by wweatherholtz - keel's human, not a stakeholder of that project - and 236 judgedAgainst SHAs that do not resolve in that repository (sampled e146621: 'Not a valid object name'). So every downstream project's verification surface is inflated with another project's human attestations, and its evidence chain contains 236 references that D0129 requires to resolve and which cannot. D0237 fixed the symptom this surfaced (the package collision on the first recorded decision) and deliberately did not fix this. Note the constraint on any fix: .engine/rules/rules.sysml carries 10 real #JustifiedBy edges to dNNNN targets, so the reference copy cannot simply be dropped from the walk - the edges would dangle. Aggravating, and the reason this shipped: both gates covering init (sprint 104's live check and sprint 115's init smoke test) assert init-then-validate and never record a fact, which is the one state in which every defect in this class is invisible.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from scopeReferenceCopyOutOfTheInstanceModel to issue292;

    part issue293 : Issue {
        :>> id = "42488fde-9b75-44e7-b5e4-c83032b5fed4";
        :>> title = "keel activate/deactivate rewrites activation.toml and loses charteredBy plus non-switchable processes";
        :>> createdAt = "2026-08-27";
        :>> createdBy = "claudeOpus5";
        :>> description = "Found because the KEYSTONE LOCK refused a commit, which is the control working exactly as designed: I ran `keel deactivate knowledge-graph-memory` and `keel activate knowledge-graph-memory` to verify a kill switch, believed I had restored the file, and the pre-commit gate reported a locked-file change with no co-committed Decision. The diff showed two losses from one round trip. (1) `charteredBy = 'd0226'` was DELETED along with its explanatory comment - that is the D0225 charter reference, and the comment it removed says a set without a charter is a collection of switches nobody can account for. (2) `decision-authoring` was dropped from `active`, because it is reported `[always]` (asserts no guard, nothing to switch off) and the writer emits only switchable processes. The second is a semantic change, not untidiness: an ABSENT section means everything is active (D0138/D0164), but a PRESENT list naming 10 of 11 processes means the 11th is INACTIVE - so silently omitting a process from a list that exists can deactivate it. Both losses are in a file under the keystone lock, so any legitimate use of `keel activate` now either loses these facts or requires a signed Decision to re-add them, which is a write path fighting its own control. Nothing detected either loss; the lock detected the FILE changing.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from activateDoesNotLoseFacts to issue293;

    part issue294 : Issue {
        :>> id = "08e84944-fdf2-467b-8575-cd7456e119dc";
        :>> title = "four-letter domain words never seed recall: they are dropped for length before frequency is consulted";
        :>> createdAt = "2026-08-28";
        :>> createdBy = "claudeOpus5";
        :>> description = "A prompt token shorter than five characters is dropped for LENGTH before its corpus frequency is ever consulted (prompt_tokens), so four-letter DOMAIN words never seed recall: gate, hook, land, push, view. Five-letter hooks seeds and four-letter hook does not, which is arbitrary from the reader side and cannot be predicted by anyone using it. The five-character floor exists because a four-character floor admits been, does, made, that and this, and the English function-word list was sized against the longer floor. Found because a unit test asserted that gate would be dropped as TOO COMMON and it was instead dropped as TOO SHORT - the test premise was wrong, not the code, and nothing else would have surfaced it. Severity Medium rather than High: the identifier path and longer synonyms still work, so this narrows recall rather than breaking it, and the eight-case comparison gives a way to measure any fix instead of guessing.";
        :>> discoveredInField = true;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from shortDomainWordsCanSeed to issue294;
}
