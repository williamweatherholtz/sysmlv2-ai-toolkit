#!/usr/bin/env python3
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


def main(argv: list[str]) -> int:
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
