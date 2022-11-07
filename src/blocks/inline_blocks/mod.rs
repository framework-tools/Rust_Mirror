
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{mark::Mark, steps_generator::StepError};

use self::text_block::TextBlock;

pub mod text_block;

#[derive(Debug, PartialEq, Clone)]
pub struct InlineBlock {
    pub _id: ObjectId,
    pub content: InlineBlockType,
    pub marks: Vec<Mark>,
    pub parent: ObjectId, //StandardBlock
}

impl InlineBlock {
    pub fn new_text_block(text: String, marks: Vec<Mark>, parent: ObjectId) -> Self {
        InlineBlock {
            _id: ObjectId::new(),
            content: InlineBlockType::TextBlock(TextBlock { text }),
            marks,
            parent
        }
    }

    pub fn id(&self) -> ObjectId {
        self._id
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
            _id: self._id,
            content: self.content.update_block(text),
            marks: self.marks,
            parent: self.parent
        })
    }

    pub fn to_new_block(self) -> Self {
        InlineBlock {
            _id: ObjectId::new(),
            content: self.content,
            marks: self.marks,
            parent: self.parent
        }
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum InlineBlockType {
    TextBlock(TextBlock),
}

impl InlineBlockType {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, StepError> {
        let _type = json.get("_type").ok_or(StepError("Block does not have _type field".to_string()))?.as_str().ok_or(StepError("Block _type field is not a string".to_string()))?;
        match _type {
            "text" => {
                let text_block = json.get("content").ok_or(StepError("Block does not have block field".to_string()))?;
                return Ok(InlineBlockType::TextBlock(TextBlock {
                    text: text_block.get("text").ok_or(StepError("Block does not have text field".to_string()))?.as_str().ok_or(StepError("Block text field is not a string".to_string()))?.to_string()
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
            InlineBlockType::TextBlock(block) => InlineBlockType::TextBlock(TextBlock { text })
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