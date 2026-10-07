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
