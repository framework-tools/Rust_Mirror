

use serde_json::json;
use wasm_bindgen::JsValue;

use crate::{mark::Mark, steps_generator::StepError, new_ids::{self, NewIds}, frontend_interface::{get_js_field_as_string, get_js_field}};

use self::text_block::TextBlock;

use super::{BlockMap, standard_blocks::StandardBlock};

pub mod text_block;

#[derive(Debug, PartialEq, Clone)]
pub struct InlineBlock {
    pub _id: String,
    pub content: InlineBlockType,
    pub marks: Vec<Mark>,
    pub parent: String, //StandardBlock
}

impl InlineBlock {
    pub fn new_text_block(text: String, marks: Vec<Mark>, parent: String, new_ids: &mut NewIds) -> Result<Self, StepError> {
        Ok(InlineBlock {
            _id: new_ids.get_id()?,
            content: InlineBlockType::TextBlock(TextBlock { text }),
            marks,
            parent
        })
    }

    pub fn id(&self) -> String {
        return self._id.clone()
    }

    pub fn text(&self) -> Result<&String, StepError> {
        match &self.content {
            InlineBlockType::TextBlock(block) => Ok(&block.text),
            _ => Err(StepError("Block does not have text".to_string()))
        }
    }

    pub fn update_text(self, text: String) -> Result<Self, StepError> {
        Ok(InlineBlock {
            _id: self._id,
            content: self.content.update_block(text),
            marks: self.marks,
            parent: self.parent
        })
    }

    pub fn index(&self, block_map: &BlockMap) -> Result<usize, StepError> {
        let parent = block_map.get_standard_block(&self.parent)?;
        return parent.index_of(&self._id)
    }

    pub fn is_same_type(&self, block_type: &InlineBlockType) -> bool {
        match self.content {
            InlineBlockType::TextBlock(_) => match block_type {
                InlineBlockType::TextBlock(_) => true,
                _ => false
            }
        }
    }

    pub fn merge(self, merge_with: Self) -> Result<Self, StepError> {
        let text = self.text()?.to_string() + merge_with.text()?.as_str();
        Ok(InlineBlock {
            _id: self.id(),
            content: self.content.update_block(text),
            marks: self.marks,
            parent: self.parent
        })
    }

    pub fn to_new_block(self, new_ids: &mut NewIds) -> Result<Self, StepError> {
        Ok(InlineBlock {
            _id: new_ids.get_id()?,
            content: self.content,
            marks: self.marks,
            parent: self.parent
        })
    }

    /// -> Remove any marks of the same type that exist
    /// -> add mark
    pub fn add_mark(mut self, mark: Mark) -> Self {
        self.marks.retain(|m| !m.is_same_type(&mark));
        self.marks.push(mark);
        return self
    }

    pub fn remove_mark(mut self, mark: Mark) -> Self {
        self.marks.retain(|m| !m.is_same_type(&mark));
        return self
    }

    pub fn apply_mark(self, mark: Mark, add_mark: bool) -> Self {
        if add_mark {
            return self.add_mark(mark);
        } else {
            return  self.remove_mark(mark);
        }
    }

    pub fn get_parent(&self, block_map: &BlockMap) -> Result<StandardBlock, StepError> {
        return block_map.get_standard_block(&self.parent)
    }

    pub fn previous_block(&self, block_map: &BlockMap) -> Result<InlineBlock, StepError> {
        let parent_block = self.get_parent(block_map)?;
        let previous_block_id = parent_block.content_block()?.inline_blocks[parent_block.index_of(&self._id)? - 1].clone();
        return block_map.get_inline_block(&previous_block_id)
    }

    pub fn split(mut self, offset: usize, new_ids: &mut NewIds) -> Result<(Self, Self), StepError> {
        let text = self.text()?;
        if offset > text.len() {
            return Err(StepError(format!("Offset is larger than text size. Offset: {}, text: {}", offset, text)))
        }
        let first_half = text[..offset].to_string();
        let second_half = text[offset..].to_string();
        self = self.update_text(first_half)?;
        let new_block = self.clone().to_new_block(new_ids)?.update_text(second_half)?;
        return Ok((self, new_block))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum InlineBlockType {
    TextBlock(TextBlock),
}

impl InlineBlockType {
    pub fn from_js_block(obj: &JsValue) -> Result<Self, StepError> {
        let _type = get_js_field_as_string(obj, "_type")?;
        let content = get_js_field(obj, "content")?;

        match _type.as_str() {
            "text" => {
                return Ok(InlineBlockType::TextBlock(TextBlock {
                    text:  get_js_field_as_string(&content, "text")?
                }))
            },
            _ => Err(StepError(format!("Block kind {} not found", _type)))
        }
    }

    pub fn text(&self) -> Result<&String, StepError> {
        match self {
            InlineBlockType::TextBlock(block) => Ok(&block.text),
        }
    }

    pub fn update_block(self, text: String) -> Self {
        match self {
            InlineBlockType::TextBlock(_) => InlineBlockType::TextBlock(TextBlock { text })
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            InlineBlockType::TextBlock(block) => json!({
                "_type": "text",
                "content": {
                    "text": block.text
                }
            })
        }
    }
}