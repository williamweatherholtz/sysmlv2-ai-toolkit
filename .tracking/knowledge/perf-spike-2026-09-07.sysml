// KNOWLEDGE RECORD (D0196) - the performance spike: where keel's time goes, measured, and the four repetitions D0367 removes.
package KnowledgePerfSpike20260907 {
    private import EngineElement::*;
    private import EngineVerification::*;

    verification kPerfSpike : Test {
        :>> id = "f0de4a9c-1715-415f-b6aa-5624e1c9ff78";
        :>> title = "Measured: the corpus parses in 275 ms once per process; keel guard takes 25.5 s and orient 5.1 s from serial guards, per-guard corpus reopens, 56 untimed git spawns and 40 model clones";
        :>> createdAt = "2026-09-07"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::analyze;
        :>> procedureText = "REPLICATION (D0196). HOST: Windows 11, 20 cores, Defender on-access scanning with no exclusion for the repository; release binary from 018315a; warm cache unless stated. METHOD: KEEL_PERF=2 on each command with a shell timer around it, plus three measurement-only patches wrapping run_all's run_one call, the fourteen steps of orient::compute and the Model cache clone in crate::perf::phase (docs/reviews/perf-spike-2026-09-07/*.py; each fails when its anchor is not found exactly once; all reverted before commit). NUMBERS: validate 0.46 s; gate --fast 0.86 s; orient 5.1 s (Model::build x7, 6 cached; parse 281 ms; git x23); guard 25.5 s (Model::build x41, 40 cached; parse 273 ms; git x56 reported as 100 ms because only govern.rs times spawns - issue410). Per guard in-process: priority-inversion 6825 ms, decision-scaffolding 1842, attribute-vocabulary 1376, untrusted-taint 1281, sequence-multiplicity 1053, edge-endpoints 1004, then a long tail of 0.2-0.9 s; model:cache_clone 1573 ms over 40 hits. Per orient step: criterion_suspects 1797 ms, deliverable 955, narrowing 510, activation 340, in_progress 277, burndown 269, indexer 224, the rest under 200. Cold read of the 11.6 MB corpus by Python 51 s, warm 0.4 s; git rev-parse HEAD 79 ms per spawn. FINDING: the computing is one percent of the run; the rest is repetition - serial execution on 20 cores, each text-scanning guard reopening the corpus (39 collect_sysml and 61 read_to_string sites in guards.rs), git spawns whose answers are facts about immutable commits, and the whole run paid at every turn boundary on an unchanged tree. WHAT IT PRODUCED: D0367 (four repetitions removed in rank, two conditional items, three approaches declined with numbers, the host lever left to the human), issue409, issue410, and six backlog items. Full tables: docs/reviews/perf-spike-2026-09-07.md.";
    }
    part kPerfSpikeR1 : TestResult {
        :>> id = "3758447d-ae41-437a-8acd-d94facab3bc0";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "59c43368d735a299c1a2556d72390ec9a07c496f";
        :>> judgedAt = "2026-09-07";
        :>> judgedBy = "claudeOpus5";
        :>> notes = "Every number read from a KEEL_PERF line or a shell timer on this host; one anomaly (parse 18306 ms in a single run) did not reproduce and is recorded as an attribution to a cold Defender scan cache, not as a finding.";
    }
    dependency from kPerfSpike to d0196;
    dependency from kPerfSpike to d0367;
    dependency from kPerfSpike to issue409;
    dependency from kPerfSpike to issue410;
    dependency from kPerfSpike to dcGuardsRunInParallelAndTimed;
    dependency from kPerfSpike to dcGitFactsAreContentAddressed;
    dependency from kPerfSpike to dcOneCorpusPerProcess;
    dependency from kPerfSpike to dcGateAnswersFromItsReceipt;
    dependency from kPerfSpike to dcGitReadsStayInProcess;
    dependency from kPerfSpike to dcHookDaemonOnlyIfTheReceiptIsNotEnough;
}
