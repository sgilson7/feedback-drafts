# Second-order notebook

Stage 6, kept by the agent as the work happens, not afterwards. `B3 Build in self-questioning`.

One row per assumption, surprise or open item. **Kind** is *divergence* (what was assumed, what turned out to be true, what changed), *finding*, or *worklist* (the person's to do). **Status** is *done*, *open*, or *person*.

| # | Kind | What was assumed or found | What is true | What changed | Status |
| --- | --- | --- | --- | --- | --- |
| 1 | worklist | The brief says "a whole number of points" without a range. | I chose 0 to 1000 (0 counts as whole; 1000 keeps totals small and catches typos). | PLAN decision 3. | person |
| 2 | worklist | The brief says the opening line carries the name "if one was given" without saying where. | I chose a `{name}` marker; without a name the marker and the space before it are removed; with no marker the name is put in front ("Jordan, ..."). | PLAN decision 10. | person |
| 3 | worklist | The draft box is editable, but the draft is recomposed when an input changes. | Edits in the box are replaced when the composed draft changes. The page says so beside the box. | PLAN decision 15. | person |
| 4 | finding | Empty texts: the brief does not say whether a criterion name or level label may be empty. | Allowed, with "Criterion N" / "Level N" shown in the draft and missing list. A new criterion starts with an empty name. | PLAN decision 4. | done |
| 5 | finding | Draft wording ("Total:", "point"/"points", "Choose a level for:") is reader-visible text in the core, not in `data/`. | D5 puts reader-visible text in `data/`; the template's own convention keeps rule-dependent wording beside the rule. I kept it in the core so the draft is fully tested there. | PLAN decision 17. | done |
| 6 | finding | Breaking a rule and running `cargo test` shows every test that watches it. | `cargo test` stops after the first test binary with a failure, so `tests/hand_computed.rs` never ran while a unit test was failing. | Reran every break with `--no-fail-fast`; the record below comes from that run. | done |
| 7 | finding | Every line in `Session::apply` was covered by a test. | Removing the `self.check()?` at the top of `apply` (break D45) failed no test: only `check()` itself was tested. | Added `apply_refuses_a_session_that_does_not_fit`; it fails under D45. | done |
| 8 | finding | Every branch in the draft has an observable effect. | Two do not: the early `return None` in `draft()` when criteria are missing (the loop returns `None` anyway) and the empty-comment branch (the later trim gives the same text). They are kept because they state the rule plainly; a mutation audit will report them as unkillable. | Nothing. | done |
| 9 | divergence | The core's `+ 1` on an index in an error message was harmless. | `api::tests::no_input_makes_any_function_panic` failed: `crates/core/src/rubric.rs:286` "attempt to add with overflow" for `criterion: 18446744073709551615`. Expected an `{"error": ...}`; got a panic. | Fixed in the core: `saturating_add(1)` in the three index messages (`rubric.rs`). The page was not touched. | done |
| 10 | divergence | `page.wait_for_function("...")` could wait on page state in the browser check. | Playwright evaluates the string with `eval`, which the page's CSP (`script-src 'self' 'wasm-unsafe-eval'`) blocks: `EvalError ... 'unsafe-eval' is not an allowed source`. | `interactions.py` waits with locators (`#copy-status:not(:empty)`, `wait_for(state="hidden")`) instead. The CSP was not loosened. | done |
| 11 | finding | Browser form history or field restoration could keep a student's name between visits. | Inputs carry `autocomplete="off"` and are not in a form; the page uses no storage API. Whether a given browser's session restore brings back typed text after a crash is not tested. | Worklist row 14. | done |
| 12 | finding | The strings file, the sample rubric and the template's placeholder tests in `page.rs`. | The template's `a_complete_file_is_accepted` and `an_empty_intro_is_rejected_by_name` tested the two-field placeholder; they were replaced by tests on the real strings file (`the_shipped_file_is_accepted`, `an_empty_banner_is_rejected_by_name`, `a_missing_key_is_rejected`). The boundary tests are unchanged. | `page.rs` | done |
| 13 | worklist | The sample rubric (`data/sample-rubric.json`) was written by the agent. | Its wording has not been checked by a teacher. The page shows a banner while it is loaded. | Teacher to check or replace it. | person |
| 14 | worklist | Nothing is kept between visits. | So a rubric edited in the page is lost when the page is closed unless the teacher presses "Save rubric". The page does not warn before closing. A warning (`beforeunload`) stores nothing, but it was not asked for, so it is not added. | Teacher to decide whether a close warning is wanted. | person |
| 15 | finding | The reopen checks in `interactions.py` waited for the banner to disappear. | Under break W1 (banner never shown) the wait returned at once, before the file had been read (`file.text()` is asynchronous), and "opening a file clears the levels" failed for the wrong reason. In the unbroken run the check passed only because the banner happened to be the signal. | The check now marks the editor before opening and waits for the rebuilt editor. Rerun of W1 fails only the banner check. | done |
| 16 | finding | Break X3 (core keeps the missing message when every criterion has a level) would fail "the missing line is gone". | It did not: the page hides the missing line whenever the view has a draft, so the stale message never shows. The page draws from `view.draft`, a value the core decides, so this is drawing and not a copy of a rule. | The check was watched failing with X3 plus the page showing the line (X3b). | done |
| 17 | finding | Breaks that make Playwright time out (W5, W16, the first X7) stop the run, so later checks are not seen. | A check is only "watched failing" when its own line says FAIL. | Reran those cases with breaks that fail without a time-out, and changed the "choice labels drop the points" check to count the matching label before reading it. | done |

## Watched failing (rule f)

Each break was a temporary edit to the core, made and restored by a scratch script; the tests named failed, with the first failure message shown. Line numbers are those of the file at the time.

### `crates/core/src/rubric.rs` (23 tests, step 2)

| Break | Tests that failed | First message |
| --- | --- | --- |
| R1 unknown fields allowed | `an_unknown_field_is_named` | rubric.rs:393 `unwrap_err()` on an `Ok` value |
| R2 missing-field wording "has no" → "lacks" | `a_missing_criterion_name_is_named`, `a_missing_level_field_is_named_with_its_place`, `a_missing_top_level_field_is_named` | left: `"Criterion 1 lacks \"name\"."` |
| R3 lower bound `< 2` → `< 1` | `one_level_is_rejected` | `unwrap_err()` on `Ok` with one level |
| R4 upper bound `> 6` → `> 7` | `seven_levels_are_rejected` | `unwrap_err()` on `Ok` with seven levels |
| R24 upper bound `> 6` → `>= 6` | `two_and_six_levels_are_accepted` | `"Criterion 1 has 6 levels; each criterion needs 2 to 6 levels."` |
| R5 bad points read as 0 | `negative_points_are_rejected`, `non_integer_points_are_rejected`, `points_as_text_or_above_the_limit_are_rejected` | `unwrap_err()` on `Ok`, points 0 |
| R6 limit `<=` → `<` | `zero_and_the_limit_are_accepted_as_points` | `"... not 1000."` |
| R7 typed points: no upper limit | `typed_points_are_parsed_or_refused` | left: `Ok(1001)` |
| R8 typed points: no trim | `typed_points_are_parsed_or_refused` | left: `Err("... not \" 7 \".")` |
| R9 refused typed points stored as 0 | `refused_points_leave_the_rubric_unchanged` | `assertion failed: r.set_points(0, 1, "2.5").is_err()` |
| R10 last criterion removable | `the_last_criterion_cannot_be_removed` | `unwrap_err()` on `Ok(())` |
| R11 criterion number off by one | `edits_out_of_range_are_refused` | left: `"There is no criterion 3."` |
| R12 fallback "Criterion {index}" | `display_names_fall_back_when_blank` | left: `"Criterion 4"` |
| R13 label not trimmed | `display_names_fall_back_when_blank` | left: `" Good "` |
| R14 max → last level | `max_points_is_the_highest_level_in_any_order` | left: `2` |
| R15 save writes only the criteria | `a_saved_file_reads_back_the_same` | `"The rubric file must be a JSON object."` |
| R16 label read from "comment" | `a_small_rubric_is_read` | left: `Level { label: "c2", ... }` |
| R17 not-JSON wording | `text_that_is_not_json_is_rejected_with_a_message` | `Not JSON (expected ident at line 1 column 2).` |
| R18 object wording | `a_file_that_is_not_an_object_is_rejected`, `wrong_types_are_named` | left: `"The rubric file must be an object."` |
| R19 empty criteria list allowed | `a_rubric_with_no_criteria_is_rejected` | `unwrap_err()` on `Ok` with `criteria: []` |
| R20 no capital at sentence start | 8 message tests | left: `"the rubric file has no \"closing\"."` |
| R21 bool wording | `wrong_types_are_named` | left: `"... must be a boolean."` |
| R22 "1 levels" | `one_level_is_rejected` | left: `"Criterion 1 has 1 levels; ..."` |
| R23 show_points inverted | `a_saved_file_reads_back_the_same`, `a_small_rubric_is_read` | left: `show_points: true` vs right `false` |

### `crates/core/src/draft.rs` (16 tests) and `crates/core/tests/hand_computed.rs` (13 tests), step 3

The first run stopped at the first failing test binary, so the hand-computed tests were not run for most breaks (see row 6). The table is from the rerun with `--no-fail-fast`. Every one of the 29 tests failed under at least one break.

| Break | Tests that failed |
| --- | --- |
| D1 `{name}` removed without its space | `the_marker_takes_the_name_or_goes_with_its_space` (left: `"Hello , here."`), `full_draft_with_note_and_points_but_no_name`, `full_draft_without_name_note_or_points` |
| D2 name not trimmed | `the_marker_takes_the_name_or_goes_with_its_space` |
| D3 no-marker name dropped | `without_a_marker_the_name_goes_in_front` |
| D4 "1 points" | `one_point_is_singular`, `a_draft_with_blank_label_empty_comment_and_one_point` |
| D5, D6 slug separator `_`, slug not lowercased | `slugs_and_file_names`, `the_sample_saved_is_the_sample_file_byte_for_byte` |
| D7, D8 file-name fallback and prefix | `slugs_and_file_names` |
| D9 nothing ever missing | `no_draft_and_no_total_until_every_criterion_has_a_level` (left: `[]`), `choices_follow_added_and_removed_criteria`, `next_student_after_the_full_draft`, `a_partial_set_names_the_missing_criteria_and_has_no_draft`, `the_rubric_after_adding_a_criterion` |
| D10 missing-message wording | `no_draft_and_no_total_until_every_criterion_has_a_level`, `a_partial_set_names_the_missing_criteria_and_has_no_draft` |
| D11 possible = earned | 6 tests incl. `the_total_for_the_hand_set_is_12_of_16`, `full_draft_with_name_note_and_points` (left: `Some((1, 1))` in the unit test) |
| D12 earned not summed | 6 tests incl. `the_total_for_the_hand_set_is_12_of_16`, `the_rubric_after_removing_a_criterion` |
| D13 points shown when hidden; D14 total shown when hidden | `blank_paragraphs_are_left_out_and_comments_trimmed`, `full_draft_without_name_note_or_points`, `full_draft_with_name_but_no_note_or_points` |
| D15 comment not trimmed | `blank_paragraphs_are_left_out_and_comments_trimmed` |
| D16 blank paragraphs kept | 4 tests incl. `full_draft_without_name_note_or_points` |
| D17 one newline between paragraphs | 7 tests incl. `editing_a_comment_changes_the_draft` (left: `"A: Hi. a5\nCriterion 2: Y. b2"`) |
| D18 note after the closing | `full_draft_with_note_and_points_but_no_name`, `full_draft_with_name_note_and_points` |
| D19 check ignores alignment | `a_session_whose_choices_do_not_fit_is_refused`, `apply_refuses_a_session_that_does_not_fit` |
| D45 apply skips the check | `apply_refuses_a_session_that_does_not_fit` (test added after this break first passed unnoticed; row 7) |
| D21 choose stores level 0 | 11 tests (left: `[Some(0), Some(0), None]`) |
| D22 choose not validated | `choosing_a_level_that_does_not_exist_is_refused` |
| D23 add does not extend choices | `choices_follow_added_and_removed_criteria`, `the_rubric_after_adding_a_criterion` |
| D24 remove drops the last choice | `choices_follow_added_and_removed_criteria`, `the_rubric_after_removing_a_criterion` |
| D25 next student keeps the name / the choices | `next_student_clears_name_levels_and_note_and_keeps_the_rubric`, `next_student_after_the_full_draft` |
| D26 open keeps the sample flag | `opening_a_file_resets_levels_keeps_name_and_note_and_ends_the_sample` |
| D26b open ignores a bad file | `a_bad_file_or_bad_points_leaves_the_session_unchanged` |
| D27, D28, D29 view: chosen flag, choice text, can_remove | `the_view_marks_choices_and_whether_removal_is_allowed` |
| D30, D40 label edit / criterion-name edit misdirected | `edits_reach_the_rubric` |
| D31 comment edit sets the label | `edits_reach_the_rubric`, `editing_a_comment_changes_the_draft` |
| D32 name ignored | 5 tests incl. `full_draft_with_name_but_no_note_or_points` |
| D33 added criterion copies comments | `the_rubric_after_adding_a_criterion` |
| D34 added criterion gets a name | `choices_follow_added_and_removed_criteria`, `the_rubric_after_adding_a_criterion` |
| D35 draft uses the raw (blank) name | `a_draft_with_blank_label_empty_comment_and_one_point`, `blank_paragraphs_are_left_out_and_comments_trimmed` |
| D36 start ignores the sample flag | `the_sample_is_four_criteria_by_four_levels_worth_one_to_four` |
| D37 saved file without final newline | `the_sample_saved_is_the_sample_file_byte_for_byte` |
| D38, D39, D41, D42, D43, D44 an edit ignored (show points, opening, note, points, remove, title) | each fails at least one test; remove fails `removing_down_to_one_criterion_and_then_the_last_is_refused` |

### `crates/core/src/api.rs` (7 tests) and `crates/core/src/page.rs` (6 tests), step 4

| Break | Tests that failed | First message |
| --- | --- | --- |
| A1 load drops the error message | `load_wraps_failure_in_error` | left: `{"error":""}` |
| A2 start not marked sample | `start_gives_state_and_view_with_the_sample_flag` | left: `Bool(false)` |
| A3 api skips the session check | `a_state_whose_choices_do_not_fit_the_rubric_is_refused` | `assertion failed: rubric_file(...).starts_with("{\"error\"")` |
| A4 action-error wording | `errors_come_back_as_messages` | `starts_with("The page sent an action the tool does not know (")` failed |
| A5, A6 file names fixed or empty | `the_rubric_file_and_the_draft_file_name` | left: `String("x.json")`; left: `{"ok":""}` |
| A7 act returns the old session | `act_chooses_and_returns_the_next_state`, `the_rubric_file_and_the_draft_file_name` | left: `Null` |
| A8 `criterion + 1` restored | `no_input_makes_any_function_panic` | `rubric.rs:292:17: attempt to add with overflow` (the real failure of row 9) |
| P1 blank strings allowed | `an_empty_title_is_rejected_by_name`, `an_empty_banner_is_rejected_by_name` | `unwrap_err()` on `Ok` |
| P2 unknown strings allowed | `an_unknown_field_is_rejected` | `matches!(..., Err(StringsError::NotJson(_)))` failed |
| P3 banner key renamed | `the_shipped_file_is_accepted` and 3 others | `unknown field \`sample_banner\`` |
| P4 `copy` optional | `a_missing_key_is_rejected` | `Empty("copy")` |
| P5 not-JSON reported as empty | `text_that_is_not_json_is_rejected`, `an_unknown_field_is_rejected`, `load_wraps_failure_in_error` | `left: "{\"error\":\"The strings file has an empty ..."` |

### `tests/browser/interactions.py` (52 checks), step 5

Each break was built with `scripts/build.sh` and checked in Chromium (`BUILDER_IDLE=0`). Every interaction check printed FAIL under at least one break.

| Break | Checks that failed |
| --- | --- |
| W1 view never marks the sample | the sample banner shows at start |
| X1 banner drawn from the intro string (page) | the banner says the rubric is sample content |
| W2 missing-message wording (core) | all four missing at start; Writing missing; Criterion 5 missing; opening clears the levels; Next student clears every level; the one criterion left is Writing |
| W3 one newline between paragraphs (core) | the draft matches the hand-written text, and 7 other draft comparisons |
| X2 never a draft (core) | the draft appears once every criterion has a level, and 6 draft comparisons |
| X3b missing message kept and shown | the missing line is gone once the draft appears |
| W4 comment edit ignored (core) | editing the comment changes the draft; saved rubric has the edited comment; reopened rubric has it; Next student keeps the rubric; 6 more |
| W5 show-points edit ignored (core) | Playwright `uncheck` timed out (row 17) |
| X7 choice labels always show points (core) | choice labels drop the points too |
| W6 refused typed points stored as 0 (core) | points of 2.5 are refused with a message |
| W15 title from intro, message never cleared (page) | title drawn; no error at start; valid points clear the message; the saved file opens without an error |
| X4 add does not rebuild (page) | five in the editor; five in the choices |
| X8 added criterion has blank labels (core) | the new criterion copies the level labels |
| W7 added criterion copies comments, points 0 (core) | copies the points; comments are empty |
| W8 remove ignored (core) | removing the new criterion leaves four |
| W9 copy status `?` (page) | Copy reports what happened |
| W10 file names fixed (core) | draft file named after the student; rubric file named after its title |
| W17 saved draft is the note (page) | the saved draft is the text in the box |
| X5 saved rubric gains a criterion (core) | the saved rubric has four criteria; the refused file leaves the rubric; 5 more |
| W11 open keeps sample and choices, clears the name (core) | banner gone after opening; opening clears the levels; keeps the student name; reopened rubric gives the same draft |
| W18 missing-field wording (core) | a file with a missing field is refused with a message |
| X9 an error drops the last editor fieldset (page) | the refused file leaves the rubric as it was |
| W12 Next student keeps everything (core) | clears the name; the note; every level; no level selected; no draft after Next student |
| W14 draft area always shown (page) | no draft before any level; while a criterion is missing; while the new criterion has none; after Next student |
| X6 removing criterion 1 ignored (core) | three removed leaves one; the last Remove button is disabled; the one left is Writing |
| W13 last criterion removable (core) | the last criterion's Remove button is disabled |
