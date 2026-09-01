"""Exec-summary template tests (needExecSummary).

Static contract + REAL browser behavior:
  - desktop (Chromium): ONE click on the top copy button puts the markdown digest on the
    clipboard (read back via granted clipboard permission) and flips the button to Copied.
  - mobile (WebKit, iPhone-class viewport + touch): ONE tap on the BOTTOM copy button flips
    to Copied (WebKit denies headless clipboard read, so success is asserted via the button's
    success state, which only fires after a copy path succeeded); tabs switch on one tap;
    all touch targets are >= 44 CSS px tall.

Run:  .venv/Scripts/python -m pytest tests/exec_summary -q
Set EXEC_SUMMARY_FILE to test a filled report instance instead of the skill template.
"""
import os
import re
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent.parent
PAGE = Path(os.environ.get(
    "EXEC_SUMMARY_FILE", ROOT / "templates" / "exec-summary" / "exec-summary.html"))
RAW = PAGE.read_text(encoding="utf-8")
# static checks look at MARKUP only - CSS/JS legitimately mention the same selectors
HTML = re.sub(r"<style>[\s\S]*?</style>|<script>[\s\S]*?</script>", "", RAW)


# ---------- static contract: ONE implementation, shared with the commit gate ----------
# scripts/check_templates.py is what .githooks/pre-commit runs (D0237). These tests CALL it
# rather than restating its rules, so the gate and the suite cannot drift (issue003's lesson).

sys.path.insert(0, str(ROOT / "scripts"))
import check_templates as CT  # noqa: E402


def test_template_contract_over_every_family():
    problems = CT.check_tree(ROOT / "templates")
    assert not problems, "template contract violations: " + " | ".join(problems)


def test_this_page_satisfies_the_contract():
    # EXEC_SUMMARY_FILE points the suite at one file (a report under review, say)
    tmpl = next(p for p in PAGE.parent.glob("*.html")
                if CT.is_template(p.read_text(encoding="utf-8")))
    tfields = None if PAGE == tmpl else CT.fields(
        CT.markup_only(tmpl.read_text(encoding="utf-8")))
    problems = CT.check_file(PAGE, tfields)
    assert not problems, "contract violations: " + " | ".join(problems)


# ---------- browser behavior (Playwright) ----------

@pytest.fixture(scope="module")
def pw():
    from playwright.sync_api import sync_playwright
    with sync_playwright() as p:
        yield p


def test_desktop_one_click_copy_puts_digest_on_clipboard(pw):
    browser = pw.chromium.launch()
    ctx = browser.new_context(permissions=["clipboard-read", "clipboard-write"])
    page = ctx.new_page()
    page.goto(PAGE.resolve().as_uri())
    # a pristine reader: no stored selections, so the digest must say so (order-independent)
    page.evaluate("try{localStorage.clear()}catch(e){}")
    page.reload()
    top_btn = page.locator("[data-copy]").first
    top_btn.click()  # ONE click
    page.wait_for_selector('[data-copy][data-done="1"]', timeout=2000)
    clip = page.evaluate("navigator.clipboard.readText()")
    title = page.locator('[data-digest="title"]').text_content().strip()
    assert clip.startswith("# " + title), "digest must lead with the report title"
    assert "## " in clip and "DECISION:" in clip
    assert "- [ ]" in clip and "(no selection made yet)" in clip, \
        "unselected decisions must say so, never fabricate a choice"
    assert "|---|" in clip or "| --- |" in clip, "pros/cons must arrive as a markdown table"
    # the reasoning must travel with the verdict, or the receiving AI inherits a conclusion
    # it cannot argue with
    for needed in ("**Strongest case:**", "**Why not:**", "**What decides it:**",
                   "**Why this wins:**", "**What would change this:**", "**Confidence:**"):
        assert needed in clip, f"digest missing {needed}"
    assert clip.rstrip().endswith("My notes:"), "digest must end inviting the human's notes"
    browser.close()


def test_desktop_selection_flows_into_digest(pw):
    browser = pw.chromium.launch()
    ctx = browser.new_context(permissions=["clipboard-read", "clipboard-write"])
    page = ctx.new_page()
    page.goto(PAGE.resolve().as_uri())
    # ONE click selects the second choice of decision 1
    second = page.locator('[data-d="choices"]').first.locator("label.choice").nth(1)
    second.click()
    assert second.locator("input").is_checked()
    page.locator('[data-d="note"]').first.fill("prefer flat if reuse appears")
    page.locator("[data-copy]").first.click()
    page.wait_for_selector('[data-copy][data-done="1"]', timeout=2000)
    clip = page.evaluate("navigator.clipboard.readText()")
    chosen = second.locator("input").get_attribute("value")
    assert f"- [x] {chosen}" in clip, "the selected choice must arrive checked in the digest"
    assert "- [ ]" in clip, "unselected choices arrive unchecked"
    assert "**Note:** prefer flat if reuse appears" in clip
    browser.close()


def test_selections_persist_across_reload(pw):
    browser = pw.chromium.launch()
    page = browser.new_page()
    page.goto(PAGE.resolve().as_uri())
    target = page.locator('[data-d="choices"]').first.locator("label.choice").nth(2)
    target.click()
    page.reload()
    assert page.locator('[data-d="choices"]').first.locator("label.choice").nth(2) \
        .locator("input").is_checked(), "selection must survive a reload (localStorage)"
    browser.close()


def test_desktop_tab_switch_and_keyboard(pw):
    browser = pw.chromium.launch()
    page = browser.new_page()
    page.goto(PAGE.resolve().as_uri())
    tabs = page.locator('[role="tab"]')
    second = tabs.nth(1)
    second.click()  # one click switches
    assert second.get_attribute("aria-selected") == "true"
    panel2 = page.locator('[role="tabpanel"]').nth(1)
    assert panel2.is_visible()
    assert page.locator('[role="tabpanel"]').nth(0).is_hidden()
    second.press("ArrowRight")  # keyboard nav, wrapping at the end
    nxt = (1 + 1) % tabs.count()
    assert tabs.nth(nxt).get_attribute("aria-selected") == "true"
    browser.close()


def test_mobile_webkit_one_tap_copy_and_tabs(pw):
    browser = pw.webkit.launch()
    ctx = browser.new_context(
        viewport={"width": 390, "height": 844},
        device_scale_factor=3, has_touch=True, is_mobile=True,
        user_agent=("Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) "
                    "AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile Safari/604.1"))
    page = ctx.new_page()
    page.goto(PAGE.resolve().as_uri())
    # bottom copy button: ONE tap must reach the success state (WebKit headless denies
    # clipboard *read*, so the asserted signal is the success state that only fires
    # after clipboard.writeText or the execCommand fallback returned success)
    bottom = page.locator("[data-copy]").last
    bottom.scroll_into_view_if_needed()
    bottom.tap()
    page.wait_for_selector('[data-copy][data-done="1"]', timeout=2000)
    # one tap switches tabs
    page.locator('[role="tab"]').nth(1).tap()
    assert page.locator('[role="tabpanel"]').nth(1).is_visible()
    # one tap selects a choice
    choice = page.locator('[role="tabpanel"]').nth(1).locator("label.choice").first
    choice.tap()
    assert choice.locator("input").is_checked(), "one tap must select a choice on mobile"
    browser.close()


def test_touch_targets_are_44px(pw):
    browser = pw.webkit.launch()
    ctx = browser.new_context(viewport={"width": 390, "height": 844}, has_touch=True,
                              is_mobile=True)
    page = ctx.new_page()
    page.goto(PAGE.resolve().as_uri())
    heights = page.eval_on_selector_all(
        '[data-copy], [role="tab"], label.choice',
        "els => els.filter(e => e.offsetParent !== null)"
        ".map(e => e.getBoundingClientRect().height)")
    assert heights and all(h >= 43.5 for h in heights), f"touch targets under 44px: {heights}"
    browser.close()
