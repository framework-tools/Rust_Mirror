use std::{str::FromStr, collections::HashMap};

use serde_json::json;

use crate::{steps_generator::{event::Event, selection::Selection, generate_steps, StepError}, new_ids::NewIds, blocks::BlockMap, steps_executor::execute_steps};

pub fn execute_event(
    selection_json: String,
    new_ids_json: String,
    block_map_json: String,
    event_json: String,
) -> String {
    let (selection, mut new_ids, block_map, event) = match parse_json_from_interface(selection_json, new_ids_json, block_map_json, event_json) {
        Ok(value) => value,
        Err(err) => return javascript_return_json("", Some(&err))
    };

    let steps = match generate_steps(&event, &block_map, selection, &mut new_ids) {
        Ok(steps) => steps,
        Err(StepError(err)) => return javascript_return_json("", Some(&err))
    };

    return match execute_steps(steps, block_map, &mut new_ids) {
        Ok(BlockMap(updated_block_map)) => match serde_json::to_string(&updated_block_map) {
            Ok(updated_block_map_json) => javascript_return_json(&updated_block_map_json, None),
            Err(_) => javascript_return_json("", Some("Failed to convert blockmap to json"))
        },
        Err(StepError(err)) => javascript_return_json("", Some(&err))
    }
}

fn javascript_return_json(data: &str, err: Option<&str>) -> String {
    return match err {
        Some(err) => json!({ "data": "", "error": err }).to_string(),
        None => json!({ "data": data, "error": "" }).to_string()
    }
}

fn parse_json_from_interface(
    selection_json: String,
    new_ids_json: String,
    block_map_json: String,
    event_json: String,
) -> Result<(Selection, NewIds, BlockMap, Event), String> {
    let selection: Selection = match serde_json::from_str(&selection_json) {
        Ok(selection) => selection,
        Err(_) => return Err("Selection json could not be parsed".to_string())
    };
    let new_ids: Vec<String> = match serde_json::from_str(&new_ids_json) {
        Ok(new_ids) => new_ids,
        Err(_) => return Err("new_ids json could not be parsed".to_string())
    };
    let block_map: HashMap<String, String> = match serde_json::from_str(&block_map_json) {
        Ok(block_map) => block_map,
        Err(_) => return Err("Block Map json could not be parsed".to_string())
    };

    let event_json = match serde_json::Value::from_str(&event_json) {
        Ok(event_json) => event_json,
        Err(_) => return Err("Event json is not valid json".to_string())
    };
    let event = match Event::from_json(&event_json) {
        Ok(event) => event,
        Err(_) => return Err("Event json could not be parsed".to_string())
    };
    return Ok((selection, NewIds(new_ids), BlockMap(block_map), event))
}