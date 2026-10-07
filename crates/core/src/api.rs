//! The one door into the core. Every function takes strings and returns a JSON
//! string: `{"ok": ...}` on success or `{"error": "..."}` with a message the
//! page can show as it is. The shim forwards these calls and nothing else.
//!
//! The page holds the `state` object from the last `{"ok": {"state", "view"}}`
//! and hands it back, unchanged, with the next call. On an error it keeps the
//! state it had.

use serde::Serialize;

use crate::draft::{rubric_filename, Action, Session, View};
use crate::page;

fn ok<T: Serialize>(value: &T) -> String {
    serde_json::json!({ "ok": value }).to_string()
}

fn error(message: String) -> String {
    serde_json::json!({ "error": message }).to_string()
}

fn reply(result: Result<String, String>) -> String {
    result.unwrap_or_else(error)
}

/// What every session call returns: the new state and what to draw.
#[derive(Serialize)]
struct Step<'a> {
    state: &'a Session,
    view: View,
}

fn step(session: &Session) -> String {
    ok(&Step {
        state: session,
        view: session.view(),
    })
}

#[derive(Serialize)]
struct SavedFile {
    filename: String,
    text: String,
}

fn read_session(state_json: &str) -> Result<Session, String> {
    let session: Session = serde_json::from_str(state_json)
        .map_err(|_| "The page sent a state the tool cannot use. Reload the page.".to_string())?;
    session.check()?;
    Ok(session)
}

/// Checks the strings file and returns the strings the page should draw.
pub fn load(strings_json: &str) -> String {
    match page::parse_strings(strings_json) {
        Ok(strings) => ok(&strings),
        Err(e) => error(e.message()),
    }
}

/// Starts a session on the sample rubric the page loads by default.
pub fn start(sample_json: &str) -> String {
    reply(Session::start(sample_json, true).map(|s| step(&s)))
}

/// Applies one action (see `draft::Action`) to the session.
pub fn act(state_json: &str, action_json: &str) -> String {
    reply(read_session(state_json).and_then(|session| {
        let action: Action = serde_json::from_str(action_json)
            .map_err(|e| format!("The page sent an action the tool does not know ({e})."))?;
        session.apply(action).map(|s| step(&s))
    }))
}

/// The rubric as a file to save: its name and its text.
pub fn rubric_file(state_json: &str) -> String {
    reply(read_session(state_json).map(|s| {
        ok(&SavedFile {
            filename: rubric_filename(&s.rubric),
            text: s.rubric.to_file_text(),
        })
    }))
}

/// The file name for the draft saved as text.
pub fn draft_filename(state_json: &str) -> String {
    reply(read_session(state_json).map(|s| ok(&s.draft_filename())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    const SAMPLE: &str = include_str!("../../../data/sample-rubric.json");

    fn parse(s: &str) -> Value {
        serde_json::from_str(s).unwrap()
    }

    fn started() -> Value {
        parse(&start(SAMPLE))
    }

    fn state_of(reply: &Value) -> String {
        reply["ok"]["state"].to_string()
    }

    #[test]
    fn load_wraps_failure_in_error() {
        assert_eq!(
            load(r#"{"title": "", "intro": "I"}"#),
            r#"{"error":"The strings file could not be read: missing field `privacy` at line 1 column 27"}"#
        );
    }

    #[test]
    fn start_gives_state_and_view_with_the_sample_flag() {
        let r = started();
        assert_eq!(r["ok"]["view"]["sample"], Value::Bool(true));
        assert_eq!(r["ok"]["state"]["sample"], Value::Bool(true));
        assert_eq!(r["ok"]["view"]["draft"], Value::Null);
        assert_eq!(
            r["ok"]["view"]["missing_message"],
            "Choose a level for: Research question, Evidence and sources, Structure, Writing."
        );
    }

    #[test]
    fn act_chooses_and_returns_the_next_state() {
        let r = started();
        let next = parse(&act(
            &state_of(&r),
            r#"{"kind": "choose", "criterion": 1, "level": 3}"#,
        ));
        assert_eq!(next["ok"]["state"]["choices"][1], 3);
        assert_eq!(
            next["ok"]["view"]["criteria"][1]["levels"][3]["chosen"],
            true
        );
    }

    #[test]
    fn errors_come_back_as_messages() {
        let state = state_of(&started());
        assert!(
            parse(&act(&state, r#"{"kind": "remove_everything"}"#))["error"]
                .as_str()
                .unwrap()
                .starts_with("The page sent an action the tool does not know (")
        );
        assert_eq!(
            parse(&act(&state, r#"{"kind": "open_rubric", "text": "{}"}"#)),
            serde_json::json!({"error": "The rubric file has no \"title\"."})
        );
        assert_eq!(
            parse(&act("{}", r#"{"kind": "next_student"}"#)),
            serde_json::json!({"error": "The page sent a state the tool cannot use. Reload the page."})
        );
    }

    #[test]
    fn a_state_whose_choices_do_not_fit_the_rubric_is_refused() {
        let mut state = started()["ok"]["state"].clone();
        state["choices"] = serde_json::json!([null]);
        assert_eq!(
            parse(&act(&state.to_string(), r#"{"kind": "next_student"}"#)),
            serde_json::json!({"error": "The page sent a state the tool cannot use. Reload the page."})
        );
        assert!(rubric_file(&state.to_string()).starts_with("{\"error\""));
    }

    #[test]
    fn the_rubric_file_and_the_draft_file_name() {
        let state = state_of(&started());
        let saved = parse(&rubric_file(&state));
        assert_eq!(saved["ok"]["filename"], "sample-rubric-project-report.json");
        assert_eq!(saved["ok"]["text"], SAMPLE);
        let named = state_of(&parse(&act(
            &state,
            r#"{"kind": "set_name", "text": "Jordan Lee"}"#,
        )));
        assert_eq!(
            draft_filename(&named),
            r#"{"ok":"feedback-jordan-lee.txt"}"#
        );
    }

    #[test]
    fn no_input_makes_any_function_panic() {
        let state = state_of(&started());
        let junk = [
            "",
            "null",
            "{}",
            "[]",
            "\"x\"",
            "{\"kind\": \"choose\"}",
            "{\"kind\": \"choose\", \"criterion\": 99999999999, \"level\": 0}",
            "{\"kind\": \"choose\", \"criterion\": -1, \"level\": 0}",
            "{\"kind\": \"remove_criterion\", \"criterion\": 4}",
            "{\"kind\": \"remove_criterion\", \"criterion\": 18446744073709551615}",
            "{\"kind\": \"choose\", \"criterion\": 0, \"level\": 18446744073709551615}",
            "{\"kind\": \"set_criterion_name\", \"criterion\": 18446744073709551615, \"text\": \"x\"}",
            "{\"kind\": \"set_level_points\", \"criterion\": 0, \"level\": 0, \"text\": \"99999999999999999999\"}",
            "{\"kind\": \"open_rubric\", \"text\": \"{\\\"criteria\\\": 1}\"}",
        ];
        for j in junk {
            assert!(load(j).starts_with("{\"error\""), "load {j}");
            assert!(start(j).starts_with("{\"error\""), "start {j}");
            assert!(act(&state, j).starts_with("{\"error\""), "act {j}");
            assert!(act(j, j).starts_with("{\"error\""), "act state {j}");
            assert!(rubric_file(j).starts_with("{\"error\""), "rubric_file {j}");
            assert!(
                draft_filename(j).starts_with("{\"error\""),
                "draft_filename {j}"
            );
        }
    }
}
