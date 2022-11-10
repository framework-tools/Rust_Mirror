use std::{str::FromStr, collections::HashMap};

use serde_json::json;

use crate::{steps_generator::{event::Event, selection::Selection, generate_steps, StepError}, new_ids::NewIds, blocks::BlockMap, steps_executor::{execute_steps, UpdatedState}};

pub fn execute_event(
    selection_json: String,
    new_ids_json: String,
    block_map_json: String,
    event_json: String,
) -> String {
    let (selection, mut new_ids, block_map, event) = match parse_json_from_interface(selection_json, new_ids_json, block_map_json, event_json) {
        Ok(value) => value,
        Err(err) => return ReturnJson::Err(err).create_response()
    };

    let steps = match generate_steps(&event, &block_map, selection, &mut new_ids) {
        Ok(steps) => steps,
        Err(StepError(err)) => return ReturnJson::Err(err).create_response()
    };

    return match execute_steps(steps, block_map, &mut new_ids) {
        Ok(UpdatedState { block_map: BlockMap(updated_block_map), selection }) => {
            // let updated_block_map_json = match serde_json::to_string(&updated_block_map) {
            //     Ok(updated_block_map_json) => updated_block_map_json,
            //     Err(_) => return ReturnJson::Err("Updated blockmap could not be converted to JSON".to_string()).create_response()
            // };
            // let updated_selection_json = match serde_json::to_string(&selection) {
            //     Ok(updated_selection_json) => updated_selection_json,
            //     Err(_) => return ReturnJson::Err("Updated Selection could not be converted to JSON".to_string()).create_response()
            // };
            // return json!({ "data": updated_block_map_json, "err": "" })
            ReturnJson::Data{ updated_block_map, updated_selection: selection }.create_response()
        },
        Err(StepError(err)) => ReturnJson::Err(err).create_response()
    }
}

enum ReturnJson {
    Data {
        updated_block_map: HashMap<String, String>,
        updated_selection: Selection
    },
    Err(String)
}

impl ReturnJson {
    fn create_response(self) -> String {
        return match self {
            Self::Data { updated_block_map, updated_selection } => json!({
                "data": {
                    "block_map": updated_block_map,
                    "selection": updated_selection
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