
use serde_json::json;

use crate::{mark::Mark, steps_generator::StepError};

use self::content_block::ContentBlock;

use super::{inline_blocks::InlineBlock, BlockMap, Block};

pub mod content_block;

#[derive(Debug, PartialEq, Clone)]
pub struct StandardBlock {
    pub _id: String,
    pub content: StandardBlockType,
    pub children: Vec<String>, //Vec<StandardBlock>
    pub parent: String, //StandardBlock
    pub marks: Vec<Mark>,
}

impl StandardBlock {
    pub fn id(&self) -> String {
        return self._id.clone()
    }
    pub fn parent(&self) -> String {
        return self.parent.clone()
    }

    pub fn new_paragraph_block(_id: String, inline_blocks: Vec<String>, marks: Vec<Mark>, children: Vec<String>, parent: String) -> Self {
        StandardBlock {
            _id,
            content: StandardBlockType::Paragraph(ContentBlock::new(inline_blocks)),
            parent,
            children,
            marks,
        }
    }

    pub fn update_block_content(self, content_block: ContentBlock) -> Result<Self, StepError> {
        Ok(StandardBlock {
            _id: self._id,
            content: StandardBlockType::update_block_content(&self.content, content_block)?,
            parent: self.parent,
            children: self.children,
            marks: self.marks,
        })
    }

    pub fn get_parent(&self, block_map: &BlockMap) -> Result<Block, StepError> {
        return block_map.get_block(&self.parent)
    }

    pub fn index(&self, block_map: &BlockMap) -> Result<usize, StepError> {
        let parent: Block = self.get_parent(block_map)?;
        return parent.index_of_child(&self._id)
    }

    pub fn get_previous(&self, block_map: &BlockMap) -> Result<Option<StandardBlock>, StepError> {
        if self.index(block_map)? == 0 {
            return Ok(None)
        } else {
            let parent: Block = self.get_parent(block_map)?;
            return Ok(Some(block_map.get_standard_block(&parent.get_child_from_index(self.index(block_map)? - 1)?)?))
        }
    }

    pub fn inline_blocks_length(&self) -> Result<usize, StepError> {
        let content_block = self.content_block()?;
        Ok(content_block.inline_blocks.len())
    }

    pub fn index_of(&self, id: &str) -> Result<usize, StepError> {
        let content_block = self.content_block()?;
        content_block.index_of(id)
    }
    pub fn index_of_child(&self, id: &str) -> Result<usize, StepError> {
        match self.children.iter().position(|block_id| *block_id == id) {
            Some(index) => Ok(index),
            None => Err(StepError("Block not found".to_string()))
        }
    }

    pub fn get_inline_block_from_index(&self, index: usize) -> Result<String, StepError> {
        return Ok(self.content_block()?.inline_blocks[index].clone())

    }

    pub fn get_child_from_index(&self, index: usize) -> Result<String, StepError> {
        match self.children.get(index) {
            Some(block_id) => Ok(block_id.clone()),
            None => Err(StepError("Block not found".to_string()))
        }
    }

    pub fn content_block(&self) -> Result<&ContentBlock, StepError> {
        match &self.content {
            StandardBlockType::Paragraph(block) | StandardBlockType::H1(block) |
            StandardBlockType::H2(block) | StandardBlockType::H3(block) => Ok(block),
            _ => Err(StepError("Block does not have a content block".to_string()))
        }
    }

    pub fn remove_blocks_between_offsets(self, from_offset: usize, to_offset: usize) -> Result<Self, StepError> {
        let content_block = self.content_block()?.clone();
        let new_inline_blocks = content_block.inline_blocks[from_offset + 1..to_offset].to_vec();
        return self.update_block_content(ContentBlock { inline_blocks: new_inline_blocks })
    }

    pub fn all_blocks_have_identical_mark(&self, mark: &Mark, from: usize, to: usize, block_map: &BlockMap) -> Result<bool, StepError> {
        let inline_blocks = self.content_block()?.inline_blocks.clone();
        let mut i = from;
        while i < to + 1 && i < inline_blocks.len() {
            let inline_block = block_map.get_inline_block(&inline_blocks[i])?;
            if !inline_block.marks.contains(mark) {
                return Ok(false)
            }
            i += 1;
        }
        return Ok(true)
    }

    pub fn get_last_inline_block(&self, block_map: &BlockMap) -> Result<InlineBlock, StepError> {
        let inline_blocks = &self.content_block()?.inline_blocks;
        let last_block_id = inline_blocks[inline_blocks.len() - 1].clone();
        return block_map.get_inline_block(&last_block_id)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StandardBlockType {
    Paragraph(ContentBlock),
    H1(ContentBlock),
    H2(ContentBlock),
    H3(ContentBlock),
}

impl StandardBlockType {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, StepError> {
        let block_type = json.get("_type").ok_or(StepError("Block does not have _type field".to_string()))?.as_str().ok_or(StepError("Block _type field is not a string".to_string()))?;
        match block_type {
            "paragraph" => {
                let paragraph_block_json = json.get("content").ok_or(StepError("Block does not have block field".to_string()))?;
                let block = ContentBlock::from_json(json)?;
                Ok(StandardBlockType::Paragraph(block))
            },
            "h1" => {
                let h1_block_json = json.get("content").ok_or(StepError("Block does not have block field".to_string()))?;
                let block = ContentBlock::from_json(json)?;
                Ok(StandardBlockType::H1(block))
            },
            "h2" => {
                let h2_block_json = json.get("content").ok_or(StepError("Block does not have block field".to_string()))?;
                let block = ContentBlock::from_json(json)?;
                Ok(StandardBlockType::H2(block))
            },
            "h3" => {
                let h3_block_json = json.get("content").ok_or(StepError("Block does not have block field".to_string()))?;
                let block = ContentBlock::from_json(json)?;
                Ok(StandardBlockType::H3(block))
            },
            _ => Err(StepError(format!("Block type {} not found", block_type)))
        }
    }
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            StandardBlockType::Paragraph(block) => {
                json!({
                    "_type": "paragraph",
                    "content": {
                        "inline_blocks": block.inline_blocks.iter().map(|inline_block| inline_block.to_string()).collect::<Vec<String>>()
                    }
                })
            }
            StandardBlockType::H1(block) => {
                json!({
                    "_type": "h1",
                    "content": {
                        "inline_blocks": block.inline_blocks.iter().map(|inline_block| inline_block.to_string()).collect::<Vec<String>>()
                    }
                })
            }
            StandardBlockType::H2(block) => {
                json!({
                    "_type": "h2",
                    "content": {
                        "inline_blocks": block.inline_blocks.iter().map(|inline_block| inline_block.to_string()).collect::<Vec<String>>()
                    }
                })
            }
            StandardBlockType::H3(block) => {
                json!({
                    "_type": "h3",
                    "content": {
                        "inline_blocks": block.inline_blocks.iter().map(|inline_block| inline_block.to_string()).collect::<Vec<String>>()
                    }
                })
            }
        }
    }

    pub fn update_block_content(&self, content_block: ContentBlock) -> Result<Self, StepError> {
        match self {
            StandardBlockType::Paragraph(_) => Ok(StandardBlockType::Paragraph(content_block)),
            StandardBlockType::H1(_) => Ok(StandardBlockType::H1(content_block)),
            StandardBlockType::H2(_) => Ok(StandardBlockType::H2(content_block)),
            StandardBlockType::H3(_) => Ok(StandardBlockType::H3(content_block)),
        }
    }
}

