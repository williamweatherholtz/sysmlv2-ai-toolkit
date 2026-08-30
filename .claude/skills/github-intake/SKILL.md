# github-intake — an issue becomes their words, then my judgment

Deploys `.engine/processes/github-intake.sysml` (D0263). The inbound half of the federation loop:
the decision-channel carries decisions **out** and approval gestures **in**; nothing carried an
**issue** in, so a defect found downstream lived in a browser tab.

## Procedure

1. **Ingest verbatim.**
   `keel github-ingest --repo O/N --issue N --by <you> --at <today>`
   The body is stored character-for-character. `saidBy` is the **GitHub login**, not an enrolled
   actor — an outside reporter has no actor id here, and inventing one misattributes their words.
   `sourceUrl` makes a re-ingest **refuse** rather than store the same words twice.
   **Never retype an issue by hand.** A paraphrase in a field labelled verbatim is the defect this
   whole path exists to prevent.
2. **Triage.** `keel record story --from-statement <st> --implication <kind> --triage-note "..."`.
   A compound issue becomes **several** stories. The note is the reviewable part.
   A kind the vocabulary cannot express is a **change to the vocabulary** through a Decision — that
   rule is why `github` was added to `StatementChannel` instead of filed under `other`.
3. **Route.** Author the downstream item and `#Implicates` from the story to it. A `bug` needs an
   Issue, and `record issue` needs a **resolver** — if none exists, author the backlog task. Routing
   to the nearest plausible owner looks handled and is not.
4. **Answer where they are standing.** Comment the tracked id and the verdict on the GitHub issue,
   and close it there. The model is the truth, but the reporter is not reading the model. A declined
   issue gets the same courtesy and the reason.

## What this does NOT do

It does not create an `Issue` directly. A GitHub issue is someone's words; what it implicates is a
judgment (D0216), and `record issue` requires a resolver that ingestion cannot know. An
ingest-to-Issue path would have to invent one.

## Removal path

Delete this skill + registry + the process file, and the `github-ingest` dispatch arm. The
`github` channel member and `sourceUrl` attribute may stay — both are harmless and `sourceUrl` is
`[0..1]`.
