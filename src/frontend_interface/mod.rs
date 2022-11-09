use std::str::FromStr;

use crate::steps_generator::{StepError, event::Event, selection::Selection};

pub fn execute_event(json: String, selection_json: String) -> Result<String, StepError> {
    let rust_json = serde_json::Value::from_str(&json);
    let rust_json = match rust_json {
        Ok(rust_json) => rust_json,
        Err(_) => return Err(StepError("json argument should be valid json".to_string()))
    };
    let event_json = rust_json.get("event");
    let event_json = match event_json {
        Some(event_json) => event_json,
        None => return Err(StepError("json should contain 'event' field".to_string()))
    };
    let block_map_json = rust_json.get("block_map");
    let block_map_json = match block_map_json {
        Some(block_map_json) => block_map_json,
        None => return Err(StepError("json should contain 'block_map' field".to_string()))
    };

    let event = Event::from_json(event_json)?;
    let selection: Selection = match serde_json::from_str(&selection_json) {
        Ok(selection) => selection,
        Err(_) => return Err(StepError("Selection json could not be parsed".to_string()))
    };

    return Ok("it worked".to_string())
}