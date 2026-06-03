package Decisions {
    import Conventions::*;

    part d0001 : Decision {
        :>> title = "Text files are the sole source of truth";
        :>> status = "accepted";
        :>> date = "2026-06-03";

        :>> context = "Two patterns were considered: (a) .sysml text "
                    + "files in the repo as source of truth, with the "
                    + "graph DB as a derived index; (b) graph DB as "
                    + "source of truth, with text files as exports. "
                    + "Earlier critique flagged 'racing the API' as a "
                    + "failure mode when state has two homes.";

        :>> decision = "Text files are truth. The graph DB is rebuilt "
                    + "from files on every change. Writes go: API -> "
                    + "AST mutation -> text emission -> file write -> "
                    + "git commit -> re-index. The index can be "
                    + "deleted at any time without data loss.";

        :>> consequences = "Pros: git-native review, no sync tax, "
                    + "editor-friendly, single source of truth. "
                    + "Cons: round-tripping text without destroying "
                    + "formatting is non-trivial and becomes the "
                    + "hardest engineering problem in the writer "
                    + "component.";

        :>> supersededBy = "";
    }
}
