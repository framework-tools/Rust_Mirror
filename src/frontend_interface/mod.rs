use std::{str::FromStr};

use js_sys::Map;
use serde_json::json;

use crate::{steps_generator::{event::Event, selection::Selection, generate_steps, StepError},
new_ids::NewIds, blocks::BlockMap, steps_executor::{execute_steps, UpdatedState}};

pub fn execute_event(
    selection_json: String,
    new_ids_json: String,
    block_map: Map,
    event_json: String,
) -> String {
    let block_map = BlockMap::from_js_map(block_map);

    let (selection, mut new_ids, event) = match parse_json_from_interface(selection_json, new_ids_json, event_json) {
        Ok(value) => value,
        Err(err) => return ReturnJson::Err(err).create_response()
    };

    let steps = match generate_steps(&event, &block_map, selection) {
        Ok(steps) => steps,
        Err(StepError(err)) => return ReturnJson::Err(err).create_response()
    };

    return match execute_steps(steps, block_map, &mut new_ids) {
        Ok(UpdatedState { block_map, selection }) => {
            ReturnJson::Data{ updated_selection: selection, new_ids: new_ids.0 }.create_response()
        },
        Err(StepError(err)) => ReturnJson::Err(err).create_response()
    }
}

enum ReturnJson {
    Data {
        updated_selection: Option<Selection>,
        new_ids: Vec<String>
    },
    Err(String)
}

impl ReturnJson {
    fn create_response(self) -> String {
        return match self {
            Self::Data { updated_selection, new_ids } => json!({
                "data": {
                    "selection": updated_selection,
                    "new_ids": new_ids,
                },
                "error": ""
            }).to_string(),
            Self::Err(err_msg) => json!({ "data": {}, "error": err_msg }).to_string()
        }
    }
}


fn parse_json_from_interface(
    selection_json: String,
    new_ids_json: String,
    event_json: String,
) -> Result<(Selection, NewIds, Event), String> {
    let selection: Selection = match serde_json::from_str(&selection_json) {
        Ok(selection) => selection,
        Err(_) => return Err("Selection json could not be parsed".to_string())
    };
    let new_ids: Vec<String> = match serde_json::from_str(&new_ids_json) {
        Ok(new_ids) => new_ids,
        Err(_) => return Err("new_ids json could not be parsed".to_string())
    };

    let event_json = match serde_json::Value::from_str(&event_json) {
        Ok(event_json) => event_json,
        Err(_) => return Err("Event json is not valid json".to_string())
    };
    let event = match Event::from_json(&event_json) {
        Ok(event) => event,
        Err(_) => return Err("Event json could not be parsed".to_string())
    };
    return Ok((selection, NewIds(new_ids), event))
}