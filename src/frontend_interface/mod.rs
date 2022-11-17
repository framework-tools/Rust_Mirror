
use wasm_bindgen::JsValue;

use crate::{steps_generator::{event::Event, selection::Selection, generate_steps, StepError},
new_ids::NewIds, blocks::BlockMap, steps_executor::{execute_steps, UpdatedState}};

pub fn execute_event(
    selection_js: js_sys::Object,
    new_ids_arr: js_sys::Array,
    block_map_js: js_sys::Map,
    event_js: js_sys::Object,
) -> Response {
    let block_map = BlockMap::from_js_map(block_map_js);
    let selection = Selection::from_js_obj(selection_js).unwrap();
    let event = Event::from_js_obj(event_js).unwrap();
    let mut new_ids = NewIds::Js(new_ids_arr);

    let steps = match generate_steps(&event, &block_map, selection) {
        Ok(steps) => steps,
        Err(StepError(err)) => return Response { selection: None, err: Some(err) }
    };

    return Response { selection: None, err: None }
    // return match execute_steps(steps, block_map, &mut new_ids) {
    //     Ok(UpdatedState { selection, .. }) => {
    //         let selection = match selection {
    //             Some(selection) => Some(selection.to_js_obj().unwrap()),
    //             None => None
    //         };
    //         Response { selection, err: None }
    //     },
    //     Err(StepError(err)) => Response { selection: None, err: Some(err) }
    // }
}

pub struct Response {
    pub selection: Option<JsValue>,
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
        Err(_) => {
            return Err(StepError(
                "Failed to get _type from block js obj".to_string(),
            ))
        },
        Err(e) => Err(StepError(e.as_string().unwrap()))
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
        Err(e) => Err(StepError(e.as_string().unwrap()))
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
        Err(e) => Err(StepError(e.as_string().unwrap()))
    }
}