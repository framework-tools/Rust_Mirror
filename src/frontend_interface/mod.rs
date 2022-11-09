use std::{str::FromStr, collections::HashMap};

use serde_json::json;

use crate::{steps_generator::{StepError, event::Event, selection::Selection}, blocks::BlockMap};

pub fn execute_event(
    selection_json: String,
    new_ids_json: String,
    block_map_json: String,
    event_json: String,
) -> String {
    let selection: Selection = match serde_json::from_str(&selection_json) {
        Ok(selection) => selection,
        Err(_) => return javascript_return_json("", Some("Selection json could not be parsed"))
    };
    let new_ids: Vec<String> = match serde_json::from_str(&new_ids_json) {
        Ok(new_ids) => new_ids,
        Err(_) => return javascript_return_json("", Some("new_ids json could not be parsed"))
    };
    let block_map: HashMap<String, serde_json::Value> = match serde_json::from_str(&block_map_json) {
        Ok(block_map) => block_map,
        Err(_) => return javascript_return_json("", Some("Block Map json could not be parsed"))
    };

    let event_json = match serde_json::Value::from_str(&event_json) {
        Ok(event_json) => event_json,
        Err(_) => return javascript_return_json("", Some("Event json is not valid json"))
    };
    let event = match Event::from_json(&event_json) {
        Ok(event) => event,
        Err(_) => return javascript_return_json("", Some("Event json could not be parsed"))
    };
    return selection.anchor.block_id
}

pub fn javascript_return_json(data: &str, err: Option<&str>) -> String {
    return match err {
        Some(err) => json!({ "data": "", "error": err }).to_string(),
        None => json!({ "data": data, "error": "" }).to_string()
    }
}