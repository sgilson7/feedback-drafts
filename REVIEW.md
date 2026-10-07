# Review after the session

The session that built this tool ran from 13:30:40 to 13:58:02 on 7 October 2026 (`TIMING.md`), from prompt set C in the Builder repository (`prompts/sets/C-feedback-drafts.md` in https://github.com/sgilson7/builder-setting). Its own record is `PLAN.md`, `MEASUREMENTS.md`, `SECOND-ORDER.md`, `HANDOFF.md` and the eight commits after "Start from the Builder template". Everything below was done afterwards, outside the session, and changed no code.

| Check | Result |
| --- | --- |
| `./scripts/check.sh` rerun at `7bf952b` | passed: format, lint, 78 core tests (52 unit, 13 hand-computed, 2 no-storage, 11 boundary), release build, import audit, browser check in Chromium, Firefox and WebKit |
| Shared template files | unchanged from the template commit: `crates/core/tests/boundary.rs`, `scripts/`, `tests/browser/check.py`, both crate manifests |
| Shim | five functions, one core call each |
| Page | read in full: every value drawn, every enabled state and the draft come from the core's `view`; no rule found in `web/app.js` |
| A draft checked by hand | name "Jordan", level 3 on all four sample criteria, a note: the draft reads "Total: 12 of 16 points." (4 × 3 of 4 × 4) with the four level-3 comments in order |
| Mutation testing, all mutants of the core | `cargo mutants --package tool_core --jobs 4`: 122 mutants, 112 caught, 10 unviable, 0 missed, in 51 s. The session ran the same audit once at the end and added no test after it |
| Deploy | `dist/` from `7bf952b` pushed to `gh-pages`; served at https://sgilson7.github.io/feedback-drafts/ |
| Browser check against the live origin | passed in Chromium, Firefox and WebKit, run twice |
| Phone width | 375 px in Chromium and WebKit: no horizontal scroll |

## Still with a person

The six items in `HANDOFF.md`, above all the sample rubric, which no teacher has checked. A teacher should also decide whether a saved draft's file name may carry the student's name, and whether the draft box should keep a teacher's edits when a level changes (today it is rewritten, and the page says so).
