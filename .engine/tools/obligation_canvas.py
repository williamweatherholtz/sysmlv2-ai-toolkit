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
     "An AI cannot sign these. Until you do, the work each charters stays blocked."),
    ("authority", "Authority queue", "#0e7c86",
     "Judgments only a person may make."),
    ("finding", "Findings awaiting disposition", "#b5651d",
     "Findings at Medium or above, undispositioned."),
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


def trim_title(name, title):
    """Drop a title's leading self-reference, because the card already shows the id.

    `d0166` followed by "D0166: a human statement is a tracked item..." printed the identifier twice in
    eight characters - what the human meant by duplicate titles and IDs. Only a prefix matching THIS
    item's own name is removed; a title that mentions some other item keeps it.
    """
    low = title.lower()
    for sep in (": ", " - ", " -- "):
        head, found, tail = low.partition(sep)
        if found and head.strip() == name.lower() and tail.strip():
            return title[len(head) + len(sep):].strip()
    return title


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
        shown = trim_title(name, title or info.get("title") or name)
        items.append({
            "cls": cls, "name": name, "uid": uid, "title": shown,
            "meta": meta, "body": body, "sig": sig(shown, body),
        })

    # ORDER MATTERS. The authority queue is the AGGREGATE of what awaits a person, so its rows overlap
    # the specific classes. Collect the specific ones first and authority last, or dedupe silently
    # reclassifies every finding as "authority queue" - which it did on the first attempt, taking the
    # findings count from 22 to 0 while the totals still looked plausible.
    for d in keel(root, "orient").get("pendingAcceptances", []):
        add("acceptance", d, decl.get(d, {}).get("title", d), "proposed")

    for f in keel(root, "dispositions").get("findings", []):
        if not f.get("dispositioned"):
            fn = f.get("finding", "?")
            add("finding", fn, decl.get(fn, {}).get("title", fn),
                "severity %s" % f.get("severity", "?"))

    for s in keel(root, "sitting-coverage").get("due_sprints", []):
        add("sitting", s, decl.get(s, {}).get("title", s), "unreviewed")

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
.grp[data-local-open=true] .caret{transform:rotate(-135deg)}
.grpb{display:none;padding:0 14px 14px;border-top:1px solid var(--line)}
.grp[data-local-open=true] .grpb{display:block}
.blurb{margin:12px 0;color:var(--ink3);font-size:13px}
.card{border:1px solid var(--line);border-left:4px solid var(--h);border-radius:var(--r);
  padding:12px;margin:10px 0;background:var(--card)}
.card[data-verdict=accept]{border-color:var(--accept);border-left-color:var(--accept);background:color-mix(in srgb,var(--accept) 9%,var(--card))}
.card[data-verdict=maybe]{border-color:var(--maybe);border-left-color:var(--maybe);background:color-mix(in srgb,var(--maybe) 9%,var(--card))}
.card[data-verdict=reject]{border-color:var(--reject);border-left-color:var(--reject);background:color-mix(in srgb,var(--reject) 9%,var(--card))}
.vd{display:none;font:700 10px/1 var(--sans);text-transform:uppercase;letter-spacing:.08em;
  padding:4px 8px;border-radius:99px;color:#fff}
.card[data-verdict] .vd{display:inline-block}
.card[data-verdict=""] .vd{display:none}
.card[data-verdict=accept] .vd{background:var(--accept)}
.card[data-verdict=maybe] .vd{background:var(--maybe);color:#1a1200}
.card[data-verdict=reject] .vd{background:var(--reject)}
.sv{font:11px var(--mono);color:var(--ink3)}
.sv::after{content:attr(data-local-sv)}
#livenote::after{content:attr(data-local-note);color:var(--maybe)}
#dbglog{font:10px/1.6 var(--mono);color:var(--ink3);margin:10px 0;max-height:9em;overflow-y:auto;
  border:1px dashed var(--line);border-radius:8px;padding:6px 9px}
#dbglog:empty{display:none}
#copy::after{content:attr(data-local-copied)}
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
.ro{display:none;margin:12px 0;padding:10px 12px;border:1px solid var(--maybe);
  border-left:4px solid var(--maybe);border-radius:var(--r);color:var(--ink2);font-size:13px}
body[data-local-readonly=true] .ro{display:block}
/* COPY IS A FALLBACK, NOT A ROUTE. The point of the live doc is that a verdict reaches the watching
   session with no copy step, and an always-visible "Copy for Claude" button teaches the reader that
   copying is what they are meant to do - which is how the deck trained a habit it did not need. The
   button exists only for a view that genuinely cannot write, and there it is the ONLY thing that
   works, so it appears exactly then and never otherwise. */
#exp{display:none}
body[data-local-readonly=true] #exp{display:inline-block}
/* A SIGNATURE IS NOT A VERDICT. The acceptance class asks for something no other class does and no AI
   may supply, so it carries a heavier rail and its own ribbon rather than only a different hue - a
   colour alone is a legend lookup, and the reader should not have to consult a legend to notice that
   a card needs their name on it. */
.card[data-cls=acceptance]{border-left-width:5px}
.card[data-cls=acceptance]>header{display:flex;align-items:center;gap:8px}
.card[data-cls=acceptance]>header::after{content:'needs your signature';margin-left:auto;
  font-size:10px;text-transform:uppercase;letter-spacing:.08em;color:var(--h);font-weight:700}
body[data-local-readonly=true] .live{display:none}
.live{margin:12px 0 0;color:var(--ink3);font-size:12px}
/* SELF-REPORT (issue188). `script ok` appears only if the script ran to its last statement;
   its ABSENCE means every listener above the throw never registered, which is how a deck with
   dead buttons was published and reported as working. */
.sub::after{content:' \00b7 script did NOT finish';color:var(--reject);font-weight:700}
body[data-local-js=ok] .sub::after{content:' \00b7 script ok';color:var(--accept);font-weight:600}
@media (prefers-reduced-motion:reduce){*{transition:none!important}}
@media (min-width:560px){.stats{grid-template-columns:repeat(4,1fr)}}
"""

JS = """
// THE PAGE IS THE RECORD (artifact live-doc capability). On a live doc the runtime treats <body> as the
// sync region: whatever a WRITER'S OWN GESTURE changes in the DOM - an attribute, a text input's value -
// is appended to the document as them and reaches every view and the watching Claude session. So a tap on
// Accept and a typed note ARE the submission. There is no copy step, which is the whole point of this
// version: the human asked to stop copying and pasting their own judgments back.
//
// THREE RULES FROM THE CONTRACT, and each one is a thing that would silently break sync:
//   1. Content is authored AS HTML by the generator and mutated directly in handlers. Nothing shared is
//      rendered from a JS object, and nothing shared is touched at load - a load-time render saves
//      nothing and can switch the element off for this view.
//   2. Per-viewer chrome (which class sections are expanded) lives on `data-local-*`, so collapsing a
//      section is not broadcast as an edit to everyone.
//   3. Notes are <input> elements, never <textarea> or contenteditable-with-children: input values are
//      captured, textarea values are not.
//
// localStorage is NOT used to hold verdicts any more. It would be a second record disagreeing with the
// document, and the document is the one that reaches Claude.

// Read-only is a REJECTION, not an assumption. A viewer who cannot write gets capture turned off and every
// region - the adopted <body> included - gets artifact-sync-state="off". Only that body-level signal means
// read-only; a single region switching off is a different fault and must not be reported as read-only
// (that mistake is in the canvas skill's anti-pattern list).
document.addEventListener('claude:sync-off', function(e){
  // DO NOT FLAG READ-ONLY HERE (issue188). A region can go off for reasons that say nothing about
  // whether this viewer may write - including a script touching the DOM, which is how this page
  // disabled itself. Read-only is now concluded ONLY from a write that was actually REJECTED, in
  // confirmSaved, where the runtime gives a reason code. This handler only reports what happened.
  // async event: data-local only, never textContent (that write is what killed the region).
  if (e.target === document.body || e.target === document.documentElement) {
    var el = document.getElementById('livenote');
    if (el) el.setAttribute('data-local-note', 'the document stopped accepting changes - a verdict may not reach Claude');
    dbg('sync-off on body');
  }
});
document.addEventListener('claude:sync-lost', function(){
  var el = document.getElementById('livenote');
  if (el) el.setAttribute('data-local-note', 'a change did not reach the document yet - it will go with the next one');
  dbg('sync-lost');
});

var LABEL = { accept:'accepted', maybe:'needs work', reject:'rejected', '':'' };

// THE PAGE SAYS WHETHER ITS OWN SCRIPT SURVIVED (issue188, the lesson of issue152 applied here).
// A script that throws leaves every listener below the throw unregistered, and the page looks
// identical - which is exactly how a deck with dead buttons got published and reported as working.
// This runs LAST, so `script ok` in the header means every listener above it registered. It writes a
// `data-local-*` attribute rather than text: per-viewer chrome is exempt from the sync region, so the
// marker cannot itself trip the region-off that broke the buttons in the first place.
function markScriptAlive(){ document.body.dataset.localJs = 'ok'; }

// POSITIVE CONFIRMATION, not an assumption. `claude.use('artifact')` resolves null when this view cannot
// write at all; `sync(fn)` resolves once what fn changed has been appended and REJECTS with a code if it
// was not. Calling it with a no-op after the gesture asks the runtime the only question that matters:
// did my change land? Anything other than success says so ON THE CARD, because a verdict the human
// believes they recorded and which never reached the document is the worst outcome available here.
var ART = (window.claude && claude.use) ? claude.use('artifact') : Promise.resolve(null);
function confirmSaved(card){
  // ASYNC FEEDBACK NEVER TOUCHES SYNCED DOM (issue188 round 2). The first version of this function
  // wrote sv.textContent from promise callbacks. That is an ASYNC script write to the synced region,
  // so the runtime switched the region off and REVERTED the tap it was meant to confirm - the
  // feedback mechanism was destroying the save. All status now goes to a data-local-* attribute
  // (exempt from sync) rendered by CSS attr(), and the verdict is RE-ASSERTED inside sync(fn), where
  // changes are attributed to the write explicitly.
  var sv = card.querySelector('.sv');
  var want = card.dataset.verdict;
  var say = function(msg){ if (sv) sv.setAttribute('data-local-sv', msg); };
  say('saving\u2026');
  ART.then(function(a){
    if (!a || !a.sync) { say('not saved - this view cannot write'); flagReadonly(); return; }
    return a.sync(function(){
      // Inside sync(fn) the change is attributed beyond doubt. Idempotent if the tap already held.
      if (card.dataset.verdict !== want) { card.dataset.verdict = want; }
    }).then(
      function(){
        say('saved');
        // READBACK: if anything later reverts the attribute, the card says so instead of lying.
        setTimeout(function(){
          if (card.dataset.verdict !== want) { say('REVERTED - tell Claude you saw this'); dbg('revert on ' + card.dataset.uid); }
        }, 1200);
      },
      function(err){
        var code = (err && err.code) || 'unknown';
        say('NOT saved (' + code + ')');
        dbg('sync rejected: ' + code);
        if (code === 'not_writer' || code === 'not_granted') { flagReadonly(); }
      });
  }).catch(function(e){ say('not saved'); dbg('sync threw: ' + e); });
}

// data-local-readonly is per-viewer chrome; setting it from async code is safe by the same rule.
function flagReadonly(){ document.body.dataset.localReadonly = 'true'; }

// THE EVENT LOG lives in an <artifact-local> element, which the contract exempts from sync entirely -
// script may write it freely, async included. It exists so the next field report can say WHICH step
// failed instead of describing a symptom.
function dbg(msg){
  var el = document.getElementById('dbglog');
  if (!el) return;
  var line = document.createElement('div');
  line.textContent = new Date().toISOString().slice(11, 19) + ' ' + msg;
  el.insertBefore(line, el.firstChild);
  while (el.children.length > 8) { el.removeChild(el.lastChild); }
}
window.onerror = function(m, s, l){ dbg('ERROR ' + m + ' @' + l); return false; };


var cards = [].slice.call(document.querySelectorAll('.card'));
function judged(){ return cards.filter(function(c){ return c.dataset.verdict; }).length; }
function count(){
  document.getElementById('cnt').textContent = judged() + ' of ' + cards.length + ' judged';
}
// NO DOM WRITE AT LOAD - THIS IS WHAT BROKE THE BUTTONS (issue188).
// On a live doc the runtime switches a sync region OFF when the region's DOM is changed by SCRIPT
// rather than by a gesture. Calling count() here wrote `#cnt.textContent` the instant the page loaded,
// so the body's region went off before the human touched anything, the sync-off handler flagged the
// whole view read-only, and every subsequent tap reported `not saved`. The initial count is now
// RENDERED INTO THE HTML by the generator - the contract's own instruction, "write content as HTML in
// the page and mutate it directly in handlers" - and count() runs only from inside a handler.

document.addEventListener('click', function(e){
  var h = e.target.closest('.grph');
  if (h) { var g = h.closest('.grp'); var open = g.dataset.localOpen !== 'true';
    g.dataset.localOpen = open ? 'true' : 'false';
    h.setAttribute('aria-expanded', open ? 'true' : 'false'); return; }
  var b = e.target.closest('.verdicts button');
  if (b) {
    // THE GESTURE IS THE WRITE: setting data-verdict inside this handler is what gets appended to the
    // document as this viewer. But a silent write is indistinguishable from a dead button - which is
    // exactly what they reported - so the card SAYS its verdict in words and then says whether it saved.
    var c = b.closest('.card');
    var next = c.dataset.verdict === b.dataset.v ? '' : b.dataset.v;
    c.dataset.verdict = next;
    var vd = c.querySelector('.vd'); if (vd) vd.textContent = LABEL[next] || '';
    count();
    confirmSaved(c);
    return;
  }
  if (e.target.id === 'exp') { buildExport(); document.getElementById('dlg').showModal(); copyOut(); return; }
  if (e.target.id === 'close') { document.getElementById('dlg').close(); return; }
  if (e.target.id === 'copy') { copyOut(); return; }
});

// The clipboard API REJECTS inside a sandboxed artifact frame without clipboard-write permission, and a
// .then() with no rejection handler fails silently - which shipped once and made the button look dead.
// Every path ends in either 'Copied' or an instruction.
function copyOut(){
  var ta = document.getElementById('out'), btn = document.getElementById('copy');
  function manual(){
    ta.removeAttribute('readonly');
    ta.focus();
    try { ta.setSelectionRange(0, ta.value.length); } catch (err) { ta.select(); }
    var done = false;
    try { done = document.execCommand('copy'); } catch (err) { done = false; }
    ta.setAttribute('readonly', '');
    btn.textContent = done ? 'Copied' : 'Text selected - copy it';
  }
  if (navigator.clipboard && navigator.clipboard.writeText) {
    navigator.clipboard.writeText(ta.value).then(function(){ btn.setAttribute('data-local-copied', 'Copied'); // async: data-local only (issue188) }, manual);
  } else { manual(); }
}

// The export is now a FALLBACK, for a read-only view or a session where the grant is not active. It reads
// the DOM, because the DOM is the record.
function buildExport(){
  var L = ['# Keel obligations review', '', 'Generated ' + STAMP, ''];
  var names = { accept:'ACCEPT', maybe:'NEEDS WORK', reject:'REJECT' };
  ['accept','maybe','reject'].forEach(function(v){
    var got = cards.filter(function(c){ return c.dataset.verdict === v; });
    if (!got.length) return;
    L.push('## ' + names[v] + ' (' + got.length + ')', '');
    got.forEach(function(c){
      var n = c.querySelector('.note'), note = n ? n.value.trim() : '';
      // One line per item: id, title, note. The class is the section heading above, so repeating it
      // on every line was pure noise (their words: duplicate titles, IDs, etc).
      L.push('- `' + c.querySelector('code').textContent + '` ' + c.querySelector('h3').textContent
        + (note ? ' -- ' + note : ''));
  - note: ' + note : ''));
    });
    L.push('');
  });
  var noted = cards.filter(function(c){
    var n = c.querySelector('.note');
    return !c.dataset.verdict && n && n.value.trim(); });
  if (noted.length) {
    L.push('## Notes without a verdict (' + noted.length + ')', '');
    noted.forEach(function(c){
      L.push('- **' + c.querySelector('code').textContent + '** - '
        + c.querySelector('.note').value.trim()); });
    L.push('');
  }
  L.push(judged() || noted.length ? '_' + judged() + ' of ' + cards.length + ' items judged._'
                                  : '_No verdicts recorded yet._');
  document.getElementById('out').value = L.join('
');
  document.getElementById('copy').setAttribute('data-local-copied', 'Copy');
}

// LAST STATEMENT IN THE SCRIPT. If the header does not say `script ok`, everything above threw.
markScriptAlive();
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
            '<header><code>%s</code><span class=vd></span><span class=sv></span></header>'
            '<h3>%s</h3><p class=meta>%s</p>%s'
            '<div class=verdicts role=group aria-label="verdict">'
            '<button data-v=accept>%s</button>'
            '<button data-v=maybe>Needs work</button>'
            '<button data-v=reject>Reject</button></div>'
            '<input class=note type=text placeholder="why, or what to change&hellip;" aria-label="note">'
            '</article>' % (
                esc(i["uid"]), i["sig"], key, esc(i["name"]), esc(i["title"]), esc(i["meta"]),
                ('<p class=body>%s</p>' % esc(i["body"])) if i["body"] else "",
                # A DECISION IS SIGNED, NOT VOTED ON. `method=confirmation` records a HUMAN's word and
                # an AI cannot supply it, so the verb on an acceptance card says SIGN rather than
                # Accept - the same gesture on a finding means "I agree with the disposition", and
                # conflating the two is how an instruction gets read as a signature.
                "Sign" if key == "acceptance" else "Accept",
            )
            for i in rows
        ) or '<p class=empty>Nothing outstanding in this class.</p>'
        opened = "true" if rows else "false"
        sections.append(
            '<section class=grp data-local-open="%s" style="--h:%s">'
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
        + '  <p class=how>Tap a class to open it, then give each item a verdict and a note. '
          '<b>Your taps save themselves and reach Claude &mdash; there is nothing to copy.</b></p>'
        + '  <p class=live id=livenote>Live document: every verdict and note is appended as you, '
          'the moment you make it.</p>'
        + '<artifact-local><div id=dbglog></div></artifact-local>'
        + '  <p class=ro>This view is <b>read-only</b>, so nothing you change here is saved. Use '
          '<b>Copy for Claude</b> at the bottom and paste the text back instead.</p>'
        + '</header>\n'
        + "".join(sections)
        + "\n</div>\n"
        # The count is RENDERED HERE, not written by script at load (issue188): a load-time DOM
        # write switches the live-doc sync region off and every button stops saving.
        + ('<div class=bar><span class=cnt id=cnt>0 of %d judged</span>' % total)
        + '<button class=ghost id=clear>Reset</button>'
        + '<button class=ghost id=exp>Copy for Claude</button></div>\n'
        + '<dialog id=dlg><textarea id=out readonly></textarea>'
          '<div class=dlgrow><button id=copy>Copy</button>'
          '<button class=ghost id=close>Close</button></div></dialog>\n'
        + '<script>\nvar STAMP = %s;\n%s</script>\n' % (json.dumps("%s / HEAD %s / deck v3" % (generated_at, head_sha)), JS)
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
