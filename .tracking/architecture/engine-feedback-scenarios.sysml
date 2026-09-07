// STPA on the FEEDBACK half of this project's own control structure (st-instruments, D0361).
//
// WHY THIS FILE. Step 2 for keel computes its feedback channels from `CliCommand` facts (D0271:
// effect=reads -> feedback) and CI workflow statuses - 77 channels, all derived. Measured 2026-09-06:
// of the twelve instruments declared in `.engine/contracts/instruments.toml`, NOT ONE appears in that
// computed structure. The analysis that exists to find exactly this class could not see the channels
// that produced six defects in one day. So the scenarios below are step 4 over the feedback paths,
// authored because a loss scenario is a judgment and cannot be computed.
//
// EVERY SCENARIO IS A CAUSAL CHAIN ending in a hazard, and every feedback-based one says WHY the
// feedback is inadequate - the step-4 gate's requirement, and the thing a factor list never does.
// Eight of the nine are EVIDENCED by an incident this project has already had; the ninth is the
// credible one nobody has hit yet, named so it is not discovered the same way.
package EngineFeedbackScenarios {
    private import EngineElement::*;
    private import EngineSafety::*;

    // ---- One new hazard: no existing one covers a measure that MISREPORTS -------------------
    // EHZ4 is completion computed from absent/invalid evidence; EHZ8 is a STALE computed layer served
    // as current. Neither covers a measure that is fresh, present, and WRONG - or one whose verdict
    // cannot be reproduced. That state is what the six incidents share.
    part ehz10 : Hazard {
        :>> id = "b7c1f0a2-4d38-4e19-9f52-1c8a3e60d4b7";
        :>> title = "EHZ10 - a measure reports a value the tree does not support, and it is consumed as measured";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> statement = "Worst case EL3: a number, verdict or figure this project publishes differs from what the tree would support - wrong, incomplete, ambiguous or irreproducible - and nothing distinguishes it from a sound one at the point of use. A wrong REFUSAL announces itself and gets argued with; a wrong NUMBER gets believed, quoted and acted on. CONTROLS: the computed-facts contract (D0215), the executive-brief checker, the seeded benchmark's own reproducibility. OPEN: no instrument is in the computed control structure, none is propriety-assessed (D0361), and determinism is undeclared per instrument.";
    }

    // ---- Scenario 1: a plausible wrong value (EVIDENCED, issue389) -------------------------
    part sfWrongValuePlausible : LossScenario {
        :>> id = "5e2a8c14-9b7d-4f03-8a61-7d92c4e15f30";
        :>> title = "S-F1 - the instrument measures correctly against the wrong subject";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "process-model";
        :>> statement = "The facts script asked git about a release BY NAME in seven places. A new release shipped, so every since-the-release fact went on measuring against a superseded tag - the arithmetic correct, the subject wrong - and each carried a `how` string naming a real command anyone could run, which is what made it credible. The agent's process model of 'what this number is about' was maintained by a field the instrument itself supplied, so no feedback existed that could contradict it. The human would have read commits-since and days-since a release two versions old [EHZ10 -> EL3]. WHY THE FEEDBACK IS INADEQUATE: the instrument reports its VALUE and its METHOD, and nothing reports its SUBJECT, so a subject error is indistinguishable from a correct reading. MITIGATION: the subject is derived and emitted as its own fact, and the script refuses to run while a version literal appears in its source (shipped); the general form is that an instrument must emit what it measured against, not only what it measured.";
    }

    // ---- Scenario 2: the figure drops what the data had (EVIDENCED, issue390) ---------------
    part sfFigureDropsContent : LossScenario {
        :>> id = "2c9f47b8-1e6a-4d55-b0f3-8a41e7c9d206";
        :>> title = "S-F2 - correct values, and the rendering hides half of them";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "feedback";
        :>> statement = "The chart builder placed legend entries at even intervals, so a long label overprinted its neighbour and its own count; the exhibit builder emitted a note as one SVG text element, and SVG does not wrap, so the canvas edge cut it mid-word. In both cases the measured values were right and the human's copy was wrong, with no error, no ellipsis and no cue - a truncated sentence reads as a finished one. The reader forms a belief from the picture, not from the data behind it [EHZ10 -> EL3]. WHY THE FEEDBACK IS INADEQUATE: the guarantee 'a figure built from data cannot drift' covers the numbers and stops at the LAYOUT, and nothing in the pipeline measures whether the rendered output fits its canvas. MITIGATION: the legend lays out by measured width and wraps, notes break on word boundaries (both shipped); the open half is that nothing CHECKS fit - the naive check gave five false positives on right-anchored labels, so a real one measures in a browser (issue390).";
    }

    // ---- Scenario 3: the diagnostic clipped by position (EVIDENCED, issue384) ---------------
    part sfDiagnosticClipped : LossScenario {
        :>> id = "8d3b61e5-7a04-4c92-95fe-2b18d7a3c6f4";
        :>> title = "S-F3 - the report shows the preamble and hides the finding";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "feedback";
        :>> statement = "A failed engine update printed the first forty lines of the gate run it reverted on. A gate prints one PASS per check, so forty lines is forty passes, and the single violation that caused the revert sat below the cut - in the printed report AND in the stored attempt record, which kept the same clipped text. The operator's process model of 'why it reverted' was maintained by a window chosen by POSITION rather than by relevance, so the cause was structurally unreachable [EHZ10 -> EL3]. WHY THE FEEDBACK IS INADEQUATE: a diagnostic truncated by position is uninformative exactly when it matters, because failures come last. MITIGATION: select the lines that name a failure, cap with a remainder count, fall back to the tail (shipped); the general form is that a clipped diagnostic must say it clipped and keep the relevant end.";
    }

    // ---- Scenario 4: two meanings in one field (EVIDENCED, issue386) ------------------------
    part sfVerdictKindCollapsed : LossScenario {
        :>> id = "4a76d2c9-3f18-4b60-8e25-9c07f1a5b3e8";
        :>> title = "S-F4 - could-not-measure is reported in the field for measured-and-failed";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "feedback";
        :>> statement = "The suite runner shells out to a build that must relink the binary running it. On this host the running image cannot be replaced, so no test ever compiled and the command printed `fail - 0 passed, 0 failed`, writing that verdict into a receipt keyed to the deliverable's fingerprint. A reader - or a later gate - cannot distinguish 'the code is broken' from 'the measurement could not start', because one field carries both, and the only evidence of the real cause was a line inside a log the command names but does not read back. It fired twice in one hour, the second time on a different holder of the file [EHZ10 -> EL3]. WHY THE FEEDBACK IS INADEQUATE: the verdict vocabulary has pass and fail and no INCONCLUSIVE, so an instrument that cannot run must lie in one direction or the other. MITIGATION: an instrument that could not run reports that distinctly and writes NO receipt, since a receipt is a claim that a measurement happened (issue386, open).";
    }

    // ---- Scenario 5: a stochastic instrument read as deterministic (EVIDENCED, issue392) ----
    part sfStochasticReadAsFixed : LossScenario {
        :>> id = "6b18f3a7-2d49-4e01-97c3-5f8a2e64d19b";
        :>> title = "S-F5 - one sample from a stochastic instrument is published as a measurement";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "process-model";
        :>> statement = "The routing probe runs a real model, so its verdict per case is drawn rather than computed: on identical prompts minutes apart, two of four cases changed verdict - one from a wrong route to a correct one, one from routed to silent. A single-sample sweep over forty-seven skills would have produced a coverage number indistinguishable in form from the seeded benchmark's, and it would have moved on re-run; worse, a skill scored not-routing on one sample invites rewriting a description that was never broken, after which the rewrite appears to have fixed it [EHZ10 -> EL3]. WHY THE FEEDBACK IS INADEQUATE: nothing on an instrument declares whether it is a function of the tree, seeded, stochastic or externally answered, so one run of each reads the same. MITIGATION: `determinism` is now declared per instrument (shipped, instruments.toml); a stochastic instrument's published number must carry sample count and disagreement, and the shape to buy is issue392's open question.";
    }

    // ---- Scenario 6: the measure counts the wrong thing (EVIDENCED, issue391) ---------------
    part sfConstructInvalid : LossScenario {
        :>> id = "9f42c8b1-6e37-4a25-b8d0-3c71e05f4a26";
        :>> title = "S-F6 - the instrument is sound and measures something other than the claim";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "process-model";
        :>> statement = "The retrieval bar counts a hit only when a NAMED record is among the rows shown. On two of the three cases it currently fails, the payload's first row is an element that answers the question - a requirement whose title states the rule, where the named Decision sits two hops behind it - and on another the named Decision uses the query's key word once in its whole file while records that do discuss the subject come back. The number is reproducible and the mechanism is sound; the CONSTRUCT is contested, so effort is spent moving a mechanism toward a bar that may be measuring the wrong thing [EHZ10 -> EL3]. WHY THE FEEDBACK IS INADEQUATE: the definition of a hit is embedded in the harness rather than stated as a decision, so it cannot be argued with - and an agent that loosens the definition of the test it is failing has destroyed the measure. MITIGATION: the definition goes to the human with both options priced, and the criterion states whichever is chosen (issue391, open).";
    }

    // ---- Scenario 7: the meta-scenario, evidenced today ------------------------------------
    part sfInstrumentOutsideStructure : LossScenario {
        :>> id = "1d59a7e4-8c02-4f36-a91b-7e34b6d2c805";
        :>> title = "S-F7 - the channel that misreports is outside the analysis that would find it";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "process-model";
        :>> statement = "This project's feedback channels are DERIVED from CliCommand facts and CI statuses, which is why 77 of them exist without anyone authoring one. Twelve instruments live outside the CLI, and none appears in the computed structure - so the STPA self-analysis walked a control structure with those channels absent, the currency guard could not warn about an action it cannot see, and the propriety panel's forty findings all landed on things that BLOCK rather than things that MEASURE. Six defects in one day occurred in channels no analysis reached [EHZ10 -> EL4 -> EL3]. WHY THE FEEDBACK IS INADEQUATE: a derived inventory is complete only over its source, and its completeness LOOKS total precisely because it was computed. MITIGATION: the out-of-band instruments are declared (shipped, instruments.toml) and a guard holds the declaration against the tree in both directions (shipped, guard 65); folding them into the computed structure so stpa-self can reach them is the next step.";
    }

    // ---- Scenario 8: a control's clean pass credited as a catch (EVIDENCED, D0360) ----------
    part sfCleanPassCreditedAsCatch : LossScenario {
        :>> id = "3e07b4d6-5a91-42c8-86f4-0b27a9e13d5c";
        :>> title = "S-F8 - a control is credited for passing on a tree that has no defect in it";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "feedback";
        :>> statement = "A control's daily evidence is that it passes on a clean tree, which is evidence about the tree and not about the control. A self-check written to stop a version literal reaching a facts script PASSED while scanning only the source above itself, excluding the whole section it existed to police; a deliberate probe found it, and nothing else would have. A census then showed the general shape: of 64 guards, 16 were named in a test asserting a failure and 27 in no test body at all [EHZ5 -> EL4]. WHY THE FEEDBACK IS INADEQUATE: DECLARED and ARMED are computed and PROVEN is not, so a blind control is indistinguishable from a working one by every surface the project has. MITIGATION: proof state becomes a third computed state beside declared and armed (D0360, chartered); the guard that exposed this now carries the demonstrated catch it should have shipped with.";
    }

    // ---- Scenario 9: NOT YET OBSERVED, and the one to name before it happens ----------------
    part sfMeasuredTreeIsNotTheClaimedTree : LossScenario {
        :>> id = "7c81e35a-4b60-4d29-9f18-2a53c7b09e14";
        :>> title = "S-F9 - the instrument measures the working tree while the report names a commit";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5";
        :>> causalFactor = "process-model";
        :>> statement = "Every instrument here reads FILES ON DISK, and the reports built from them cite a SHA - the standing brief's provenance strip names the commit it was computed against. Nothing enforces that the disk and that commit agree, so an uncommitted edit is attributed to a commit that does not contain it, and a reader who re-runs against that commit gets a different answer and concludes the instrument is broken rather than the attribution. This has not yet caused an incident and would be indistinguishable from one of the others when it does [EHZ10 -> EL3]. WHY THE FEEDBACK IS INADEQUATE: the SHA is authored INTO the report by the same process that read the dirty tree, so the citation and the reading share a single point of failure and cannot contradict each other. MITIGATION: an instrument reports the tree state it read - the SHA plus whether the tree was dirty - and a report that cites a commit while the tree was dirty says so; cheap, and this is the recommendation to implement next.";
    }

    // ---- Trace: every scenario to its hazard, the new hazard to its losses ------------------
    #DerivedFrom dependency from sfWrongValuePlausible to ehz10;
    #DerivedFrom dependency from sfFigureDropsContent to ehz10;
    #DerivedFrom dependency from sfDiagnosticClipped to ehz10;
    #DerivedFrom dependency from sfVerdictKindCollapsed to ehz10;
    #DerivedFrom dependency from sfStochasticReadAsFixed to ehz10;
    #DerivedFrom dependency from sfConstructInvalid to ehz10;
    #DerivedFrom dependency from sfInstrumentOutsideStructure to ehz10;
    #DerivedFrom dependency from sfCleanPassCreditedAsCatch to ehz5;
    #DerivedFrom dependency from sfMeasuredTreeIsNotTheClaimedTree to ehz10;
    #DerivedFrom dependency from ehz10 to el3;
}
