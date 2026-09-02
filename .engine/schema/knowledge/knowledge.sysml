// =====================================================================
// EngineKnowledge — the knowledge store's two declared fact types (D0161).
// OPTIONAL MODULE — the feature's entire input is human-authored Questions
// and Aliases under `.knowledge/`; delete them and `keel show why` /
// `keel show knowledge question-coverage` report NOTHING DECLARED (data-level
// removability, D0161 part 3i). For keel's own corpus the MODEL IS THE
// GRAPH: no entities, relations or answer text are stored here — these two
// types only declare what a human wants findable and the words they use.
// =====================================================================
package EngineKnowledge {
    doc /* Knowledge-store declared facts (D0161): Question (what a human wants answerable from the model) and Alias (the words they use, mapped to the elements that answer them via plain dependency edges). */

    private import ScalarValues::*;
    private import EngineElement::*;

    // A question a human wants the model to answer. Coverage is COMPUTED
    // (`keel show knowledge question-coverage`), never recorded: whether seeding
    // finds an entity and traversal reaches an answer is re-derived per run.
    part def Question :> Element {
        attribute questionText : String;   // the question in the human's own words
    }

    // A lexicon entry: the word a human uses, mapped to the element(s) that
    // answer it by a plain `dependency from <alias> to <element>;` edge
    // (vanilla SysML v2, D0196 precedent — no marker needed for a lens).
    part def Alias :> Element {
        attribute term : String;           // the human's word, matched case-insensitively
    }
}
