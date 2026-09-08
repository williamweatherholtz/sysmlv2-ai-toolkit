# Self-hosted trackers spike — Linear, Jira and the self-hosted field against keel (2026-09-07)

**Question (the human's words):** *"contrast what we're doing with his linear and Jira. best practices for
those with AI agents? is linear self hostable?"* then *"I do like the self hosting surface. how really
comparable are the competing self hosted options?"* then *"ok. save this as a spike for now."*

**Answer:** Linear cannot be self-hosted; Jira Data Center is on a published clock (read-only 28 Mar 2029) and
its agents are cloud-only. The self-hosted trackers are comparable to each other and to Linear; none is
comparable to keel on the axis that matters here, because every one stores status as a field someone sets.
The self-hosting surface transfers; the computed-truth bet does not. Decision: D0374. Nothing is adopted.

Every external fact below was checked on 2026-09-07 against the sources listed at the end, not recalled. The
two claims marked UNVERIFIED were not resolvable from public documentation and are stated as such.

## Linear and Jira

**Linear: no.** No on-prem, no community edition, no enterprise self-hosted tier; all workspace data sits on
Linear's US servers. The near-equivalents for a hard self-hosting requirement are Plane, Huly, Taiga and GitLab
issues.

**Jira: yes, on a clock.** Atlassian's Data Center end-of-life: sales to new customers ended 30 Mar 2026,
existing customers cannot buy new licences or expansions after 30 Mar 2028, licences expire read-only on
28 Mar 2029. Data Center is in feature freeze. Rovo agents need Cloud; Data Center gets connectors that sync
data *to* the cloud. "Self-hosted Jira with AI agents" is not on the table.

## The contrast

The axis is not features. It is **where truth lives and who computes state**.

| | Jira / Linear | keel |
|---|---|---|
| Truth | rows in a hosted database | `.sysml` text in git |
| Status | a stored field someone sets | computed from typed edges and test results |
| "Done" | a human dragged a card | a `verify`-linked test passed against a resolvable SHA |
| Priority | a stored enum, set by opinion | declaration order, re-ranked against computed signals (D0052, D0311) |
| History | audit log in the vendor's DB | `git log`; rewriting it orphans evidence by design (D0129) |
| Agent identity | app actor in the vendor's model | actor + kind; writes refuse without provenance |
| Human sign-off | a status transition | an attestation that must quote them and name the record (D0289) |
| Exit cost | export API, lossy | already text you own |

The sharp difference: Jira and Linear will let an agent mark work done; keel structurally cannot. In the
session this was written, a passing unit test and a passing live reproduction both stayed green while three
real tests were failing, and what stopped "done" from being recorded was that the gate would not produce a
number. In Jira that is a drag to the Done column.

The cost of the bet is equally real: keel has one serious user, a terminal UI, no ecosystem, no mobile, no
notifications anyone else reads, and authoring friction CLAUDE.md names as the #1 risk (D0054). Linear is
pleasant, and pleasantness is why people record things. The table is not keel winning.

## Agent practice on Linear

Linear built a purpose-made agent surface with specific rules:

- Answer within 10 seconds of the `created` webhook with a `thought` activity or be shown unresponsive;
  sessions go stale after 30 minutes without activity (recoverable).
- Use the typed activities (`thought`, `response`, `elicitation`, `error`), not comments; activities are frozen
  snapshots and are what reconstructs history.
- Agents are distinct workspace members with `actor=app` auth, so actions attribute to the app, not a person.
  This is the one thing to get right: it stops an agent's work from silently reading as a human's.
- Move the issue to the first "started" state and set the agent as delegate; if an automation delegated it,
  leave it in triage for a human.
- Subscribe to permission-change webhooks and honour `teamAccessChanged`.
- Known gap: sessions are isolated; context is lost on hand-off.

## Agent practice on Jira

- Rovo agents sit on the Teamwork Graph and reach outward via connectors and MCP. Give an agent a narrow role
  and scoped knowledge, not the whole graph.
- Requires Cloud Standard/Premium/Enterprise. Data Center gets sync connectors only.
- The pattern that works: agent drafts in Confluence, decomposes into epics/issues, supports execution.
  Meeting transcript to issues is the highest-value low-risk starter.

## What neither gives, and what cost real time in the same session

1. **Machine-checkable agent evidence.** Linear attributes *who* acted; it does not check whether the claim is
   true. keel's answer is the `// RAN:` receipt and `audit-ci-runs` (D0232, D0323). A suite that printed
   `0 passed, 0 failed` over a file lock nearly became a recorded pass three times that day (issue386).
2. **An instrument that cannot run must fail loudly, not return a plausible number.** Neither tool has the
   concept; keel's is D0361/D0363 and the `instruments-declared` guard.
3. **An agent never records a human's approval.** Linear's `actor=app` is the right primitive. keel adds that
   a sign-off quotes the human and names what they approved (D0289, D0308); a schema change sat built, green and
   unsigned that day because the control refused a bare "proceed", correctly.

## The self-hosted field

| | License / self-host | Agent surface | The catch |
|---|---|---|---|
| **GitLab** | MIT-ish core; self-managed first-class for a decade | Duo + MCP on Self-Managed since 18.2, as client and server | Duo is a paid add-on; heavy install; issues are a side-feature of a DevOps platform |
| **Plane** | AGPL-3.0 Community; Docker/K8s; air-gapped supported | Native MCP server, agent framework with `@mention`, Agent Run lifecycle | See the caveat below |
| **OpenProject** | GPLv3 core; mature; self-host normal | MCP server in v17.2 (Mar 2026), Enterprise add-on only | Classic PM/Gantt orientation; community MCP is third-party |
| **Huly** | Self-hostable all-in-one (issues, chat, docs, video) | No first-class agent API found | CockroachDB + Redpanda: materially heavier to run |
| **Taiga / Redmine / Vikunja** | Genuinely open, easy to run | None to speak of | Taiga is dev-only agile; Redmine is 2006; Vikunja is tasks, not engineering work |

### The Plane caveat

Plane is the closest thing to self-hosted Linear and markets hardest on agents. Before betting on it:

1. Community and Commercial are different codebases, by Plane's own docs. Community is "at par with the Free
   tier of the Cloud edition", not with Cloud. "Plane has an agent framework" and "the AGPL edition you
   self-host has an agent framework" are different claims and the documentation does not connect them.
   UNVERIFIED either way.
2. The Free tier excludes integrations and marketplace access, SAML/OIDC/LDAP, dashboards, templates, time
   tracking. If the MCP server sits under integrations, the self-hosted free path may not include it. The
   pricing page does not mention MCP or agents on any tier. UNVERIFIED.
3. Plane's own materials assert the Community user limit four different ways (none / 12 per workspace /
   12 seats / no hard limit), an open issue since 16 May 2026 with no maintainer response (#9086).

The third point weighs heaviest, not because seat limits matter, but because adopting a tool whose own licensing
terms are asserted four ways and left unresolved for months is adopting the failure mode keel exists to avoid.

### Ranking within the field, on the self-hosting surface alone

- **Works today, with agents, self-hosted, from a vendor not retracting self-hosting:** GitLab. The only one
  where self-managed plus AI agents is a supported, documented, shipping combination. Cost: a DevOps platform
  whose issue tracker is not why people use it. Which GitLab tier gates Duo on self-managed: UNVERIFIED.
- **The Linear feel:** Plane, with the caveat above, and the agent path verified on the AGPL build first.
- **Everything else** is a tracker you host, not an agent platform.

## What this does not change

None of these changes keel's position. Work put in Plane or GitLab has "done" as a field an agent can set. The
three near-misses of the session (a suite lying over a file lock, a write filing work where nothing looks for
it, a benchmark that could not see the mechanism) would have been caught by none of them, because none has an
opinion about whether a claim is true.

The combination worth trying, if any: one of these as the surface humans and outside collaborators touch, git
as the thing that decides done. GitLab makes that unusually clean because the repo and the tracker are one
system. It is not adopted by this spike; adoption is a separate Decision with its own trial.

## Sources (checked 2026-09-07)

Linear agent best practices — https://linear.app/developers/agent-best-practices ·
Linear agents — https://linear.app/developers/agents ·
Atlassian Data Center end of life — https://www.atlassian.com/licensing/data-center-end-of-life ·
Atlassian Rovo — https://www.atlassian.com/software/rovo · Rovo in Jira — https://www.atlassian.com/software/jira/ai ·
Plane editions — https://developers.plane.so/self-hosting/editions-and-versions ·
Plane MCP server — https://developers.plane.so/dev-tools/mcp-server · Plane pricing — https://plane.so/pricing ·
Plane Community seat-limit contradiction — https://github.com/makeplane/plane/issues/9086 ·
OpenProject MCP — https://www.openproject.org/docs/system-admin-guide/integrations/mcp-server/ ·
GitLab Duo MCP — https://docs.gitlab.com/user/gitlab_duo/model_context_protocol/mcp_clients/ ·
GitLab Duo agent platform — https://about.gitlab.com/blog/duo-agent-platform-with-mcp/ ·
Plane vs Huly vs Taiga — https://www.pistack.xyz/posts/plane-vs-huly-vs-taiga-self-hosted-project-management-guide-2026/
