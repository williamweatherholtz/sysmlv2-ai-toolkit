#!/usr/bin/env python3
# instrument: fit_check - measures rendered text geometry against its canvas in a real browser.
# No `# ci-probe:` marker: the probe drives headless Chromium through Python playwright, which the CI
# runner does not have. The enforcement point is the build instead - `assert_fits` runs the probe
# and then the page before a builder is allowed to leave a page on disk (D0402), so no brief is
# published unmeasured; CI's gap is the runner's, not the control's, and D0402 records it.
"""fit_check - an exhibit that cannot fit its content is refused, not published (issue390, D0402).

The defect: two exhibit builders lost text without saying so - a legend overprinted itself, and a note was
cut mid-word at the canvas edge. SVG does not wrap and does not clip visibly, so a figure can carry perfectly
correct numbers and still tell the reader something false by hiding half of it. Both builders were fixed;
nothing CHECKED that a rendered exhibit fits. A length-estimate probe written then reported five false
positives on right-anchored labels - an estimate cannot know where the browser will put the glyphs.

So this measures. The page is loaded in headless Chromium and every SVG `<text>` reports its rendered box
(`getBBox`, in the canvas's own units). A finding is one of:

  * OVERFLOW  - a text box crosses its svg's viewBox edge (what the reader sees as a sentence that stops);
                a crossing under 2px is a glyph's side bearing on a label set at the edge and is not one;
  * OVERLAP   - two text boxes in one svg intersect by more than a pixel each way (the legend defect);
  * OVERRUN   - a text that starts inside a drawn box ends past the box's right edge;
  * SCROLL    - the page body scrolls horizontally (a figure wider than its column).

Two passes: with the page's web fonts, and with them BLOCKED so the fallback face renders - a host that
cannot reach fonts.gstatic.com is a real reader, and the fallback is wider. Both passes must be clean.

    python scripts/exec_brief/fit_check.py PAGE.html      # exit 0 fits, 1 findings (each named), 2 usage
    python scripts/exec_brief/fit_check.py --probe        # the known cases, constructed before any page is read

    from fit_check import assert_fits; assert_fits(out_path)   # probe, then measure; on findings the page is
                                                               # REMOVED (artefact.claim semantics) and the run exits 1
"""
from __future__ import annotations

import os
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from charts import DEFS, STYLE, W   # noqa: E402  - the brief's own type, so fixtures measure on the real path

_MEASURE_JS = r"""
() => {
  const out = [];
  const TOL = 2;   // px: a glyph's side bearing at the edge; below the width of a stem at this size
  document.querySelectorAll('figure svg').forEach((svg, fi) => {
    const vb = svg.viewBox.baseVal;
    const cap = (svg.closest('figure').querySelector('figcaption') || {}).textContent || '';
    const rects = [...svg.querySelectorAll('rect')].map(r => r.getBBox());
    const texts = [...svg.querySelectorAll('text')]
      .map(t => { const b = t.getBBox(); return {x: b.x, y: b.y, w: b.width, h: b.height, s: t.textContent.trim()}; })
      .filter(t => t.w > 0 && t.s.length > 0);
    for (const t of texts) {
      const over = Math.max(t.x + t.w - (vb.x + vb.width), vb.x - t.x, t.y + t.h - (vb.y + vb.height), vb.y - t.y);
      if (over > TOL) out.push({fig: fi + 1, cap: cap.trim(), kind: 'OVERFLOW', text: t.s, by: Math.round(over)});
      // the box a text STARTS in: its left edge and vertical middle fall inside a rect
      const my = t.y + t.h / 2;
      const box = rects.find(r => t.x >= r.x - 0.5 && t.x <= r.x + r.width && my >= r.y && my <= r.y + r.height);
      if (box && t.x + t.w > box.x + box.width + TOL)
        out.push({fig: fi + 1, cap: cap.trim(), kind: 'OVERRUN', text: t.s, by: Math.round(t.x + t.w - (box.x + box.width))});
    }
    for (let i = 0; i < texts.length; i++) for (let j = i + 1; j < texts.length; j++) {
      const a = texts[i], b = texts[j];
      const ix = Math.min(a.x + a.w, b.x + b.w) - Math.max(a.x, b.x);
      const iy = Math.min(a.y + a.h, b.y + b.h) - Math.max(a.y, b.y);
      if (ix > TOL && iy > TOL) out.push({fig: fi + 1, cap: cap.trim(), kind: 'OVERLAP', text: a.s, other: b.s, by: Math.round(Math.min(ix, iy))});
    }
  });
  const se = document.scrollingElement;
  if (se && se.scrollWidth > se.clientWidth + 1) out.push({kind: 'SCROLL', by: se.scrollWidth - se.clientWidth});
  return out;
}
"""

_FONT_HOSTS = ("fonts.googleapis.com", "fonts.gstatic.com")


def _fmt(f: dict, pass_name: str) -> str:
    where = f"fig {f['fig']} ({f['cap'][:48]})" if "fig" in f else "page"
    if f["kind"] == "OVERLAP":
        return f"[{pass_name}] {where}: OVERLAP {f['by']}px between '{f['text'][:40]}' and '{f['other'][:40]}'"
    if f["kind"] == "SCROLL":
        return f"[{pass_name}] page scrolls horizontally by {f['by']}px"
    return f"[{pass_name}] {where}: {f['kind']} by {f['by']}px: '{f['text'][:60]}'"


def measure(path: str, width: int = 1180) -> list[str]:
    """Every finding on the rendered page, both font passes, formatted. Empty list = fits."""
    return [_fmt(f, pass_name) for pass_name, f in measure_raw(path, width)]


def measure_raw(path: str, width: int = 1180) -> list[tuple[str, dict]]:
    """(pass name, finding) for every finding the browser reports."""
    from playwright.sync_api import sync_playwright   # imported here so --help and the import of this module need no browser

    url = "file:///" + os.path.abspath(path).replace("\\", "/").lstrip("/")
    findings: list[tuple[str, dict]] = []
    with sync_playwright() as p:
        browser = p.chromium.launch()
        for pass_name, block_fonts in (("web fonts", False), ("fallback face", True)):
            page = browser.new_page(viewport={"width": width, "height": 900})
            if block_fonts:
                page.route(lambda u: any(h in u for h in _FONT_HOSTS), lambda route: route.abort())
            page.goto(url, wait_until="load")
            try:
                page.wait_for_load_state("networkidle", timeout=15000)
            except Exception:
                pass                                   # blocked font requests never go idle; measure anyway
            page.evaluate("() => document.fonts.ready")
            for f in page.evaluate(_MEASURE_JS):
                findings.append((pass_name, f))
            page.close()
        browser.close()
    return findings


# ------------------------------------------------------------------------------------------- probe
def _fixture(body: str) -> str:
    """One figure in the brief's own type, so the measurement path is the one the real page takes."""
    return ('<!doctype html><meta charset="utf-8"><style>:root{--card:#fff;--fill:#eee;--accent:#07c;--muted:#666;'
            '--head:#111;--ok:#080;--bad:#b00;--warn:#a60;--line:#ccc}body{margin:0;width:900px}figure{margin:0}</style>'
            f'<figure><figcaption>fixture</figcaption><svg viewBox="0 0 {W} 120" width="{W}" role="img" aria-label="fixture">'
            f'{STYLE}{DEFS}{body}</svg></figure>')


LONG = "already running and waiting for your signature before anything downstream can move at all"
PROBE_CASES = {
    # known positives - the two shapes issue390 found, and the box overrun the note fix does not cover
    "overflow: a note longer than the canvas": (_fixture(f'<text class="sm" x="400" y="40">{LONG}</text>'), "OVERFLOW"),
    "overlap: two legend entries at one position": (
        _fixture('<text class="lb" x="20" y="40">already running, needs your signature</text>'
                 '<text class="lb" x="60" y="40">12 waiting</text>'), "OVERLAP"),
    "overrun: a title wider than its box": (
        _fixture('<rect class="bx" x="20" y="20" width="120" height="50"/>'
                 '<text class="lb" x="30" y="40">a title that is far wider than the box it sits in</text>'), "OVERRUN"),
    # known negative - the same shapes at widths that fit, including a right-anchored label, the false-positive class
    "fits: short texts, boxed title, right-anchored count": (
        _fixture('<text class="sm" x="20" y="40">short note</text>'
                 '<rect class="bx" x="20" y="60" width="200" height="50"/><text class="lb" x="30" y="80">fits its box</text>'
                 f'<text class="big" x="{W - 4}" y="40" text-anchor="end">513 calls</text>'), None),
}


def probe() -> list[str]:
    """Run the known cases; return the ones that came back wrong (empty = discriminates)."""
    wrong = []
    with tempfile.TemporaryDirectory() as d:
        for name, (html, expect) in PROBE_CASES.items():
            p = os.path.join(d, "fixture.html")
            with open(p, "w", encoding="utf-8") as fh:
                fh.write(html)
            raw = measure_raw(p)
            kinds = {f["kind"] for _pass, f in raw}
            shown = [_fmt(f, pn) for pn, f in raw]
            if expect is None and raw:
                wrong.append(f"{name}: expected clean, got {shown}")
            elif expect is not None and expect not in kinds:
                wrong.append(f"{name}: expected {expect}, got {shown or 'clean'}")
    return wrong


def assert_fits(path: str) -> None:
    """The builder's exit: probe first (D0388), then measure; findings REMOVE the page and end the run."""
    wrong = probe()
    if wrong:
        os.remove(path)
        sys.exit("fit_check: the probe did not discriminate, so the page was removed unmeasured:\n  " + "\n  ".join(wrong))
    found = measure(path)
    if found:
        os.remove(path)
        sys.exit(f"fit_check: {len(found)} finding(s); the page was removed:\n  " + "\n  ".join(found))
    print(f"fit_check: {path} fits ({len(PROBE_CASES)} probe cases discriminated first)")


def main(argv: list[str]) -> int:
    if argv[1:2] == ["--probe"]:
        wrong = probe()
        for w in wrong:
            print("  " + w, file=sys.stderr)
        print(f"fit_check --probe: {'pass' if not wrong else f'{len(wrong)} of {len(PROBE_CASES)} cases wrong'}")
        return 1 if wrong else 0
    if len(argv) != 2 or not os.path.exists(argv[1]):
        print(__doc__, file=sys.stderr)
        return 2
    found = measure(argv[1])
    for f in found:
        print("  " + f, file=sys.stderr)
    print(f"fit_check: {'fits' if not found else f'{len(found)} finding(s)'}")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
