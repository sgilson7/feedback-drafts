//! The rubric: what it holds, how a rubric file is read and checked, and every
//! edit the teacher can make to it. Messages that depend on a rule are written
//! beside the rule, and the page shows them as they are.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// A criterion has from `MIN_LEVELS` to `MAX_LEVELS` levels (the brief).
pub const MIN_LEVELS: usize = 2;
pub const MAX_LEVELS: usize = 6;
/// Points are whole numbers from 0 to `MAX_POINTS` (PLAN decision 3).
pub const MAX_POINTS: u32 = 1000;

/// The fields of a rubric file, in the order a saved file writes them.
const RUBRIC_FIELDS: &[&str] = &["title", "opening", "closing", "show_points", "criteria"];
const CRITERION_FIELDS: &[&str] = &["name", "levels"];
const LEVEL_FIELDS: &[&str] = &["label", "points", "comment"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Level {
    pub label: String,
    pub points: u32,
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Criterion {
    pub name: String,
    pub levels: Vec<Level>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rubric {
    pub title: String,
    pub opening: String,
    pub closing: String,
    pub show_points: bool,
    pub criteria: Vec<Criterion>,
}

impl Level {
    /// The label as the draft shows it: "Level N" when the label is blank.
    /// `index` counts from 0.
    pub fn display_label(&self, index: usize) -> String {
        let label = self.label.trim();
        if label.is_empty() {
            format!("Level {}", index + 1)
        } else {
            label.to_string()
        }
    }
}

impl Criterion {
    /// The name as the draft and the missing list show it: "Criterion N"
    /// when the name is blank. `index` counts from 0.
    pub fn display_name(&self, index: usize) -> String {
        let name = self.name.trim();
        if name.is_empty() {
            format!("Criterion {}", index + 1)
        } else {
            name.to_string()
        }
    }

    /// The most a student can get on this criterion: the highest points of
    /// any level, whatever order the levels are in.
    pub fn max_points(&self) -> u32 {
        self.levels.iter().map(|l| l.points).max().unwrap_or(0)
    }
}

/// "the rubric file" becomes "The rubric file" at the start of a sentence.
fn sentence(place: &str) -> String {
    let mut chars = place.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

fn as_object<'a>(value: &'a Value, place: &str) -> Result<&'a Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| format!("{} must be a JSON object.", sentence(place)))
}

fn only_fields(obj: &Map<String, Value>, allowed: &[&str], place: &str) -> Result<(), String> {
    match obj.keys().find(|k| !allowed.contains(&k.as_str())) {
        Some(key) => Err(format!(
            "{} has a field this tool does not use: \"{key}\".",
            sentence(place)
        )),
        None => Ok(()),
    }
}

fn field<'a>(obj: &'a Map<String, Value>, name: &str, place: &str) -> Result<&'a Value, String> {
    obj.get(name)
        .ok_or_else(|| format!("{} has no \"{name}\".", sentence(place)))
}

fn text_field(obj: &Map<String, Value>, name: &str, place: &str) -> Result<String, String> {
    field(obj, name, place)?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("In {place}, \"{name}\" must be text."))
}

fn bool_field(obj: &Map<String, Value>, name: &str, place: &str) -> Result<bool, String> {
    field(obj, name, place)?
        .as_bool()
        .ok_or_else(|| format!("In {place}, \"{name}\" must be true or false."))
}

fn list_field<'a>(
    obj: &'a Map<String, Value>,
    name: &str,
    place: &str,
) -> Result<&'a Vec<Value>, String> {
    field(obj, name, place)?
        .as_array()
        .ok_or_else(|| format!("In {place}, \"{name}\" must be a list."))
}

/// How a rejected points value is quoted back to the teacher.
fn shown(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{s}\""),
        other => other.to_string(),
    }
}

fn points_message(place: &str, found: &str) -> String {
    format!("In {place}, points must be a whole number from 0 to {MAX_POINTS}, not {found}.")
}

fn points_field(obj: &Map<String, Value>, place: &str) -> Result<u32, String> {
    let value = field(obj, "points", place)?;
    match value.as_u64() {
        Some(n) if n <= u64::from(MAX_POINTS) => Ok(n as u32),
        _ => Err(points_message(place, &shown(value))),
    }
}

fn level_count_message(place: &str, count: usize) -> String {
    let noun = if count == 1 { "level" } else { "levels" };
    format!(
        "{} has {count} {noun}; each criterion needs {MIN_LEVELS} to {MAX_LEVELS} levels.",
        sentence(place)
    )
}

fn read_level(value: &Value, place: &str) -> Result<Level, String> {
    let obj = as_object(value, place)?;
    only_fields(obj, LEVEL_FIELDS, place)?;
    Ok(Level {
        label: text_field(obj, "label", place)?,
        points: points_field(obj, place)?,
        comment: text_field(obj, "comment", place)?,
    })
}

fn read_criterion(value: &Value, index: usize) -> Result<Criterion, String> {
    let place = format!("criterion {}", index + 1);
    let obj = as_object(value, &place)?;
    only_fields(obj, CRITERION_FIELDS, &place)?;
    let name = text_field(obj, "name", &place)?;
    let list = list_field(obj, "levels", &place)?;
    if list.len() < MIN_LEVELS || list.len() > MAX_LEVELS {
        return Err(level_count_message(&place, list.len()));
    }
    let levels = list
        .iter()
        .enumerate()
        .map(|(j, v)| read_level(v, &format!("{place}, level {}", j + 1)))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Criterion { name, levels })
}

/// Reads and checks a rubric file. Every problem comes back as a message the
/// page shows; nothing here panics on any input.
pub fn parse_rubric(text: &str) -> Result<Rubric, String> {
    let value: Value = serde_json::from_str(text)
        .map_err(|e| format!("This file is not a rubric: it is not valid JSON ({e})."))?;
    let place = "the rubric file";
    let obj = as_object(&value, place)?;
    only_fields(obj, RUBRIC_FIELDS, place)?;
    let title = text_field(obj, "title", place)?;
    let opening = text_field(obj, "opening", place)?;
    let closing = text_field(obj, "closing", place)?;
    let show_points = bool_field(obj, "show_points", place)?;
    let list = list_field(obj, "criteria", place)?;
    if list.is_empty() {
        return Err("The rubric file has no criteria; a rubric needs at least one.".to_string());
    }
    let criteria = list
        .iter()
        .enumerate()
        .map(|(i, v)| read_criterion(v, i))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Rubric {
        title,
        opening,
        closing,
        show_points,
        criteria,
    })
}

/// Reads points the teacher typed into a box. Spaces around the number are
/// ignored; anything else that is not a whole number from 0 to `MAX_POINTS`
/// is refused.
pub fn parse_points(text: &str) -> Result<u32, String> {
    match text.trim().parse::<u32>() {
        Ok(n) if n <= MAX_POINTS => Ok(n),
        _ => Err(format!(
            "Points must be a whole number from 0 to {MAX_POINTS}, not \"{text}\"."
        )),
    }
}

impl Rubric {
    /// The text of a saved rubric file: the same format `parse_rubric` reads.
    pub fn to_file_text(&self) -> String {
        let mut text = serde_json::to_string_pretty(self).unwrap_or_default();
        text.push('\n');
        text
    }

    /// The highest total a student can reach: the sum of each criterion's
    /// highest points.
    pub fn max_total(&self) -> u64 {
        self.criteria
            .iter()
            .map(|c| u64::from(c.max_points()))
            .sum()
    }

    pub fn criterion_mut(&mut self, criterion: usize) -> Result<&mut Criterion, String> {
        self.criteria
            .get_mut(criterion)
            .ok_or_else(|| format!("There is no criterion {}.", criterion.saturating_add(1)))
    }

    pub fn level_mut(&mut self, criterion: usize, level: usize) -> Result<&mut Level, String> {
        let c = self.criterion_mut(criterion)?;
        c.levels.get_mut(level).ok_or_else(|| {
            format!(
                "Criterion {} has no level {}.",
                criterion.saturating_add(1),
                level.saturating_add(1)
            )
        })
    }

    pub fn set_points(&mut self, criterion: usize, level: usize, text: &str) -> Result<(), String> {
        let points = parse_points(text)?;
        self.level_mut(criterion, level)?.points = points;
        Ok(())
    }

    /// Appends a criterion with no name whose levels copy the first
    /// criterion's labels and points, with empty comments.
    pub fn add_criterion(&mut self) {
        let levels = self
            .criteria
            .first()
            .map(|first| {
                first
                    .levels
                    .iter()
                    .map(|l| Level {
                        label: l.label.clone(),
                        points: l.points,
                        comment: String::new(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.criteria.push(Criterion {
            name: String::new(),
            levels,
        });
    }

    /// Removes a criterion, but never the last one.
    pub fn remove_criterion(&mut self, criterion: usize) -> Result<(), String> {
        if criterion >= self.criteria.len() {
            return Err(format!(
                "There is no criterion {}.",
                criterion.saturating_add(1)
            ));
        }
        if self.criteria.len() == 1 {
            return Err(
                "A rubric needs at least one criterion, so the last one cannot be removed."
                    .to_string(),
            );
        }
        self.criteria.remove(criterion);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small valid rubric: one criterion, two levels.
    fn small() -> Value {
        serde_json::json!({
            "title": "T",
            "opening": "O",
            "closing": "C",
            "show_points": true,
            "criteria": [
                {"name": "N", "levels": [
                    {"label": "Low", "points": 0, "comment": "c0"},
                    {"label": "High", "points": 2, "comment": "c2"}
                ]}
            ]
        })
    }

    fn levels(n: usize) -> Value {
        Value::Array(
            (0..n)
                .map(|i| serde_json::json!({"label": format!("L{i}"), "points": i, "comment": ""}))
                .collect(),
        )
    }

    fn parse(v: &Value) -> Result<Rubric, String> {
        parse_rubric(&v.to_string())
    }

    #[test]
    fn a_small_rubric_is_read() {
        let r = parse(&small()).unwrap();
        assert_eq!(r.title, "T");
        assert!(r.show_points);
        assert_eq!(r.criteria.len(), 1);
        assert_eq!(
            r.criteria[0].levels[1],
            Level {
                label: "High".into(),
                points: 2,
                comment: "c2".into()
            }
        );
    }

    #[test]
    fn text_that_is_not_json_is_rejected_with_a_message() {
        let e = parse_rubric("title: T").unwrap_err();
        assert!(
            e.starts_with("This file is not a rubric: it is not valid JSON ("),
            "{e}"
        );
    }

    #[test]
    fn a_file_that_is_not_an_object_is_rejected() {
        assert_eq!(
            parse_rubric("[1, 2]").unwrap_err(),
            "The rubric file must be a JSON object."
        );
    }

    #[test]
    fn a_missing_top_level_field_is_named() {
        let mut v = small();
        v.as_object_mut().unwrap().remove("closing");
        assert_eq!(
            parse(&v).unwrap_err(),
            "The rubric file has no \"closing\"."
        );
    }

    #[test]
    fn a_missing_level_field_is_named_with_its_place() {
        let mut v = small();
        v["criteria"][0]["levels"][1]
            .as_object_mut()
            .unwrap()
            .remove("comment");
        assert_eq!(
            parse(&v).unwrap_err(),
            "Criterion 1, level 2 has no \"comment\"."
        );
    }

    #[test]
    fn a_missing_criterion_name_is_named() {
        let mut v = small();
        v["criteria"][0].as_object_mut().unwrap().remove("name");
        assert_eq!(parse(&v).unwrap_err(), "Criterion 1 has no \"name\".");
    }

    #[test]
    fn an_unknown_field_is_named() {
        let mut v = small();
        v["criteria"][0]["levels"][0]["comments"] = Value::from("x");
        assert_eq!(
            parse(&v).unwrap_err(),
            "Criterion 1, level 1 has a field this tool does not use: \"comments\"."
        );
    }

    #[test]
    fn wrong_types_are_named() {
        let mut v = small();
        v["title"] = Value::from(3);
        assert_eq!(
            parse(&v).unwrap_err(),
            "In the rubric file, \"title\" must be text."
        );
        let mut v = small();
        v["show_points"] = Value::from("yes");
        assert_eq!(
            parse(&v).unwrap_err(),
            "In the rubric file, \"show_points\" must be true or false."
        );
        let mut v = small();
        v["criteria"] = Value::from("none");
        assert_eq!(
            parse(&v).unwrap_err(),
            "In the rubric file, \"criteria\" must be a list."
        );
        let mut v = small();
        v["criteria"][0] = Value::from(7);
        assert_eq!(parse(&v).unwrap_err(), "Criterion 1 must be a JSON object.");
    }

    #[test]
    fn a_rubric_with_no_criteria_is_rejected() {
        let mut v = small();
        v["criteria"] = Value::Array(vec![]);
        assert_eq!(
            parse(&v).unwrap_err(),
            "The rubric file has no criteria; a rubric needs at least one."
        );
    }

    #[test]
    fn one_level_is_rejected() {
        let mut v = small();
        v["criteria"][0]["levels"] = levels(1);
        assert_eq!(
            parse(&v).unwrap_err(),
            "Criterion 1 has 1 level; each criterion needs 2 to 6 levels."
        );
    }

    #[test]
    fn seven_levels_are_rejected() {
        let mut v = small();
        v["criteria"][0]["levels"] = levels(7);
        assert_eq!(
            parse(&v).unwrap_err(),
            "Criterion 1 has 7 levels; each criterion needs 2 to 6 levels."
        );
    }

    #[test]
    fn two_and_six_levels_are_accepted() {
        for n in [2, 6] {
            let mut v = small();
            v["criteria"][0]["levels"] = levels(n);
            assert_eq!(parse(&v).unwrap().criteria[0].levels.len(), n);
        }
    }

    #[test]
    fn negative_points_are_rejected() {
        let mut v = small();
        v["criteria"][0]["levels"][1]["points"] = Value::from(-1);
        assert_eq!(
            parse(&v).unwrap_err(),
            "In criterion 1, level 2, points must be a whole number from 0 to 1000, not -1."
        );
    }

    #[test]
    fn non_integer_points_are_rejected() {
        let v = small()
            .to_string()
            .replace("\"points\":2", "\"points\":2.5");
        assert_eq!(
            parse_rubric(&v).unwrap_err(),
            "In criterion 1, level 2, points must be a whole number from 0 to 1000, not 2.5."
        );
    }

    #[test]
    fn points_as_text_or_above_the_limit_are_rejected() {
        let mut v = small();
        v["criteria"][0]["levels"][0]["points"] = Value::from("3");
        assert_eq!(
            parse(&v).unwrap_err(),
            "In criterion 1, level 1, points must be a whole number from 0 to 1000, not \"3\"."
        );
        let mut v = small();
        v["criteria"][0]["levels"][0]["points"] = Value::from(1001);
        assert!(parse(&v).unwrap_err().ends_with("not 1001."));
    }

    #[test]
    fn zero_and_the_limit_are_accepted_as_points() {
        let mut v = small();
        v["criteria"][0]["levels"][1]["points"] = Value::from(1000);
        let r = parse(&v).unwrap();
        assert_eq!(r.criteria[0].levels[0].points, 0);
        assert_eq!(r.criteria[0].levels[1].points, 1000);
    }

    #[test]
    fn typed_points_are_parsed_or_refused() {
        assert_eq!(parse_points(" 7 "), Ok(7));
        assert_eq!(parse_points("0"), Ok(0));
        assert_eq!(parse_points("1000"), Ok(1000));
        for bad in ["2.5", "-1", "", "three", "1001"] {
            assert_eq!(
                parse_points(bad),
                Err(format!(
                    "Points must be a whole number from 0 to 1000, not \"{bad}\"."
                ))
            );
        }
    }

    #[test]
    fn refused_points_leave_the_rubric_unchanged() {
        let mut r = parse(&small()).unwrap();
        let before = r.clone();
        assert!(r.set_points(0, 1, "2.5").is_err());
        assert_eq!(r, before);
        r.set_points(0, 1, "5").unwrap();
        assert_eq!(r.criteria[0].levels[1].points, 5);
    }

    #[test]
    fn edits_out_of_range_are_refused() {
        let mut r = parse(&small()).unwrap();
        assert_eq!(
            r.set_points(3, 0, "1").unwrap_err(),
            "There is no criterion 4."
        );
        assert_eq!(
            r.set_points(0, 8, "1").unwrap_err(),
            "Criterion 1 has no level 9."
        );
        assert_eq!(
            r.remove_criterion(5).unwrap_err(),
            "There is no criterion 6."
        );
    }

    #[test]
    fn the_last_criterion_cannot_be_removed() {
        let mut r = parse(&small()).unwrap();
        let before = r.clone();
        assert_eq!(
            r.remove_criterion(0).unwrap_err(),
            "A rubric needs at least one criterion, so the last one cannot be removed."
        );
        assert_eq!(r, before);
    }

    #[test]
    fn display_names_fall_back_when_blank() {
        let c = Criterion {
            name: "  ".into(),
            levels: vec![],
        };
        assert_eq!(c.display_name(4), "Criterion 5");
        let l = Level {
            label: String::new(),
            points: 0,
            comment: String::new(),
        };
        assert_eq!(l.display_label(1), "Level 2");
        let named = Level {
            label: " Good ".into(),
            points: 0,
            comment: String::new(),
        };
        assert_eq!(named.display_label(1), "Good");
    }

    #[test]
    fn max_points_is_the_highest_level_in_any_order() {
        let c = Criterion {
            name: "N".into(),
            levels: vec![
                Level {
                    label: "a".into(),
                    points: 5,
                    comment: String::new(),
                },
                Level {
                    label: "b".into(),
                    points: 2,
                    comment: String::new(),
                },
            ],
        };
        assert_eq!(c.max_points(), 5);
    }

    #[test]
    fn a_saved_file_reads_back_the_same() {
        let r = parse(&small()).unwrap();
        assert_eq!(parse_rubric(&r.to_file_text()).unwrap(), r);
    }
}
