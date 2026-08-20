#!/usr/bin/env python3
"""Generate the outstanding-obligations review canvas as ONE self-contained HTML file.

Deployed by the `obligation-review` skill. Reads the computed views through the `keel` binary -- the
authority (CLAUDE.md section 2) -- so the canvas can never disagree with `keel orient`, and needs no
running server. Nothing here is authored state: the page is a #View, regenerable and never truth.

Per-item state (verdict, note) is keyed by the item's MINTED UUID, never by its display label, so a
regeneration that renames or renumbers items does not orphan the human's judgments. A content
signature is stored alongside each verdict, so an item edited after it was judged re-surfaces with a
"changed" badge rather than silently keeping a stale verdict.

Usage:  python .engine/tools/obligation_canvas.py [ROOT] [-o OUT.html]
"""
import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

# One hue per obligation class (categorical), deliberately distinct from the semantic verdict colors.
CLASSES = [
    ("acceptance", "Decisions awaiting your acceptance", "#4c5fd7",
     "An AI actor cannot supply this (D0106). Until you sign, the work each charters stays blocked."),
    ("authority", "Authority queue", "#0e7c86",
     "Judgments only a registered person may make, with how long each has waited."),
    ("finding", "Findings awaiting disposition", "#b5651d",
     "Critique results at or above Medium with no disposition recorded."),
    ("sitting", "Sitting reviews due", "#7d4f9c",
     "Sprints with no sitting review since D0155's grandfather line."),
]


def keel(root, *args):
    """Run a keel view and parse its JSON. The binary is the authority; nothing is reimplemented here."""
    exe = root / "target" / "release" / "keel.exe"
    if not exe.exists():
        exe = root / "target" / "release" / "keel"
    out = subprocess.run([str(exe), *args, str(root)], capture_output=True, text=True)
    if out.returncode != 0:
        print("warning: keel " + " ".join(args) + " exited %d" % out.returncode, file=sys.stderr)
    try:
        return json.loads(out.stdout)
    except json.JSONDecodeError:
        print("warning: keel " + " ".join(args) + " produced no JSON", file=sys.stderr)
        return {}


def scan_items(root):
    """name -> {id, title} for every declared item, read from the .sysml text.

    Text-scanned rather than modelled because this tool needs only identity and a label. The UUID is
    what makes saved judgments survive a rename; the label is display only.
    """
    out = {}
    pat = re.compile(r'^\s*(?:#\w+\s+)?part\s+(\w+)\s*:\s*(\w+)\s*\{', re.M)
    for d in (".tracking", ".engine"):
        base = root / d
        if not base.exists():
            continue
        for f in base.rglob("*.sysml"):
            try:
                text = f.read_text(encoding="utf-8")
            except OSError:
                continue
            for m in pat.finditer(text):
                name = m.group(1)
                if name in out:
                    continue
                tail = text[m.end():m.end() + 4000]
                uid = re.search(r':>>\s*id\s*=\s*"([^"]+)"', tail)
                title = re.search(r':>>\s*title\s*=\s*"([^"]*)"', tail)
                if uid:
                    out[name] = {"id": uid.group(1), "title": title.group(1) if title else ""}
    return out


def sig(*parts):
    """Content signature, so a verdict on an item that later CHANGED can be re-surfaced."""
    return hashlib.sha256("\x1f".join(parts).encode("utf-8")).hexdigest()[:12]


def collect(root):
    items = []
    decl = scan_items(root)

    seen = set()
    skipped = []

    def add(cls, name, title, meta, body=""):
        info = decl.get(name, {})
        # AN AGGREGATE ROW IS NOT A JUDGEABLE ITEM (issue158). The authority queue emits summary rows whose
        # `item` is a SENTENCE rather than an item name ("274 sprint(s) awaiting a sitting review"), and the
        # deck rendered them as cards with verdict buttons. The human tapped ACCEPT on one twice, because a
        # card with buttons says it can be acted on - and there is nothing to attest against a count. Every
        # declared item resolves to a UUID; a row that does not is a summary, and a summary belongs in the
        # class HEADER, not in a card. I first saw this as a "legitimate synthetic uid" and waved past it.
        if name not in decl:
            skipped.append((cls, name))
            return
        uid = info["id"]
        # ONE CARD PER ITEM. The authority queue reports the same decisionAcceptance rows that orient
        # reports as pendingAcceptances, so without this every proposed Decision appeared twice and the
        # human had to judge it twice - which they did, and said so. First class wins, and acceptances are
        # collected first because that class states what the item actually needs.
        if uid in seen:
            return
        seen.add(uid)
        shown = title or info.get("title") or name
        items.append({
            "cls": cls, "name": name, "uid": uid, "title": shown,
            "meta": meta, "body": body, "sig": sig(shown, body),
        })

    # ORDER MATTERS. The authority queue is the AGGREGATE of what awaits a person, so its rows overlap
    # the specific classes. Collect the specific ones first and authority last, or dedupe silently
    # reclassifies every finding as "authority queue" - which it did on the first attempt, taking the
    # findings count from 22 to 0 while the totals still looked plausible.
    for d in keel(root, "orient").get("pendingAcceptances", []):
        add("acceptance", d, decl.get(d, {}).get("title", d), "proposed Decision - needs your sign-off")

    for f in keel(root, "dispositions").get("findings", []):
        if not f.get("dispositioned"):
            fn = f.get("finding", "?")
            add("finding", fn, decl.get(fn, {}).get("title", fn),
                "severity %s - no disposition recorded" % f.get("severity", "?"))

    for s in keel(root, "sitting-coverage").get("due_sprints", []):
        add("sitting", s, decl.get(s, {}).get("title", s), "sprint has no sitting review")

    for a in keel(root, "authority-queue").get("awaiting", []):
        item = a.get("item", "?")
        escalated = " - ESCALATED" if a.get("escalated") else ""
        add("authority", item, decl.get(item, {}).get("title", item),
            "%s - waiting %sd%s" % (a.get("kind", "?"), a.get("waitingDays", "?"), escalated),
            a.get("note", ""))

    if skipped:
        print("skipped %d aggregate row(s) that name no declared item:" % len(skipped), file=sys.stderr)
        for cls, name in skipped:
            print("  [%s] %s" % (cls, name), file=sys.stderr)
    return items


def esc(s):
    """HTML-escape, and force pure ASCII by numeric-entity-encoding everything above 127.

    The artifact runtime supplies the <head>, so this file cannot guarantee a charset declaration - and a
    guessed encoding turns every em-dash in a Decision title into a replacement character. Emitting
    entities makes the output encoding-independent instead of hoping the host declares UTF-8.
    """
    out = (str(s).replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
           .replace('"', "&quot;"))
    return "".join(c if ord(c) < 128 else "&#%d;" % ord(c) for c in out)


CSS = """
:root{
  --ink:#0e1a1f; --ink2:#3c4f57; --ink3:#6d8189;
  --paper:#f3f6f6; --card:#ffffff; --line:#d5dfe0;
  --accept:#2e7d4f; --maybe:#a67c00; --reject:#b5342b;
  --r:12px;
  --sans:ui-sans-serif,system-ui,"Segoe UI",Roboto,sans-serif;
  --mono:ui-monospace,"SF Mono",Consolas,"Liberation Mono",monospace;
}
@media (prefers-color-scheme:dark){:root:not([data-theme="light"]){
  --ink:#e8f0f1; --ink2:#a6bcc2; --ink3:#7b959d;
  --paper:#0b1418; --card:#121f24; --line:#25373d;
  --accept:#5fbf85; --maybe:#d9ad3c; --reject:#e8736a;
}}
:root[data-theme="dark"]{
  --ink:#e8f0f1; --ink2:#a6bcc2; --ink3:#7b959d;
  --paper:#0b1418; --card:#121f24; --line:#25373d;
  --accept:#5fbf85; --maybe:#d9ad3c; --reject:#e8736a;
}
*{box-sizing:border-box}
body{margin:0;background:var(--paper);color:var(--ink);font:16px/1.5 var(--sans);
  -webkit-text-size-adjust:100%}
.wrap{max-width:680px;margin:0 auto;padding:0 14px 116px}
header.top{padding:22px 0 10px}
h1{margin:0;font-size:26px;font-weight:800;letter-spacing:-.02em;text-wrap:balance}
.sub{margin:6px 0 0;color:var(--ink3);font:12px/1.5 var(--mono)}
.stats{display:grid;grid-template-columns:repeat(2,1fr);gap:8px;margin:16px 0 6px}
.stat{background:var(--card);border:1px solid var(--line);border-left:4px solid var(--h);
  border-radius:var(--r);padding:10px 12px;display:flex;flex-direction:column;gap:2px}
.stat b{font:700 22px/1 var(--mono);font-variant-numeric:tabular-nums}
.stat span{font-size:11px;color:var(--ink3);text-transform:uppercase;letter-spacing:.07em}
.how{margin:14px 0 4px;color:var(--ink2);font-size:14px}
.grp{margin:18px 0 0;border:1px solid var(--line);border-radius:var(--r);background:var(--card);
  overflow:hidden}
.grph{width:100%;min-height:52px;display:flex;align-items:center;gap:10px;padding:12px 14px;
  background:none;border:0;border-left:4px solid var(--h);color:var(--ink);font:inherit;
  font-weight:650;text-align:left;cursor:pointer}
.grph .n{font:700 15px/1 var(--mono);color:var(--h);min-width:2ch;font-variant-numeric:tabular-nums}
.grph .t{flex:1}
.caret{width:9px;height:9px;border-right:2px solid var(--ink3);border-bottom:2px solid var(--ink3);
  transform:rotate(45deg);transition:transform .15s}
.grp[data-open=true] .caret{transform:rotate(-135deg)}
.grpb{display:none;padding:0 14px 14px;border-top:1px solid var(--line)}
.grp[data-open=true] .grpb{display:block}
.blurb{margin:12px 0;color:var(--ink3);font-size:13px}
.card{border:1px solid var(--line);border-left:4px solid var(--h);border-radius:var(--r);
  padding:12px;margin:10px 0;background:var(--card)}
.card[data-verdict=accept]{border-left-color:var(--accept)}
.card[data-verdict=maybe]{border-left-color:var(--maybe)}
.card[data-verdict=reject]{border-left-color:var(--reject)}
.card header{display:flex;align-items:center;gap:8px;flex-wrap:wrap}
.card code{font:600 12px/1 var(--mono);color:var(--ink3)}
.chg{font:600 10px/1 var(--sans);text-transform:uppercase;letter-spacing:.06em;
  color:var(--maybe);border:1px solid var(--maybe);border-radius:99px;padding:3px 7px}
.card h3{margin:7px 0 0;font-size:15px;font-weight:650;line-height:1.35;text-wrap:balance}
.meta{margin:6px 0 0;font:11px/1.5 var(--mono);color:var(--ink3)}
.body{margin:8px 0 0;font-size:13px;color:var(--ink2)}
.verdicts{display:flex;gap:6px;margin:11px 0 0}
.verdicts button{flex:1;min-height:44px;border:1px solid var(--line);border-radius:9px;
  background:transparent;color:var(--ink2);font:600 13px var(--sans);cursor:pointer}
.card[data-verdict=accept] button[data-v=accept]{background:var(--accept);border-color:var(--accept);color:#fff}
.card[data-verdict=maybe] button[data-v=maybe]{background:var(--maybe);border-color:var(--maybe);color:#1a1200}
.card[data-verdict=reject] button[data-v=reject]{background:var(--reject);border-color:var(--reject);color:#fff}
.note{width:100%;min-height:44px;margin:8px 0 0;padding:10px;border:1px solid var(--line);
  border-radius:9px;background:transparent;color:var(--ink);font:14px var(--sans)}
.note::placeholder{color:var(--ink3)}
.empty{color:var(--ink3);font-size:13px;margin:10px 0}
.bar{position:fixed;left:0;right:0;bottom:0;background:var(--card);
  border-top:1px solid var(--line);padding:10px 14px;display:flex;gap:8px;align-items:center}
.bar .cnt{flex:1;font:12px var(--mono);color:var(--ink3)}
.bar button{min-height:44px;padding:0 15px;border-radius:9px;border:1px solid var(--ink);
  background:var(--ink);color:var(--paper);font:650 14px var(--sans);cursor:pointer}
.bar button.ghost{background:transparent;color:var(--ink);border-color:var(--line)}
dialog{border:1px solid var(--line);border-radius:var(--r);background:var(--card);color:var(--ink);
  max-width:min(92vw,660px);width:100%;padding:14px}
dialog textarea{width:100%;height:46vh;border:1px solid var(--line);border-radius:9px;
  background:var(--paper);color:var(--ink);font:12px/1.5 var(--mono);padding:10px}
.dlgrow{display:flex;gap:8px;margin-top:10px}
:focus-visible{outline:2px solid var(--ink);outline-offset:2px}
@media (prefers-reduced-motion:reduce){*{transition:none!important}}
@media (min-width:560px){.stats{grid-template-columns:repeat(4,1fr)}}
"""

JS = """
// STATE LIVES IN THE DOM + localStorage, keyed by each item's MINTED UUID and never by its label: this
// deck is regenerated from a moving model, and label-keyed state orphans exactly when the most judgment
// has been invested in it. `sigs` records the content signature at judgement time, so an item edited
// after being judged re-surfaces with a badge instead of silently keeping a stale verdict.
var KEY = 'keelObligations.v1';
var S = JSON.parse(localStorage.getItem(KEY) || '{}');
S.verdicts = S.verdicts || {}; S.notes = S.notes || {}; S.sigs = S.sigs || {};
function save(){ localStorage.setItem(KEY, JSON.stringify(S)); }
var cards = [].slice.call(document.querySelectorAll('.card'));
function apply(){
  cards.forEach(function(c){
    var u = c.dataset.uid;
    if (S.verdicts[u]) c.dataset.verdict = S.verdicts[u];
    var n = c.querySelector('.note'); if (n && S.notes[u]) n.value = S.notes[u];
    c.querySelector('.chg').hidden = !(S.sigs[u] && S.sigs[u] !== c.dataset.sig);
  });
  var judged = cards.filter(function(c){ return c.dataset.verdict; }).length;
  document.getElementById('cnt').textContent = judged + ' of ' + cards.length + ' judged';
}
apply();
document.addEventListener('click', function(e){
  var h = e.target.closest('.grph');
  if (h) { var g = h.closest('.grp'); var open = g.dataset.open !== 'true';
    g.dataset.open = open ? 'true' : 'false';
    h.setAttribute('aria-expanded', open ? 'true' : 'false'); return; }
  var b = e.target.closest('.verdicts button');
  if (b) { var c = b.closest('.card'), u = c.dataset.uid;
    var next = c.dataset.verdict === b.dataset.v ? '' : b.dataset.v;
    c.dataset.verdict = next;
    if (next) { S.verdicts[u] = next; S.sigs[u] = c.dataset.sig; }
    else { delete S.verdicts[u]; delete S.sigs[u]; }
    save(); apply(); return; }
  if (e.target.id === 'exp') { buildExport(); document.getElementById('dlg').showModal(); return; }
  if (e.target.id === 'close') { document.getElementById('dlg').close(); return; }
  if (e.target.id === 'copy') { copyOut(); return; }
  if (e.target.id === 'clear') {
    if (confirm('Clear every verdict and note stored in this browser?')) {
      S = { verdicts:{}, notes:{}, sigs:{} }; save();
      cards.forEach(function(c){ c.dataset.verdict = '';
        var n = c.querySelector('.note'); if (n) n.value = ''; });
      apply(); }
    return; }
});
document.addEventListener('input', function(e){
  var n = e.target.closest('.note');
  if (n) { var u = n.closest('.card').dataset.uid;
    if (n.value) S.notes[u] = n.value; else delete S.notes[u];
    save(); }
});
// The clipboard API REJECTS inside a sandboxed artifact frame that lacks clipboard-write permission, and
// the first version had a .then with no .catch - so the promise failed silently, the fallback never ran,
// and the button did not even change its label. Every path now ends in either 'Copied' or an instruction,
// because a control that appears to do nothing is the affordance defect this session keeps producing.
function copyOut(){
  var ta = document.getElementById('out'), btn = document.getElementById('copy');
  function manual(){
    // readonly blocks programmatic selection on iOS, so lift it for the copy and put it back.
    ta.removeAttribute('readonly');
    ta.focus();
    try { ta.setSelectionRange(0, ta.value.length); } catch (e) { ta.select(); }
    var done = false;
    try { done = document.execCommand('copy'); } catch (e) { done = false; }
    ta.setAttribute('readonly', '');
    btn.textContent = done ? 'Copied' : 'Text selected - copy it';
  }
  if (navigator.clipboard && navigator.clipboard.writeText) {
    navigator.clipboard.writeText(ta.value).then(function(){ btn.textContent = 'Copied'; }, manual);
  } else { manual(); }
}
function buildExport(){
  var L = ['# Keel obligations review', '', 'Generated ' + STAMP, ''];
  var names = { accept:'ACCEPT', maybe:'NEEDS WORK', reject:'REJECT' };
  ['accept','maybe','reject'].forEach(function(v){
    var got = cards.filter(function(c){ return c.dataset.verdict === v; });
    if (!got.length) return;
    L.push('## ' + names[v] + ' (' + got.length + ')', '');
    got.forEach(function(c){
      var note = (S.notes[c.dataset.uid] || '').trim();
      L.push('- **' + c.querySelector('code').textContent + '** (' + c.dataset.cls + ') - '
        + c.querySelector('h3').textContent + (note ? '\\n  - note: ' + note : ''));
    });
    L.push('');
  });
  var noted = cards.filter(function(c){
    return !c.dataset.verdict && (S.notes[c.dataset.uid] || '').trim(); });
  if (noted.length) {
    L.push('## Notes without a verdict (' + noted.length + ')', '');
    noted.forEach(function(c){
      L.push('- **' + c.querySelector('code').textContent + '** - ' + S.notes[c.dataset.uid].trim()); });
    L.push('');
  }
  var judged = cards.filter(function(c){ return c.dataset.verdict; }).length;
  L.push(judged || noted.length ? '_' + judged + ' of ' + cards.length + ' items judged._'
                               : '_No verdicts recorded yet._');
  document.getElementById('out').value = L.join('\\n');
  document.getElementById('copy').textContent = 'Copy';
}
"""


def render(items, generated_at, head_sha):
    by_cls = {c[0]: [i for i in items if i["cls"] == c[0]] for c in CLASSES}
    total = len(items)

    stats = "".join(
        '<div class=stat style="--h:%s"><b>%d</b><span>%s</span></div>' % (hue, len(by_cls[key]), label)
        for key, label, hue, _blurb in CLASSES
    )

    sections = []
    for key, label, hue, blurb in CLASSES:
        rows = by_cls[key]
        cards = "".join(
            '<article class=card data-uid="%s" data-sig="%s" data-cls="%s" data-verdict="">'
            '<header><code>%s</code><span class=chg hidden>changed since you judged it</span></header>'
            '<h3>%s</h3><p class=meta>%s</p>%s'
            '<div class=verdicts role=group aria-label="verdict">'
            '<button data-v=accept>Accept</button>'
            '<button data-v=maybe>Needs work</button>'
            '<button data-v=reject>Reject</button></div>'
            '<input class=note type=text placeholder="why, or what to change&hellip;" aria-label="note">'
            '</article>' % (
                esc(i["uid"]), i["sig"], key, esc(i["name"]), esc(i["title"]), esc(i["meta"]),
                ('<p class=body>%s</p>' % esc(i["body"])) if i["body"] else "",
            )
            for i in rows
        ) or '<p class=empty>Nothing outstanding in this class.</p>'
        opened = "true" if rows else "false"
        sections.append(
            '<section class=grp data-open="%s" style="--h:%s">'
            '<button class=grph aria-expanded="%s"><span class=n>%d</span>'
            '<span class=t>%s</span><span class=caret aria-hidden=true></span></button>'
            '<div class=grpb><p class=blurb>%s</p>%s</div></section>'
            % (opened, hue, opened, len(rows), label, esc(blurb), cards)
        )

    stamp = "%s &middot; HEAD %s" % (esc(generated_at), esc(head_sha))
    return (
        "<title>Keel Obligations Deck</title>\n"
        '<meta name=viewport content="width=device-width,initial-scale=1">\n'
        "<style>%s</style>\n" % CSS
        + '<div class=wrap>\n<header class=top>\n  <h1>Keel Obligations Deck</h1>\n'
        + '  <p class=sub>%d item(s) waiting on you &middot; %s</p>\n' % (total, stamp)
        + '  <div class=stats>%s</div>\n' % stats
        + '  <p class=how>Tap a class to open it. Give each item a verdict, add a note if you have one, '
          'then <b>Copy for Claude</b> and paste it back. Judgments are stored in this browser and keyed '
          'by each item&rsquo;s UUID, so regenerating the deck keeps them.</p>\n</header>\n'
        + "".join(sections)
        + "\n</div>\n"
        + '<div class=bar><span class=cnt id=cnt></span>'
          '<button class=ghost id=clear>Reset</button>'
          '<button id=exp>Copy for Claude</button></div>\n'
        + '<dialog id=dlg><textarea id=out readonly></textarea>'
          '<div class=dlgrow><button id=copy>Copy</button>'
          '<button class=ghost id=close>Close</button></div></dialog>\n'
        + '<script>\nvar STAMP = %s;\n%s</script>\n' % (json.dumps("%s / HEAD %s" % (generated_at, head_sha)), JS)
    )


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("root", nargs="?", default=".")
    ap.add_argument("-o", "--out", default="obligations.html")
    a = ap.parse_args()
    root = Path(a.root).resolve()

    items = collect(root)
    sha = subprocess.run(["git", "-C", str(root), "rev-parse", "--short", "HEAD"],
                         capture_output=True, text=True).stdout.strip() or "unknown"
    date = subprocess.run(["git", "-C", str(root), "show", "-s", "--format=%ad", "--date=short",
                           "HEAD"], capture_output=True, text=True).stdout.strip() or "unknown"

    Path(a.out).write_text(render(items, date, sha), encoding="utf-8")
    print("%d item(s) -> %s" % (len(items), a.out))
    for key, label, _h, _b in CLASSES:
        print("  %3d  %s" % (sum(1 for i in items if i["cls"] == key), label))
    return 0


if __name__ == "__main__":
    sys.exit(main())
