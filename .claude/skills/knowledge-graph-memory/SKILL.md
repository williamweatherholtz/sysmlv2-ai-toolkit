---
name: knowledge-graph-memory
description: |
  Deploys the Knowledge-Graph Memory process (.engine/processes/knowledge-graph-memory.sysml): give
  an AI deterministic, constant-cost recall over a document corpus by extracting a typed graph and
  INJECTING the relevant facts before the model is asked — retrieval becomes code, not conversation.
  Use when an assistant must answer questions spanning documents that share no vocabulary; when
  search is returning wrong answers, or right answers slowly and expensively; when someone asks for
  "memory", RAG, or a knowledge graph over their docs; or when answer quality depends on which model
  is affordable. Do NOT use for a single-document lookup, for a corpus small enough to read whole, or
  where the answer is one keyword away — the graph is overhead there.
metadata:
  version: 0.1.0
  domain: [knowledge-graph, retrieval, memory, ontology, prompt-injection, RAG]
  writePolicy: direct
  engine: keel-ai-toolkit
  deploys: [.engine/processes/knowledge-graph-memory.sysml]
  source: "Applying Knowledge Graphs — Glitch Cat Club, 16 Aug 2026; github.com/Glitch-Cat-Club/graph-memory-starter (D0153)"
---

# knowledge-graph-memory — push the facts, don't let the model hunt

## The one idea

**Pull:** the model hunts for facts during the conversation — grep, read, grep. Cost grows with the
corpus. Accuracy depends on which model you can afford.

**Push:** code finds the facts *before* the model is involved and hands them over. Fixed cost at any
corpus size. Accuracy comes from structure rather than from model strength.

That swap is the whole thing. Everything below serves it, and one rule survives every implementation
choice: **the model never does the traversal.** A graph the model must *decide* to query is still
pull, and inherits every cost this exists to remove.

## Expert vocabulary payload

**Ontology** — the closed lists of entity types and relationship types, derived from the QUESTIONS.
**Logical model** — the shapes a fact can take; one table per shape.
**Seeding** — matching a question's words to entity names and aliases, to find where to start.
**Walking** — collecting everything connected to the seeds, hop by hop, to a bounded depth.
**Computed identity** — an entity's id is a hash of `type + normalised name`, so the same thing named
in two documents is one node. No matching service, no ML, and re-extraction MERGES rather than
duplicating — which is what makes the build re-runnable at all.

**Tiers** (pick by scale; the two jobs never change): **1** relational store, three tables, one
recursive query, up to a few hundred thousand facts — most systems live here; **2** embedded graph
engine, native traversal, one file, no server; **3** server graph database, traversal inside the
engine, team scale.

## Behavioral instructions

1. **Ask for the questions first.** Not topics — questions. If the human offers documents, ask what
   they need answered from them. A vocabulary derived from a corpus describes the corpus; a
   vocabulary derived from questions answers them.
2. **Draft the vocabulary; let the human decide it.** Entity types are the kinds of thing the
   questions mention; relationship types are how those connect. Keep both small and CLOSED. Push
   conditions — amounts, dates, windows — into entity descriptions, never into new types.
3. **List fact shapes, one table each.** Three tables is common, not canonical. A new shape gets a
   table; folding it into a nullable column is how a query becomes inexpressible later.
4. **Extract with computed identity.** Hash type + normalised name. Never look identity up.
5. **Build the two jobs.** Seeding and walking, output as text carrying facts AND their conditions.
6. **Inject before the model thinks.** Wire recall into the harness's prompt-submit path; zero model
   calls on that path; show a visible recall count so the human can see it fired.
7. **Verify on a trap you built to be hard**: a multi-hop chain whose documents share no words, with
   decoys — a stale variant, an unrelated document reusing the same distinctive number. Compare
   against the raw corpus. Also verify the honest negative: out-of-vocabulary questions must return
   no match rather than a guess.
8. **Record the limits** — lexical seeding, top-k crowding, store size, meaning at scale — with the
   remedy and its kind. The limit is never hops.

## Anti-pattern watchlist

1. **The decorative graph** — the model still searches and reads; the graph is scenery. *Detection:*
   the model makes tool calls to answer. *Resolution:* move retrieval to the prompt-submit path.
2. **Vocabulary from documents** — types invented by surveying the corpus. *Detection:* entity types
   nothing in the question set mentions. *Resolution:* re-derive from questions; delete the rest.
3. **Open vocabulary** — types added ad hoc during extraction. *Detection:* the type list grows per
   document. *Resolution:* close the list; an unmatched thing is a description, not a new type.
4. **Looked-up identity** — a matching service or fuzzy join deciding whether two mentions are one
   thing. *Detection:* re-running the build duplicates nodes. *Resolution:* hash type + normalised
   name.
5. **Easy-question verification** — demonstrating on a question keyword search already answers.
   *Detection:* the before/after control also passes. *Resolution:* build the multi-hop trap.
6. **Silent empty recall** — no facts found, and the model answers anyway from its own priors.
   *Detection:* an out-of-vocabulary question still gets a confident answer. *Resolution:* recall
   reports no match explicitly, and that is the honest outcome, not a failure.
7. **Depth panic** — treating hop count as the scaling limit. *Detection:* effort spent capping hops.
   *Resolution:* six hops is six edges; the real axes are density, size, vagueness and ambiguity.

## For a keel project: the engine's steps and the optional ones (D0332)

When the corpus IS a keel model, most of the eight steps above are already the engine's, and the skill
must not send a project off to rebuild them:

| Step | Who does it | What a project does |
|---|---|---|
| 1-3 vocabulary, shapes | **the engine** - seeding is corpus-derived (D0243): names, titles, bodies, typed edges | nothing |
| 4-5 identity, seed + walk | **the engine** - `keel recall` / `keel why` over the model (D0161, D0243) | nothing |
| 6 inject before the model | **the engine** - `keel init` and `keel sync-claude` wire the UserPromptSubmit hook (claude_surface.rs) | run `keel sync-claude` after an engine update |
| 7 verify on a trap | the engine's own benches (recall_bench.py, recall_ab.py) are THIS repository's | measure on your own model if you change the ranker |
| declare questions | **optional** - `.knowledge/questions` makes `keel knowledge question-coverage` computable | declare them if you want coverage measured |
| declare aliases | **optional** - `.knowledge/lexicon` routes words your people use that the corpus does not | declare them if seeding misses your vocabulary |

`.knowledge/` is INSTANCE data: `keel init` ships none of it, and this repository's questions never
travel to another project (the issue243 class). A fresh project therefore recalls over its own model with
an empty `.knowledge/` and the hook already wired - verified on a scaffold: the hook is present,
`keel recall` answers over the scaffold's 326 items, no question of this repository is present, and
`sync-claude --check` is clean.

## What this process does NOT bring

**No guards.** A keel process unit is definition + deploying skill + declared rules/guards + metadata,
and this one declares no guards deliberately: every candidate — "the graph is fresh with respect to
the corpus", "the vocabulary is closed" — depends on a store this engine does not own and cannot
read, so a guard here would either be unenforceable or would gate on a file the engine has no
business knowing. If a project wants freshness enforced, it declares that guard in its own
`.engine/`, where the corpus lives.

**No claim to the source's numbers.** The artefact this comes from measured its results on one
corpus and one question. Those measurements are recorded in D0153 as ITS claims. This skill
reproduces the METHOD; a project that follows it must measure its own.

## Questions this skill answers

- "Add memory / a knowledge graph / RAG to this assistant"
- "The AI keeps missing answers that span several documents"
- "Retrieval is slow and expensive and gets worse as we add docs"
- "Only the expensive model gets this right — can a cheap one?"
- "How do I model an ontology for my domain?"
