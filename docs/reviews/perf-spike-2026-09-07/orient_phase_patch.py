"""Measurement-only: phase-wrap each step of orient::compute so KEEL_PERF=2 attributes its 4.7s.

Every anchor must be found exactly once or the patch FAILS. Reverse with `git checkout keel-cli/src/orient.rs`.
"""
import io
import sys

p = "keel-cli/src/orient.rs"
s = io.open(p, encoding="utf-8").read()

P = "crate::perf::phase"
edits = [
    ("    let idx = crate::indexer::extract(&tracking);\n",
     "    let idx = " + P + "(\"orient:indexer\", || crate::indexer::extract(&tracking));\n"),
    ("    let sync_state = crate::sync::divergence(repo);\n",
     "    let sync_state = " + P + "(\"orient:sync1\", || crate::sync::divergence(repo));\n"),
    ("    let sha_valid = valid_commits(repo, &shas);\n",
     "    let sha_valid = " + P + "(\"orient:valid_commits\", || valid_commits(repo, &shas));\n"),
    ("    let Narrowing { superseded, blocked, claimed_by_others, compute_failures } = narrowing_filters(repo);\n",
     "    let Narrowing { superseded, blocked, claimed_by_others, compute_failures } = " + P + "(\"orient:narrowing\", || narrowing_filters(repo));\n"),
    ("    let dod_files = build_dod_files(repo);\n",
     "    let dod_files = " + P + "(\"orient:dod_files\", || build_dod_files(repo));\n"),
    ("    for (name, reason) in criterion_suspects(repo, &tasks, &ordering_only, &done_map, &verified_at, &dod_files) {\n",
     "    for (name, reason) in " + P + "(\"orient:criterion_suspects\", || criterion_suspects(repo, &tasks, &ordering_only, &done_map, &verified_at, &dod_files)) {\n"),
    ("    propagate_transitive_suspect(&tasks, &ordering_only, &done_map, &mut suspect_set, &mut suspect_reasons);\n",
     "    " + P + "(\"orient:propagate\", || propagate_transitive_suspect(&tasks, &ordering_only, &done_map, &mut suspect_set, &mut suspect_reasons));\n"),
    ("    apply_deliverable_suspicion(repo, &done_map, &verified_at, &mut suspect_set, &mut suspect_reasons);\n",
     "    " + P + "(\"orient:deliverable\", || apply_deliverable_suspicion(repo, &done_map, &verified_at, &mut suspect_set, &mut suspect_reasons));\n"),
    ("    let open_issues = crate::view::open_issue_names(repo, &done_set).unwrap_or_default();\n",
     "    let open_issues = " + P + "(\"orient:open_issues\", || crate::view::open_issue_names(repo, &done_set).unwrap_or_default());\n"),
    ("        in_progress_sprints: in_progress_sprints(repo),\n",
     "        in_progress_sprints: " + P + "(\"orient:in_progress\", || in_progress_sprints(repo)),\n"),
    ("        burndown: crate::view::burndown_summary_json(repo).unwrap_or_default(),\n",
     "        burndown: " + P + "(\"orient:burndown\", || crate::view::burndown_summary_json(repo).unwrap_or_default()),\n"),
    ("        inactive_processes: crate::activation::Activation::load(repo).inactive_processes(),\n",
     "        inactive_processes: " + P + "(\"orient:activation\", || crate::activation::Activation::load(repo).inactive_processes()),\n"),
    ("        sync: crate::sync::divergence(repo).to_json(),\n",
     "        sync: " + P + "(\"orient:sync2\", || crate::sync::divergence(repo).to_json()),\n"),
    ("        pending_acceptances: crate::view::pending_acceptances(repo)\n",
     "        pending_acceptances: " + P + "(\"orient:pending\", || crate::view::pending_acceptances(repo))\n"),
]
for old, new in edits:
    if s.count(old) != 1:
        sys.exit("anchor not found exactly once - refusing to patch: " + old.strip()[:70])
    s = s.replace(old, new, 1)
io.open(p, "w", encoding="utf-8", newline="").write(s)
print("patched %d anchors" % len(edits))
