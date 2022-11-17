use std::{str::FromStr};

use js_sys::Map;
use serde_json::json;

use crate::{steps_generator::{event::Event, selection::Selection, generate_steps, StepError},
new_ids::NewIds, blocks::BlockMap, steps_executor::{execute_steps, UpdatedState}};

pub fn execute_event(
    selection_js: js_sys::Object,
    new_ids_arr: js_sys::Array,
    block_map_js: js_sys::Map,
    event_js: js_sys::Object,
) -> Reponse {
    let block_map = BlockMap::from_js_map(block_map_js);
    let selection = Selection::from_js_obj(selection_js).unwrap();
    let event = Event::from_js_obj(event_js).unwrap();
    let mut new_ids = NewIds::Js(new_ids_arr);

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