# Feedback drafts

A one-page browser tool for a teacher writing individual feedback on student projects.

- **Rubric.** The page starts with a sample rubric, or the teacher opens a rubric file (JSON). The teacher can edit the title, the opening and closing lines, whether the draft shows points, each criterion's name, and each level's label, points and comment. They can also add a criterion (it copies the first criterion's level labels and points, with empty comments), remove any criterion but the last, and save the rubric as a file to use again.
- **Draft.** The teacher types an optional student name, picks one level per criterion and can add a personal note. Until every criterion has a level, the page names the criteria still missing. Once every criterion has one, the page writes the draft: the opening line (with the name where the opening says `{name}`), one paragraph per criterion (name, level label, points if shown, comment), the total ("Total: N of M points.") if points are shown, the note, and the closing line. The teacher can edit the draft in its box, copy it, or save it as a text file. "Next student" clears the name, the levels and the note, and keeps the rubric.

Nothing is sent anywhere. The page makes no request to another origin and uses no browser storage, so names and notes stay in the open page and are gone when it is closed. Saving uses a download link created inside the page, and Copy uses the browser's clipboard on the teacher's own device.

## The sample rubric is sample content

`data/sample-rubric.json` ("Sample rubric: project report": four criteria, four levels each, 1 to 4 points) was written to show how the tool works. It has not been checked by a teacher. While it is loaded, the page shows a banner saying so. Check it, or replace it with your own rubric, before you use the tool with a class.

## Rubric file format

```json
{
  "title": "...",
  "opening": "Hello {name}, ...",
  "closing": "...",
  "show_points": true,
  "criteria": [
    { "name": "...", "levels": [ { "label": "...", "points": 3, "comment": "..." } ] }
  ]
}
```

Every field is required, and a field the tool does not use is refused, so a misspelt field is reported instead of being ignored. A rubric needs at least one criterion; a criterion needs 2 to 6 levels; points are whole numbers from 0 to 1000. A file that breaks a rule is refused with a message naming the place, for example `In criterion 2, level 3, points must be a whole number from 0 to 1000, not 2.5.`, and the rubric already open stays as it was. A file saved by the tool opens again unchanged.

## Build, test and serve

```bash
./scripts/setup.sh          # once: Python venv with Playwright and three browsers
cargo test --workspace      # core, hand-computed and boundary tests, in seconds
./scripts/build.sh          # release WebAssembly build into dist/
python3 scripts/serve.py    # http://127.0.0.1:8000/
./scripts/check.sh          # everything: format, lint, tests, build, import audit, browser check
```

Needs Rust with the `wasm32-unknown-unknown` target, `wasm-bindgen-cli` 0.2.127, and Python 3. `BUILDER_ENGINES=chromium ./scripts/check.sh` runs the browser check in one engine while iterating; the full check runs Chromium, Firefox and WebKit.

The output in `dist/` is a set of static files. It can be served by any static host; deploying is the person's decision and is not done by these scripts.

## Layout

| Path | Holds |
| --- | --- |
| `crates/core/src/rubric.rs` | the rubric, reading and checking a rubric file, rubric edits |
| `crates/core/src/draft.rs` | the session, every action, the missing list, the total, the draft, file names, the view the page draws |
| `crates/core/src/api.rs` | the one door the shim calls: strings in, `{"ok": ...}` or `{"error": "..."}` out |
| `crates/core/src/page.rs` | checks the interface strings file |
| `crates/core/tests/hand_computed.rs` | drafts, totals and rubric edits worked out by hand on the sample rubric |
| `crates/core/tests/boundary.rs` | the template's architecture checks |
| `crates/shim` | the `wasm-bindgen` functions the page calls; each is one call into the core |
| `web/` | the static page; it draws the view the core returns |
| `data/strings.json` | interface text |
| `data/sample-rubric.json` | the sample rubric (sample content) |
| `tests/browser/` | the three-engine browser check and the teacher's interactions |
| `scripts/` | build, check, serve, import audit, mutation audit |

The project follows the Builder template: `BUILDER-SPEC.md` holds its rules, `PLAN.md` the decisions and their reasons, `SECOND-ORDER.md` the notebook kept while building, and `HANDOFF.md` the state at handoff.

## Licence

MIT. See `LICENSE`.
