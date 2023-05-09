use std::str::FromStr;


use wasm_bindgen::JsValue;

use crate::{steps_generator::StepError, frontend_interface::get_js_field, new_ids::NewIds};


#[derive(Debug, PartialEq, Clone)]
pub struct ContentBlock {
    pub inline_blocks: Vec<String>, //Vec<InlineBlock>
}

impl ContentBlock {
    pub fn new(inline_blocks: Vec<String>) -> Self {
        Self {
            inline_blocks
        }
    }

    pub fn from_js_block(obj: &JsValue) -> Result<Self, StepError> {
        let content = get_js_field(obj, "content")?;
        let inline_blocks = js_sys::Array::from(&get_js_field(&content, "inline_blocks")?)
        .iter().map(|id| {
            id.as_string().ok_or(StepError("Block inline_blocks field is not an array of strings".to_string())).map_err(|e| e)
        }).collect::<Result<Vec<String>, StepError>>()?;
        return Ok(Self::new(inline_blocks))
    }

    pub fn from_json(block: &serde_json::Value) -> Result<Self, StepError> {
        let block = block.get("content").ok_or(StepError("Block does not have block field".to_string()))?;
        let inline_blocks = block.get("inline_blocks")
            .ok_or(StepError("Block does not have inline_blocks field".to_string()))?
            .as_array().ok_or(StepError("Block inline_blocks field is not an array".to_string()))?
            .iter().map(|id| {
                String::from_str(id.as_str().ok_or(StepError("Block inline_blocks field is not an array of strings".to_string()))?).map_err(|e| StepError(e.to_string()))
            }).collect::<Result<Vec<String>, StepError>>()?;
        return Ok(Self::new(inline_blocks))
    }

    pub fn index_of(&self, id: &str) -> Result<usize, StepError> {
        match self.inline_blocks.iter().position(|block_id| *block_id == id) {
            Some(index) => Ok(index),
            None => Err(StepError("Block not found".to_string()))
        }
    }

    // remove any inline blocks after offset + 1 (if they exist)
    pub fn remove_blocks_after_offset(self, offset: usize) -> Result<Self, StepError> {
        let mut inline_blocks = self.inline_blocks;
        if offset + 1 < inline_blocks.len() {
            inline_blocks.truncate(offset + 1);
        }
        Ok(Self::new(inline_blocks))
    }

    pub fn remove_blocks_before_offset(self, offset: usize) -> Result<Self, StepError> {
        let mut inline_blocks = self.inline_blocks;
        inline_blocks.drain(0..offset);
        Ok(Self::new(inline_blocks))
    }
}