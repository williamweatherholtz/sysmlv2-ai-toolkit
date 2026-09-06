"""SVG chart primitives for the executive summary — DRAWN FROM DATA, never hand-placed.

Every figure takes numbers and emits themed SVG: colours come from the page's CSS custom properties
so both themes work, each carries role="img" and a real aria-label, and each is titled with its
MESSAGE (what it proves), not its subject. viewBox width is always 640.

The rule these exist to enforce: a number that appears in a figure came from the facts file, so a
figure cannot drift from the tree the way a hand-drawn one does.
"""
from html import escape

W = 640
DEFS = (
    '<defs><marker id="ar" viewBox="0 0 8 8" refX="7" refY="4" markerWidth="7" markerHeight="7" '
    'orient="auto"><path d="M0,0 L8,4 L0,8 z" fill="var(--muted)"/></marker>'
    '<marker id="arb" viewBox="0 0 8 8" refX="7" refY="4" markerWidth="7" markerHeight="7" '
    'orient="auto"><path d="M0,0 L8,4 L0,8 z" fill="var(--bad)"/></marker>'
    '<marker id="aro" viewBox="0 0 8 8" refX="7" refY="4" markerWidth="7" markerHeight="7" '
    'orient="auto"><path d="M0,0 L8,4 L0,8 z" fill="var(--ok)"/></marker></defs>'
)
STYLE = (
    '<style>'
    '.bx{fill:var(--card); stroke:var(--accent); stroke-width:1.5; rx:5}'
    '.bx2{fill:var(--fill); stroke:var(--muted); stroke-width:1.2; rx:5}'
    '.lb{font:500 12px "Roboto Condensed",sans-serif; fill:var(--head)}'
    '.sm{font:400 10.5px "Roboto Condensed",sans-serif; fill:var(--muted)}'
    '.st{font:500 10px "Roboto Condensed",sans-serif; fill:var(--muted); letter-spacing:.14em}'
    '.big{font:500 15px "Roboto Condensed",sans-serif; fill:var(--head)}'
    '.ln{stroke:var(--muted); stroke-width:1.2; fill:none; marker-end:url(#ar)}'
    '.lnb{stroke:var(--bad); stroke-width:1.6; fill:none; marker-end:url(#arb)}'
    '.lno{stroke:var(--ok); stroke-width:1.6; fill:none; marker-end:url(#aro)}'
    '.bad{font:500 10.5px "Roboto Condensed",sans-serif; fill:var(--bad)}'
    '.ok{font:500 10.5px "Roboto Condensed",sans-serif; fill:var(--ok)}'
    '</style>'
)
TONE = {"accent": "var(--accent)", "muted": "var(--muted)", "ok": "var(--ok)",
        "bad": "var(--bad)", "warn": "var(--warn)", "line": "var(--line)"}


def _wrap(height, aria, body, message):
    """One figure: message title above (an action title - what it PROVES), svg below."""
    return (
        f'<figure class="diagram"><figcaption class="msg">{escape(message)}</figcaption>'
        f'<svg viewBox="0 0 {W} {height}" width="{W}" role="img" aria-label="{escape(aria)}">'
        f'{STYLE}{DEFS}{body}</svg></figure>'
    )


def bars(message, rows, aria=None, unit=""):
    """Proportional comparison. rows = [(label, value, tone, note)]. Longest bar sets the scale."""
    top, rh, lw = 8, 34, 210
    hi = max((r[1] for r in rows), default=1) or 1
    body = []
    for i, (label, val, tone, note) in enumerate(rows):
        y = top + i * rh
        w = max(2, round((W - lw - 96) * val / hi))
        body.append(f'<text class="lb" x="0" y="{y + 16}">{escape(label)}</text>')
        body.append(f'<rect x="{lw}" y="{y + 4}" width="{w}" height="17" rx="3" fill="{TONE[tone]}" opacity=".85"/>')
        u = unit if isinstance(unit, str) else (unit[0] if val == 1 else unit[1])
        body.append(f'<text class="big" x="{lw + w + 8}" y="{y + 18}">{val:,}{escape(u)}</text>')
        if note:
            body.append(f'<text class="sm" x="0" y="{y + 29}">{escape(note)}</text>')
    h = top + len(rows) * rh + 4
    ustr = unit if isinstance(unit, str) else unit[1]
    txt = "; ".join(f"{r[0]}: {r[1]}{ustr}" + (f" ({r[3]})" if r[3] else "") for r in rows)
    return _wrap(h, aria or f"{message}. {txt}", "".join(body), message)


def stacked(message, segments, aria=None, total_label=""):
    """Composition of one whole. segments = [(label, count, tone)]."""
    total = sum(s[1] for s in segments) or 1
    x, y, h = 0, 26, 40
    body = [f'<text class="st" x="0" y="14">{escape(total_label.upper())}</text>']
    legend = []
    for label, count, tone in segments:
        w = round(W * count / total)
        body.append(f'<rect x="{x}" y="{y}" width="{max(w,2)}" height="{h}" fill="{TONE[tone]}" opacity=".85"/>')
        if w > 46:
            body.append(f'<text class="big" x="{x + 8}" y="{y + 26}" fill="var(--paper)">{count}</text>')
        legend.append((x, label, count, tone))
        x += w
    step = W // max(len(legend), 1)
    for i, (_lx, label, count, tone) in enumerate(legend):
        lx = i * step
        body.append(f'<rect x="{lx}" y="{y + h + 10}" width="9" height="9" fill="{TONE[tone]}"/>')
        body.append(f'<text class="sm" x="{lx + 13}" y="{y + h + 19}">{escape(label)} ({count})</text>')
    txt = ", ".join(f"{s[0]}: {s[1]}" for s in segments)
    return _wrap(y + h + 30, aria or f"{message}. Of {total}: {txt}", "".join(body), message)


def timeline(message, days_total, marks, aria=None, axis_label=""):
    """Elapsed time with events. marks = [(day, label, tone, above)]."""
    x0, x1, y = 8, W - 8, 52
    body = [f'<line x1="{x0}" y1="{y}" x2="{x1}" y2="{y}" stroke="var(--line)" stroke-width="2"/>']
    if axis_label:
        body.append(f'<text class="st" x="{x0}" y="18">{escape(axis_label.upper())}</text>')
    body.append(f'<text class="sm" x="{x0}" y="{y + 40}">day 0</text>')
    body.append(f'<text class="sm" x="{x1}" y="{y + 40}" text-anchor="end">day {days_total}</text>')
    for day, label, tone, above in marks:
        px = x0 + round((x1 - x0) * min(day, days_total) / (days_total or 1))
        ty = y - 12 if above else y + 24
        body.append(f'<line x1="{px}" y1="{y - 7}" x2="{px}" y2="{y + 7}" stroke="{TONE[tone]}" stroke-width="2.5"/>')
        anchor = "start" if px < W * 0.7 else "end"
        body.append(f'<text class="sm" x="{px + (4 if anchor == "start" else -4)}" y="{ty}" '
                    f'text-anchor="{anchor}" fill="{TONE[tone]}">{escape(label)}</text>')
    txt = "; ".join(f"day {m[0]}: {m[1]}" for m in marks)
    return _wrap(96, aria or f"{message}. Over {days_total} days - {txt}", "".join(body), message)


def two_lane(message, before, after, boundary, aria=None, note=""):
    """The change itself: the same flow twice, so the eye finds the delta.

    before/after = (lane_label, [(label, sub, tone)]); boundary = (index, label) - the vertical line
    the decision moves something across, drawn in both lanes at the same x.
    """
    bw, gap, y0, y1, h = 132, 14, 40, 150, 46
    bi, blabel = boundary
    body = []
    bx = 8 + bi * (bw + gap) - gap // 2
    body.append(f'<line x1="{bx}" y1="24" x2="{bx}" y2="{y1 + h + 8}" stroke="var(--bad)" '
                f'stroke-width="1.2" stroke-dasharray="4 4"/>')
    body.append(f'<text class="bad" x="{bx + 6}" y="18">{escape(blabel)}</text>')
    for lane_y, (lane_label, steps) in ((y0, before), (y1, after)):
        body.append(f'<text class="st" x="0" y="{lane_y - 8}">{escape(lane_label.upper())}</text>')
        for i, (label, sub, tone) in enumerate(steps):
            x = 8 + i * (bw + gap)
            cls = "bx" if tone == "accent" else "bx2"
            body.append(f'<rect class="{cls}" x="{x}" y="{lane_y}" width="{bw}" height="{h}"/>')
            body.append(f'<text class="lb" x="{x + 10}" y="{lane_y + 20}">{escape(label)}</text>')
            body.append(f'<text class="sm" x="{x + 10}" y="{lane_y + 36}">{escape(sub)}</text>')
            if i:
                px = x - gap
                body.append(f'<line class="ln" x1="{px - 2}" y1="{lane_y + 23}" x2="{x - 2}" y2="{lane_y + 23}"/>')
    if note:
        body.append(f'<text class="sm" x="0" y="{y1 + h + 26}">{escape(note)}</text>')
    aria_txt = (f"{message}. Before: " + " then ".join(f"{s[0]} ({s[1]})" for s in before[1]) +
                ". After: " + " then ".join(f"{s[0]} ({s[1]})" for s in after[1]) +
                f". The line marked '{blabel}' is what the change moves work across.")
    return _wrap(y1 + h + (34 if note else 16), aria or aria_txt, "".join(body), message)


def dots(message, total, marked, mark_label, rest_label, aria=None, cols=30):
    """Unit chart: one dot per item, so a proportion is COUNTED, not estimated."""
    r, sp, x0, y0 = 5, 15, 6, 34
    body = [f'<text class="st" x="0" y="16">{escape(f"{marked} of {total}").upper()}</text>']
    for i in range(total):
        cx, cy = x0 + (i % cols) * sp, y0 + (i // cols) * sp
        tone = "bad" if i < marked else "line"
        body.append(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="{TONE[tone]}" opacity=".9"/>')
    rows = (total + cols - 1) // cols
    ly = y0 + rows * sp + 6
    body.append(f'<circle cx="{x0}" cy="{ly}" r="{r}" fill="{TONE["bad"]}"/>')
    body.append(f'<text class="sm" x="{x0 + 12}" y="{ly + 4}">{escape(mark_label)}</text>')
    body.append(f'<circle cx="{x0 + 250}" cy="{ly}" r="{r}" fill="{TONE["line"]}"/>')
    body.append(f'<text class="sm" x="{x0 + 262}" y="{ly + 4}">{escape(rest_label)}</text>')
    return _wrap(ly + 16, aria or f"{message}. {marked} of {total} are {mark_label}; the rest, {rest_label}.", "".join(body), message)
