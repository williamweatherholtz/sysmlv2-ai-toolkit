"""Logic diagrams: how it works today, what the change does to that, and what lands downstream.

Two forms, both built from data:
  logic_lanes  - the causal chain twice (today / after), same geometry, the changed link highlighted.
                 Reads: CONDITION -> because -> MECHANISM -> so -> OUTCOME.
  downstream   - what the change lands on, who each thing is, and what happens to it.

Neither draws artifacts, records or process names. A box is a thing that happens or a thing that
exists; an edge is labelled with what travels along it.
"""
from html import escape
from charts import STYLE, DEFS, TONE, W, _wrap


def _box(x, y, w, h, title, sub, tone, sub2=""):
    cls = "bx" if tone == "accent" else "bx2"
    out = [f'<rect class="{cls}" x="{x}" y="{y}" width="{w}" height="{h}"/>',
           f'<text class="lb" x="{x + 10}" y="{y + 19}">{escape(title)}</text>']
    if sub:
        out.append(f'<text class="sm" x="{x + 10}" y="{y + 34}">{escape(sub)}</text>')
    if sub2:
        out.append(f'<text class="sm" x="{x + 10}" y="{y + 47}">{escape(sub2)}</text>')
    return "".join(out)


def _wrap_words(text, width):
    """Break a note on word boundaries. SVG does not wrap, so an unbroken line is silently CUT at the
    canvas edge - the reader sees a sentence that stops mid-word and nothing says it was truncated."""
    words, lines, line = text.split(), [], ""
    for w in words:
        candidate = f"{line} {w}".strip()
        if len(candidate) > width and line:
            lines.append(line)
            line = w
        else:
            line = candidate
    if line:
        lines.append(line)
    return lines


def logic_lanes(message, today, after, aria=None, note=""):
    """Two causal chains, same shape, so the eye finds the changed link.

    today/after = (lane_label, [(title, sub, tone, sub2)], [edge_label, edge_label])
    The lane label sits left; edges carry what travels ("so", "which means", counts).
    """
    # geometry is COMPUTED from the step count so a lane can never overrun the canvas
    n = max(len(today[1]), len(after[1]))
    gap = 96 if n <= 3 else 62
    bw = (W - 12 - gap * (n - 1)) // n
    h = 62
    cap = max(8, int(gap / 6.4))
    def _fit(t, n=cap):
        return t if len(t) <= n else t[: n - 1] + chr(8230)
    y0, y1 = 42, 168
    body = []
    for lane_y, (lane_label, steps, edges) in ((y0, today), (y1, after)):
        body.append(f'<text class="st" x="0" y="{lane_y - 9}">{escape(lane_label.upper())}</text>')
        for i, (title, sub, tone, sub2) in enumerate(steps):
            x = 6 + i * (bw + gap)
            body.append(_box(x, lane_y, bw, h, title, sub, tone, sub2))
            if i:
                x0e = 6 + (i - 1) * (bw + gap) + bw
                body.append(f'<line class="ln" x1="{x0e + 2}" y1="{lane_y + h / 2}" x2="{x - 4}" y2="{lane_y + h / 2}"/>')
                lab = edges[i - 1] if i - 1 < len(edges) else ""
                if lab:
                    body.append(f'<text class="sm" x="{x0e + (gap / 2) + 2}" y="{lane_y + h / 2 - 8}" '
                                f'text-anchor="middle">{escape(_fit(lab))}</text>')
    note_lines = _wrap_words(note, 104) if note else []
    for i, line in enumerate(note_lines):
        body.append(f'<text class="sm" x="6" y="{y1 + h + 24 + i * 15}">{escape(line)}</text>')
    aria_txt = (f"{message}. Today: " + " then ".join(f"{s[0]}, {s[1]}" for s in today[1]) +
                ". After the change: " + " then ".join(f"{s[0]}, {s[1]}" for s in after[1]) + ".")
    return _wrap(y1 + h + (19 + 15 * len(note_lines) if note_lines else 12), aria or aria_txt,
                 STYLE + DEFS + "".join(body), message)


def downstream(message, source, effects, aria=None, foot=""):
    """What the change lands on. source = (label, sub); effects = [(who, what happens, count, tone)].

    Drawn as a fan: one origin, N consequences, each carrying its own magnitude - so the reader sees
    both the spread and where the weight is.
    """
    sw, sh = 190, 66
    rows = len(effects)
    rh = 46
    top = 34
    total_h = max(sh + 20, rows * rh) + top + (26 if foot else 8)
    sy = top + (rows * rh - sh) // 2 if rows * rh > sh else top
    body = [f'<text class="st" x="0" y="20">THE CHANGE</text>',
            _box(0, sy, sw, sh, source[0], source[1], "accent",
                 source[2] if len(source) > 2 else "")]
    hub_x, hub_y = sw + 26, sy + sh / 2
    body.append(f'<line x1="{sw}" y1="{hub_y}" x2="{hub_x}" y2="{hub_y}" stroke="var(--muted)" stroke-width="1.2"/>')
    body.append(f'<text class="st" x="{hub_x + 8}" y="20">LANDS ON</text>')
    for i, (who, what, count, tone) in enumerate(effects):
        y = top + i * rh
        cy = y + rh / 2 - 6
        body.append(f'<path class="ln" d="M{hub_x},{hub_y} V{cy} H{hub_x + 26}" />')
        body.append(f'<text class="lb" x="{hub_x + 34}" y="{cy + 4}">{escape(who)}</text>')
        body.append(f'<text class="sm" x="{hub_x + 34}" y="{cy + 18}">{escape(what)}</text>')
        if count:
            body.append(f'<text class="big" x="{W}" y="{cy + 4}" text-anchor="end" '
                        f'fill="{TONE[tone]}">{escape(count)}</text>')
    if foot:
        body.append(f'<text class="sm" x="0" y="{total_h - 8}">{escape(foot)}</text>')
    aria_txt = f"{message}. {source[0]} lands on: " + "; ".join(f"{e[0]} - {e[1]} ({e[2]})" for e in effects)
    return _wrap(total_h, aria or aria_txt, STYLE + DEFS + "".join(body), message)
