#!/usr/bin/env python3
"""ASI template render wrapper + DoD harness (D0235 A5, hardened per the 2026-08-27 critique).

Usage:
  python scripts/render.py <instance.json> [--out out/NAME.pdf] [--check] [--update-golden]

Steps (in order; any failure stops the run):
  T. toolchain assert  - typst/python/libs AND the .typ cmarker pin must match the manifest
  0. schema validation - instance validated against its docType schema; refuse to render
  C. content contract  - token-level CommonMark gate (issue003: ONE enforcement semantics,
                         not a drifting regex): excluded constructs refused by name; the
                         dollar/hash scan is opaque to code spans, link URLs, and escapes
  G. glyph coverage    - characters outside the vendored fonts refuse by codepoint
  1. compile           - vendored fonts only, pinned creation timestamp, ZERO warnings
  2. field read-back   - driven by the doctype's outputs.json fieldMap (issue005): every
                         readback=text field asserted in extracted text via a comparison
                         tolerant of extraction artifacts (issue004: kern spaces, U+FFFD)
  3. golden diff       - --check/--update-golden only; page PNGs pixel-exact vs golden/<x>/;
                         re-baseline requires double-compile byte-identity first
  L. hex lint          - no color literals in templates (3-8 digit hex, rgb(), luma())
"""
import argparse
import json
import re
import shutil
import subprocess
import sys
import unicodedata
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MANIFEST = json.loads((ROOT / "toolchain-manifest.json").read_text(encoding="utf-8"))

SCHEMA_BY_DOCTYPE = {"Requirements Document": "requirements-doc.schema.json"}
TEMPLATE_BY_DOCTYPE = {"Requirements Document": "templates/requirements-doc/requirements-doc.typ"}
OUTPUTS_BY_DOCTYPE = {"Requirements Document": "templates/requirements-doc/outputs.json"}


def die(msg):
    print(f"FAIL: {msg}", file=sys.stderr)
    sys.exit(1)


def find_typst():
    for cand in (shutil.which("typst"),
                 str(Path.home() / "AppData/Local/Microsoft/WinGet/Links/typst.exe")):
        if cand and Path(cand).exists():
            return cand
    die("typst binary not found")


def assert_toolchain(typst):
    out = subprocess.run([typst, "--version"], capture_output=True, text=True).stdout
    want = MANIFEST["typst"]
    if f"typst {want}" not in out:
        die(f"toolchain pin: typst is '{out.strip()}', manifest pins {want} "
            "(winget may have bumped it - see toolchain-manifest.json)")
    pyv = f"{sys.version_info.major}.{sys.version_info.minor}"
    if pyv != MANIFEST["pythonMajorMinor"]:
        die(f"toolchain pin: python {pyv}, manifest pins {MANIFEST['pythonMajorMinor']}")
    from importlib.metadata import version as pkg_version
    lock = (ROOT / MANIFEST["pipLockfile"]).read_text(encoding="utf-8")
    for name in ("pypdf", "pillow", "jsonschema", "fonttools", "markdown-it-py"):
        ver = pkg_version(name)
        if f"{name}=={ver}" not in lock:
            die(f"toolchain pin: {name} {ver} not in {MANIFEST['pipLockfile']}")
    # The .typ imports must match the manifest's typst package pins (critique finding 7).
    pin_rx = re.compile(r'@preview/(\w[\w-]*):(\S+?)"')
    for f in (ROOT / "templates").rglob("*.typ"):
        for pkg, ver in pin_rx.findall(f.read_text(encoding="utf-8")):
            pinned = MANIFEST.get("typstPackages", {}).get(pkg)
            if pinned != ver:
                die(f"toolchain pin: {f.relative_to(ROOT)} imports @preview/{pkg}:{ver} "
                    f"but the manifest pins {pinned!r}")


def validate_instance(instance_path):
    from jsonschema import Draft202012Validator
    from referencing import Registry, Resource

    data = json.loads(instance_path.read_text(encoding="utf-8"))
    doctype = data.get("meta", {}).get("docType") or die("instance has no meta.docType")
    schema_name = SCHEMA_BY_DOCTYPE.get(doctype) or die(f"no schema for docType '{doctype}'")
    schemas_dir = ROOT / "schemas"
    registry = Registry()
    for f in schemas_dir.glob("*.schema.json"):
        res = Resource.from_contents(json.loads(f.read_text(encoding="utf-8")))
        registry = registry.with_resource(f.name, res)
        registry = registry.with_resource(res.contents.get("$id", f.name), res)
    schema = json.loads((schemas_dir / schema_name).read_text(encoding="utf-8"))
    validator = Draft202012Validator(schema, registry=registry)
    errors = sorted(validator.iter_errors(data), key=lambda e: e.json_path)
    if errors:
        for e in errors[:10]:
            print(f"  schema: {e.json_path}: {e.message}", file=sys.stderr)
        die(f"instance fails {schema_name} ({len(errors)} error(s)) - refusing to render")
    ids = [r["id"] for r in data.get("requirements", [])]
    if len(ids) != len(set(ids)):
        die("duplicate requirement ids: " + ", ".join(sorted({i for i in ids if ids.count(i) > 1})))
    return data, doctype


def _prose_fields(data):
    if "content" in data:
        yield "content", data["content"]
    for r in data.get("requirements", []):
        for k in ("rationale", "notes"):
            if k in r:
                yield f"{r.get('id', '?')}.{k}", r[k]


# Constructs the contract EXCLUDES, detected on the PARSED token stream so detection matches
# CommonMark semantics (setext headings, footnotes, tables without edge pipes, HTML comments
# and closing tags — every leak the critique demonstrated).
_EXCLUDED_TOKEN = {
    "html_block": "raw HTML", "html_inline": "raw HTML",
    "heading_open": "headings", "blockquote_open": "block quotes",
    "table_open": "tables", "image": "images",
}


def _md_parser():
    from markdown_it import MarkdownIt
    from mdit_py_plugins.footnote import footnote_plugin
    return MarkdownIt("commonmark").enable(["table", "strikethrough"]).use(footnote_plugin)


def _walk(tokens):
    for t in tokens:
        yield t
        if t.children:
            yield from _walk(t.children)


def check_content_contract(data):
    md = _md_parser()
    for where, text in _prose_fields(data):
        if not text or not text.strip():
            continue
        for t in _walk(md.parse(text)):
            if t.type in _EXCLUDED_TOKEN:
                die(f"content contract: {_EXCLUDED_TOKEN[t.type]} excluded by "
                    f"CONTENT-FORMAT.md (in {where})")
            if t.type.startswith("footnote"):
                die(f"content contract: footnotes excluded by CONTENT-FORMAT.md (in {where})")
        # Dollar/hash scan (D0236) on a masked source: backslash escapes, code spans, and
        # link destinations are opaque to the scan — matching cmarker's real precedence
        # (issue003's false-refusal cases: `$PATH` in code, $ in URLs, \$ anywhere).
        masked = re.sub(r"\\[!-/:-@\[-`{-~]", "\uE000", text)          # CommonMark escapes
        masked = re.sub(r"(`+)([\s\S]*?)\1", lambda m: "\uE000" * len(m.group(0)), masked)
        masked = re.sub(r"\]\([^)]*\)", "]()", masked)                   # link destinations
        segments = masked.split("$")
        if len(segments) % 2 == 0:
            die(f"content contract (D0236): unbalanced $ math delimiter in {where} - "
                "escape a literal dollar as \\$ (dollars inside `code spans` are fine as-is)")
        for i, seg in enumerate(segments):
            if i % 2 == 1 and "#" in seg:
                die(f"content contract (D0236): # forbidden inside a math segment in {where} "
                    "(the proven eval-injection vector)")


def _instance_strings(data):
    for k, v in data.get("meta", {}).items():
        if k != "$comment":
            yield f"meta.{k}", v
    yield from _prose_fields(data)
    for r in data.get("requirements", []):
        yield f"{r.get('id', '?')}.id", r.get("id", "")
        yield f"{r.get('id', '?')}.statement", r.get("statement", "")


def check_glyph_coverage(data):
    """srCharFidelity: refuse when any character lacks a vendored glyph (tofu is silent)."""
    from fontTools.ttLib import TTFont
    cov = set()
    for f in (ROOT / MANIFEST["fontsDir"]).glob("*.ttf"):
        cov |= set(TTFont(str(f)).getBestCmap().keys())
    problems = []
    for where, text in _instance_strings(data):
        bad = sorted({c for c in unicodedata.normalize("NFC", str(text))
                      if ord(c) > 127 and ord(c) not in cov and not c.isspace()})
        if bad:
            problems.append(f"{where}: " + " ".join(f"U+{ord(c):04X}" for c in bad))
    if problems:
        for p in problems[:15]:
            print(f"  coverage: {p}", file=sys.stderr)
        die("glyph coverage (srCharFidelity): characters above have no glyph in the vendored "
            "fonts and would tofu-render SILENTLY - refusing. Vendor a covering font or "
            "change the text.")


def compile_doc(typst, template, instance_path, out_path, fmt=None):
    try:
        rel = "/" + instance_path.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        die(f"instance must live under the repo root ({ROOT}) so typst --root can read it: "
            f"{instance_path}")
    cmd = [typst, "compile", "--root", str(ROOT),
           "--font-path", str(ROOT / MANIFEST["fontsDir"]), "--ignore-system-fonts",
           "--creation-timestamp", str(MANIFEST["creationTimestamp"]),
           "--input", f"data={rel}", str(ROOT / template), str(out_path)]
    if fmt:
        cmd += ["--format", fmt]
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        die(f"compile failed:\n{r.stderr[:2000]}")
    if "warning:" in r.stderr:
        die(f"zero-warnings gate (D0235 A5.1):\n{r.stderr[:2000]}")
    return r


# Typst shapes ASCII punctuation typographically (hyphen -> U+2010/U+00AD, quotes -> smart)
# and composes combining marks; extraction folds the same way so verbatim assertions compare
# content, not glyph choices.
_FOLD = str.maketrans({"­": "-", "‐": "-", "‑": "-", "‒": "-",
                       "–": "-", "—": "-", "‘": "'", "’": "'",
                       "“": '"', "”": '"', "\x92": "'", "ﬁ": "fi",
                       "ﬂ": "fl"})


def norm(s):
    return re.sub(r"\s+", " ", unicodedata.normalize("NFC", s).translate(_FOLD)).strip()


def contains_tolerant(hay, needle):
    """Verbatim containment tolerant of extraction artifacts (issue004), in three passes:
    1. normalized containment; 2. whitespace-stripped containment (kern-inserted spaces,
    e.g. 'CRT -001'); 3. whitespace-stripped with U+FFFD in the extraction matching any
    single character (pypdf maps some glyphs to the replacement char)."""
    h, n = norm(hay), norm(needle)
    if n in h:
        return True
    hs, ns = re.sub(r"\s+", "", h), re.sub(r"\s+", "", n)
    if ns in hs:
        return True
    if "\ufffd" in hs:
        rx = re.compile("".join(f"(?:{re.escape(c)}|\ufffd)" for c in ns))
        return rx.search(hs) is not None
    return False


def _resolve_field(data, field):
    """Yield (label, value) for a fieldMap field path."""
    if field.startswith("meta."):
        key = field[5:]
        if key in data["meta"]:
            yield field, str(data["meta"][key])
    elif field.startswith("requirements[]."):
        key = field.split(".", 1)[1]
        for r in data.get("requirements", []):
            if key in r:
                yield f"{r.get('id', '?')}.{key}", str(r[key])
    elif field in data:
        yield field, str(data[field])


def assert_fields(pdf_path, data, doctype):
    """A5.2, driven by the doctype's outputs.json fieldMap (issue005: the map is data the
    harness executes, not prose)."""
    from pypdf import PdfReader
    outputs = json.loads((ROOT / OUTPUTS_BY_DOCTYPE[doctype]).read_text(encoding="utf-8"))
    fmap = outputs["declared"]["pdf"]["fieldMap"]
    text = " ".join(p.extract_text() or "" for p in PdfReader(str(pdf_path)).pages)
    for entry in fmap:
        if entry["readback"] != "text":
            continue  # golden-covered or declared-none fields
        for label, value in _resolve_field(data, entry["field"]):
            probe = value.upper() if entry.get("transform") == "upper" else value
            if not (contains_tolerant(text, probe) or contains_tolerant(text.upper(), value.upper())):
                kind = "VERBATIM SURVIVAL (issue001 regression)" if "statement" in entry["field"] \
                    else "field population (A5.2)"
                die(f"{kind}: {label} not found in PDF text: {value!r}")


def _page_key(p):
    m = re.search(r"(\d+)", p.name)
    return int(m.group(1)) if m else 0


def golden(typst, template, instance_path, update):
    stem = instance_path.stem
    gdir = ROOT / "golden" / stem
    tmp = ROOT / "out" / f"_golden_{stem}"
    tmp.mkdir(parents=True, exist_ok=True)
    for old in tmp.glob("p*.png"):
        old.unlink()
    compile_doc(typst, template, instance_path, tmp / "p{n}.png", fmt="png")
    pages = sorted(tmp.glob("p*.png"), key=_page_key)
    if update:
        a = ROOT / "out" / f"_bytecheck_a_{stem}.pdf"
        b = ROOT / "out" / f"_bytecheck_b_{stem}.pdf"
        compile_doc(typst, template, instance_path, a)
        compile_doc(typst, template, instance_path, b)
        if a.read_bytes() != b.read_bytes():
            die("sequencing invariant (A5.3): double-compile is NOT byte-identical; "
                "no baseline may be committed")
        gdir.mkdir(parents=True, exist_ok=True)
        for old in gdir.glob("p*.png"):
            old.unlink()
        for p in pages:
            shutil.copy2(p, gdir / p.name)
        print(f"  golden: baselined {len(pages)} page(s) -> {gdir.relative_to(ROOT)} "
              "(byte-identity verified; commit requires author approval)")
        return
    if not gdir.exists():
        die(f"golden diff: no baseline at {gdir.relative_to(ROOT)} (run --update-golden "
            "after author approval)")
    from PIL import Image, ImageChops
    base = sorted(gdir.glob("p*.png"), key=_page_key)
    if len(base) != len(pages):
        die(f"golden diff: page count {len(pages)} != baseline {len(base)}")
    for bp, np_ in zip(base, pages):
        if ImageChops.difference(Image.open(bp).convert("RGB"),
                                 Image.open(np_).convert("RGB")).getbbox() is not None:
            die(f"golden diff: {bp.name} differs from baseline (re-baseline needs "
                "explicit author approval)")
    print(f"  golden: {len(pages)} page(s) match baseline")


def hex_lint():
    rx = re.compile(r'"#[0-9A-Fa-f]{3,8}"|rgb\(\s*\d|luma\(')
    hits = [f"{f.relative_to(ROOT)}:{i + 1}"
            for f in (ROOT / "templates").rglob("*.typ")
            for i, line in enumerate(f.read_text(encoding="utf-8").splitlines())
            if rx.search(line)]
    if hits:
        die("hex lint (A1): color literals in templates (use brand/style-tokens.json): "
            + ", ".join(hits))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("instance")
    ap.add_argument("--out", default=None)
    ap.add_argument("--check", action="store_true", help="run all checks incl. golden diff")
    ap.add_argument("--update-golden", action="store_true")
    args = ap.parse_args()

    instance = Path(args.instance).resolve()
    typst = find_typst()
    assert_toolchain(typst)
    data, doctype = validate_instance(instance)
    check_content_contract(data)
    check_glyph_coverage(data)
    template = TEMPLATE_BY_DOCTYPE[doctype]

    out = Path(args.out) if args.out else ROOT / "out" / f"{instance.stem}.pdf"
    out.parent.mkdir(parents=True, exist_ok=True)
    compile_doc(typst, template, instance, out)
    print(f"  compiled (zero warnings): {out}")
    assert_fields(out, data, doctype)
    print("  field read-back per outputs.json fieldMap: OK")
    hex_lint()
    print("  hex lint: OK")
    if args.check or args.update_golden:
        golden(typst, template, instance, args.update_golden)
    print("PASS")


if __name__ == "__main__":
    main()
