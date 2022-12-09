use wasm_bindgen::JsValue;

use crate::{steps_generator::StepError, frontend_interface::{get_js_field, get_js_field_as_bool}};

use super::content_block::ContentBlock;

#[derive(Debug, PartialEq, Clone)]
pub struct ListBlock {
    pub content: ContentBlock,
    pub completed: bool
}

impl ListBlock {
    pub fn new() ->Self {
        return Self {
            content: ContentBlock { inline_blocks: vec![] },
            completed: false
        }
    }

    pub fn from_js_block(obj: &JsValue) -> Result<Self, StepError> {
        let content = get_js_field(obj, "content")?;
        return Ok(Self {
            content: ContentBlock::from_js_block(obj)?,
            completed: get_js_field_as_bool(&content, "completed")?
        })
    }

    pub fn from_json(block: &serde_json::Value) -> Result<Self, StepError> {
        let block = block.get("content").ok_or(StepError("Block does not have block field".to_string()))?;
        let completed = match block.get("content").ok_or(StepError("Block does not have block field".to_string()))?
        .as_bool() {
            Some(completed) => completed,
            None => return Err(StepError("'Completed' value is not a bool".to_string()))
        };
        return Ok(Self {
            content: ContentBlock::from_json(block)?,
            completed
        })
    }
}