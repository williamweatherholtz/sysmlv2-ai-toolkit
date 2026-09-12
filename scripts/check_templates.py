#!/usr/bin/env python3
# ci-probe: --self-test
"""HTML template + instance contract check (D0237). ONE implementation, two callers:
the keel pre-commit gate runs it, and tests/exec_summary imports it — so the check the
gate enforces and the check the suite asserts can never drift apart (the defect class
recorded as issue003).

What it proves, per family under templates/<family>/:
  * exactly one SOURCE TEMPLATE (the file carrying REPLACE marks); the rest are INSTANCES
  * FIELD INVENTORY: every data-d / data-digest field key in the template exists in each
    instance — a dropped field can no longer ship silently
  * NO UNREPLACED PLACEHOLDERS: no instance field may still read as the template's own
    placeholder text (differential, not a hand-maintained blocklist), and no instance may
    carry REPLACE marks or a placeholder date
  * STRUCTURE: copy control top and bottom; tabs paired with panels; one decision per tab
    (>=2 radios sharing one name); nothing pre-selected; every choice steel-manned with a
    cost; the reasoning chain present and ordered (criterion named before the comparison)
  * PROSE BUDGETS: the executive-summary word caps
  * PORTABILITY: viewport meta, both themes, self-contained but for Google Fonts, external
    links open a new tab (sandboxed-iframe hosts block same-frame navigation)

Usage:  python scripts/check_templates.py [path ...]      (default: templates/)
        python scripts/check_templates.py --brief PAGE ...  (the executive-brief contract, D0355)
        python scripts/check_templates.py --budgets         (every skill Budget table row names its check, issue413)
        python scripts/check_templates.py --probe           (the --budgets known-positive / known-negative pair, D0388)
        python scripts/check_templates.py --self-test       (every fixture case, brief and budgets; CI runs this)
Exit 0 = clean, 1 = violations (each named with its file), 2 = usage error.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# word caps for the reasoning fields; an executive summary that grows paragraphs stops being one
BUDGETS = {"stake": 20, "steel": 18, "cost": 14, "driver": 20, "why": 32, "flip": 20, "conf": 16}
# the reasoning chain, in the order that makes a recommendation arguable
CHAIN = ["driver", "why", "flip", "conf"]
# fields whose TEXT is authored per report; matching the template's wording means unreplaced.
# Container and boilerplate fields (choices, proscons, note, provenance) are excluded: their
# shared text is the format, not a placeholder.
PROSE_KEYS = {f"data-d:{k}" for k in ("steel", "cost", "stake", "driver", "why", "flip", "conf")}
PROSE_KEYS |= {"data-digest:title", "data-digest:subtitle"}
PLACEHOLDER_DATE = re.compile(r"YYYY-MM-DD")


def markup_only(raw: str) -> str:
    """CSS and JS legitimately mention the same selectors; contract checks read markup."""
    return re.sub(r"<style>[\s\S]*?</style>|<script>[\s\S]*?</script>", "", raw)


def field_text(fragment: str) -> str:
    """Visible text of a field, minus its label and any inline markup."""
    body = re.sub(r"<b>[\s\S]*?</b>", " ", fragment)
    body = re.sub(r"<[^>]+>", " ", body)
    body = re.sub(r"&[a-z]+;", " ", body)
    return " ".join(body.split())


def fields(html: str) -> dict[str, list[str]]:
    """Every data-d / data-digest field in the document: key -> list of its texts."""
    out: dict[str, list[str]] = {}
    for attr in ("data-d", "data-digest"):
        for m in re.finditer(
            r'%s="([a-zA-Z-]+)"[^>]*>([\s\S]*?)</(?:small|p|h1|span|td|div|section|table|fieldset|label)>'
            % attr, html):
            out.setdefault(f"{attr}:{m.group(1)}", []).append(field_text(m.group(2)))
    return out


def is_template(raw: str) -> bool:
    return "REPLACE" in raw


def check_file(path: Path, template_fields: dict[str, list[str]] | None) -> list[str]:
    raw = path.read_text(encoding="utf-8")
    html = markup_only(raw)
    bad: list[str] = []
    def fail(msg: str) -> None:
        # A path outside the repo is legitimate - a probe, a scratch render, a page under review -
        # and formatting its name must never crash the check (the issue009 class: a checker that
        # dies on an unexpected location cannot be used where a reader actually puts a file).
        try:
            where = path.relative_to(ROOT)
        except ValueError:
            where = path
        bad.append(f"{where}: {msg}")

    # ---- copy control top and bottom ----
    if len(re.findall(r"<button[^>]*data-copy", html)) < 2:
        fail("needs a copy control at top AND bottom")
    first_panel = html.find('role="tabpanel"')
    if first_panel != -1:
        if html.find("data-copy") > first_panel:
            fail("first copy control must precede the content")
        if html.rfind("data-copy") < html.rfind('role="tabpanel"'):
            fail("last copy control must follow the content")

    # ---- tabs paired with panels ----
    tabs = re.findall(r'role="tab"', html)
    panels = re.findall(r'role="tabpanel"', html)
    if len(tabs) < 2 or len(tabs) != len(panels):
        fail(f"tabs/panels mismatch: {len(tabs)} tabs, {len(panels)} panels (need >=2, paired)")
    if html.count('aria-selected="true"') != 1:
        fail("exactly one tab may start selected")

    # ---- per-decision structure ----
    for i, panel in enumerate(re.split(r'role="tabpanel"', html)[1:], start=1):
        for key in ('data-d="choices"', 'data-d="stake"', 'data-d="note"',
                    'data-d="proscons"') + tuple(f'data-d="{k}"' for k in CHAIN):
            if key not in panel:
                fail(f"decision {i} missing {key}")
        radios = re.findall(r'<input type="radio" name="([^"]+)"', panel)
        if len(radios) < 2 or len(set(radios)) != 1:
            fail(f"decision {i} must offer >=2 choices sharing ONE radio name")
        if " checked" in panel:
            fail(f"decision {i} pre-selects a choice (that fabricates a decision)")
        for card in re.findall(r'<label class="choice"[\s\S]*?</label>', panel):
            if 'data-d="steel"' not in card:
                fail(f"decision {i}: a choice has no strongest case (strawman)")
            if 'data-d="cost"' not in card:
                fail(f"decision {i}: a choice has no cost — every option costs something")
        order = [panel.find(f'data-d="{k}"') for k in CHAIN]
        if order != sorted(order):
            fail(f"decision {i}: reasoning chain out of order "
                 "(name the criterion before comparing against it)")

    # ---- prose budgets ----
    for key, limit in BUDGETS.items():
        for m in re.finditer(r'data-d="%s"[^>]*>([\s\S]*?)</(?:small|p)>' % key, html):
            words = len(field_text(m.group(1)).split())
            if words > limit:
                fail(f'{key}: {words} words over the {limit}-word budget')

    # ---- ADJUDICATION PROVENANCE: the source project, in the tab and in the digest ----
    # st010: "make it clear what the source project was when askign for adjudication. lots of
    # browser pages". A sign-off given against the wrong project's claims is the failure this
    # prevents, and it is invisible from inside the page - so all three carriers are checked.
    if 'data-digest="project"' not in html:
        fail('no data-digest="project" field - an adjudication page must name its source project')
    script = raw.split("<script>", 1)[1] if "<script>" in raw else ""
    # Assert the EMISSION, not the lookup. Checking only that the selector is mentioned passes a
    # page that reads the project and never puts it in the digest - which is the failure that
    # matters, since the digest is what travels.
    if 'data-digest="project"' not in script or "Source project:" not in script:
        fail("the digest builder must EMIT the project as a 'Source project:' line - a pasted "
             "digest is read by a session that cannot see which tree produced it")
    title_m = re.search(r"<title>([^<]*)</title>", raw)
    if not title_m or not title_m.group(1).strip():
        fail("no <title> - the browser tab is where a reader with many pages open reads it")
    elif template_fields is not None:
        # instance only: the template's own project field is still a REPLACE mark
        proj = fields(html).get("data-digest:project", [])
        slug = proj[0].split()[0].strip().lower() if proj and proj[0].split() else ""
        if slug and slug not in title_m.group(1).lower():
            fail('<title> does not name the project (%s) - the tab is where it is read' % slug)

    # ---- portability ----
    if not re.search(r'<meta name="viewport"[^>]*width=device-width', raw):
        fail("missing the mobile viewport meta")
    for needed in ("prefers-color-scheme: dark", ':root[data-theme="dark"]',
                   ':root:not([data-theme="light"])'):
        if needed not in raw:
            fail(f"theme tokens incomplete: {needed} absent")
    if not re.search(r"body\{[^}]*background:var\(", raw):
        fail("body needs an explicit token background (a transparent body borrows the host's)")
    for url in re.findall(r'src="(https?://[^"]+)"', raw):
        fail(f"external loaded resource: {url}")
    for url in re.findall(r'<link[^>]*href="(https?://[^"]+)"', raw):
        if not url.startswith("https://fonts.googleapis.com"):
            fail(f"external stylesheet: {url}")
    for a in re.finditer(r'<a\s[^>]*href="https?://[^"]+"[^>]*>', raw):
        if 'target="_blank"' not in a.group(0):
            fail(f"external link needs target=_blank: {a.group(0)[:60]}")
    if "<svg" in raw and "var(--" not in raw.split("<svg", 1)[1].split("</svg>")[0]:
        fail("inline SVG must take its colors from the theme tokens")

    # ---- instance-only: field inventory + unreplaced placeholders ----
    if template_fields is not None:
        inst = fields(html)
        for key in template_fields:
            if key not in inst:
                fail(f"field dropped relative to the template: {key}")
        tmpl_texts = {t.lower() for key, texts in template_fields.items() if key in PROSE_KEYS
                      for t in texts if t}
        for key, texts in inst.items():
            if key not in PROSE_KEYS:
                continue
            for t in texts:
                if t and t.lower() in tmpl_texts:
                    fail(f'{key} still carries the template placeholder: "{t[:60]}"')
        if "REPLACE" in raw:
            fail("instance still carries REPLACE marks")
        if PLACEHOLDER_DATE.search(raw):
            fail("instance still carries the placeholder date")
        if not re.search(r'data-digest="provenance">\s*\d{4}-\d{2}-\d{2}', raw):
            fail("instance footer needs a real ISO date")
    return bad



# ── the EXECUTIVE BRIEF contract (D0355) ──────────────────────────────────────────────────────────
# A brief is answer-first, self-contained and diagram-led. These are the rules a regex can hold; the
# skill names the rest as judgment rather than faking a check for them.

# Any record id in text the READER reads. Identity is allowed only inside the copy-for-AI digest.
BRIEF_ID = re.compile(r"\b(?:[Dd]0\d{3}|issue\d{3}|GH#\d+|st\d{3}|us\d{3}|dc[A-Z][A-Za-z]{4,}|sr[A-Z][A-Za-z]{4,})\b")
# A message title must be a sentence: a finite verb somewhere in it. Cheap, catches "CLI surface today".
BRIEF_VERB = re.compile(r"\b(is|are|was|were|has|have|had|does|do|did|costs?|moves?|lands?|changes?|"
                        r"breaks?|wins?|shortens?|catches?|waits?|accepts?|stops?|runs?|rewrites?|"
                        r"touches?|grows?|takes?|needs?|gets?|makes?|leaves?|sits?|says?|shows?|"
                        r"would|will|can|cannot)\b", re.I)


# The section budgets of asi-templates 2.1, as (label, capture of the section's markup, words).
BRIEF_BUDGETS = [
    ("headline", r'data-digest="title"[^>]*>([^<]+)<', 18),
    ("the ask", r'<div class="ask">([\s\S]*?)</div>', 70),
    ("provenance strip", r'data-digest="provenance"[^>]*>([\s\S]*?)</footer>', 60),
]
# The whole-page ceiling of the terse register. It is a clause of D0377 and holds only once that
# Decision is accepted: the check reads the decision file's status, so the control lands with the
# human's word and not before it (D0337). None = no ceiling.
TERSE_DECISION = ROOT / ".engine" / "decisions" / "0377-the-brief-is-terse-and-capped.sysml"
TERSE_CEILING = 450
# The encoding declaration a host reads before it parses: <meta charset=utf-8> in any quoting or case, or
# the http-equiv form. Anchored to the first 1024 bytes by the caller (the browsers' prescan window).
CHARSET_META = re.compile(r'<meta\s[^>]*(?:charset\s*=\s*["\']?\s*utf-?8|content\s*=\s*["\'][^"\']*charset=utf-?8)',
                          re.IGNORECASE)

# D0405: the reader prose carries no implementation duration. A dated fact ("since 2026-09-01") is not
# a duration; "a day each", "five days", "two sprints", "~3 h" are. Word-number or digit, unit, optional plural.
DURATION_RE = re.compile(
    r"(?<![\w-])(?:~?\d+(?:\.\d+)?|an?|one|two|three|four|five|six|seven|eight|nine|ten|half an?|"
    r"several|few)\s*(?:hours?|hrs?|h|days?|weeks?|wks?|months?|sprints?|minutes?|mins?)\b(?![\w-])",
    re.I)
# D0406: an aggregate verb applied to a counted set must come with the set's members.
AGGREGATE_RE = re.compile(
    r"\b(?:collaps\w*|fold\w*|merg\w*|consolidat\w*|renam\w*|remov\w*|retir\w*)\s+"
    r"(?:the\s+|every\s+|all\s+)?(?:remaining\s+|other\s+|more\s+)?"
    r"(?:\d+|two|three|four|five|six|seven|eight|nine|ten|\w+teen|twenty|thirty|forty|fifty)\s+"
    r"(?:more\s+)?\w+", re.I)
# D0407: the ceiling is measured per tab (frame + one panel) once that Decision is accepted; summed until then.
PER_TAB_DECISION = ROOT / ".engine" / "decisions" / "0407-briefCeilingIsMeasuredPerTab.sysml"


def brief_ceiling_per_tab() -> bool:
    try:
        return "DecisionStatus::accepted" in PER_TAB_DECISION.read_text(encoding="utf-8")
    except OSError:
        return False


def brief_page_ceiling() -> int | None:
    try:
        text = TERSE_DECISION.read_text(encoding="utf-8")
    except OSError:
        return None
    return TERSE_CEILING if "DecisionStatus::accepted" in text else None


def check_brief(path: Path, ceiling: int | None = None, raw: str | None = None,
                per_tab: bool | None = None) -> list[str]:
    """The brief contract. Returns one string per violation, each naming the file."""
    raw = path.read_text(encoding="utf-8") if raw is None else raw
    ceiling = brief_page_ceiling() if ceiling is None else ceiling
    per_tab = brief_ceiling_per_tab() if per_tab is None else per_tab
    bad: list[str] = []
    rel = path.name

    def fail(msg: str) -> None:
        bad.append(f"{rel}: {msg}")

    markup = markup_only(raw)          # style/script stripped: what the reader actually reads
    prose = re.sub(r"<[^>]+>", " ", markup)

    # 0. the page declares its encoding where a host that sends none will look (issue401): a <meta charset>
    # inside the first 1024 bytes, which is the prescan window every browser reads before guessing. The
    # standing brief once carried its typographic characters as raw UTF-8 with no declaration; the publisher's
    # wrapper supplied one, so the artifact looked right while the file rendered mojibake on any host that
    # guessed. The characters stay - a declaration is the fix, entities are a workaround for its absence.
    head_bytes = raw.encode("utf-8")[:1024].decode("utf-8", errors="ignore")
    if not CHARSET_META.search(head_bytes):
        fail("no charset declaration in the first 1024 bytes - a host that sends no charset guesses, and "
             "the typographic characters render as mojibake; declare <meta charset=\"utf-8\"> first in the page")

    # 1. no references in the reader's text
    ids = sorted(set(BRIEF_ID.findall(prose)))
    if ids:
        fail(f"record id(s) in reader-facing text - state the fact, not the pointer: {', '.join(ids[:6])}")

    # 2. answer first: a title, and it must be the recommendation (a verb), not a topic
    title = re.search(r'data-digest="title"[^>]*>([^<]+)<', markup)
    if not title:
        fail('no data-digest="title" - the brief has no headline recommendation')
    elif not BRIEF_VERB.search(title.group(1)):
        fail(f'the headline is a topic, not a recommendation (no verb): "{title.group(1)[:60]}"')

    # 3. the ask, with response options that declare what they decide
    if 'class="ask"' not in markup:
        fail("no ask block - a brief states what is being asked before it argues")
    opt_groups = re.findall(r'<div class="opts"([^>]*)>([\s\S]*?)</div>', markup)
    if not opt_groups:
        fail("no response options - a brief the reader cannot answer is a report")
    for attrs, body in opt_groups:
        names = re.findall(r'name="([^"]+)"', body)
        if len(names) < 2 or len(set(names)) != 1:
            fail("an ask offers fewer than 2 options, or its options do not share one name")
        if "data-records" not in attrs:
            fail("an ask does not declare what it decides (data-records) - a pasted answer cannot "
                 "then be attached to any record, which is how one answer was lost")
        if " checked" in body:
            fail("an option is pre-selected, which fabricates a decision the reader never made")

    # 3b. one tab per ask (D0404): tabs, panels and asks are one count; exactly one tab starts selected
    tabs = re.findall(r'role="tab"', markup)
    panels = re.findall(r'role="tabpanel"', markup)
    if not tabs:
        fail("no tabs - a brief renders each ask as its own tab, with the verdict and provenance outside them")
    elif not (len(tabs) == len(panels) == len(opt_groups)):
        fail(f"tabs, panels and asks disagree: {len(tabs)} tabs, {len(panels)} panels, {len(opt_groups)} asks")
    if tabs and len(re.findall(r'role="tab"[^>]*aria-selected="true"', markup)) != 1:
        fail("exactly one tab must start selected")

    # 3c. no implementation duration in anything the reader reads (D0405); a date is not a duration
    dur = DURATION_RE.search(field_text(prose))
    if dur:
        fail(f'reader prose states an implementation duration: "{dur.group(0)}" - cost is what changes and what breaks, never time')

    # 3d. an aggregate verb on a counted set names its members, and the names reconcile (D0406)
    agg = AGGREGATE_RE.search(field_text(prose))
    members = re.findall(r'<table[^>]*data-members="(\d+)"[^>]*>([\s\S]*?)</table>', markup)
    if agg and not members:
        fail(f'"{agg.group(0)}" counts a set the page never names - add a members table (data-members) listing them')
    for declared, body in members:
        named = len(re.findall(r'class="m"', body))
        if named != int(declared):
            fail(f"members table declares {declared} members and names {named} - the names must reconcile with the count")

    # 4. figures: message titles that are sentences, and labels that are not ids
    figs = re.findall(r"<figure[^>]*>([\s\S]*?)</figure>", raw)
    if len(figs) < 2:
        fail(f"{len(figs)} figure(s) - a brief carries at least the two logic exhibits per ask "
             "(today versus changed, and what lands downstream)")
    for i, fig in enumerate(figs, start=1):
        msg = re.search(r'class="msg"[^>]*>([^<]+)<', fig)
        if not msg:
            fail(f"figure {i} has no message title - an untitled figure lets the reader install "
                 "their own conclusion")
        elif not BRIEF_VERB.search(msg.group(1)):
            fail(f'figure {i} title states no claim (no verb): "{msg.group(1)[:60]}"')
        elif " and " in msg.group(1).lower() and msg.group(1).count(",") == 0:
            pass  # two clauses joined by "and" is a smell, not a defect; the skill says split it
        labels = re.findall(r"<text[^>]*>([^<]*)</text>", fig)
        fig_ids = sorted({m for lab in labels for m in BRIEF_ID.findall(lab)})
        if fig_ids:
            fail(f"figure {i} labels carry record id(s): {', '.join(fig_ids[:4])} - a label must be "
                 "a thing that happens or a thing that exists")
        if not re.search(r'role="img"', fig) or not re.search(r'aria-label="[^"]{25,}"', fig):
            fail(f"figure {i} has no role=img with a real aria-label")

    # 5. provenance instead of citations
    prov = re.search(r'data-digest="provenance"[^>]*>([\s\S]{0,1600}?)</', markup)
    if not prov:
        fail("no provenance strip - a brief with no references must say how its numbers were measured")
    elif not re.search(r"\d{4}-\d{2}-\d{2}", prov.group(1)):
        fail("the provenance strip carries no ISO date")

    # 5b. the budgets the contract's section table declares (asi-templates 2.1). Until issue413 they
    # lived in the skill alone and a 1334-word page with a 26-word headline passed clean; a budget
    # nothing holds is a reminder (D0047). The reader's prose is everything outside style, script
    # and the copy-for-AI digest, figure labels included.
    for label, pattern, limit in BRIEF_BUDGETS:
        m = re.search(pattern, markup)
        if m:
            words = len(field_text(m.group(1)).split())
            if words > limit:
                fail(f"{label}: {words} words over its {limit}-word budget")
    if ceiling is not None:
        panel_html = re.findall(r'<[a-z]+[^>]*role="tabpanel"[^>]*>([\s\S]*?)(?=<[a-z]+[^>]*role="tabpanel"|<label class="note-row"|<div class="copy-bottom"|<footer)', markup)
        if per_tab and panel_html:
            frame = markup
            for ph in panel_html:
                frame = frame.replace(ph, " ", 1)
            frame_words = len(field_text(re.sub(r"<[^>]+>", " ", frame)).split())
            for i, ph in enumerate(panel_html, start=1):
                seen = frame_words + len(field_text(re.sub(r"<[^>]+>", " ", ph)).split())
                if seen > ceiling:
                    fail(f"tab {i}: {seen} words in view (frame {frame_words} + panel {seen - frame_words}) over the {ceiling}-word ceiling (terse register, measured per tab)")
        else:
            total = len(field_text(prose).split())
            if total > ceiling:
                fail(f"reader prose: {total} words over the {ceiling}-word page ceiling (terse register)")

    # 6. the machinery the reader needs
    if raw.count("data-copy") < 2:
        fail("copy-for-AI control missing at top or bottom")
    if "data-records" in raw and "answers: " not in raw:
        fail("the digest does not emit what each ask decides, so a pasted answer stays unattachable")
    if 'name="viewport"' not in raw:
        fail("no viewport meta - taps mis-register on a phone")
    if "prefers-color-scheme" not in raw or 'data-theme="dark"' not in raw:
        fail("both themes must be defined (media query AND the explicit stamp)")
    for m in re.finditer(r'<a\s[^>]*href="https?://[^"]+"[^>]*>', raw):
        if 'target="_blank"' not in m.group(0):
            fail("an external link does not open a new tab; sandboxed hosts block same-frame navigation")
    if re.search(r"REPLACE", raw) and "template" not in path.name:
        fail("instance still carries REPLACE marks")
    return bad

def check_tree(root: Path) -> list[str]:
    """Every family directory under root: one template, N instances checked against it."""
    problems: list[str] = []
    families = sorted({p.parent for p in root.rglob("*.html")})
    for fam in families:
        htmls = sorted(fam.glob("*.html"))
        templates = [p for p in htmls if is_template(p.read_text(encoding="utf-8"))]
        if len(templates) != 1:
            problems.append(f"{fam.relative_to(ROOT)}: expected exactly one source template "
                            f"(file with REPLACE marks), found {len(templates)}")
            continue
        tmpl = templates[0]
        tfields = fields(markup_only(tmpl.read_text(encoding="utf-8")))
        problems += check_file(tmpl, None)
        for inst in htmls:
            if inst != tmpl:
                problems += check_file(inst, tfields)
    return problems


# ── DECLARED BUDGETS NAME THEIR CHECK (issue413, D0047) ──────────────────────────────────────────
# A skill's contract table declares numeric budgets. A budget nobody measures is a reminder, and D0047
# says a reminder is not a control: three briefs went out over the 2.1 word budgets while the prose under
# the table said a checker held them. So every row of a table whose header names a Budget column, when
# its budget carries a number, names IN THE ROW the check that measures it - a `check_templates.py --<flag>`
# this script dispatches, a guard from the catalogue (.engine/docs/guards.md), or a pytest path that exists
# - or carries the literal word `unheld`, which is an honest declaration and passes. A section-level
# mention is not accepted: it cannot say which row it holds, and that is the shape that failed. A named
# check that does not exist fails too - a pointer to nothing is the same hope with a filename.
SKILLS_DIR = ROOT / ".engine" / "skills"
GUARDS_DOC = ROOT / ".engine" / "docs" / "guards.md"
BUDGET_HEADER = re.compile(r"\bbudget\b", re.I)
TABLE_SEP = re.compile(r"^\|?\s*:?-{3,}:?\s*(?:\|\s*:?-{3,}:?\s*)*\|?\s*$")
HEADING = re.compile(r"^#{1,6}\s+(.*?)\s*$")
CT_FLAG = re.compile(r"check_templates\.py\s+(--[a-z][a-z-]*)")
PYTEST_PATH = re.compile(r"(?<![\w/])(tests/[\w./-]*[\w/])")
GUARD_TICK = re.compile(r"`([a-z][a-z0-9-]*)`")
GUARD_CMD = re.compile(r"keel gate guard\s+([a-z][a-z0-9-]*)")
UNHELD = re.compile(r"\bunheld\b", re.I)


def own_flags() -> set[str]:
    """The flags this script dispatches on, read from its own source: the `argv[1:2] ==` list literals."""
    return set(re.findall(r'\["(--[a-z][a-z-]*)"\]', Path(__file__).read_text(encoding="utf-8")))


def catalogue_guards(doc: Path = GUARDS_DOC) -> set[str]:
    """Every guard the catalogue lists: the backticked first cell of a table row."""
    try:
        text = doc.read_text(encoding="utf-8")
    except OSError:
        return set()
    return set(re.findall(r"^\|\s*`([a-z][a-z0-9-]*)`\s*\|", text, re.M))


def _cells(line: str) -> list[str]:
    return [c.strip() for c in line.strip().strip("|").split("|")]


def budget_tables(text: str) -> list[tuple[str, list[str], list[tuple[int, list[str]]]]]:
    """(section heading, header cells, [(line number, row cells)]) for every table with a Budget column."""
    out: list[tuple[str, list[str], list[tuple[int, list[str]]]]] = []
    lines = text.splitlines()
    section = "(top)"
    i = 0
    while i < len(lines):
        h = HEADING.match(lines[i])
        if h:
            section = h.group(1)
            i += 1
            continue
        if lines[i].lstrip().startswith("|"):
            block_start = i
            while i < len(lines) and lines[i].lstrip().startswith("|"):
                i += 1
            block = lines[block_start:i]
            if len(block) >= 2 and TABLE_SEP.match(block[1]):
                header = _cells(block[0])
                if any(BUDGET_HEADER.search(re.sub(r"[*`]", "", c)) for c in header):
                    rows = [(block_start + k + 1, _cells(block[k])) for k in range(2, len(block))]
                    out.append((section, header, rows))
            continue
        i += 1
    return out


def check_budgets_text(skill: str, text: str, guards: set[str], flags: set[str],
                       exists=None) -> tuple[list[str], list[tuple[str, str, str, str, str]]]:
    """One skill's text. Returns (violations, census); census rows are (skill, section, label, budget, status)
    where status is `held -> <check>`, `unheld`, `no number` or `FAIL`. `exists` decides whether a pytest
    path is real (default: the repo tree), so a fixture can be judged without reading the tree (D0388)."""
    exists = (lambda rel: (ROOT / rel).exists()) if exists is None else exists
    bad: list[str] = []
    census: list[tuple[str, str, str, str, str]] = []
    for section, header, rows in budget_tables(text):
        bcol = next(i for i, c in enumerate(header) if BUDGET_HEADER.search(re.sub(r"[*`]", "", c)))
        for lineno, cells in rows:
            if len(cells) <= bcol:
                continue
            budget = cells[bcol]
            label_cells = [re.sub(r"[*`]", "", c).strip() for j, c in enumerate(cells) if j != bcol]
            label = next((c for c in label_cells if c and not re.fullmatch(r"\d+", c)), f"line {lineno}")
            where = f"{skill} §{section} row \"{label}\" (line {lineno}) budget \"{budget}\""
            row_text = " | ".join(cells)
            held: list[str] = []
            broken: list[str] = []
            for flag in CT_FLAG.findall(row_text):
                (held if flag in flags else broken).append(f"check_templates.py {flag}" +
                                                           ("" if flag in flags else " (this script does not dispatch it)"))
            for path in PYTEST_PATH.findall(row_text):
                (held if exists(path) else broken).append(path + ("" if exists(path) else " (no such path)"))
            for name in GUARD_CMD.findall(row_text):
                (held if name in guards else broken).append(f"guard {name}" +
                                                            ("" if name in guards else " (not in the catalogue)"))
            for name in GUARD_TICK.findall(row_text):
                if name in guards and f"guard {name}" not in held:
                    held.append(f"guard {name}")
            numeric = re.search(r"\d", budget) is not None
            for b in broken:
                bad.append(f"{where} names a check that does not exist: {b}")
            if broken:
                status = "FAIL"          # already reported above, once per broken name
            elif held:
                status = "held -> " + ", ".join(held)
            elif UNHELD.search(row_text):
                status = "unheld"
            elif not numeric:
                status = "no number"
            else:
                status = "FAIL"
                bad.append(f"{where} names no check and does not say unheld")
            census.append((skill, section, label, budget, status))
    return bad, census


def check_budgets(skills_dir: Path = SKILLS_DIR) -> tuple[list[str], list[tuple[str, str, str, str, str]]]:
    """Every .engine/skills/*/SKILL.md, in name order."""
    guards, flags = catalogue_guards(), own_flags()
    bad: list[str] = []
    census: list[tuple[str, str, str, str, str]] = []
    for skill_md in sorted(skills_dir.glob("*/SKILL.md")):
        b, c = check_budgets_text(skill_md.parent.name, skill_md.read_text(encoding="utf-8"), guards, flags)
        bad += b
        census += c
    return bad, census


def print_census(census: list[tuple[str, str, str, str, str]], out=None) -> None:
    out = sys.stdout if out is None else out
    for skill, section, label, budget, status in census:
        print(f"  {status:<9} {skill} §{section} | {label} | {budget}" if status in ("unheld", "no number", "FAIL")
              else f"  {'held':<9} {skill} §{section} | {label} | {budget} -> {status[len('held -> '):]}", file=out)


# The D0388 pair, written before the live skill set was read. The positive table holds every row shape the
# check accepts (a dispatched flag, a pytest path, a catalogue guard, an honest `unheld`, a budget with no
# number); the negative is the SAME table with the Title row's check removed, so the one difference is
# the one the check must see.
_BUDGET_TABLE = (
    "## 2.1 The section order\n\n"
    "| # | Section | What it must carry | Budget | Held by |\n"
    "|---|---|---|---|---|\n"
    "| 0 | **Title** | The recommendation | ≤ 18 words | {title_check} |\n"
    "| 1 | **Why now** | The clock | ≤ 60 words | `tests/exec_summary` |\n"
    "| 2 | **Courses** | 2–4 with a do-nothing | table | unheld |\n"
    "| 3 | **Falsifier** | One per ask | ≤ 30 words each | unheld |\n"
    "| 4 | **Exhibits** | Two per ask | — | `check_templates.py --brief` |\n"
    "| 5 | **Verdict** | One line | ≤ 12 words | guard `doc-sync` |\n"
)
_NO_BUDGET_TABLE = "## 3. Forms\n\n| The claim says | Form |\n|---|---|\n| costs 3 times more | bars, 12 max |\n"
_PROBE_GUARDS = {"doc-sync"}
_PROBE_FLAGS = {"--brief", "--budgets"}
_PROBE_EXISTS = lambda rel: rel == "tests/exec_summary"  # noqa: E731


def budgets_probe() -> int:
    """Known-positive and known-negative, judged against fixed guards/flags/paths so the verdict is about the
    check and not about the tree it happens to run in. Returns the number of disagreeing cases."""
    args = (_PROBE_GUARDS, _PROBE_FLAGS, _PROBE_EXISTS)
    good = _BUDGET_TABLE.format(title_check="`check_templates.py --brief`") + _NO_BUDGET_TABLE
    bad = _BUDGET_TABLE.format(title_check="") + _NO_BUDGET_TABLE
    missing = _BUDGET_TABLE.format(title_check="`tests/no_such_suite`")
    unknown_flag = _BUDGET_TABLE.format(title_check="`check_templates.py --nowhere`")
    cases = [
        ("known-positive: every numeric row names an existing check or says unheld", check_budgets_text("fx", good, *args)[0], []),
        ("known-negative: the Title row with no check and no unheld is refused, naming the row and the number",
         check_budgets_text("fx", bad, *args)[0], ['row "Title"', '"≤ 18 words"', "names no check"]),
        ("a named pytest path that does not exist is refused", check_budgets_text("fx", missing, *args)[0], ["tests/no_such_suite (no such path)"]),
        ("a check_templates flag this script does not dispatch is refused", check_budgets_text("fx", unknown_flag, *args)[0], ["--nowhere (this script does not dispatch it)"]),
        ("a table with no Budget column is not a budget table", check_budgets_text("fx", _NO_BUDGET_TABLE, *args)[1], None),
    ]
    failed = 0
    for name, got, want in cases:
        if want is None:
            ok = got == []
        elif not want:
            ok = got == []
        else:
            ok = len(got) == 1 and all(w in got[0] for w in want)
        print(f"  {'ok  ' if ok else 'FAIL'} {name}" + ("" if ok else f": {got}"))
        failed += not ok
    census = check_budgets_text("fx", good, *args)[1]
    statuses = [s.split(" -> ")[0] for *_, s in census]
    ok = statuses == ["held", "held", "unheld", "unheld", "held", "held"]
    print(f"  {'ok  ' if ok else 'FAIL'} the census reads held/held/unheld/unheld/held/held" + ("" if ok else f": {statuses}"))
    failed += not ok
    print(f"check_templates --probe (budgets): {'pass' if not failed else f'{failed} failed'}")
    return failed


def _brief_fixture(title: str, ask: str, filler_words: int = 0, panels: int = 1, tabs: int | None = None,
                   selected: int = 1, panel_words: int = 0, extra: str = "", charset: bool = True) -> str:
    """A minimal tabbed page that satisfies every brief clause, for the self-test to vary: `panels` asks each
    in its own panel, `tabs` tab buttons (defaults to panels), `selected` of them selected, `panel_words` of
    filler inside every panel, `extra` markup inside the first panel."""
    fig = ('<figure><p class="msg">The page shows what waits.</p><svg role="img" '
           'aria-label="a figure whose label is long enough to count as real"><text>a thing</text></svg></figure>')
    filler = " ".join(["word"] * filler_words)
    pf = " ".join(["word"] * panel_words)
    tabs = panels if tabs is None else tabs
    strip = "".join(f'<button role="tab" aria-selected="{"true" if i < selected else "false"}">t{i}</button>'
                    for i in range(tabs))
    body = "".join(
        f'<section role="tabpanel"><p>{pf}</p>{extra if i == 0 else ""}{fig}{fig}'
        f'<div class="opts" data-records="d000{i}"><label><input type="radio" name="a{i}" value="x">x</label>'
        f'<label><input type="radio" name="a{i}" value="y">y</label></div></section>'
        for i in range(panels))
    meta = '<meta charset="utf-8">' if charset else ""
    return (f'{meta}<title>Brief</title><meta name="viewport" content="width=device-width">'
            f'<style>:root{{}} @media (prefers-color-scheme: dark){{}} [data-theme="dark"]{{}}</style>'
            f'<h1 data-digest="title">{title}</h1><button data-copy></button>'
            f'<div class="ask"><p>{ask}</p></div><p>{filler}</p>{strip}{body}'
            f'<div class="copy-bottom"><button data-copy></button></div>'
            f'<footer data-digest="provenance">Computed on 2026-09-08.</footer>'
            f'<script>const s = "answers: ";</script>')


def _members(n: int, declared: int | None = None) -> str:
    names = "".join(f'<span class="m">verb{i}</span> ' for i in range(n))
    return f'<table data-members="{n if declared is None else declared}"><tbody><tr><td>{names}</td></tr></tbody></table>'


def self_test() -> int:
    """The budget clauses in both directions (issue413): a page over any declared budget is refused
    naming the section; a page inside them passes; the whole-page ceiling refuses a page over it and a
    generous ceiling passes a long one (D0377 is accepted, so ceiling=None resolves to 450, not off)."""
    p = Path("fixture.html")
    good = _brief_fixture("The page shows a receipt when nothing waits.", "Accept the receipt shape.")
    long_title = " ".join(["word"] * 26) + " is"
    long_ask = " ".join(["word"] * 80)
    fx = _brief_fixture
    checks = [
        ("in-budget page passes", check_brief(p, ceiling=450, raw=good, per_tab=False), []),
        ("27-word headline refused", check_brief(p, ceiling=None, raw=fx(long_title, "Accept.")), ["headline: 27 words"]),
        ("80-word ask refused", check_brief(p, ceiling=None, raw=fx("The page waits.", long_ask)), ["the ask: 80 words"]),
        ("ceiling refused when given", check_brief(p, ceiling=450, raw=fx("The page waits.", "Accept.", 500), per_tab=False), ["reader prose: 5"]),
        ("a generous ceiling does not refuse a long page", check_brief(p, ceiling=100_000, raw=fx("The page waits.", "Accept.", 500), per_tab=False), []),
        # issue401: the encoding is declared in the prescan window; a page carrying the character without it is refused
        ("no charset refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", charset=False, extra="<p>Read \u2014 then answer.</p>")), ["no charset declaration in the first 1024 bytes"]),
        ("charset past the first 1024 bytes refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", charset=False, extra="<p>Read \u2014 then answer.</p>").replace("<title>", "<!--" + "x" * 1100 + "--><meta charset=\"utf-8\"><title>", 1)), ["no charset declaration in the first 1024 bytes"]),
        ("an http-equiv declaration passes", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", charset=False).replace("<title>", "<meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\"><title>", 1)), []),
        # D0404: tabs, panels and asks are one count; one selected
        ("two asks, two tabs, one selected passes", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", panels=2)), []),
        ("two asks under one tab refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", panels=2, tabs=1)), ["tabs, panels and asks disagree: 1 tabs, 2 panels, 2 asks"]),
        ("no tab starting selected refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", selected=0)), ["exactly one tab must start selected"]),
        ("two tabs starting selected refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", panels=2, selected=2)), ["exactly one tab must start selected"]),
        # D0405: a duration is refused; a date is not a duration
        ("a day each refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", extra="<p>A five collapses, a day each.</p>")), ["implementation duration: \"a day\""]),
        ("five days refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", extra="<p>Five days of work.</p>")), ["implementation duration: \"Five days\""]),
        ("a date passes", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", extra="<p>Waiting since 2026-09-01.</p>")), []),
        # D0406: an aggregate names its members, and the count reconciles
        ("collapse five families with no members refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", extra="<p>Collapse five families.</p>")), ["counts a set the page never names"]),
        ("collapse with a reconciled members table passes", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", extra="<p>Collapse five families.</p>" + _members(5))), []),
        ("members table off by one refused", check_brief(p, ceiling=None, raw=fx("The page waits.", "Accept.", extra="<p>Fold 4 verbs.</p>" + _members(4, declared=5))), ["declares 5 members and names 4"]),
        # D0407: the ceiling per tab - two 300-word panels pass per tab and fail summed
        ("two 300-word tabs pass the ceiling per tab", check_brief(p, ceiling=450, raw=fx("The page waits.", "Accept.", panels=2, panel_words=300), per_tab=True), []),
        ("the same page fails the ceiling summed", check_brief(p, ceiling=450, raw=fx("The page waits.", "Accept.", panels=2, panel_words=300), per_tab=False), ["reader prose: 6"]),
        ("a 460-word tab fails per tab, naming it", check_brief(p, ceiling=450, raw=fx("The page waits.", "Accept.", panels=2, panel_words=460), per_tab=True), ["tab 1: ", "tab 2: "]),
    ]
    failed = 0
    for name, got, want in checks:
        ok = (not got) if not want else all(any(w in g for g in got) for w in want) and len(got) == len(want)
        print(f"  {'ok  ' if ok else 'FAIL'} {name}" + ("" if ok else f": {got}"))
        failed += not ok
    # the budget-table pair rides the same CI marker, so a skill table that stops naming its checks is
    # caught by the run that already exists rather than by a second marker nobody added
    failed += budgets_probe()
    # and the LIVE census, so the skill set is judged by the run CI already makes - fixtures alone would
    # prove the check works while nothing ran it over the table it exists for
    live, census = check_budgets()
    ok = not live
    print(f"  {'ok  ' if ok else 'FAIL'} live skill set: every numeric budget row names its check or says unheld"
          + ("" if ok else ": " + "; ".join(live)))
    failed += not ok
    print(f"check_templates --self-test: {'pass' if not failed else f'{failed} failed'}")
    return 1 if failed else 0


def main(argv: list[str]) -> int:
    if argv[1:2] == ["--self-test"]:
        return self_test()
    if argv[1:2] == ["--probe"]:
        return 1 if budgets_probe() else 0
    # `--budgets` walks every deployed skill's contract tables (issue413): the census is printed in full
    # because the list of what is held, unheld and failing IS the answer, not only the verdict line.
    if argv[1:2] == ["--budgets"]:
        problems, census = check_budgets()
        numeric = [c for c in census if re.search(r"\d", c[3])]
        held = sum(1 for c in numeric if c[4].startswith("held"))
        unheld = sum(1 for c in numeric if c[4] == "unheld")
        skills = len({c[0] for c in census})
        print_census(census, out=sys.stderr if problems else sys.stdout)
        if problems:
            print(f"budget contract: {len(problems)} violation(s)", file=sys.stderr)
            for pr in problems:
                print(f"  {pr}", file=sys.stderr)
            return 1
        print(f"budget contract: clean - {len(numeric)} budget row(s) with a number across {skills} skill(s): "
              f"{held} held, {unheld} declared unheld")
        return 0
    # `--brief FILE ...` applies the executive-brief contract (D0355) instead of the template-family
    # checks: a brief is a rendered deliverable, not a member of a template family.
    if argv[1:2] == ["--brief"]:
        problems: list[str] = []
        for a in argv[2:]:
            p = Path(a)
            p = p if p.is_absolute() else (ROOT / p)
            if not p.exists():
                print(f"check_templates: no such path: {p}", file=sys.stderr)
                return 2
            problems += check_brief(p)
        if problems:
            print(f"brief contract: {len(problems)} violation(s)", file=sys.stderr)
            for pr in problems:
                print(f"  {pr}", file=sys.stderr)
            return 1
        print("brief contract: clean")
        return 0
    targets = [Path(a) for a in argv[1:]] or [ROOT / "templates"]
    problems: list[str] = []
    for t in targets:
        t = t if t.is_absolute() else (ROOT / t)
        if not t.exists():
            print(f"check_templates: no such path: {t}", file=sys.stderr)
            return 2
        problems += check_tree(t) if t.is_dir() else check_file(t, None)
    if problems:
        print(f"template contract: {len(problems)} violation(s)", file=sys.stderr)
        for p in problems:
            print(f"  {p}", file=sys.stderr)
        return 1
    print("template contract: clean")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
