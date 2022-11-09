use std::str::FromStr;

use serde_json::json;

use crate::{steps_generator::{StepError, event::Event, selection::Selection}, blocks::BlockMap};

pub fn execute_event(selection_json: String, new_ids_json: String) -> String { //json: String, , block_map_json: String
    // let rust_json = serde_json::Value::from_str(&json);
    // let rust_json = match rust_json {
    //     Ok(rust_json) => rust_json,
    //     Err(_) => return javascript_return_json("", Some("json argument should be valid json"))
    // };
    // let event_json = rust_json.get("event");
    // let event_json = match event_json {
    //     Some(event_json) => event_json,
    //     None => return Err(StepError("json should contain 'event' field".to_string()))
    // };

    // let event = Event::from_json(event_json)?;
    let selection: Selection = match serde_json::from_str(&selection_json) {
        Ok(selection) => selection,
        Err(_) => return javascript_return_json("", Some("Selection json could not be parsed"))
    };
    let new_ids: Vec<String> = match serde_json::from_str(&new_ids_json) {
        Ok(new_ids) => new_ids,
        Err(_) => return javascript_return_json("", Some("new_ids json could not be parsed"))
    };
    // let block_map: BlockMap = match serde_json::from_str(&block_map_json) {
    //     Ok(block_map) => block_map,
    //     Err(_) => return Err(StepError("Block Map json could not be parsed".to_string()))
    // };
    return selection.anchor.block_id
}

pub fn javascript_return_json(data: &str, err: Option<&str>) -> String {
    return match err {
        Some(err) => json!({ "data": "", "error": err }).to_string(),
        None => json!({ "data": data, "error": "" }).to_string()
    }
}