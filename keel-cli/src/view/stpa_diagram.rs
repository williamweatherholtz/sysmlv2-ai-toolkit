//! The STPA control-structure diagram, drawn in the binary (D0285, dcStpaDiagramInTheBinary).
//!
//! Input is the JSON of `keel show control-structure` (D0284); output is an `<svg>` fragment styled
//! through CSS variables (`--ctl --fb --proc --ctl-bg --proc-bg --panel --ink --muted`), so the host
//! page owns the palette. Nothing here is authored about a specific project: roles, processes and
//! edges come from the JSON.
//!
//! The five properties the human set on 2026-09-02 hold BY CONSTRUCTION and are unit-tested on the
//! computed [`Layout`], never checked by eye: authority is the vertical axis; control leaves an
//! issuer's bottom-left and enters a process's top-left while feedback leaves a process's top-right
//! and enters a receiver's bottom-right; every edge is vertical / horizontal / vertical and owns its
//! channel, so no two segments share a line; every edge carries a label naming what passes; a
//! vertical crossing another edge's horizontal hops it with a semicircle.
//!
//! This is the port of `.engine/tools/stpa_diagram.py` (the D0221 interim), byte-for-byte on the
//! same JSON. Two consequences shape the code: numbers carry Python's int/float distinction
//! ([`Num`]), because `2176.0` and `2176` are different bytes; and the golden test holds the last
//! output the Python tool produced, so the port is judged against the artefact it replaced.
//!
//! Lints: this module is arithmetic on page coordinates that must round-trip Python's. The int/float
//! casts ARE the port (a coordinate never exceeds 10^5, far inside f64's exact-integer range), the
//! single-letter names are the geometry's own (x, y, w, h), and every index is into a constant table
//! or a map filled from the same list the index came from - the edges are filtered to known roles
//! before any lookup, so a miss is a programming error and should stop the run, not draw a wrong box.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names,
    clippy::indexing_slicing,
    clippy::float_cmp,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::suboptimal_flops,
    clippy::neg_cmp_op_on_partial_ord
)]

use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::path::Path;

use serde_json::Value;

use super::ViewError;

// ---------- geometry constants (the Python tool's, verbatim)

const PROCS: [&str; 6] = ["work", "model", "enforcement-surface", "main-ref", "agent-turn", "deliverable"];
const PW: i64 = 200;
const PH: i64 = 58;
const CW: i64 = 180;
const CH: i64 = 58;
const COL: i64 = 400;
const X0: i64 = 120;
const LEVELS: [&[&str]; 5] = [&["human"], &["remote", "ci", "channel"], &["commit-gate"], &["hooks"], &["console", "agent"]];
const LEVEL_NAMES: [&str; 5] = [
    "human authority",
    "delegated and independent controls",
    "the local commit gate",
    "the turn boundary",
    "the operators: the human's console · the agent",
];
const GAP_OF: [(&str, usize); 8] =
    [("human", 1), ("channel", 0), ("remote", 3), ("ci", 4), ("commit-gate", 3), ("hooks", 4), ("console", 0), ("agent", 1)];
const LH: i64 = 12;
const CHAR_W: f64 = 5.55;
const HOP_R: i64 = 6;

// ---------- a number with Python's int / float distinction

/// A coordinate as the Python tool computed it.
///
/// An `int` stays an int through `+ - *` with ints and prints without a decimal point; any operation
/// touching a float, and every `/`, yields a float that prints as Python's `repr` (shortest
/// round-trip, always with a decimal point).
#[derive(Clone, Copy, Debug)]
pub enum Num {
    I(i64),
    F(f64),
}

impl Num {
    const fn f(self) -> f64 {
        match self {
            Self::I(i) => i as f64,
            Self::F(x) => x,
        }
    }
    /// Python's `int()`: truncation toward zero.
    const fn trunc(self) -> i64 {
        match self {
            Self::I(i) => i,
            Self::F(x) => x as i64,
        }
    }
    fn lt(self, o: Self) -> bool {
        self.f() < o.f()
    }
    fn min(self, o: Self) -> Self {
        if o.lt(self) {
            o
        } else {
            self
        }
    }
    fn max(self, o: Self) -> Self {
        if self.lt(o) {
            o
        } else {
            self
        }
    }
    fn same(self, o: Self) -> bool {
        self.f() == o.f()
    }
}

impl From<i64> for Num {
    fn from(i: i64) -> Self {
        Self::I(i)
    }
}

macro_rules! num_op {
    ($tr:ident, $m:ident, $op:tt) => {
        impl $tr for Num {
            type Output = Self;
            fn $m(self, o: Self) -> Self {
                match (self, o) {
                    (Self::I(a), Self::I(b)) => Self::I(a $op b),
                    (a, b) => Self::F(a.f() $op b.f()),
                }
            }
        }
        impl $tr<i64> for Num {
            type Output = Self;
            fn $m(self, o: i64) -> Self {
                self $op Self::I(o)
            }
        }
    };
}
num_op!(Add, add, +);
num_op!(Sub, sub, -);
num_op!(Mul, mul, *);

impl Div<i64> for Num {
    type Output = Self;
    /// Python's `/` is true division: the result is a float even for two ints.
    fn div(self, o: i64) -> Self {
        Self::F(self.f() / o as f64)
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I(i) => write!(f, "{i}"),
            Self::F(x) => {
                // Rust's Debug for f64 is the same shortest-round-trip text Python's repr prints, with a
                // decimal point kept on integral values - inside the range where neither switches to an
                // exponent. Coordinates here are 0..10^5; the assertion names the assumption.
                debug_assert!(*x == 0.0 || (x.abs() >= 1e-4 && x.abs() < 1e16_f64), "coordinate {x} outside the repr-equivalent range");
                write!(f, "{x:?}")
            }
        }
    }
}

// ---------- the layout

/// One control-action or feedback edge.
///
/// Its endpoints (roles), the label lines naming what passes, and once laid out its channel `cy`
/// and its two verticals (`x_from` at the source box, `x_to` at the target box).
#[derive(Clone, Debug)]
pub struct Edge {
    /// Control: the issuing controller. Feedback: the sensed process.
    pub from: String,
    /// Control: the process acted on. Feedback: the receiving controller.
    pub to: String,
    pub lines: Vec<String>,
    pub cy: Num,
    pub x_from: Num,
    pub x_to: Num,
}

/// A drawn segment: vertical or horizontal, control or feedback, owned by one edge.
#[derive(Clone, Copy, Debug)]
pub struct Seg {
    pub vertical: bool,
    pub ctl: bool,
    pub x1: Num,
    pub y1: Num,
    pub x2: Num,
    pub y2: Num,
    /// Index into the edge list: `0..ctl.len()` are control edges, the rest feedback.
    pub edge: usize,
}

/// A box on the page: a controller (in a gap between process columns) or a process (along the bottom).
#[derive(Clone, Debug)]
pub struct BoxAt {
    pub role: String,
    pub x: Num,
    pub y: Num,
    pub w: Num,
    pub h: Num,
    pub is_proc: bool,
    pub what: String,
    pub anchor: Option<String>,
}

/// A label placed on its edge's channel.
#[derive(Clone, Debug)]
pub struct Label {
    pub edge: usize,
    pub x: Num,
    pub y: Num,
    pub w: Num,
    pub h: Num,
    /// `Some(from_x)` when the channel had to be extended to reach the label (a leader from the
    /// channel's far end to the label's left edge).
    pub leader_from: Option<Num>,
}

/// The computed picture, before serialisation.
#[derive(Clone, Debug)]
pub struct Layout {
    pub width: Num,
    pub height: Num,
    pub row_y: Vec<Num>,
    pub proc_y: Num,
    pub boxes: Vec<BoxAt>,
    pub ctl: Vec<Edge>,
    pub fb: Vec<Edge>,
    pub segs: Vec<Seg>,
    pub labels: Vec<Label>,
    absent: Vec<(String, String, String)>,
}

impl Layout {
    /// The edge a segment or label belongs to.
    #[must_use]
    pub fn edge(&self, idx: usize) -> &Edge {
        if idx < self.ctl.len() {
            &self.ctl[idx]
        } else {
            &self.fb[idx - self.ctl.len()]
        }
    }

    /// The horizontals of OTHER edges a vertical at `x` between `y1` and `y2` crosses, ascending.
    #[must_use]
    pub fn crossings(&self, x: Num, y1: Num, y2: Num, own: usize) -> Vec<Num> {
        let (lo, hi) = (y1.min(y2), y2.max(y1));
        let mut out: Vec<Num> = self
            .segs
            .iter()
            .filter(|s| !s.vertical && s.edge != own)
            .filter(|s| s.x1.min(s.x2).lt(x) && x.lt(s.x1.max(s.x2)) && lo.lt(s.y1) && s.y1.lt(hi))
            .map(|s| s.y1)
            .collect();
        out.sort_by(|a, b| a.f().total_cmp(&b.f()));
        out
    }
}

fn s(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}

fn chars_from(text: &str, n: usize) -> String {
    text.chars().skip(n).collect()
}

fn chars_upto(text: &str, n: usize) -> String {
    text.chars().take(n).collect()
}

fn clen(text: &str) -> usize {
    text.chars().count()
}

/// `re.match(r"keel (show )?([a-z-]+)", title)`: the command a title names, with its `show ` kept.
fn keel_verb(title: &str, allow_show: bool) -> Option<String> {
    let rest = title.strip_prefix("keel ")?;
    let (prefix, rest) = match rest.strip_prefix("show ") {
        Some(r) if allow_show => ("show ", r),
        _ => ("", rest),
    };
    let word: String = rest.chars().take_while(|c| c.is_ascii_lowercase() || *c == '-').collect();
    if word.is_empty() {
        return None;
    }
    Some(format!("{prefix}{word}"))
}

/// Python's `html.escape(text)` with `quote=True`.
fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            c => out.push(c),
        }
    }
    out
}

/// Pack comma-separated items into lines no wider than `width`, each starting with `indent`.
fn wrap(items: &[String], width: usize, indent: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut row = indent.to_string();
    for c in items {
        if clen(&row) + clen(c) + 2 > width {
            lines.push(row.trim_end_matches([',', ' ']).to_string());
            row = indent.to_string();
        }
        row.push_str(c);
        row.push_str(", ");
    }
    lines.push(row.trim_end_matches([',', ' ']).to_string());
    lines
}

fn label_size(lines: &[String]) -> (Num, Num) {
    let longest = lines.iter().map(|l| clen(l)).max().unwrap_or(0);
    (Num::F(longest as f64 * CHAR_W + 14.0), Num::I(LH * lines.len() as i64 + 10))
}

/// Fragments grouped by kind, insertion-ordered like the Python `defaultdict(list)`.
fn grouped(frags: &[(String, String)]) -> Vec<(String, Vec<String>)> {
    let mut k: Vec<(String, Vec<String>)> = Vec::new();
    for (kind, v) in frags {
        match k.iter_mut().find(|(kk, _)| kk == kind) {
            Some((_, vs)) => vs.push(v.clone()),
            None => k.push((kind.clone(), vec![v.clone()])),
        }
    }
    k
}

fn of<'a>(k: &'a [(String, Vec<String>)], kind: &str) -> &'a [String] {
    k.iter().find(|(kk, _)| kk == kind).map_or(&[], |(_, vs)| vs.as_slice())
}

fn ctl_label(issuer: &str, proc: &str, frags: &[(String, String)]) -> Vec<String> {
    let k = grouped(frags);
    let owned = |v: &[&str]| v.iter().map(|x| (*x).to_string()).collect::<Vec<_>>();
    if issuer == "human" {
        if proc == "work" {
            return owned(&["DIRECTION, as prose: chat · a Statement recorded verbatim ·", "a direction Decision  (no receiving control parses it)"]);
        }
        if proc == "model" {
            let mut l = owned(&["ACCEPTANCE {decision, by, date, note, SHA}", "CONFIRMATION + quote receipt · OVERRIDE 'reject <why>'"]);
            if of(&k, "other").iter().any(|v| v == "humanDecidesOnChannel") {
                l.push("CHANNEL JUDGMENT: a comment by a declared login".to_string());
            }
            return l;
        }
    }
    let mut l: Vec<String> = Vec::new();
    let hooks = of(&k, "hook");
    if !hooks.is_empty() {
        l.push("VERDICT JSON {block | deny | allow, reason}, per event:".to_string());
        l.extend(hooks.iter().map(|v| format!("  {v}")));
    }
    let githooks = of(&k, "githook");
    if !githooks.is_empty() {
        l.push("REFUSAL (exit ≠ 0) unless green — what each hook runs:".to_string());
        l.extend(githooks.iter().map(|v| format!("  {}", chars_upto(v, 60))));
    }
    let workflows = of(&k, "workflow");
    if !workflows.is_empty() {
        l.push(
            if issuer == "ci" { "RED / GREEN CHECK on the pushed range — steps:" } else { "ACCEPTANCE EVENT by delegation (auto-accept, D0207);" }
                .to_string(),
        );
        if issuer != "ci" {
            l.push("accept / reject recorded for a declared login — steps:".to_string());
        }
        for v in workflows {
            let (name, rest) = v.split_once(": ").unwrap_or((v, ""));
            let items: Vec<String> = rest.split(", ").map(str::to_string).collect();
            l.extend(wrap(&items, 58, &format!("  {name}: ")).into_iter().take(2));
        }
    }
    let cmds = of(&k, "cmd");
    if !cmds.is_empty() {
        let head = match proc {
            "model" => "AUTHORED FACTS · typed edges · AI-judged verdicts, via:",
            "main-ref" => "COMMITS + PUSHES (merge, never rebase), via:",
            "enforcement-surface" => "UNIT + SURFACE CHANGES (signed Decision required), via:",
            "work" => "A CLAIM on an item, via:",
            _ => "WRITES via:",
        };
        l.push(head.to_string());
        l.extend(wrap(cmds, 46, "  "));
    }
    for v in of(&k, "other") {
        match v.as_str() {
            "remoteRefusesRewrite" => l.push("REJECTED PUSH: force-push and deletion refused".to_string()),
            "consoleApprovesWrite" => l.extend(owned(&["APPROVAL of an ask-tier write", "{path, requesting run, approver} → an obligation record"])),
            "agentEditsDeliverable" => l.extend(owned(&[
                "SOURCE EDITS with the harness's own Write / Edit / Bash",
                "(no keel command mediates; drift makes done work suspect after)",
            ])),
            _ => {}
        }
    }
    l
}

fn fb_label(frags: &[(String, String)]) -> Vec<String> {
    let k = grouped(frags);
    let mut l: Vec<String> = Vec::new();
    let reads = of(&k, "read");
    if !reads.is_empty() {
        let lenses: Vec<String> = reads.iter().filter(|r| r.starts_with("show ")).map(|r| chars_from(r, 5)).collect();
        let verbs: Vec<String> = reads.iter().filter(|r| !r.starts_with("show ")).cloned().collect();
        l.push("COMPUTED STATE, via read commands:".to_string());
        l.extend(wrap(&verbs, 46, "  "));
        if !lenses.is_empty() {
            let shown: Vec<String> = lenses.iter().take(4).cloned().collect();
            l.push(format!("  + show <lens> ×{}: {}, …", lenses.len(), shown.join(", ")));
        }
    }
    let status = of(&k, "status");
    if !status.is_empty() {
        l.push(format!("CI RUN STATUS red / green: {}", status.join(", ")));
        l.push("  by email, or `gh run list` if the agent looks (D0266)".to_string());
    }
    for v in of(&k, "other") {
        l.push(match v.as_str() {
            "consoleLenses" => "CONSOLE LENSES + the approve queue (127.0.0.1:7777)".to_string(),
            "deliverableDrift" => "MANIFEST DRIFT → done work computes as suspect".to_string(),
            other => other.to_string(),
        });
    }
    l
}

/// Group actions / feedback by (from, to) in first-seen order, each carrying its typed fragments.
fn collect_edges(rows: &[Value], from_key: &str, to_key: &str, frag: impl Fn(&Value) -> (String, String)) -> Vec<((String, String), Vec<(String, String)>)> {
    let mut out: Vec<((String, String), Vec<(String, String)>)> = Vec::new();
    for a in rows {
        let key = (s(a, from_key), s(a, to_key));
        let f = frag(a);
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => v.push(f),
            None => out.push((key, vec![f])),
        }
    }
    out
}

fn ctl_fragment(a: &Value) -> (String, String) {
    let n = s(a, "name");
    let title = s(a, "title");
    if n.starts_with("cmd") {
        ("cmd".to_string(), keel_verb(&title, false).unwrap_or_else(|| n.clone()))
    } else if n.starts_with("hook") {
        let kind = title.split_once(": ").map_or("", |(_, k)| k);
        ("hook".to_string(), format!("{} → {kind}", chars_from(&n, 4)))
    } else if n.starts_with("githook") {
        ("githook".to_string(), format!("{}: {}", chars_from(&n, 7), s(a, "data").replace("keel ", "")))
    } else if n.starts_with("workflow") {
        let data = s(a, "data");
        let steps = data.split_once("steps: ").map_or("", |(_, r)| r);
        let steps = steps.split("; runs:").next().unwrap_or("");
        let steps: Vec<String> = steps.split(" | ").map(|st| st.split(" (").next().unwrap_or("").trim().to_string()).collect();
        ("workflow".to_string(), format!("{}: {}", chars_from(&n, 8), steps.join(", ")))
    } else {
        ("other".to_string(), n)
    }
}

fn fb_fragment(f: &Value) -> (String, String) {
    let n = s(f, "name");
    if n.starts_with("read") {
        ("read".to_string(), keel_verb(&s(f, "title"), true).unwrap_or_else(|| n.clone()))
    } else if n.starts_with("status") {
        ("status".to_string(), chars_from(&n, 6))
    } else {
        ("other".to_string(), n)
    }
}

fn arr<'a>(d: &'a Value, key: &str) -> &'a [Value] {
    d.get(key).and_then(Value::as_array).map_or(&[], Vec::as_slice)
}

/// Lay the structure out.
///
/// # Errors
/// When the JSON is not a control structure (no `controllers` array).
pub fn layout(d: &Value) -> Result<Layout, String> {
    if d.get("controllers").and_then(Value::as_array).is_none() {
        return Err("not a control-structure JSON: no `controllers` array".to_string());
    }
    let ctl_info: HashMap<String, &Value> = arr(d, "controllers").iter().map(|c| (s(c, "role"), c)).collect();
    let proc_info: HashMap<String, &Value> = arr(d, "processes").iter().map(|p| (s(p, "role"), p)).collect();
    let proc_x: HashMap<&str, Num> = PROCS.iter().enumerate().map(|(i, p)| (*p, Num::I(X0 + i as i64 * COL))).collect();
    let gap_x: Vec<Num> = (0..PROCS.len() - 1).map(|i| Num::I(X0 + PW + i as i64 * COL) + (Num::I(COL - PW - CW) / 2)).collect();
    let width = Num::I(X0 + (PROCS.len() as i64 - 1) * COL + PW + 120);
    // A role the project does not have (absentRoles - nothing wires it) is drawn nowhere; the legend
    // names it with what would wire it, so the diagram does not silently lose a controller either.
    let levels: Vec<Vec<&str>> = LEVELS.iter().map(|lv| lv.iter().copied().filter(|r| ctl_info.contains_key(*r)).collect()).collect();
    let gap_of: Vec<(&str, usize)> = GAP_OF.iter().copied().filter(|(r, _)| ctl_info.contains_key(*r)).collect();
    let absent: Vec<(String, String, String)> = arr(d, "absentRoles").iter().map(|a| (s(a, "role"), s(a, "what"), s(a, "wiredBy"))).collect();

    // ---------- edges as records, with their labels
    let proc_index = |p: &str| PROCS.iter().position(|q| *q == p);
    let mut ctl: Vec<Edge> = Vec::new();
    for ((issuer, proc), frags) in collect_edges(arr(d, "actions"), "issuedBy", "actsOn", ctl_fragment) {
        let lines = ctl_label(&issuer, &proc, &frags);
        // An edge whose ends the picture cannot place (the Python tool raised here) is left out.
        if !lines.is_empty() && gap_of.iter().any(|(r, _)| *r == issuer) && proc_index(&proc).is_some() {
            ctl.push(Edge { from: issuer, to: proc, lines, cy: Num::I(0), x_from: Num::I(0), x_to: Num::I(0) });
        }
    }
    let mut fb: Vec<Edge> = Vec::new();
    for ((proc, recv), frags) in collect_edges(arr(d, "feedback"), "sensedFrom", "reportsTo", fb_fragment) {
        let lines = fb_label(&frags);
        if !lines.is_empty() && gap_of.iter().any(|(r, _)| *r == recv) && proc_index(&proc).is_some() {
            fb.push(Edge { from: proc, to: recv, lines, cy: Num::I(0), x_from: Num::I(0), x_to: Num::I(0) });
        }
    }

    // ---------- vertical layout: row of boxes, then that row's channel band (one channel per edge)
    let mut pos: HashMap<String, (Num, Num, Num, Num)> = HashMap::new();
    let mut row_y: Vec<Num> = Vec::new();
    let mut y = Num::I(60);
    for roles in &levels {
        row_y.push(y);
        for r in roles {
            let gap = gap_of.iter().find(|(rr, _)| rr == r).map_or(0, |(_, g)| *g);
            pos.insert((*r).to_string(), (gap_x[gap], y, Num::I(CW), Num::I(CH)));
        }
        y = y + (CH + 26);
        // channels for this row: control edges (sorted by target column, left to right) then feedback edges
        let mut es: Vec<usize> = (0..ctl.len()).filter(|i| roles.contains(&ctl[*i].from.as_str())).collect();
        es.sort_by_key(|i| proc_index(&ctl[*i].to));
        let mut fs: Vec<usize> = (0..fb.len()).filter(|i| roles.contains(&fb[*i].to.as_str())).collect();
        fs.sort_by_key(|i| proc_index(&fb[*i].from));
        for (is_fb, i) in es.iter().map(|i| (false, *i)).chain(fs.iter().map(|i| (true, *i))) {
            let e = if is_fb { &mut fb[i] } else { &mut ctl[i] };
            let (_, h) = label_size(&e.lines);
            y = y + (h / 2 + 4);
            e.cy = y;
            y = y + (h / 2 + 10);
        }
        y = y + 30;
    }
    let proc_y = y + 20;
    for p in PROCS {
        pos.insert(p.to_string(), (proc_x[p], proc_y, Num::I(PW), Num::I(PH)));
    }
    let height = proc_y + PH + 40;

    // ---------- horizontal x assignment: trunk x per issuer slot, drop x per process slot (unique everywhere)
    let count = |edges: &[Edge], pick: fn(&Edge) -> &String| {
        let mut m: HashMap<String, i64> = HashMap::new();
        for e in edges {
            *m.entry(pick(e).clone()).or_insert(0) += 1;
        }
        m
    };
    let n_in = count(&ctl, |e| &e.to);
    let n_fb_up = count(&fb, |e| &e.from);
    let mut out_ct: HashMap<String, i64> = HashMap::new();
    let mut in_ct: HashMap<String, i64> = HashMap::new();
    let mut order: Vec<usize> = (0..ctl.len()).collect();
    order.sort_by(|a, b| ctl[*a].cy.f().total_cmp(&ctl[*b].cy.f()));
    for i in order {
        let e = &mut ctl[i];
        let (x, _, _, _) = pos[&e.from];
        let k = *out_ct.entry(e.from.clone()).or_insert(0);
        out_ct.insert(e.from.clone(), k + 1);
        e.x_from = x + 14 + Num::I(k * 14); // trunk x: left part of the issuer's bottom
        let px = proc_x[e.to.as_str()];
        let j = *in_ct.entry(e.to.clone()).or_insert(0);
        in_ct.insert(e.to.clone(), j + 1);
        let n = n_in.get(&e.to).copied().unwrap_or(1).max(1);
        // drop x: left half of the process top
        e.x_to = px + 14 + Num::F(j as f64 * (PW as f64 * 0.45 / n as f64));
    }
    let mut fb_in_ct: HashMap<String, i64> = HashMap::new();
    let mut fb_up_ct: HashMap<String, i64> = HashMap::new();
    let mut order: Vec<usize> = (0..fb.len()).collect();
    order.sort_by(|a, b| fb[*a].cy.f().total_cmp(&fb[*b].cy.f()));
    for i in order {
        let f = &mut fb[i];
        let (x, _, w, _) = pos[&f.to];
        let k = *fb_in_ct.entry(f.to.clone()).or_insert(0);
        fb_in_ct.insert(f.to.clone(), k + 1);
        f.x_to = x + w - 14 - Num::I(k * 14); // riser x into the receiver's bottom-right
        let px = proc_x[f.from.as_str()];
        let j = *fb_up_ct.entry(f.from.clone()).or_insert(0);
        fb_up_ct.insert(f.from.clone(), j + 1);
        let n = n_fb_up.get(&f.from).copied().unwrap_or(1).max(1);
        // rise x: right half of the process top
        f.x_from = px + PW - 14 - Num::F(j as f64 * (PW as f64 * 0.4 / n as f64));
    }

    // ---------- segments
    let mut segs: Vec<Seg> = Vec::new();
    for (i, e) in ctl.iter().enumerate() {
        let (_, yy, _, h) = pos[&e.from];
        segs.push(Seg { vertical: true, ctl: true, x1: e.x_from, y1: yy + h, x2: e.x_from, y2: e.cy, edge: i });
        segs.push(Seg { vertical: false, ctl: true, x1: e.x_from, y1: e.cy, x2: e.x_to, y2: e.cy, edge: i });
        segs.push(Seg { vertical: true, ctl: true, x1: e.x_to, y1: e.cy, x2: e.x_to, y2: proc_y, edge: i });
    }
    for (j, f) in fb.iter().enumerate() {
        let i = ctl.len() + j;
        let (_, yy, _, h) = pos[&f.to];
        segs.push(Seg { vertical: true, ctl: false, x1: f.x_from, y1: proc_y, x2: f.x_from, y2: f.cy, edge: i });
        segs.push(Seg { vertical: false, ctl: false, x1: f.x_from, y1: f.cy, x2: f.x_to, y2: f.cy, edge: i });
        segs.push(Seg { vertical: true, ctl: false, x1: f.x_to, y1: f.cy, x2: f.x_to, y2: yy + h, edge: i });
    }

    // ---------- boxes
    let mut boxes: Vec<BoxAt> = Vec::new();
    let anchor_of = |info: &Value| info.get("anchor").and_then(Value::as_str).filter(|a| !a.is_empty()).map(str::to_string);
    for (r, _) in &gap_of {
        let info = ctl_info[*r];
        let (x, y, w, h) = pos[*r];
        boxes.push(BoxAt { role: (*r).to_string(), x, y, w, h, is_proc: false, what: s(info, "what"), anchor: anchor_of(info) });
    }
    for p in PROCS {
        let (x, y, w, h) = pos[p];
        let (what, anchor) = proc_info.get(p).map_or((String::new(), None), |info| (s(info, "what"), anchor_of(info)));
        boxes.push(BoxAt { role: p.to_string(), x, y, w, h, is_proc: true, what, anchor });
    }

    let mut lay = Layout { width, height, row_y, proc_y, boxes, ctl, fb, segs, labels: Vec::new(), absent };

    // ---------- labels: on the channel, where no other edge's vertical runs behind
    let mut labels = Vec::new();
    for idx in 0..lay.ctl.len() + lay.fb.len() {
        let e = lay.edge(idx);
        let (w, h) = label_size(&e.lines);
        let (x1, x2) = (e.x_from, e.x_to);
        let (lo, hi) = (x1.min(x2), x1.max(x2));
        let (ilo, ihi, iw) = (lo.trunc(), hi.trunc(), w.trunc());
        // Sit ON the channel at the first position along it that no other edge's vertical passes behind;
        // the channel may be extended past its far end when the span is shorter than the label.
        let inside = (ilo + 14..(ilo + 15).max(ihi - iw - 8)).step_by(8);
        let beyond = ((ilo + 14).max(ihi - iw - 8)..(ihi + 260).min((lay.width - w - 20).trunc())).step_by(8);
        let cy = e.cy;
        let verticals_through = |lx: i64| -> i64 {
            // Other edges' vertical segments that would pass behind a label box at (lx, cy-h/2, w, h).
            lay.segs
                .iter()
                .filter(|sg| sg.vertical && sg.edge != idx)
                .filter(|sg| {
                    let vx = sg.x1.f();
                    ((lx - 3) as f64) < vx && vx < (lx as f64) + w.f() + 3.0 && sg.y1.min(sg.y2).lt(cy + h / 2) && (cy - h / 2).lt(sg.y1.max(sg.y2))
                })
                .count() as i64
        };
        // fewest verticals behind the box; then nearest the channel's own span; a crossing behind a label
        // near its line beats a clean label floating far from it
        let mut best: Option<(i64, (i64, f64))> = None;
        for cx in inside.chain(beyond) {
            let key = (verticals_through(cx) + i64::from(!((cx as f64) <= (hi - w).f())), (cx as f64 - (lo + 14).f()).abs());
            let better = match &best {
                None => true,
                Some((_, bk)) => key.0 < bk.0 || (key.0 == bk.0 && key.1 < bk.1),
            };
            if better {
                best = Some((cx, key));
            }
        }
        let lx = Num::I(best.map_or(ilo + 14, |(cx, _)| cx));
        let ly = cy - h / 2;
        let leader_from = if hi.lt(lx) { Some(hi) } else { None };
        labels.push(Label { edge: idx, x: lx, y: ly, w, h, leader_from });
    }
    lay.labels = labels;
    Ok(lay)
}

// ---------- serialisation

/// A vertical from y1 to y2 with a semicircular hop over every horizontal it crosses.
fn vpath(lay: &Layout, x: Num, y1: Num, y2: Num, own: usize) -> String {
    let down = y1.lt(y2);
    let mut ys = lay.crossings(x, y1, y2, own);
    if !down {
        ys.reverse();
    }
    let r = Num::I(HOP_R);
    let mut p = vec![format!("M{x},{y1}")];
    for cy in ys {
        if down {
            p.push(format!("L{x},{} A{r},{r} 0 0 1 {x},{}", cy - r, cy + r));
        } else {
            p.push(format!("L{x},{} A{r},{r} 0 0 0 {x},{}", cy + r, cy - r));
        }
    }
    p.push(format!("L{x},{y2}"));
    p.join(" ")
}

/// Serialise a layout as the `<svg>` fragment.
#[must_use]
pub fn svg(lay: &Layout) -> String {
    let w = lay.width;
    let h = lay.height;
    let top = lay.row_y[0];
    let proc_y = lay.proc_y;
    let mut out: Vec<String> = vec![
        format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" style=\"font-family:IBM Plex Sans,Segoe UI,Helvetica,Arial,sans-serif\">"),
        concat!(
            "<defs><marker id=\"down\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto\"><path d=\"M0,0 L10,5 L0,10 z\" fill=\"var(--ctl)\"/></marker>",
            "<marker id=\"up\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto\"><path d=\"M0,0 L10,5 L0,10 z\" fill=\"var(--fb)\"/></marker></defs>"
        )
        .to_string(),
    ];
    out.push(format!("<line x1=\"30\" y1=\"{top}\" x2=\"30\" y2=\"{proc_y}\" stroke=\"var(--muted)\" stroke-width=\"1\" stroke-dasharray=\"2 5\"/>"));
    out.push(format!("<polygon points=\"30,{} 25,{} 35,{}\" fill=\"var(--muted)\"/>", top - 10, top + 2, top + 2));
    out.push(format!(
        "<text transform=\"translate(16,{}) rotate(-90)\" text-anchor=\"middle\" font-size=\"11\" letter-spacing=\"2\" fill=\"var(--muted)\">AUTHORITY</text>",
        (top + proc_y) / 2
    ));
    out.push(format!(
        "<text x=\"{}\" y=\"{top}\" text-anchor=\"end\" font-size=\"11\" fill=\"var(--ctl)\">▼  control action — solid; leaves the issuer bottom-left, enters the process top-left</text>",
        w - 30
    ));
    out.push(format!(
        "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"11\" fill=\"var(--fb)\">▲  feedback — dashed; leaves the process top-right, enters the receiver bottom-right</text>",
        w - 30,
        top + 16
    ));
    out.push(format!(
        "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"11\" fill=\"var(--muted)\">a semicircle is a crossing, not a junction · the label on a line is what passes along it</text>",
        w - 30,
        top + 32
    ));
    for (yy, nm) in lay.row_y.iter().zip(LEVEL_NAMES.iter()) {
        out.push(format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"10\" fill=\"var(--muted)\" font-style=\"italic\">{}</text>",
            w - 30,
            *yy + (CH - 4),
            esc(nm)
        ));
    }

    // lines (verticals with hops), then boxes, then labels on top
    for sg in &lay.segs {
        let cls = if sg.ctl { "ctl" } else { "fb" };
        let dash = if sg.ctl { "" } else { " stroke-dasharray=\"6 4\"" };
        if sg.vertical {
            let e = lay.edge(sg.edge);
            let end = if sg.ctl && sg.y2.same(proc_y) {
                " marker-end=\"url(#down)\""
            } else if !sg.ctl && !sg.y2.same(e.cy) {
                " marker-end=\"url(#up)\""
            } else {
                ""
            };
            out.push(format!(
                "<path d=\"{}\" fill=\"none\" stroke=\"var(--{cls})\" stroke-width=\"1.4\"{dash}{end}/>",
                vpath(lay, sg.x1, sg.y1, sg.y2, sg.edge)
            ));
        } else {
            out.push(format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"var(--{cls})\" stroke-width=\"1.4\"{dash}/>",
                sg.x1, sg.y1, sg.x2, sg.y2
            ));
        }
    }

    for b in &lay.boxes {
        let (fill, stroke) = if b.is_proc { ("var(--proc-bg)", "var(--proc)") } else { ("var(--ctl-bg)", "var(--ctl)") };
        let mut lines = vec![
            format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"4\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"1.6\"/>", b.x, b.y, b.w, b.h),
            format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"14\" font-weight=\"600\" fill=\"var(--ink)\">{}</text>",
                b.x + b.w / 2,
                b.y + 23,
                esc(&b.role)
            ),
            format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"10\" fill=\"var(--muted)\">{}</text>",
                b.x + b.w / 2,
                b.y + 41,
                esc(&chars_upto(&b.what, 40))
            ),
        ];
        if let Some(a) = &b.anchor {
            lines.push(format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"8.5\" fill=\"var(--muted)\" font-family=\"IBM Plex Mono,Consolas,monospace\">{}</text>",
                b.x + b.w - 6,
                b.y + b.h - 5,
                esc(a)
            ));
        }
        out.push(lines.join("\n"));
    }

    for l in &lay.labels {
        let e = lay.edge(l.edge);
        let is_ctl = l.edge < lay.ctl.len();
        let cls = if is_ctl { "ctl" } else { "fb" };
        let mut lines: Vec<String> = Vec::new();
        if let Some(from) = l.leader_from {
            // the label sits past the channel's far end: extend the channel to it as a leader
            lines.push(format!(
                "<line x1=\"{from}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"var(--{cls})\" stroke-width=\"1.4\"{}/>",
                e.cy,
                l.x,
                e.cy,
                if is_ctl { "" } else { " stroke-dasharray=\"6 4\"" }
            ));
        }
        lines.push(format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"3\" fill=\"var(--panel)\" stroke=\"var(--{cls})\" stroke-width=\"0.9\"{}/>",
            l.x,
            l.y,
            l.w,
            l.h,
            if is_ctl { "" } else { " stroke-dasharray=\"4 3\"" }
        ));
        for (i, text) in e.lines.iter().enumerate() {
            lines.push(format!(
                "<text x=\"{}\" y=\"{}\" font-size=\"9.6\" font-family=\"IBM Plex Mono,Consolas,monospace\" font-weight=\"{}\" fill=\"var(--ink)\" xml:space=\"preserve\">{}</text>",
                l.x + 7,
                l.y + 13 + Num::I(i as i64 * LH),
                if i == 0 { "600" } else { "400" },
                esc(text)
            ));
        }
        out.push(lines.join("\n"));
    }
    for (i, (role, what, wired_by)) in lay.absent.iter().enumerate() {
        out.push(format!(
            "<text x=\"{X0}\" y=\"{}\" font-size=\"9.6\" font-family=\"IBM Plex Mono,Consolas,monospace\" fill=\"var(--muted)\">not drawn - {}: {}; this project has none (wired by {})</text>",
            h - 10 - Num::I(i as i64 * LH),
            esc(role),
            esc(what),
            esc(wired_by)
        ));
    }
    out.push("</svg>".to_string());
    out.join("\n")
}

/// Render the `<svg>` fragment from the JSON text of `keel show control-structure`.
///
/// # Errors
/// When the text is not JSON, or not a control structure.
pub fn render_json(json: &str) -> Result<String, String> {
    let d: Value = serde_json::from_str(json).map_err(|e| format!("control-structure JSON does not parse: {e}"))?;
    Ok(svg(&layout(&d)?))
}

/// `keel show control-structure --svg`: compute the structure for `root` and draw it.
///
/// # Errors
/// When the structure cannot be computed for `root`.
pub fn control_structure_svg(root: &Path) -> Result<String, ViewError> {
    let json = super::control_structure::control_structure(root)?;
    render_json(&json).map_err(|e| ViewError::Track("control-structure".to_string(), e))
}

/// `keel render control-structure --mode graph` and the console's `/view/control-structure`.
///
/// The diagram on a page that owns the palette (light and dark), so the SVG's CSS variables resolve.
///
/// # Errors
/// When the structure cannot be computed for `root`.
pub fn control_structure_html(root: &Path) -> Result<String, ViewError> {
    let svg = control_structure_svg(root)?;
    Ok(format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>control structure — STPA step 2</title><style>\
:root{{--ctl:#1d4ed8;--fb:#b45309;--proc:#334155;--ctl-bg:#eff6ff;--proc-bg:#f1f5f9;--panel:#ffffff;--ink:#0f172a;--muted:#64748b;color-scheme:light dark}}\
@media (prefers-color-scheme:dark){{:root{{--ctl:#93c5fd;--fb:#fcd34d;--proc:#cbd5e1;--ctl-bg:#172554;--proc-bg:#1e293b;--panel:#0f172a;--ink:#e2e8f0;--muted:#94a3b8}}}}\
body{{margin:0;background:var(--panel);color:var(--ink);font-family:IBM Plex Sans,Segoe UI,Helvetica,Arial,sans-serif}}\
.wrap{{overflow:auto;padding:12px}}svg{{max-width:none}}\
p{{margin:8px 12px;font-size:12px;color:var(--muted)}}\
</style></head><body><p>STPA step 2 for this project's own workflow, computed by <code>keel show control-structure</code> and drawn by the binary (D0284, D0285). Authority descends; control leaves bottom-left and goes down solid; feedback leaves top-right and comes up dashed; a semicircle is a crossing, not a junction.</p><div class=\"wrap\">{svg}</div></body></html>"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_JSON: &str = include_str!("../../tests/fixtures/stpa/control-structure.json");
    const FIXTURE_SVG: &str = include_str!("../../tests/fixtures/stpa/control-structure.svg");

    fn fixture() -> Layout {
        layout(&serde_json::from_str(FIXTURE_JSON).expect("fixture parses")).expect("fixture lays out")
    }

    /// The last output the Python tool produced on this JSON, byte for byte (line endings aside: the
    /// fixture is stored LF, the renderer joins with LF).
    #[test]
    fn port_is_byte_equal_to_the_python_tool_on_the_same_json() {
        let got = render_json(FIXTURE_JSON).expect("renders");
        let want = FIXTURE_SVG.replace("\r\n", "\n");
        if got != want {
            let (gl, wl): (Vec<&str>, Vec<&str>) = (got.lines().collect(), want.lines().collect());
            let first = gl.iter().zip(wl.iter()).position(|(a, b)| a != b).unwrap_or_else(|| gl.len().min(wl.len()));
            panic!(
                "diverges at line {} of {} (python {}):\n  rust:   {}\n  python: {}",
                first + 1,
                gl.len(),
                wl.len(),
                gl.get(first).unwrap_or(&"<end>"),
                wl.get(first).unwrap_or(&"<end>")
            );
        }
    }

    #[test]
    fn numbers_print_like_python() {
        assert_eq!(format!("{}", Num::I(2440)), "2440");
        assert_eq!(format!("{}", Num::F(2176.0)), "2176.0");
        assert_eq!(format!("{}", Num::I(60) + (Num::I(58) / 2)), "89.0");
        assert_eq!(format!("{}", Num::I(120) + 14 + Num::F(2.0 * (200.0 * 0.45 / 3.0))), "194.0");
        assert_eq!(format!("{}", Num::F(30.000_000_000_000_004)), "30.000000000000004");
        assert_eq!(Num::F(2.7).trunc(), 2);
    }

    // ---- the five properties (D0285), on the computed layout

    /// Property 3: no two segments of different edges are collinear over a shared span.
    #[test]
    fn no_two_segments_share_a_line() {
        let lay = fixture();
        for (i, a) in lay.segs.iter().enumerate() {
            for b in lay.segs.iter().skip(i + 1) {
                if a.edge == b.edge || a.vertical != b.vertical {
                    continue;
                }
                if a.vertical {
                    let overlap = a.y1.min(a.y2).lt(b.y1.max(b.y2)) && b.y1.min(b.y2).lt(a.y1.max(a.y2));
                    assert!(!(a.x1.same(b.x1) && overlap), "verticals of edges {} and {} share x={} over a common span", a.edge, b.edge, a.x1);
                } else {
                    let overlap = a.x1.min(a.x2).lt(b.x1.max(b.x2)) && b.x1.min(b.x2).lt(a.x1.max(a.x2));
                    assert!(!(a.y1.same(b.y1) && overlap), "horizontals of edges {} and {} share y={} over a common span", a.edge, b.edge, a.y1);
                }
            }
        }
        // and every edge owns its own channel outright
        let mut cys: Vec<f64> = lay.ctl.iter().chain(lay.fb.iter()).map(|e| e.cy.f()).collect();
        let n = cys.len();
        cys.sort_by(f64::total_cmp);
        cys.dedup();
        assert_eq!(cys.len(), n, "two edges share a channel y");
    }

    /// Property 4: every edge carries a label, placed on its own channel, whose first line names what passes.
    #[test]
    fn every_edge_is_labelled_with_what_passes() {
        let lay = fixture();
        assert_eq!(lay.labels.len(), lay.ctl.len() + lay.fb.len());
        for l in &lay.labels {
            let e = lay.edge(l.edge);
            assert!(!e.lines.is_empty() && !e.lines[0].trim().is_empty(), "edge {} -> {} has no label", e.from, e.to);
            assert!(!e.lines[0].contains("an action"), "label names a kind, not what passes: {}", e.lines[0]);
            // the label box is centred on its channel
            assert!((l.y + l.h / 2).same(e.cy), "label of {} -> {} sits off its channel", e.from, e.to);
        }
    }

    /// Property 5: every vertical/horizontal crossing between two edges is a hop in the path, and a hop
    /// is drawn at nothing else. Recomputed here by brute force, independently of `Layout::crossings`.
    #[test]
    fn every_crossing_carries_a_hop() {
        let lay = fixture();
        let mut hops = 0;
        for v in lay.segs.iter().filter(|sg| sg.vertical) {
            let (lo, hi) = (v.y1.f().min(v.y2.f()), v.y1.f().max(v.y2.f()));
            let mut brute: Vec<f64> = lay
                .segs
                .iter()
                .filter(|hseg| !hseg.vertical && hseg.edge != v.edge)
                .filter(|hseg| hseg.x1.f().min(hseg.x2.f()) < v.x1.f() && v.x1.f() < hseg.x1.f().max(hseg.x2.f()) && lo < hseg.y1.f() && hseg.y1.f() < hi)
                .map(|hseg| hseg.y1.f())
                .collect();
            brute.sort_by(f64::total_cmp);
            let path = vpath(&lay, v.x1, v.y1, v.y2, v.edge);
            assert_eq!(path.matches(" A6,6 ").count(), brute.len(), "vertical of edge {} at x={}: hops != crossings", v.edge, v.x1);
            hops += brute.len();
        }
        assert!(hops > 0, "the fixture has crossings to hop");
    }

    /// Property 1: no segment passes through a box (controllers sit in the gaps between process columns).
    #[test]
    fn no_line_passes_through_a_box() {
        let lay = fixture();
        for b in &lay.boxes {
            let (bx1, by1, bx2, by2) = (b.x.f(), b.y.f(), (b.x + b.w).f(), (b.y + b.h).f());
            for sg in &lay.segs {
                let (x1, y1, x2, y2) = (sg.x1.f().min(sg.x2.f()), sg.y1.f().min(sg.y2.f()), sg.x1.f().max(sg.x2.f()), sg.y1.f().max(sg.y2.f()));
                // a strict interior intersection - touching the box's edge is how a segment attaches to it
                let inside = x1 < bx2 && x2 > bx1 && y1 < by2 && y2 > by1;
                assert!(!inside, "segment of edge {} ({},{})-({},{}) crosses box {}", sg.edge, sg.x1, sg.y1, sg.x2, sg.y2, b.role);
            }
        }
    }

    /// Property 2: control leaves the issuer's bottom-left and enters the process's top-left; feedback
    /// leaves the process's top-right and enters the receiver's bottom-right.
    #[test]
    fn control_leaves_bottom_left_and_feedback_enters_bottom_right() {
        let lay = fixture();
        let box_of = |role: &str| lay.boxes.iter().find(|b| b.role == role).expect("box exists");
        for e in &lay.ctl {
            let issuer = box_of(&e.from);
            let proc = box_of(&e.to);
            assert!(e.x_from.f() < (issuer.x + issuer.w / 2).f(), "trunk of {} -> {} is not on the issuer's left half", e.from, e.to);
            assert!(e.x_to.f() < (proc.x + proc.w / 2).f(), "drop of {} -> {} is not on the process's left half", e.from, e.to);
            assert!(proc.y.same(lay.proc_y), "processes sit along the bottom");
            assert!(issuer.y.f() < proc.y.f(), "issuer {} is not above the process {}", e.from, e.to);
        }
        for f in &lay.fb {
            let proc = box_of(&f.from);
            let recv = box_of(&f.to);
            assert!(f.x_from.f() > (proc.x + proc.w / 2).f(), "riser of {} -> {} is not on the process's right half", f.from, f.to);
            assert!(f.x_to.f() > (recv.x + recv.w / 2).f(), "entry of {} -> {} is not on the receiver's right half", f.from, f.to);
        }
        // and the trunks start at the issuer's bottom edge, the risers end at the receiver's bottom edge
        for sg in lay.segs.iter().filter(|sg| sg.vertical) {
            let e = lay.edge(sg.edge);
            if sg.ctl && !sg.y2.same(lay.proc_y) {
                let issuer = box_of(&e.from);
                assert!(sg.y1.same(issuer.y + issuer.h), "control trunk of {} does not leave the issuer's bottom", e.from);
            }
            if !sg.ctl && !sg.y2.same(e.cy) {
                let recv = box_of(&e.to);
                assert!(sg.y2.same(recv.y + recv.h), "feedback of {} does not enter the receiver's bottom", e.to);
            }
        }
    }

    #[test]
    fn an_absent_role_is_named_in_the_legend_and_drawn_nowhere() {
        let lay = fixture();
        let out = svg(&lay);
        assert!(lay.boxes.iter().all(|b| b.role != "channel"), "channel is absent in the fixture and must not be a box");
        assert!(out.contains("not drawn - channel:"), "the legend names the absent role");
    }

    #[test]
    fn a_non_structure_json_is_refused_not_drawn_empty() {
        assert!(render_json("{\"decisions\":[]}").is_err());
        assert!(render_json("not json").is_err());
    }

    #[test]
    fn helpers_match_python() {
        assert_eq!(esc("OVERRIDE 'reject <why>' & \"x\""), "OVERRIDE &#x27;reject &lt;why&gt;&#x27; &amp; &quot;x&quot;");
        assert_eq!(keel_verb("keel show orient", true), Some("show orient".to_string()));
        assert_eq!(keel_verb("keel show orient", false), Some("show".to_string()));
        assert_eq!(keel_verb("keel append-result --file", false), Some("append-result".to_string()));
        assert_eq!(keel_verb("git push", false), None);
        assert_eq!(wrap(&[], 46, "  "), vec![String::new()]);
        assert_eq!(wrap(&["aaaa".to_string(), "bbbb".to_string(), "cc".to_string()], 12, "  "), vec!["  aaaa", "  bbbb, cc"]);
    }
}
