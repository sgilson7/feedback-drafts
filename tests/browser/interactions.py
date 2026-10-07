"""The project's own browser interactions, run by check.py in every engine.

check.py has already loaded the page, waited for it to report ready, and let
it idle. This drives the tool the way a teacher would and asserts what the
page shows with `check(condition, "what this shows")`: the text drawn, the
draft in the box, the files saved.

Expected values in these checks are worked out by hand from the brief and
data/sample-rubric.json (C4), never copied from the tool's own output. The
hand set of choices is the one in crates/core/tests/hand_computed.rs:
Research question Proficient (3), Evidence and sources Developing (2),
Structure Excellent (4), Writing Proficient (3): 12 of 16 points.
"""

import json
import pathlib

STRINGS = json.loads((pathlib.Path(__file__).resolve().parents[2] / "data" / "strings.json").read_text())

ALL_MISSING = "Choose a level for: Research question, Evidence and sources, Structure, Writing."

HAND_DRAFT = """Hello Jordan, here is my feedback on your project report.

Research question: Proficient (3 points). Your question is clear and focused enough to answer in the space you had.

Evidence and sources: Developing (2 points). You use some sources; check that each claim you rely on has one.

Structure: Excellent (4 points). The structure carries the argument: each section builds on the one before.

Writing: Proficient (3 points). Your writing is clear and mostly free of errors.

Total: 12 of 16 points.

I liked your choice of local data.

Come and see me if you want to talk any of this through."""

EDITED_DRAFT = HAND_DRAFT.replace("Your writing is clear and mostly free of errors.", "Clear writing.")

NO_POINTS_DRAFT = """Hello Jordan, here is my feedback on your project report.

Research question: Proficient. Your question is clear and focused enough to answer in the space you had.

Evidence and sources: Developing. You use some sources; check that each claim you rely on has one.

Structure: Excellent. The structure carries the argument: each section builds on the one before.

Writing: Proficient. Clear writing.

I liked your choice of local data.

Come and see me if you want to talk any of this through."""


def choice_set(page, legend):
    return page.locator("#choices fieldset").filter(has=page.locator("legend", has_text=legend))


def choose(page, legend, option):
    choice_set(page, legend).get_by_label(option, exact=True).check()


def choose_hand_set(page):
    choose(page, "Research question", "Proficient (3 points)")
    choose(page, "Evidence and sources", "Developing (2 points)")
    choose(page, "Structure", "Excellent (4 points)")
    choose(page, "Writing", "Proficient (3 points)")


def editor(page, n):
    return page.locator("#criteria fieldset").nth(n)


def mark_editor(page):
    page.evaluate("() => document.querySelector('#criteria fieldset').setAttribute('data-before-open', '')")


def run(page, check, engine):
    # The page and the sample banner.
    check(page.inner_text("#title") == STRINGS["title"], "the title from data/strings.json is drawn")
    banner = page.locator("#sample-banner")
    check(banner.is_visible(), "the sample banner shows at start")
    check(banner.inner_text() == STRINGS["sample_banner"], "the banner says the rubric is sample content")
    check(page.inner_text("#message") == "", "no error message is shown at start")
    check(page.inner_text("#missing") == ALL_MISSING, "all four criteria are named as missing at start")
    check(not page.locator("#draft").is_visible(), "no draft before any level is chosen")

    # A name, three levels: one criterion still missing, still no draft.
    page.fill("#student-name", "Jordan")
    choose(page, "Research question", "Proficient (3 points)")
    choose(page, "Evidence and sources", "Developing (2 points)")
    choose(page, "Structure", "Excellent (4 points)")
    check(page.inner_text("#missing") == "Choose a level for: Writing.", "with three levels chosen, Writing is named as missing")
    check(not page.locator("#draft").is_visible(), "no draft while a criterion is missing")

    # The fourth level and a note: the draft worked out by hand.
    choose(page, "Writing", "Proficient (3 points)")
    check(page.locator("#draft").is_visible(), "the draft appears once every criterion has a level")
    check(not page.locator("#missing").is_visible(), "the missing line is gone once the draft appears")
    page.fill("#note", "I liked your choice of local data.")
    check(page.input_value("#draft") == HAND_DRAFT, "the draft matches the hand-written text")

    # Edit a comment in the rubric: the draft follows.
    editor(page, 3).locator("textarea").nth(2).fill("Clear writing.")
    check(page.input_value("#draft") == EDITED_DRAFT, "editing the Writing/Proficient comment changes the draft")

    # Hide points, then show them again.
    page.uncheck("#show-points")
    check(page.input_value("#draft") == NO_POINTS_DRAFT, "with points hidden the draft has no points and no total")
    plain = choice_set(page, "Writing").get_by_label("Proficient", exact=True)
    check(plain.count() == 1 and plain.is_checked(), "choice labels drop the points too")
    page.check("#show-points")
    check(page.input_value("#draft") == EDITED_DRAFT, "showing points again restores them")

    # Points that are not a whole number: a message, no crash, no change.
    points = editor(page, 0).locator("input.points").first
    points.fill("2.5")
    points.press("Tab")
    check(page.inner_text("#message") == 'Points must be a whole number from 0 to 1000, not "2.5".', "points of 2.5 are refused with a message")
    check(page.input_value("#draft") == EDITED_DRAFT, "the refused points leave the draft unchanged")
    points.fill("1")
    points.press("Tab")
    check(page.inner_text("#message") == "", "valid points clear the message")

    # Add a criterion: copies of the first criterion's labels and points, empty comments.
    page.click("#add-criterion")
    check(page.locator("#criteria fieldset").count() == 5, "adding a criterion gives five in the editor")
    check(page.locator("#choices fieldset").count() == 5, "and five in the choices")
    new = editor(page, 4)
    labels = [new.locator(".level").nth(j).locator("input").first.input_value() for j in range(4)]
    pts = [new.locator("input.points").nth(j).input_value() for j in range(4)]
    comments = [new.locator("textarea").nth(j).input_value() for j in range(4)]
    check(labels == ["Beginning", "Developing", "Proficient", "Excellent"], f"the new criterion copies the level labels {labels}")
    check(pts == ["1", "2", "3", "4"], f"the new criterion copies the points {pts}")
    check(comments == ["", "", "", ""], "the new criterion's comments are empty")
    check(page.inner_text("#missing") == "Choose a level for: Criterion 5.", "the new criterion is named as missing")
    check(not page.locator("#draft").is_visible(), "no draft while the new criterion has no level")

    # Remove it again: the draft comes back unchanged.
    new.locator("button.remove").click()
    check(page.locator("#criteria fieldset").count() == 4, "removing the new criterion leaves four")
    check(page.input_value("#draft") == EDITED_DRAFT, "the draft is back after the removal")

    # Copy: either the clipboard took it or the page says how to copy by hand.
    page.click("#copy")
    page.locator("#copy-status:not(:empty)").wait_for()
    status = page.inner_text("#copy-status")
    check(status in (STRINGS["copied"], STRINGS["copy_failed"]), f"Copy reports what happened: {status!r}")

    # Save the draft as a text file.
    with page.expect_download() as info:
        page.click("#save-draft")
    saved_draft = info.value
    check(saved_draft.suggested_filename == "feedback-jordan.txt", f"the draft file is named after the student ({saved_draft.suggested_filename})")
    check(pathlib.Path(saved_draft.path()).read_text(encoding="utf-8") == EDITED_DRAFT, "the saved draft is the text in the box")

    # Save the rubric and open the saved file again.
    with page.expect_download() as info:
        page.click("#save-rubric")
    saved = info.value
    check(saved.suggested_filename == "sample-rubric-project-report.json", f"the rubric file is named after its title ({saved.suggested_filename})")
    saved_text = pathlib.Path(saved.path()).read_text(encoding="utf-8")
    rubric = json.loads(saved_text)
    check(rubric["criteria"][3]["levels"][2]["comment"] == "Clear writing.", "the saved rubric has the edited comment")
    check(len(rubric["criteria"]) == 4, "the saved rubric has four criteria")

    # Opening reads the file asynchronously; wait until the editor has been
    # rebuilt (the marked fieldset is gone) before looking at the result.
    mark_editor(page)
    page.set_input_files("#open-file", saved.path())
    page.locator("#criteria fieldset[data-before-open]").wait_for(state="detached")
    check(not banner.is_visible(), "after opening a saved file the sample banner is gone")
    check(page.inner_text("#message") == "", "the saved file opens without an error")
    check(page.inner_text("#missing") == ALL_MISSING, "opening a file clears the levels")
    check(page.input_value("#student-name") == "Jordan", "opening a file keeps the student name")
    check(editor(page, 3).locator("textarea").nth(2).input_value() == "Clear writing.", "the reopened rubric has the edited comment")

    # A file that is not a rubric: a message, and the rubric stays.
    page.set_input_files("#open-file", files=[{"name": "empty.json", "mimeType": "application/json", "buffer": b"{}"}])
    page.locator("#message:not(:empty)").wait_for()
    check(page.inner_text("#message") == 'The rubric file has no "title".', "a file with a missing field is refused with a message")
    check(page.locator("#criteria fieldset").count() == 4, "the refused file leaves the rubric as it was")

    # The reopened rubric gives the same draft.
    choose_hand_set(page)
    check(page.input_value("#draft") == EDITED_DRAFT, "the reopened rubric gives the same draft")

    # Next student: name, levels and note cleared, rubric kept.
    page.click("#next-student")
    check(page.input_value("#student-name") == "", "Next student clears the name")
    check(page.input_value("#note") == "", "Next student clears the note")
    check(page.inner_text("#missing") == ALL_MISSING, "Next student clears every level")
    check(page.locator("#choices input:checked").count() == 0, "no level is selected after Next student")
    check(not page.locator("#draft").is_visible(), "no draft after Next student")
    check(editor(page, 3).locator("textarea").nth(2).input_value() == "Clear writing.", "Next student keeps the rubric")

    # Remove down to one criterion: the last cannot be removed.
    for _ in range(3):
        editor(page, 0).locator("button.remove").click()
    check(page.locator("#criteria fieldset").count() == 1, "three criteria removed leaves one")
    check(editor(page, 0).locator("button.remove").is_disabled(), "the last criterion's Remove button is disabled")
    check(page.inner_text("#missing") == "Choose a level for: Writing.", "the one criterion left is Writing")
