//! Cases worked out by hand from the brief and `data/sample-rubric.json`
//! (C4). The expected values below were written before the tool produced
//! them, by reading the sample file and applying PLAN decisions 7 to 10.
//!
//! The set of choices used throughout ("the hand set"):
//!   Research question     -> Proficient (3)   level index 2
//!   Evidence and sources  -> Developing (2)   level index 1
//!   Structure             -> Excellent  (4)   level index 3
//!   Writing               -> Proficient (3)   level index 2
//! Total: 3 + 2 + 4 + 3 = 12. Possible: 4 + 4 + 4 + 4 = 16.

use tool_core::draft::{rubric_filename, Action, Session};
use tool_core::rubric::{Criterion, Level};

const SAMPLE: &str = include_str!("../../../data/sample-rubric.json");

fn sample() -> Session {
    Session::start(SAMPLE, true).expect("the sample rubric is valid")
}

fn act(s: Session, a: Action) -> Session {
    s.apply(a).expect("action accepted")
}

fn hand_set() -> Session {
    let mut s = sample();
    for (criterion, level) in [(0, 2), (1, 1), (2, 3), (3, 2)] {
        s = act(s, Action::Choose { criterion, level });
    }
    s
}

const FULL_DRAFT: &str = "Hello Jordan, here is my feedback on your project report.

Research question: Proficient (3 points). Your question is clear and focused enough to answer in the space you had.

Evidence and sources: Developing (2 points). You use some sources; check that each claim you rely on has one.

Structure: Excellent (4 points). The structure carries the argument: each section builds on the one before.

Writing: Proficient (3 points). Your writing is clear and mostly free of errors.

Total: 12 of 16 points.

I liked your choice of local data.

Come and see me if you want to talk any of this through.";

const BARE_DRAFT: &str = "Hello, here is my feedback on your project report.

Research question: Proficient. Your question is clear and focused enough to answer in the space you had.

Evidence and sources: Developing. You use some sources; check that each claim you rely on has one.

Structure: Excellent. The structure carries the argument: each section builds on the one before.

Writing: Proficient. Your writing is clear and mostly free of errors.

Come and see me if you want to talk any of this through.";

const NOTE_POINTS_NO_NAME: &str = "Hello, here is my feedback on your project report.

Research question: Proficient (3 points). Your question is clear and focused enough to answer in the space you had.

Evidence and sources: Developing (2 points). You use some sources; check that each claim you rely on has one.

Structure: Excellent (4 points). The structure carries the argument: each section builds on the one before.

Writing: Proficient (3 points). Your writing is clear and mostly free of errors.

Total: 12 of 16 points.

I liked your choice of local data.

Come and see me if you want to talk any of this through.";

const NAME_ONLY: &str = "Hello Jordan, here is my feedback on your project report.

Research question: Proficient. Your question is clear and focused enough to answer in the space you had.

Evidence and sources: Developing. You use some sources; check that each claim you rely on has one.

Structure: Excellent. The structure carries the argument: each section builds on the one before.

Writing: Proficient. Your writing is clear and mostly free of errors.

Come and see me if you want to talk any of this through.";

#[test]
fn the_sample_is_four_criteria_by_four_levels_worth_one_to_four() {
    let s = sample();
    let r = &s.rubric;
    assert!(r.title.starts_with("Sample"), "title marks it as sample");
    assert!(s.sample);
    assert!(r.show_points);
    assert_eq!(r.criteria.len(), 4);
    for c in &r.criteria {
        let points: Vec<u32> = c.levels.iter().map(|l| l.points).collect();
        assert_eq!(points, vec![1, 2, 3, 4], "{}", c.name);
        assert!(c.levels.iter().all(|l| !l.comment.trim().is_empty()));
    }
}

#[test]
fn full_draft_with_name_note_and_points() {
    let s = act(
        act(
            hand_set(),
            Action::SetName {
                text: "Jordan".into(),
            },
        ),
        Action::SetNote {
            text: "I liked your choice of local data.".into(),
        },
    );
    assert_eq!(s.draft().unwrap(), FULL_DRAFT);
}

#[test]
fn full_draft_without_name_note_or_points() {
    let s = act(hand_set(), Action::SetShowPoints { on: false });
    assert_eq!(s.draft().unwrap(), BARE_DRAFT);
}

#[test]
fn full_draft_with_note_and_points_but_no_name() {
    let s = act(
        hand_set(),
        Action::SetNote {
            text: "I liked your choice of local data.".into(),
        },
    );
    assert_eq!(s.draft().unwrap(), NOTE_POINTS_NO_NAME);
}

#[test]
fn full_draft_with_name_but_no_note_or_points() {
    let s = act(
        act(
            hand_set(),
            Action::SetName {
                text: "Jordan".into(),
            },
        ),
        Action::SetShowPoints { on: false },
    );
    assert_eq!(s.draft().unwrap(), NAME_ONLY);
}

#[test]
fn the_total_for_the_hand_set_is_12_of_16() {
    assert_eq!(hand_set().total(), Some((12, 16)));
}

#[test]
fn a_partial_set_names_the_missing_criteria_and_has_no_draft() {
    let mut s = sample();
    s = act(
        s,
        Action::Choose {
            criterion: 0,
            level: 2,
        },
    );
    s = act(
        s,
        Action::Choose {
            criterion: 2,
            level: 3,
        },
    );
    assert_eq!(s.missing(), vec!["Evidence and sources", "Writing"]);
    assert_eq!(
        s.missing_message().as_deref(),
        Some("Choose a level for: Evidence and sources, Writing.")
    );
    assert_eq!(s.draft(), None);
    assert_eq!(s.total(), None);
    let v = s.view();
    assert_eq!(v.draft, None);
    assert_eq!(v.missing, vec!["Evidence and sources", "Writing"]);
}

fn level(label: &str, points: u32) -> Level {
    Level {
        label: label.into(),
        points,
        comment: String::new(),
    }
}

#[test]
fn the_rubric_after_adding_a_criterion() {
    let before = sample().rubric;
    let s = act(hand_set(), Action::AddCriterion);
    let r = &s.rubric;
    assert_eq!(r.criteria.len(), 5);
    assert_eq!(&r.criteria[..4], &before.criteria[..]);
    assert_eq!(
        r.criteria[4],
        Criterion {
            name: String::new(),
            levels: vec![
                level("Beginning", 1),
                level("Developing", 2),
                level("Proficient", 3),
                level("Excellent", 4),
            ],
        }
    );
    assert_eq!(s.missing(), vec!["Criterion 5"]);
    assert_eq!(s.draft(), None);
    // Choosing Excellent for the new criterion: 12 + 4 of 16 + 4.
    let done = act(
        s,
        Action::Choose {
            criterion: 4,
            level: 3,
        },
    );
    assert_eq!(done.total(), Some((16, 20)));
}

#[test]
fn the_rubric_after_removing_a_criterion() {
    let before = sample().rubric;
    let s = act(hand_set(), Action::RemoveCriterion { criterion: 1 });
    let names: Vec<&str> = s.rubric.criteria.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, vec!["Research question", "Structure", "Writing"]);
    assert_eq!(s.rubric.criteria[0], before.criteria[0]);
    assert_eq!(s.rubric.criteria[1], before.criteria[2]);
    assert_eq!(s.rubric.criteria[2], before.criteria[3]);
    assert_eq!(s.choices, vec![Some(2), Some(3), Some(2)]);
    // 3 + 4 + 3 of 4 + 4 + 4.
    assert_eq!(s.total(), Some((10, 12)));
}

#[test]
fn removing_down_to_one_criterion_and_then_the_last_is_refused() {
    let mut s = sample();
    for _ in 0..3 {
        s = act(s, Action::RemoveCriterion { criterion: 0 });
    }
    assert_eq!(s.rubric.criteria.len(), 1);
    assert_eq!(s.rubric.criteria[0].name, "Writing");
    assert_eq!(
        s.apply(Action::RemoveCriterion { criterion: 0 })
            .unwrap_err(),
        "A rubric needs at least one criterion, so the last one cannot be removed."
    );
}

#[test]
fn editing_a_comment_changes_the_draft() {
    let s = act(
        hand_set(),
        Action::SetLevelComment {
            criterion: 3,
            level: 2,
            text: "Clear writing.".into(),
        },
    );
    let draft = s.draft().unwrap();
    assert!(draft.contains("Writing: Proficient (3 points). Clear writing.\n\n"));
    assert!(!draft.contains("mostly free of errors"));
}

#[test]
fn the_sample_saved_is_the_sample_file_byte_for_byte() {
    assert_eq!(sample().rubric.to_file_text(), SAMPLE);
    assert_eq!(
        rubric_filename(&sample().rubric),
        "sample-rubric-project-report.json"
    );
}

#[test]
fn next_student_after_the_full_draft() {
    let s = act(
        act(
            hand_set(),
            Action::SetName {
                text: "Jordan".into(),
            },
        ),
        Action::NextStudent,
    );
    assert_eq!(s.name, "");
    assert_eq!(s.choices, vec![None; 4]);
    assert_eq!(
        s.missing(),
        vec![
            "Research question",
            "Evidence and sources",
            "Structure",
            "Writing"
        ]
    );
    assert_eq!(s.rubric, sample().rubric);
}
