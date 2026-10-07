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
