use std::str::FromStr;

use wasm_bindgen::JsValue;

use crate::{steps_generator::StepError, frontend_interface::{get_js_field, get_js_field_as_string, get_js_field_as_bool}};

#[derive(Debug, PartialEq, Clone)]
pub struct LayoutBlock{
    pub horizontal: bool,
}
impl LayoutBlock {
    pub fn new() -> Self {
        LayoutBlock { horizontal: true }
    }

    pub fn from_js_block(obj: &JsValue) -> Result<Self, StepError> {
        let content = get_js_field(obj, "content")?;
        let horizontal = get_js_field_as_bool(&content, "horizontal")?;
        return Ok(LayoutBlock {  horizontal})
    }

    pub fn from_json(block: &serde_json::Value) -> Result<Self, StepError> {
        let block = block.get("content").ok_or(StepError("Block does not have content field".to_string()))?;
        let horizontal = match block.get("horizontal").ok_or(StepError("Block does not have horizontal field".to_string()))?.as_bool() {
            Some(horizontal) => horizontal,
            None => return Err(StepError("horizontal on layout block json should be a bool".to_string()))
        };
        return Ok(LayoutBlock { horizontal })
    }
}