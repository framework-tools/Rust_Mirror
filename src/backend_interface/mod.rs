use std::{collections::HashMap};

use serde_json::Value;

use crate::{blocks::BlockMap, step::Step, steps_generator::StepError, new_ids::NewIds, steps_actualisor::{actualise_steps, UpdatedState}, custom_copy::CustomCopy};


pub fn actualise_mirror_step(
    step_as_json: (String, String),
    new_ids: Vec<String>,
    block_map_rust: HashMap<String, String>
) -> Result<HashMap<String, String>, StepError> {
    let mut block_map = BlockMap::Rust(block_map_rust);
    let mut new_ids = NewIds::Rust(new_ids);

    let (_type, data) = step_as_json;
    if &_type != "copy" {
        let step = Step::from_json(&_type, &data)?;
        let updated_state =
            actualise_steps(vec![step.clone()], block_map, &mut new_ids, CustomCopy::new())?;
        block_map = updated_state.block_map;
    }
    return match block_map {
        BlockMap::Rust(block_map) => Ok(block_map),
        BlockMap::Js(_) => unreachable!("block_map should be Rust")
    }
}

pub fn get_json_field_as_string(json: &Value, field: &str) -> Result<String, StepError> {
    Ok(json.get(field)
        .ok_or(StepError(format!("json does not have {} field: {}", field, json)))?
        .as_str().ok_or(StepError(format!("field: {} is not a string in json: {}", field, json)))?.to_string()
    )
}
pub fn get_json_field_as_int(json: &Value, field: &str) -> Result<i64, StepError> {
    Ok(json.get(field)
        .ok_or(StepError(format!("json does not have {} field: {}", field, json)))?
        .as_i64().ok_or(StepError(format!("field: {} is not an i64 in json: {}", field, json)))?
    )
}
pub fn get_json_field_as_bool(json: &Value, field: &str) -> Result<bool, StepError> {
    Ok(json.get(field)
        .ok_or(StepError(format!("json does not have {} field: {}", field, json)))?
        .as_bool().ok_or(StepError(format!("field: {} is not a bool in json: {}", field, json)))?
    )
}