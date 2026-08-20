// =====================================================================
// EngineKnowledgeGraphMemory — give an AI deterministic recall over a corpus by PUSHING facts to it,
// never letting it hunt for them.
//
// SOURCE: "Applying Knowledge Graphs" (Glitch Cat Club, captured 16 Aug 2026;
// github.com/Glitch-Cat-Club/graph-memory-starter). Adapted into a portable keel process unit at the
// human's request. The artefact's measured claims are recorded in D0153 as ITS claims, not as this
// engine's — they were demonstrated on one corpus and one question, and this process reproduces the
// METHOD, not the numbers.
//
// THE ONE IDEA, and every step below exists to serve it: PULL vs PUSH. In the usual arrangement the
// model does the searching during the conversation — grep, read, grep — so cost grows with corpus
// size and accuracy depends on which model you can afford. Done properly, CODE finds the facts before
// the model is involved and hands them over. The model never takes part in the finding. That swap is
// the whole difference, and the rule survives every implementation tier: THE MODEL NEVER DOES THE
// TRAVERSAL.
//
// TIERS (choose by scale, not by ambition; the jobs never change):
//   Tier 1 — SQLite/Postgres, a real graph in three tables walked by one recursive query. Up to a few
//            hundred thousand facts. Most personal and small-business systems live here.
//   Tier 2 — an embedded graph engine with native traversal, one file, no server.
//   Tier 3 — a server graph database; traversal runs inside the engine. Team scale.
//
// WHY THIS IS A PROCESS AND NOT A FEATURE: the vocabulary decisions are the human's and the
// extraction is the AI's, in that order, and getting the order wrong produces a graph that answers
// nothing. That sequencing is exactly what a process is for.
// =====================================================================
package EngineKnowledgeGraphMemory {
    private import EngineElement::*;     // Cadence / Owner enums
    private import EngineProcess::*;

    action knowledgeGraphMemory : Process {
        :>> id = "a41d7e05-3b92-4c68-8f17-52e0d9a6c3b4";
        :>> title = "Knowledge-Graph Memory";
        :>> purpose = "Give an AI deterministic, constant-cost recall over a document corpus by extracting a typed graph from it and injecting the relevant facts before the model is asked — so retrieval is code, not conversation.";
        :>> cadence = Cadence::perItem;
    }

    action kgQuestions : ProcessStep {
        :>> id = "a41d7e05-3b92-4c68-8f17-000000000s01";
        :>> title = "Start from the questions, not from the documents";
        :>> triggerCondition = "An AI must answer questions over a corpus, and search is getting the answers wrong, slow, or expensive — typically where the answer spans documents that share no vocabulary.";
        :>> actionText = "Write down the actual questions the system must answer. Not topics, questions. These drive everything downstream: the entity types are the kinds of thing the questions mention, and the relationship types are the ways those things connect. A corpus surveyed without questions produces a vocabulary that describes the documents rather than answers anything.";
        :>> producedArtifact = "The question set, written down.";
        :>> owner = Owner::human;
    }

    action kgVocabulary : ProcessStep {
        :>> id = "a41d7e05-3b92-4c68-8f17-000000000s02";
        :>> title = "Derive a small CLOSED vocabulary — the ontology";
        :>> triggerCondition = "The question set exists.";
        :>> actionText = "List the kinds of thing the questions mention: that list is the entity types. List the ways those things connect: that list is the relationship types. Keep both SMALL and CLOSED. Put conditions — amounts, dates, windows, thresholds — in entity DESCRIPTIONS rather than inventing types for them; a threshold is a property of a thing, not a kind of thing. The AI drafts, the human decides: the vocabulary is the one part of this a model should not choose alone, because every later step is measured against it.";
        :>> producedArtifact = "A closed list of entity types and relationship types.";
        :>> owner = Owner::human;
    }

    action kgLogicalModel : ProcessStep {
        :>> id = "a41d7e05-3b92-4c68-8f17-000000000s03";
        :>> title = "List the SHAPES facts take; each shape becomes a table";
        :>> triggerCondition = "The vocabulary is closed.";
        :>> actionText = "Enumerate the shapes a fact can take in this domain, and give each shape a table. A thing exists; two things connect; a thing has another name — that is three tables, and three is not a magic number, it is what that model needs. Facts in other shapes — time-bounded facts, measurements, provenance-carrying facts — mean more tables, one per shape. Resist folding a new shape into an existing table with a nullable column: a shape that needs its own table and does not get one becomes the thing queries cannot express later.";
        :>> producedArtifact = "The store schema — one table per fact shape.";
        :>> owner = Owner::ai;
    }

    action kgExtract : ProcessStep {
        :>> id = "a41d7e05-3b92-4c68-8f17-000000000s04";
        :>> title = "Extract the corpus against the vocabulary, with COMPUTED identity";
        :>> triggerCondition = "The schema exists.";
        :>> actionText = "Have the AI extract every document against the closed vocabulary, emitting typed entities, typed relations and aliases. IDENTITY IS COMPUTED, NEVER LOOKED UP: an entity's id is a hash of its type and normalised name, so the same role named in two documents becomes one node with no matching service and no ML. This is what makes re-extraction safe — extraction varies run to run, and the hash is why re-runs MERGE instead of duplicating. Extraction works on unstructured prose; front matter carrying the vocabulary per document is maintenance discipline, not a prerequisite.";
        :>> producedArtifact = "A populated graph: entities, relations, aliases, each carrying its source document.";
        :>> owner = Owner::ai;
    }

    action kgTraversal : ProcessStep {
        :>> id = "a41d7e05-3b92-4c68-8f17-000000000s05";
        :>> title = "Implement the two jobs: seeding and walking";
        :>> triggerCondition = "The graph is populated.";
        :>> actionText = "Every implementation needs exactly two jobs, whatever the tier. SEEDING: match the question's words against entity names and aliases to find where to start. WALKING: collect everything connected to the seeds, hop by hop, to a bounded depth. At Tier 1 both live in one small file and the walk is a recursive query; at Tier 2 and 3 the walk becomes one line of the engine's query language. The jobs never change — only where they run. Output is TEXT, shaped to carry the facts and the conditions attached to them, because text is what will be handed to the model.";
        :>> producedArtifact = "A recall function: question in, facts as text out, with no model call.";
        :>> owner = Owner::ai;
    }

    action kgInjection : ProcessStep {
        :>> id = "a41d7e05-3b92-4c68-8f17-000000000s06";
        :>> title = "Inject the facts BEFORE the model thinks";
        :>> triggerCondition = "Recall returns facts as text.";
        :>> actionText = "Wire recall into the harness's prompt-submit path so the facts are placed in front of the model as additional context before it begins, and so a visible line reports how many facts were recalled and how long it took. THIS STEP IS WHAT MAKES IT PUSH RATHER THAN PULL — a graph the model must decide to query is still pull, and inherits every cost this process exists to remove. The hook contract is harness-specific; the requirement is not: zero model calls on the prompt path, and the facts already present when the model wakes.";
        :>> producedArtifact = "A prompt-submit hook that injects recalled facts, with a visible recall count.";
        :>> owner = Owner::ai;
    }

    action kgVerifyOnATrap : ProcessStep {
        :>> id = "a41d7e05-3b92-4c68-8f17-000000000s07";
        :>> title = "Verify on a deliberate multi-hop trap, not on an easy question";
        :>> triggerCondition = "Injection is wired.";
        :>> actionText = "Build a test case that DEFEATS keyword search by construction: an answer that requires chaining several facts whose documents share no words, with decoys planted around them — a stale variant, and an unrelated document using the same distinctive number. Run the same question against the raw corpus and the graph, ideally across model sizes. A graph that only beats search on questions search could already answer has demonstrated nothing. Also verify the honest negative: a question outside the vocabulary must return no memory matches rather than a guess.";
        :>> producedArtifact = "A trap case with recorded before/after results, including the empty-recall refusal.";
        :>> owner = Owner::ai;
    }

    action kgRecordLimits : ProcessStep {
        :>> id = "a41d7e05-3b92-4c68-8f17-000000000s08";
        :>> title = "Record the failure axes — the limit is never hops";
        :>> triggerCondition = "The trap case passes.";
        :>> actionText = "Write down where this will break, because the axes are known and none of them is depth: six hops is six edges. LEXICAL SEEDING — a question naming no known entity or alias finds nothing; the extension is embedding-based seeding beside the alias table. TOP-K CROWDING — dense hub entities push chain links out of a fixed-size injection; rank by path membership between seeds, prune, raise k. STORE SIZE — past a few hundred thousand edges, or heavy concurrent querying, move up a tier; the schema shape survives. MEANING AT SCALE — name collisions merge things that are not the same and stale documents pollute; the answer is disambiguation at extraction and supersession at ingest, which is modelling discipline rather than tooling.";
        :>> producedArtifact = "A recorded limits table with the remedy and its kind for each axis.";
        :>> owner = Owner::ai;
    }

    // Sequencing. The order is load-bearing at the front and merely sensible at the back: questions
    // before vocabulary before extraction is what stops the graph describing the documents instead of
    // answering anything.
    succession kgS1 first kgQuestions then kgVocabulary;
    succession kgS2 first kgVocabulary then kgLogicalModel;
    succession kgS3 first kgLogicalModel then kgExtract;
    succession kgS4 first kgExtract then kgTraversal;
    succession kgS5 first kgTraversal then kgInjection;
    succession kgS6 first kgInjection then kgVerifyOnATrap;
    succession kgS7 first kgVerifyOnATrap then kgRecordLimits;
}
