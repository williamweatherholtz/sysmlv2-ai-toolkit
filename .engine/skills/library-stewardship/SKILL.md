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
   publish. decision-channel is the worked example.
5. **Publishing back carries the unit's identity — you never mint a new one** (D0272). A unit you
   IMPORTED is filed under the library's id in `installed-units.toml`, and that is its identity
   forever; export reads it from there, not from the ids this project minted. So the version you
   publish continues the unit's series rather than restarting it. Two refusals enforce this and both
   mean *stop and reconcile, never retry*: **one process name under two unit ids** (the identity
   forked — keep the id consumers installed, delete the other), and **a version lookup that misses
   on a process you demonstrably have installed** (the registries disagree in a way export will not
   guess past). Do not work around either by re-importing or by hand-editing a version: a wrong
   number here is unrecoverable, because consumers have already compared against it — theirs would
   then report "nothing behind" over content that entirely differs.

## Removal path

Delete this skill + registry + the process file. The `keel library` commands keep working; only the
discipline text leaves the catalogue.
