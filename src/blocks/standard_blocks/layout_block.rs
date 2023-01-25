use std::str::FromStr;

use wasm_bindgen::JsValue;

use crate::{steps_generator::StepError, frontend_interface::{get_js_field, get_js_field_as_string, get_js_field_as_bool}};



#[derive(Debug, PartialEq, Clone)]
pub struct LayoutBlock{
    pub blocks: Vec<String>,
    pub horizontal: bool,
}
impl LayoutBlock {
    pub fn new() -> Self {
        LayoutBlock { blocks: vec![], horizontal: true }
    }

    pub fn from_js_block(obj: &JsValue) -> Result<Self, StepError> {
        let content = get_js_field(obj, "content")?;
        let blocks = js_sys::Array::from(&get_js_field(&content, "blocks")?)
        .iter().map(|id| {
            id.as_string().ok_or(StepError("Block 'blocks' field is not an array of strings".to_string())).map_err(|e| e)
        }).collect::<Result<Vec<String>, StepError>>()?;
        let horizontal = get_js_field_as_bool(&content, "horizontal")?;
        return Ok(LayoutBlock { blocks, horizontal})
    }

    pub fn from_json(block: &serde_json::Value) -> Result<Self, StepError> {
        let block = block.get("content").ok_or(StepError("Block does not have content field".to_string()))?;
        let blocks = block.get("blocks")
        .ok_or(StepError("Block does not have blocks field".to_string()))?
        .as_array().ok_or(StepError("Block blocks field is not an array".to_string()))?
        .iter().map(|id| {
            String::from_str(id.as_str().ok_or(StepError("Block blocks field is not an array of strings".to_string()))?).map_err(|e| StepError(e.to_string()))
        }).collect::<Result<Vec<String>, StepError>>()?;
        let horizontal = match block.get("horizontal").ok_or(StepError("Block does not have horizontal field".to_string()))?.as_bool() {
            Some(horizontal) => horizontal,
            None => return Err(StepError("horizontal on layout block json should be a bool".to_string()))
        };
        return Ok(LayoutBlock { blocks, horizontal })
    }
}