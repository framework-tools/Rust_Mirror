use std::{str::FromStr};

use js_sys::Map;
use serde_json::json;

use crate::{steps_generator::{event::Event, selection::Selection, generate_steps, StepError},
new_ids::NewIds, blocks::BlockMap, steps_executor::{execute_steps, UpdatedState}};

pub fn execute_event(
    selection_js: js_sys::Object,
    new_ids_json: String,
    block_map_js: Map,
    event_json: String,
) -> Reponse {
    let block_map = BlockMap::from_js_map(block_map_js);
    let selection = Selection::from_js_obj(selection_js).unwrap();

    let (mut new_ids, event) = match parse_json_from_interface(new_ids_json, event_json) {
        Ok(value) => value,
        Err(err) => return Reponse { map: None, selection_json: "".to_string(), err }
    };

    let steps = match generate_steps(&event, &block_map, selection) {
        Ok(steps) => steps,
        Err(StepError(err)) => return Reponse { map: None, selection_json: "".to_string(), err }
    };

    return match execute_steps(steps, block_map, &mut new_ids) {
        Ok(UpdatedState { block_map, selection }) => {
            let selection_json = match selection {
                Some(selection) => json!({ "selection": selection }).to_string(),
                None => "".to_string()
            };
            Reponse { map: Some(block_map.to_js_map().unwrap()), selection_json, err: "".to_string() }
        },
        Err(StepError(err)) => Reponse { map: None, selection_json: "".to_string(), err }
    }
}

pub struct Reponse {
    pub map: Option<js_sys::Map>,
    pub selection_json: String,
    pub err: String
}


fn parse_json_from_interface(
    new_ids_json: String,
    event_json: String,
) -> Result<(NewIds, Event), String> {
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
    return Ok((NewIds(new_ids), event))
}