// Declared Aliases (D0161 part 2): the words a human uses, mapped to the elements
// that answer them. Plain `dependency` edges (vanilla SysML v2, D0196 precedent).
// Human-owned; delete this file and `keel why` simply has fewer seeds.
package KnowledgeLexicon {
    private import EngineElement::*;
    private import EngineKnowledge::*;

    part aFinished : Alias {
        :>> id = "b68a708b-6b22-4a32-9735-938194ed3bb4";
        :>> title = "finished / done / complete";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> term = "finished";
    }
    dependency from aFinished to srK05ComputedCompletion;

    part aApprove : Alias {
        :>> id = "2d8e09da-45f2-4eb5-bb4f-774461e1c75a";
        :>> title = "approve / sign / accept";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> term = "approve";
    }
    dependency from aApprove to srK06aDelegatedRecording;

    part aLaptop : Alias {
        :>> id = "6b8eead7-bedb-4cce-aa79-6789a3f56e7f";
        :>> title = "laptop / machine / clone";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> term = "laptop";
    }
    dependency from aLaptop to srK15TreeDerivable;

    part aBuildServer : Alias {
        :>> id = "e79f86c1-55c8-4de4-af4b-ed0408cb0fd5";
        :>> title = "build server / CI / pipeline";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> term = "build server";
    }
    dependency from aBuildServer to srK15TreeDerivable;
}
