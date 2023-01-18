
use wasm_bindgen::JsValue;

use crate::{steps_generator::{event::Event, selection::Selection, generate_steps, StepError},
new_ids::NewIds, blocks::{BlockMap, standard_blocks::StandardBlockType}, steps_actualisor::{actualise_steps, UpdatedState}, custom_copy::CustomCopy};

pub fn actualise_event(
    selection_js: js_sys::Object,
    new_ids_arr: js_sys::Array,
    block_map_js: js_sys::Map,
    event_js: js_sys::Object,
    copy_js: js_sys::Object,
) -> Response {
    let block_map = BlockMap::from_js_map(block_map_js);
    let selection = Selection::from_js_obj(selection_js).unwrap();
    let copy = CustomCopy::Js(copy_js);
    let event = Event::from_js_obj(event_js).unwrap();
    let mut new_ids = NewIds::Js(new_ids_arr);

    let steps = match generate_steps(&event, &block_map, selection) {
        Ok(steps) => steps,
        Err(StepError(err)) => return Response {
            selection: None,
            blocks_to_update: JsValue::from(js_sys::Array::new()),
            err: Some(err)
        }
    };

    return match actualise_steps(steps, block_map, &mut new_ids, copy) {
        Ok(UpdatedState { selection, blocks_to_update, .. }) => {
            let selection = match selection {
                Some(selection) => Some(selection.to_js_obj().unwrap()),
                None => None
            };
            let js_blocks_to_update = js_sys::Array::new();
            for id in blocks_to_update {
                js_blocks_to_update.push(&JsValue::from_str(&id));
            }

            Response { selection, blocks_to_update: JsValue::from(js_blocks_to_update), err: None }
        },
        Err(StepError(err)) => Response { selection: None, blocks_to_update: JsValue::from(js_sys::Array::new()), err: Some(err) }
    }
}

pub struct Response {
    pub selection: Option<JsValue>,
    pub blocks_to_update: JsValue,
    pub err: Option<String>
}

pub fn get_js_field(obj: &JsValue, field: &str) -> Result<JsValue, StepError> {
    match js_sys::Reflect::get(&obj, &JsValue::from_str(field)) {
        Ok(value) => Ok(value),
        Err(e) => Err(StepError(e.as_string().unwrap()))
    }
}
pub fn get_js_field_as_string(obj: &JsValue, field: &str) -> Result<String, StepError> {
    match js_sys::Reflect::get(&obj, &JsValue::from_str(field)) {
        Ok(value) => match value.as_string() {
            Some(value) => Ok(value),
            None => {
                return Err(StepError(
                    format!("Field: '{}' on obj is not a string", field),
                ))
            }
        },
        Err(_) => return Err(StepError(
            format!("Failed to get field: '{}' from js obj", field),
        ))
    }
}

pub fn get_js_field_as_f64(obj: &JsValue, field: &str) -> Result<f64, StepError> {
    match js_sys::Reflect::get(&obj, &JsValue::from_str(field)) {
        Ok(value) => match value.as_f64() {
            Some(value) => Ok(value),
            None => {
                return Err(StepError(
                    format!("Field: '{}' on obj is not a f64", field),
                ))
            }
        },
        Err(_) => return Err(StepError(
            format!("Failed to get field: '{}' from js obj", field),
        ))
    }
}

pub fn get_js_field_as_bool(obj: &JsValue, field: &str) -> Result<bool, StepError> {
    match js_sys::Reflect::get(&obj, &JsValue::from_str(field)) {
        Ok(value) => match value.as_bool() {
            Some(value) => Ok(value),
            None => {
                return Err(StepError(
                    format!("Field: '{}' on obj is not a bool", field),
                ))
            }
        },
        Err(e) => return Err(StepError(
            format!("Failed to get field: '{}' from js obj", field),
        ))
    }
}

pub fn get_number_of_numbered_list_block(block_map_js: js_sys::Map, block_id: String) -> u64 {
    let block_map = BlockMap::from_js_map(block_map_js);
    let block = block_map.get_standard_block(&block_id).unwrap();
    let mut number_of_list_blocks = 1;
    let mut option_previous_block = block.get_previous(&block_map).unwrap();
    while option_previous_block.is_some() {
        let previous_block = option_previous_block.unwrap();
        match previous_block.content {
            StandardBlockType::NumberedList(_) => number_of_list_blocks += 1,
            _ => break
        };
        option_previous_block = block.get_previous(&block_map).unwrap();
    }
    return number_of_list_blocks
}