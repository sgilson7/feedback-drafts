# Plan

Stage 2, written by the agent from `BRIEF.md`. `B2 Require step-by-step justification`.

Each decision in build order, with the reason for it and the test that will show it holds. The brief approves the plan in advance, so the build continues without waiting; anything decided here that the brief did not settle is also a row in `SECOND-ORDER.md`.

## Shape

- `crates/core/src/rubric.rs`: the rubric, reading and checking a rubric file, and the rubric edits.
- `crates/core/src/draft.rs`: the session (rubric, name, choices, note), the missing list, the total, the draft text, file names.
- `crates/core/src/api.rs`: the one door. Strings in, `{"ok": ...}` or `{"error": "..."}` out.
- `crates/core/src/page.rs`: the page's interface strings from `data/strings.json`, checked by the core (kept from the template, extended).
- `crates/shim/src/lib.rs`: one forwarding function per api function.
- `web/`: draws the view the core returns; holds the core's state object and hands it back on every call.
- `data/strings.json` (interface text) and `data/sample-rubric.json` (sample content).

## Decisions in build order

| # | Decision | Reason | Test that shows it holds |
| --- | --- | --- | --- |
| 1 | Rubric file format: `{"title", "opening", "closing", "show_points", "criteria": [{"name", "levels": [{"label", "points", "comment"}]}]}`. Every field required; an unknown field is rejected and named. | The brief lists exactly these parts. Rejecting unknown fields catches a misspelt field (for example `"comments"`) that would otherwise be silently ignored. | `rubric.rs` tests: the sample parses; a file missing `closing`, missing a level's `comment`, or with an extra field is rejected with a message naming the field. |
| 2 | The core reads a rubric file as a `serde_json::Value` and checks it field by field, not with derived deserialisation. | Derived errors read like `invalid type: floating point 2.5, expected u32`; the teacher needs "Criterion 2, level 3: points must be a whole number from 0 to 1000; found 2.5." | Tests compare the exact message for missing field, wrong type, 1 level, 7 levels, -1 points, 2.5 points, not JSON, empty criteria list. |
| 3 | A criterion has 2 to 6 levels; a rubric has at least 1 criterion. Points are whole numbers from 0 to 1000. | The 2 to 6 and "at least one" limits come from the brief. "Whole number" includes 0. 1000 is an upper bound I chose so totals cannot overflow and a typo like 40000 is caught (worklist row for the teacher). | Tests: 1 and 7 levels rejected, 2 and 6 accepted; -1, 2.5, 1001 and `"3"` (text) rejected; 0 and 1000 accepted. |
| 4 | Texts (title, lines, names, labels, comments) may be empty. In the draft and the missing list an empty criterion name shows as "Criterion N" and an empty level label as "Level N". | A new criterion starts with no name, and the teacher clears a field to retype it; refusing empty text would make the box fight the teacher. A fallback name keeps the missing list readable. | Test: after adding a criterion, the missing list names "Criterion 5"; a draft with an empty label shows "Level 2". |
| 5 | Rubric edits are core functions that take the rubric and return a new one or an error: set title/opening/closing/show_points, criterion name, level label/points/comment, add criterion, remove criterion. Points arrive as the text the teacher typed and the core parses them. | Rule (c): every edit and validation is in the core; the page cannot parse numbers (boundary test D3). | Tests: hand-written rubric after adding (copied labels and points, empty comments, empty name) and after removing a criterion; removing the last criterion rejected; "2.5", "-1", "" as points rejected with the rubric unchanged; index out of range rejected. |
| 6 | The session is `{rubric, sample, name, choices, note}`. `choices` has one entry per criterion (a level index or none). Adding a criterion appends "none"; removing one drops its choice; opening a file resets every choice and keeps name and note. | Choices must stay aligned with criteria through every edit; keeping the name and note when another rubric is opened avoids losing typed text. | Tests on the session after add, remove, open. |
| 7 | Missing list: the display names of criteria with no level, in rubric order. The message is "Choose a level for: A, B." | The brief: "names the criteria still missing". | Hand-computed test on the sample with two of four chosen. |
| 8 | Total: N = sum of chosen points, M = sum over criteria of the highest points among that criterion's levels. Only given when every criterion has a level. | "N of M points": M is the most a student could get, which is the highest level of each criterion, not the last one (levels need not be in order). | Hand-computed total 12 of 16 on the sample; a test with levels in descending order shows M uses the maximum. |
| 9 | Draft text, paragraphs separated by one blank line: opening; `Name: Label (P points). Comment` per criterion (`(1 point)` for one; no brackets when points are hidden; no comment sentence when the comment is empty); `Total: N of M points.` when shown; the note if not blank; the closing. | The brief's order. "Total:" makes the line readable on its own. | Hand-computed full drafts on the sample: with name, note and points; with none of them; plus 1-point singular. |
| 10 | The name enters the opening through the marker `{name}`: with a name every `{name}` becomes the trimmed name; with no name `{name}` is removed with one space before it ("Hello {name}, here" becomes "Hello, here"). If the opening has no marker and a name is given, the line becomes "Name, opening". | The brief says the opening carries the name if given, without saying where. A marker lets the teacher place it; the fallback still shows the name. | Tests for marker with and without a name, and for an opening with no marker. |
| 11 | "Next student" clears name, choices and note; keeps rubric and the sample flag. | The brief. | Test on the session after next student. |
| 12 | The sample banner shows while the session started from the sample; editing does not clear it; opening any file does. | The sample stays sample content until the teacher replaces it with their own file. | Core test on the `sample` flag; browser check that the banner shows at start and is gone after opening the saved file. |
| 13 | Saving: the core gives the rubric file text (pretty JSON in the field order of decision 1) and file names: the rubric title or the student name made lowercase with runs of other characters turned into `-` (`project-report-rubric.json`, `feedback-jordan.txt`; `rubric.json` and `feedback.txt` when empty). The page saves the text with a same-origin `blob:` download. | A saved file must open again unchanged (round trip). File names are a rule, so they belong in the core. A `blob:` link stays on the device. | Round-trip test: the sample saved and reopened equals the sample. File-name tests. Browser check saves, reopens, and reads the file. |
| 14 | The api: `load(strings)`, `start(sample_text)`, `act(state, action)`, `rubric_file(state)`, `draft_filename(state)`. Each returns `{"ok": {"state", "view"}}` or `{"error": message}`; on error the page keeps its previous state and shows the message. The view holds everything the page draws (texts, levels with a `chosen` flag, `can_remove`, missing message, draft, sample flag). | One door; the page holds the core's state object but never changes it. | api tests: an unknown action, a malformed state and an invalid file each give `{"error": ...}`; no input panics (a test feeds garbage to every api function). |
| 15 | The page redraws the editor only when its shape changes (start, open, add, remove) and otherwise updates values in place, never the box that has focus. The draft box is overwritten only when the composed draft changes, so the teacher's edits survive until a choice changes. | Rebuilding inputs on every key press loses the cursor. | Browser check types into fields and sees the draft follow. |
| 16 | Copy uses `navigator.clipboard.writeText` and, if refused, selects the draft and asks the teacher to copy with the keyboard. | The clipboard is on the teacher's device; no request leaves it. Browsers in test refuse clipboard access, so the fallback is needed. | Browser check: pressing Copy shows one of the two messages and no page error. |
| 17 | Interface text in `data/strings.json`, checked by the core (every key present and non-empty). Draft wording ("Total", "point(s)") and error messages stay beside the rule in the core. | D5, and the template's convention that rule-dependent wording stays with the rule. | `page.rs` tests; browser check compares drawn strings with the file. |
| 18 | Sample rubric `data/sample-rubric.json`, titled "Sample rubric: project report", 4 criteria by 4 levels (1 to 4 points), with a banner saying it is sample content to check or replace. | The brief. | Core test parses it and checks 4 by 4 with points 1 to 4; browser check sees the banner. |
| 19 | Browser interactions (`tests/browser/interactions.py`): banner; missing list; pick levels; type name and note; compare the draft with the hand-written text; edit a comment and see the draft change; hide points; add and remove a criterion; invalid points message; save rubric, reopen it, banner gone; save draft; copy; next student. | The brief's C4 list. | It is the test. Each check is broken once and watched failing (rule f). |

## Rule (f)

Every new core test is watched failing by breaking the rule it covers (a temporary edit to the core, then restored). The break and the failure message go in `SECOND-ORDER.md`. The browser checks are watched failing the same way, by breaking the core or the page and running the check in Chromium.

## Open questions for the person

These were decided provisionally so the build could continue; each is also a worklist row in `SECOND-ORDER.md`.

1. Is 0 to 1000 the right range for points?
2. Is the `{name}` marker in the opening line a workable way to place the student's name?
3. The draft box is overwritten when a choice, the name, the note or the rubric changes. Is "make choices first, then edit the text" acceptable?

## Status

- [x] Approved in advance by the brief (2026-10-07).
