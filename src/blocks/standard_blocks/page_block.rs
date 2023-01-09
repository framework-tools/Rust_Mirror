use wasm_bindgen::JsValue;

use crate::{steps_generator::StepError, frontend_interface::{get_js_field, get_js_field_as_string}};


#[derive(Debug, PartialEq, Clone)]
pub struct PageBlock {
    pub page_id: String
}

impl PageBlock {
    pub fn new() -> Self {
        PageBlock { page_id: String::new() }
    }

    pub fn from_js_block(obj: &JsValue) -> Result<Self, StepError> {
        let content = get_js_field(obj, "content")?;
        let page_id = get_js_field_as_string(&content, "page_id")?;
        return Ok(Self { page_id })
    }

    pub fn from_json(block: &serde_json::Value) -> Result<Self, StepError> {
        let block = block.get("content").ok_or(StepError("Block does not have content field".to_string()))?;
        let page_id = match block.get("page_id").ok_or(StepError("Block does not have block field".to_string()))?.as_str() {
            Some(id) => id,
            None => return Err(StepError("Page id on page block json should be a string".to_string()))
        };
        return Ok(Self { page_id: page_id.to_string() })
    }
}