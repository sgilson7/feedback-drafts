//! tool_shim: forwards strings between the page and `tool_core::api`.
//!
//! Builder requirement: the shim decides nothing. Each function is one call
//! into the core. `crates/core/tests/boundary.rs` fails if a branch, a loop or
//! the `?` operator appears here; if a feature seems to need one, the rule
//! belongs in the core.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn load(strings_json: &str) -> String {
    tool_core::api::load(strings_json)
}

#[wasm_bindgen]
pub fn start(sample_json: &str) -> String {
    tool_core::api::start(sample_json)
}

#[wasm_bindgen]
pub fn act(state_json: &str, action_json: &str) -> String {
    tool_core::api::act(state_json, action_json)
}

#[wasm_bindgen]
pub fn rubric_file(state_json: &str) -> String {
    tool_core::api::rubric_file(state_json)
}

#[wasm_bindgen]
pub fn draft_filename(state_json: &str) -> String {
    tool_core::api::draft_filename(state_json)
}
