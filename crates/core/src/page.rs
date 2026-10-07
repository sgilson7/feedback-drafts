//! The page's interface text comes from `data/strings.json`, and the core
//! decides whether that file is usable before the page draws it: every key the
//! page needs is present, no other key is, and none is blank.

use serde::{Deserialize, Serialize};

/// The reader-visible interface strings. They live in data, not source.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Strings {
    pub title: String,
    pub intro: String,
    pub privacy: String,
    pub sample_banner: String,
    pub rubric_heading: String,
    pub open_rubric: String,
    pub save_rubric: String,
    pub rubric_title: String,
    pub opening: String,
    pub opening_hint: String,
    pub closing: String,
    pub show_points: String,
    pub criterion_name: String,
    pub level_label: String,
    pub level_points: String,
    pub level_comment: String,
    pub add_criterion: String,
    pub remove_criterion: String,
    pub student_heading: String,
    pub student_name: String,
    pub choices_heading: String,
    pub note: String,
    pub draft_heading: String,
    pub draft_hint: String,
    pub copy: String,
    pub copied: String,
    pub copy_failed: String,
    pub save_draft: String,
    pub next_student: String,
}

/// Why a strings file cannot be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringsError {
    NotJson(String),
    Empty(String),
}

impl StringsError {
    /// The message the page shows. Wording that depends on the rule stays
    /// beside the rule.
    pub fn message(&self) -> String {
        match self {
            StringsError::NotJson(detail) => {
                format!("The strings file could not be read: {detail}")
            }
            StringsError::Empty(field) => format!("The strings file has an empty \"{field}\"."),
        }
    }
}

/// Parses and checks the strings file. A field that is only whitespace counts
/// as empty, because the page would draw nothing. When several are empty, the
/// first in alphabetical order is named.
pub fn parse_strings(json: &str) -> Result<Strings, StringsError> {
    let strings: Strings =
        serde_json::from_str(json).map_err(|e| StringsError::NotJson(e.to_string()))?;
    let fields =
        serde_json::to_value(&strings).map_err(|e| StringsError::NotJson(e.to_string()))?;
    let blank = fields.as_object().and_then(|map| {
        map.iter()
            .find(|(_, v)| v.as_str().map(|s| s.trim().is_empty()).unwrap_or(true))
            .map(|(k, _)| k.clone())
    });
    match blank {
        Some(field) => Err(StringsError::Empty(field)),
        None => Ok(strings),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    const FILE: &str = include_str!("../../../data/strings.json");

    fn with(field: &str, value: &str) -> String {
        let mut v: Value = serde_json::from_str(FILE).unwrap();
        v[field] = Value::from(value);
        v.to_string()
    }

    #[test]
    fn the_shipped_file_is_accepted() {
        let s = parse_strings(FILE).unwrap();
        assert!(!s.title.is_empty());
        assert!(s.sample_banner.to_lowercase().contains("sample"));
    }

    #[test]
    fn an_empty_title_is_rejected_by_name() {
        let e = parse_strings(&with("title", "  ")).unwrap_err();
        assert_eq!(e, StringsError::Empty("title".into()));
        assert_eq!(e.message(), "The strings file has an empty \"title\".");
    }

    #[test]
    fn an_empty_banner_is_rejected_by_name() {
        let e = parse_strings(&with("sample_banner", "")).unwrap_err();
        assert_eq!(e, StringsError::Empty("sample_banner".into()));
    }

    #[test]
    fn a_missing_key_is_rejected() {
        let mut v: Value = serde_json::from_str(FILE).unwrap();
        v.as_object_mut().unwrap().remove("copy");
        let e = parse_strings(&v.to_string()).unwrap_err();
        assert!(e.message().contains("missing field `copy`"), "{e:?}");
    }

    #[test]
    fn an_unknown_field_is_rejected() {
        assert!(matches!(
            parse_strings(&with("extra", "x")),
            Err(StringsError::NotJson(_))
        ));
    }

    #[test]
    fn text_that_is_not_json_is_rejected() {
        assert!(matches!(
            parse_strings("title: T"),
            Err(StringsError::NotJson(_))
        ));
    }
}
