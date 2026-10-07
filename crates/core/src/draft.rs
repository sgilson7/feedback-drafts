//! One student's feedback: the session (rubric, name, chosen levels, note),
//! every action the page can ask for, the criteria still missing, the total,
//! the composed draft, and what the page draws.

use serde::{Deserialize, Serialize};

use crate::rubric::{parse_rubric, Rubric};

/// Where the student's name goes in the opening line.
pub const NAME_MARKER: &str = "{name}";

/// Everything the tool knows while the page is open. The page keeps this
/// object as the core returned it and hands it back with the next action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Session {
    pub rubric: Rubric,
    /// True while the rubric is the sample the page loaded at start.
    pub sample: bool,
    pub name: String,
    /// One entry per criterion: the chosen level's index, or none.
    pub choices: Vec<Option<usize>>,
    pub note: String,
}

/// What the teacher did. Indices count from 0 and come from the view.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Action {
    SetName {
        text: String,
    },
    SetNote {
        text: String,
    },
    Choose {
        criterion: usize,
        level: usize,
    },
    SetTitle {
        text: String,
    },
    SetOpening {
        text: String,
    },
    SetClosing {
        text: String,
    },
    SetShowPoints {
        on: bool,
    },
    SetCriterionName {
        criterion: usize,
        text: String,
    },
    SetLevelLabel {
        criterion: usize,
        level: usize,
        text: String,
    },
    SetLevelPoints {
        criterion: usize,
        level: usize,
        text: String,
    },
    SetLevelComment {
        criterion: usize,
        level: usize,
        text: String,
    },
    AddCriterion,
    RemoveCriterion {
        criterion: usize,
    },
    NextStudent,
    OpenRubric {
        text: String,
    },
}

/// One level as the page draws it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LevelView {
    pub label: String,
    pub points: u32,
    pub comment: String,
    /// The text beside the level's choice button.
    pub choice_text: String,
    pub chosen: bool,
}

/// One criterion as the page draws it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CriterionView {
    pub name: String,
    pub display_name: String,
    pub levels: Vec<LevelView>,
}

/// Everything the page draws, decided here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct View {
    pub sample: bool,
    pub title: String,
    pub opening: String,
    pub closing: String,
    pub show_points: bool,
    pub name: String,
    pub note: String,
    pub criteria: Vec<CriterionView>,
    pub can_remove: bool,
    pub missing: Vec<String>,
    pub missing_message: Option<String>,
    pub draft: Option<String>,
}

/// "1 point", "3 points".
pub fn points_phrase(points: u32) -> String {
    if points == 1 {
        "1 point".to_string()
    } else {
        format!("{points} points")
    }
}

/// The opening line with the student's name (PLAN decision 10).
pub fn opening_with_name(opening: &str, name: &str) -> String {
    let name = name.trim();
    if opening.contains(NAME_MARKER) {
        if name.is_empty() {
            opening
                .replace(&format!(" {NAME_MARKER}"), "")
                .replace(NAME_MARKER, "")
        } else {
            opening.replace(NAME_MARKER, name)
        }
    } else if name.is_empty() {
        opening.to_string()
    } else {
        format!("{name}, {opening}")
    }
}

/// Lowercase letters and digits, with every run of anything else turned into
/// one `-`, and no `-` at either end.
pub fn slug(text: &str) -> String {
    let mut out = String::new();
    let mut gap = false;
    for c in text.chars() {
        if c.is_alphanumeric() {
            if gap && !out.is_empty() {
                out.push('-');
            }
            gap = false;
            out.extend(c.to_lowercase());
        } else {
            gap = true;
        }
    }
    out
}

/// The file name a saved rubric gets.
pub fn rubric_filename(rubric: &Rubric) -> String {
    let s = slug(&rubric.title);
    if s.is_empty() {
        "rubric.json".to_string()
    } else {
        format!("{s}.json")
    }
}

impl Session {
    /// A new session on a rubric file. `sample` says the file is the sample.
    pub fn start(text: &str, sample: bool) -> Result<Session, String> {
        let rubric = parse_rubric(text)?;
        Ok(Session {
            choices: vec![None; rubric.criteria.len()],
            rubric,
            sample,
            name: String::new(),
            note: String::new(),
        })
    }

    /// Checks a session the page handed back: one choice per criterion, and
    /// every choice names a level that exists.
    pub fn check(&self) -> Result<(), String> {
        let aligned = self.choices.len() == self.rubric.criteria.len();
        let in_range = self
            .choices
            .iter()
            .zip(&self.rubric.criteria)
            .all(|(choice, c)| choice.map(|j| j < c.levels.len()).unwrap_or(true));
        if aligned && in_range {
            Ok(())
        } else {
            Err("The page sent a state the tool cannot use. Reload the page.".to_string())
        }
    }

    /// The session after one action, or a message and no change.
    pub fn apply(&self, action: Action) -> Result<Session, String> {
        self.check()?;
        let mut s = self.clone();
        match action {
            Action::SetName { text } => s.name = text,
            Action::SetNote { text } => s.note = text,
            Action::Choose { criterion, level } => {
                s.rubric.level_mut(criterion, level)?;
                if let Some(choice) = s.choices.get_mut(criterion) {
                    *choice = Some(level);
                }
            }
            Action::SetTitle { text } => s.rubric.title = text,
            Action::SetOpening { text } => s.rubric.opening = text,
            Action::SetClosing { text } => s.rubric.closing = text,
            Action::SetShowPoints { on } => s.rubric.show_points = on,
            Action::SetCriterionName { criterion, text } => {
                s.rubric.criterion_mut(criterion)?.name = text;
            }
            Action::SetLevelLabel {
                criterion,
                level,
                text,
            } => s.rubric.level_mut(criterion, level)?.label = text,
            Action::SetLevelPoints {
                criterion,
                level,
                text,
            } => s.rubric.set_points(criterion, level, &text)?,
            Action::SetLevelComment {
                criterion,
                level,
                text,
            } => s.rubric.level_mut(criterion, level)?.comment = text,
            Action::AddCriterion => {
                s.rubric.add_criterion();
                s.choices.push(None);
            }
            Action::RemoveCriterion { criterion } => {
                s.rubric.remove_criterion(criterion)?;
                s.choices.remove(criterion);
            }
            Action::NextStudent => {
                s.name.clear();
                s.note.clear();
                s.choices = vec![None; s.rubric.criteria.len()];
            }
            Action::OpenRubric { text } => {
                let rubric = parse_rubric(&text)?;
                s.choices = vec![None; rubric.criteria.len()];
                s.rubric = rubric;
                s.sample = false;
            }
        }
        Ok(s)
    }

    /// The display names of the criteria with no level chosen, in order.
    pub fn missing(&self) -> Vec<String> {
        self.rubric
            .criteria
            .iter()
            .enumerate()
            .filter(|(i, _)| self.choices.get(*i).copied().flatten().is_none())
            .map(|(i, c)| c.display_name(i))
            .collect()
    }

    /// "Choose a level for: A, B." while any criterion is missing.
    pub fn missing_message(&self) -> Option<String> {
        let missing = self.missing();
        if missing.is_empty() {
            None
        } else {
            Some(format!("Choose a level for: {}.", missing.join(", ")))
        }
    }

    /// (points earned, points possible) once every criterion has a level.
    pub fn total(&self) -> Option<(u64, u64)> {
        let mut earned = 0;
        for (c, choice) in self.rubric.criteria.iter().zip(&self.choices) {
            earned += u64::from(c.levels.get((*choice)?)?.points);
        }
        Some((earned, self.rubric.max_total()))
    }

    /// The feedback draft, once every criterion has a level (PLAN decision 9).
    pub fn draft(&self) -> Option<String> {
        if !self.missing().is_empty() {
            return None;
        }
        let r = &self.rubric;
        let mut paragraphs = vec![opening_with_name(&r.opening, &self.name)];
        for (i, (c, choice)) in r.criteria.iter().zip(&self.choices).enumerate() {
            let j = (*choice)?;
            let level = c.levels.get(j)?;
            let mut head = format!("{}: {}", c.display_name(i), level.display_label(j));
            if r.show_points {
                head = format!("{head} ({})", points_phrase(level.points));
            }
            let comment = level.comment.trim();
            paragraphs.push(if comment.is_empty() {
                format!("{head}.")
            } else {
                format!("{head}. {comment}")
            });
        }
        if r.show_points {
            let (earned, possible) = self.total()?;
            paragraphs.push(format!("Total: {earned} of {possible} points."));
        }
        paragraphs.push(self.note.clone());
        paragraphs.push(r.closing.clone());
        let kept: Vec<&str> = paragraphs
            .iter()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();
        Some(kept.join("\n\n"))
    }

    /// The file name a saved draft gets: "feedback-<name>.txt".
    pub fn draft_filename(&self) -> String {
        let s = slug(&self.name);
        if s.is_empty() {
            "feedback.txt".to_string()
        } else {
            format!("feedback-{s}.txt")
        }
    }

    /// Everything the page draws.
    pub fn view(&self) -> View {
        let r = &self.rubric;
        let criteria = r
            .criteria
            .iter()
            .enumerate()
            .map(|(i, c)| CriterionView {
                name: c.name.clone(),
                display_name: c.display_name(i),
                levels: c
                    .levels
                    .iter()
                    .enumerate()
                    .map(|(j, l)| LevelView {
                        label: l.label.clone(),
                        points: l.points,
                        comment: l.comment.clone(),
                        choice_text: if r.show_points {
                            format!("{} ({})", l.display_label(j), points_phrase(l.points))
                        } else {
                            l.display_label(j)
                        },
                        chosen: self.choices.get(i).copied().flatten() == Some(j),
                    })
                    .collect(),
            })
            .collect();
        View {
            sample: self.sample,
            title: r.title.clone(),
            opening: r.opening.clone(),
            closing: r.closing.clone(),
            show_points: r.show_points,
            name: self.name.clone(),
            note: self.note.clone(),
            criteria,
            can_remove: r.criteria.len() > 1,
            missing: self.missing(),
            missing_message: self.missing_message(),
            draft: self.draft(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO: &str = r#"{"title": "Unit 3: Bridges!", "opening": "Hi {name}, thanks.", "closing": "Bye.", "show_points": true,
        "criteria": [
          {"name": "A", "levels": [{"label": "Lo", "points": 1, "comment": "a1"}, {"label": "Hi", "points": 5, "comment": " a5 "}]},
          {"name": "", "levels": [{"label": "", "points": 0, "comment": ""}, {"label": "Y", "points": 2, "comment": "b2"}]}
        ]}"#;

    fn two() -> Session {
        Session::start(TWO, false).unwrap()
    }

    fn act(s: &Session, a: Action) -> Session {
        s.apply(a).unwrap()
    }

    fn chosen(choices: &[(usize, usize)]) -> Session {
        let mut s = two();
        for &(criterion, level) in choices {
            s = act(&s, Action::Choose { criterion, level });
        }
        s
    }

    #[test]
    fn the_marker_takes_the_name_or_goes_with_its_space() {
        assert_eq!(
            opening_with_name("Hello {name}, here.", " Jordan "),
            "Hello Jordan, here."
        );
        assert_eq!(
            opening_with_name("Hello {name}, here.", "  "),
            "Hello, here."
        );
        assert_eq!(opening_with_name("{name}: notes", ""), ": notes");
    }

    #[test]
    fn without_a_marker_the_name_goes_in_front() {
        assert_eq!(
            opening_with_name("Thanks for the report.", "Ana"),
            "Ana, Thanks for the report."
        );
        assert_eq!(
            opening_with_name("Thanks for the report.", ""),
            "Thanks for the report."
        );
    }

    #[test]
    fn one_point_is_singular() {
        assert_eq!(points_phrase(1), "1 point");
        assert_eq!(points_phrase(0), "0 points");
        assert_eq!(points_phrase(2), "2 points");
    }

    #[test]
    fn slugs_and_file_names() {
        assert_eq!(slug("  Unit 3: Bridges! "), "unit-3-bridges");
        assert_eq!(slug("Zoë O'Neil"), "zoë-o-neil");
        assert_eq!(slug("--"), "");
        let s = two();
        assert_eq!(rubric_filename(&s.rubric), "unit-3-bridges.json");
        assert_eq!(s.draft_filename(), "feedback.txt");
        let named = act(
            &s,
            Action::SetName {
                text: "Zoë O'Neil".into(),
            },
        );
        assert_eq!(named.draft_filename(), "feedback-zoë-o-neil.txt");
        let untitled = act(&s, Action::SetTitle { text: " ".into() });
        assert_eq!(rubric_filename(&untitled.rubric), "rubric.json");
    }

    #[test]
    fn no_draft_and_no_total_until_every_criterion_has_a_level() {
        let s = chosen(&[(0, 1)]);
        assert_eq!(s.missing(), vec!["Criterion 2"]);
        assert_eq!(
            s.missing_message().as_deref(),
            Some("Choose a level for: Criterion 2.")
        );
        assert_eq!(s.draft(), None);
        assert_eq!(s.total(), None);
    }

    #[test]
    fn a_draft_with_blank_label_empty_comment_and_one_point() {
        let s = act(
            &chosen(&[(0, 0), (1, 0)]),
            Action::SetName { text: "Al".into() },
        );
        assert_eq!(s.total(), Some((1, 7)));
        assert_eq!(s.missing_message(), None);
        assert_eq!(
            s.draft().unwrap(),
            "Hi Al, thanks.\n\nA: Lo (1 point). a1\n\nCriterion 2: Level 1 (0 points).\n\nTotal: 1 of 7 points.\n\nBye."
        );
    }

    #[test]
    fn blank_paragraphs_are_left_out_and_comments_trimmed() {
        let mut s = chosen(&[(0, 1), (1, 1)]);
        s = act(&s, Action::SetOpening { text: "  ".into() });
        s = act(&s, Action::SetClosing { text: "".into() });
        s = act(&s, Action::SetShowPoints { on: false });
        s = act(
            &s,
            Action::SetNote {
                text: "  \n ".into(),
            },
        );
        assert_eq!(s.draft().unwrap(), "A: Hi. a5\n\nCriterion 2: Y. b2");
    }

    #[test]
    fn choosing_a_level_that_does_not_exist_is_refused() {
        let s = two();
        assert_eq!(
            s.apply(Action::Choose {
                criterion: 0,
                level: 2
            })
            .unwrap_err(),
            "Criterion 1 has no level 3."
        );
        assert_eq!(
            s.apply(Action::Choose {
                criterion: 2,
                level: 0
            })
            .unwrap_err(),
            "There is no criterion 3."
        );
    }

    #[test]
    fn choices_follow_added_and_removed_criteria() {
        let s = chosen(&[(0, 1), (1, 0)]);
        let added = act(&s, Action::AddCriterion);
        assert_eq!(added.choices, vec![Some(1), Some(0), None]);
        assert_eq!(added.missing(), vec!["Criterion 3"]);
        let removed = act(&added, Action::RemoveCriterion { criterion: 0 });
        assert_eq!(removed.choices, vec![Some(0), None]);
        assert_eq!(removed.rubric.criteria[0].name, "");
    }

    #[test]
    fn next_student_clears_name_levels_and_note_and_keeps_the_rubric() {
        let mut s = chosen(&[(0, 1), (1, 0)]);
        s.sample = true;
        s = act(&s, Action::SetName { text: "Al".into() });
        s = act(&s, Action::SetNote { text: "n".into() });
        s = act(
            &s,
            Action::SetTitle {
                text: "Mine".into(),
            },
        );
        let next = act(&s, Action::NextStudent);
        assert_eq!(next.name, "");
        assert_eq!(next.note, "");
        assert_eq!(next.choices, vec![None, None]);
        assert_eq!(next.rubric, s.rubric);
        assert!(next.sample);
    }

    #[test]
    fn opening_a_file_resets_levels_keeps_name_and_note_and_ends_the_sample() {
        let mut s = chosen(&[(0, 1), (1, 0)]);
        s.sample = true;
        s = act(&s, Action::SetName { text: "Al".into() });
        s = act(&s, Action::SetNote { text: "n".into() });
        let file = TWO.replace("Bye.", "Later.");
        let opened = act(&s, Action::OpenRubric { text: file });
        assert_eq!(opened.rubric.closing, "Later.");
        assert_eq!(opened.choices, vec![None, None]);
        assert_eq!((opened.name.as_str(), opened.note.as_str()), ("Al", "n"));
        assert!(!opened.sample);
    }

    #[test]
    fn a_bad_file_or_bad_points_leaves_the_session_unchanged() {
        let s = chosen(&[(0, 1)]);
        assert_eq!(
            s.apply(Action::OpenRubric { text: "[]".into() })
                .unwrap_err(),
            "The rubric file must be a JSON object."
        );
        assert!(s
            .apply(Action::SetLevelPoints {
                criterion: 0,
                level: 0,
                text: "-1".into()
            })
            .is_err());
        assert_eq!(s, chosen(&[(0, 1)]));
    }

    #[test]
    fn edits_reach_the_rubric() {
        let mut s = two();
        s = act(
            &s,
            Action::SetCriterionName {
                criterion: 1,
                text: "B".into(),
            },
        );
        s = act(
            &s,
            Action::SetLevelLabel {
                criterion: 1,
                level: 0,
                text: "X".into(),
            },
        );
        s = act(
            &s,
            Action::SetLevelPoints {
                criterion: 1,
                level: 0,
                text: "4".into(),
            },
        );
        s = act(
            &s,
            Action::SetLevelComment {
                criterion: 1,
                level: 0,
                text: "bx".into(),
            },
        );
        let c = &s.rubric.criteria[1];
        assert_eq!(c.name, "B");
        assert_eq!(
            (
                c.levels[0].label.as_str(),
                c.levels[0].points,
                c.levels[0].comment.as_str()
            ),
            ("X", 4, "bx")
        );
    }

    #[test]
    fn a_session_whose_choices_do_not_fit_is_refused() {
        let mut s = two();
        assert_eq!(s.check(), Ok(()));
        s.choices = vec![None];
        assert!(s.check().is_err());
        s.choices = vec![None, Some(2)];
        assert!(s.check().is_err());
        s.choices = vec![None, Some(1)];
        assert_eq!(s.check(), Ok(()));
    }

    #[test]
    fn apply_refuses_a_session_that_does_not_fit() {
        let mut s = two();
        s.choices = vec![Some(0)];
        assert_eq!(
            s.apply(Action::NextStudent).unwrap_err(),
            "The page sent a state the tool cannot use. Reload the page."
        );
    }

    #[test]
    fn the_view_marks_choices_and_whether_removal_is_allowed() {
        let v = chosen(&[(0, 1)]).view();
        assert!(!v.criteria[0].levels[0].chosen);
        assert!(v.criteria[0].levels[1].chosen);
        assert_eq!(v.criteria[0].levels[1].choice_text, "Hi (5 points)");
        assert_eq!(v.criteria[1].levels[0].choice_text, "Level 1 (0 points)");
        assert_eq!(v.criteria[1].display_name, "Criterion 2");
        assert!(v.can_remove);
        assert_eq!(v.draft, None);
        let one = act(&two(), Action::RemoveCriterion { criterion: 1 }).view();
        assert!(!one.can_remove);
        let hidden = act(&two(), Action::SetShowPoints { on: false }).view();
        assert_eq!(hidden.criteria[0].levels[1].choice_text, "Hi");
    }
}
