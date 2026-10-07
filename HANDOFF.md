# Handoff

Stage 9, written by the agent. `C5 Consolidate in your own words`.

A fresh session starts from this file, not from memory.

## What changed

The template's placeholder became the feedback-drafts tool described in `BRIEF.md`.

- **Core** (`crates/core`). `rubric.rs` reads a rubric file field by field and refuses a bad one with a message naming the place. It covers: a missing or unknown field, a wrong type, no criteria, fewer than 2 or more than 6 levels, and points that are not a whole number from 0 to 1000. It also holds every rubric edit, including adding a criterion (copies the first criterion's labels and points, with empty comments and no name) and removing one (never the last). `draft.rs` holds the session (rubric, sample flag, name, one choice per criterion, note) and applies every action the page can send. It also works out the missing list, the total (N = chosen points, M = each criterion's highest level, summed), the draft text, the file names, and the view the page draws. `api.rs` is the one door: `load`, `start`, `act`, `rubric_file`, `draft_filename`, each returning `{"ok": ...}` or `{"error": "..."}`.
- **Shim** (`crates/shim`). Five one-line forwarding functions.
- **Page** (`web/`). It draws the view and keeps the core's state object to hand back. It rebuilds the editor only when the rubric's shape changes, so typing keeps its cursor. Files are saved through a same-origin `blob:` link, and Copy uses the clipboard, with a fallback that selects the text.
- **Content** (`data/`). `strings.json` holds the interface text, which the core checks. `sample-rubric.json` is the sample rubric, marked as sample in its title and by an on-screen banner.
- **Tests.** 78 core tests: 52 unit tests (rubric 23, draft 16, api 7, strings 6), 13 hand-computed tests on the sample rubric, 2 no-storage tests, and the template's 11 boundary tests, which are unchanged. There are 51 browser checks per engine in `tests/browser/interactions.py`, plus the template's 9. Every new test and check was broken once and watched failing (`SECOND-ORDER.md`, "Watched failing").
- **Fixed during the build.** A junk-input test found a panic: an error message added 1 to a huge index and overflowed (`rubric.rs`). It was fixed in the core with `saturating_add` (notebook row 9).

## What passed

`./scripts/check.sh` on 2026-10-07 13:57, at commit `55f9cfc` (plus this file and `TIMING.md` after it). Tests: 52 + 11 + 13 + 2 passed, 0 failed. WebAssembly import audit: 1 import, 1 fetch (the module load), passed. Browser check: 180 PASS lines, 0 FAIL. End of the output:

```
  PASS the last criterion's Remove button is disabled
  PASS the one criterion left is Writing
  PASS the page stored nothing in the browser [0, 0, '', 0]
  PASS no Content-Security-Policy violation []
  PASS no uncaught page error []
  PASS no console error []
  PASS no off-origin request at any point []
  PASS no WebSocket []
  PASS no popup []
  PASS no worker or service worker [] sw=0

BROWSER CHECK PASSED in chromium, firefox, webkit

BUILDER CHECK PASSED
```

Optional mutation audit (`./scripts/mutants.sh`): 122 mutants, 112 caught, 10 unviable, 0 missed.

## What remains

Open items for the teacher (worklist rows in `SECOND-ORDER.md`, status *person*):

1. **Row 13, the sample rubric.** I wrote it, and no teacher has checked it. Check its wording or replace it.
2. **Row 1, the points range.** Points are limited to 0 to 1000. Is that range right?
3. **Row 2, the name.** The name goes into the opening line where it says `{name}`. With no name, the marker and the space before it are removed ("Hello, here is..."). With a name but no marker, the line becomes "Jordan, opening line". Is that workable?
4. **Row 3, edits in the draft box.** Changing a level, the name, the note or the rubric replaces any edits made in the draft box. The page says so beside the box. Is "choose first, edit last" acceptable?
5. **Row 14, unsaved rubric edits.** These are lost when the page closes, and there is no warning. Is a close warning wanted?
6. **Row 19, file names.** A saved draft's file name contains the student's name. Should it?

What the teacher should check by hand before using the tool with a class:

- Read the sample rubric, or open your own, and check every comment a student will read.
- Work one student by hand. Pick a level per criterion, add up the points yourself, and compare your total and the full draft with what the page shows. The hand-worked case in `crates/core/tests/hand_computed.rs` gives 12 of 16.
- Save your rubric, close the page, reopen it, open the saved file, and confirm everything came back. Nothing is kept between visits, so the file is the only copy.
- In the browser you will use, press Copy and paste the draft where it goes, and save one draft to see where the file lands.
- Make sure the browser you use does not restore typed text after a crash or restart. That restore is a browser feature the tool cannot see (row 11).

The tool has not been deployed. `dist/` is static files; deploying is the person's decision.

## What the next session must not assume

- The browser check proves what happened on the paths `interactions.py` drives, in those three engines, on that run. A feature it does not drive was not seen.
- The draft wording ("Total:", "point"/"points", "Choose a level for:") and every error message live in the core (`draft.rs`, `rubric.rs`), not in `data/strings.json` (row 5). Changing them means changing tests.
- `act` refuses a state whose choices do not fit the rubric. The page must hand back the state exactly as the core returned it.
- Opening a rubric file clears the levels and keeps the name and note (PLAN decision 6). This was my decision; the brief does not settle it.
- No network request, third-party script, model or browser storage was added. Adding any of them is the person's decision (`BRIEF.md`, "Autonomy").
