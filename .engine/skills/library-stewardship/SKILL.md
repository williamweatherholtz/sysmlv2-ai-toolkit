# library-stewardship — publish deliberately, consume knowingly

Deploys `.engine/processes/library-stewardship.sysml` (D0259). The mechanics (`keel library`,
`keel process publish/import --from-library`) are probe-proven; this skill is the discipline.

## The four rules

1. **Publish from landed trees only.** The library must never hold content no project's gate has
   seen. `keel process publish <name>`; a "nothing to publish" answer is information, never a step
   to force. Push deliberately — the push is the fleet-visible act.
2. **Sync any time; READ what it says.** Availability is never activation (held by test), so sync
   is always safe — but a STALENESS statement means you are deciding against a dated cache, and a
   DIVERGENCE refusal means an unsanctioned write: investigate or discard, never merge.
3. **A project updates explicitly.** `import --from-library <name> --update`, gate, commit under
   the keystone. A merge conflict is a real disagreement — resolve in the project, and publish the
   adaptation back if it is generally right.
4. **Reusable skill → UNIT, before folklore.** No process definition = no catalogue = travels
   nowhere (issue245). Process file + engine-side skill + registry + extras (repo files only), then
   publish. exec-summary is the worked example.

## Removal path

Delete this skill + registry + the process file. The `keel library` commands keep working; only the
discipline text leaves the catalogue.
