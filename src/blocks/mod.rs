

use std::{collections::HashMap, str::FromStr};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use crate::{mark::Mark, steps_generator::StepError};

use self::{standard_blocks::{StandardBlock, StandardBlockType}, inline_blocks::{InlineBlock, InlineBlockType}};


pub mod standard_blocks;
pub mod inline_blocks;

#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    StandardBlock(StandardBlock),
    InlineBlock(InlineBlock),
    Root(RootBlock)
}

impl Block {
    pub fn from_json(json_str: &str) -> Result<Self, StepError> {
        let json = match serde_json::Value::from_str(json_str) {
            Ok(json) => json,
            Err(_) => return Err(StepError(format!("Could not parse json block from str: {}", json_str)))
        };

        let kind = json.get("kind").ok_or(StepError(format!("Block does not have kind field: {}", json)))?
            .as_str().ok_or(StepError("Block kind field is not a string".to_string()))?;
        return match kind {
            "standard" => {
                Ok(Block::StandardBlock(StandardBlock {
                    _id: id_from_json_block(&json)?,
                    content: StandardBlockType::from_json(&json)?,
                    children: children_from_json_block(&json)?,
                    parent: parent_from_json_block(&json)?,
                    marks: marks_from_json_block(&json)?,
                }))
            },
            "inline" => {
                Ok(Block::InlineBlock(InlineBlock {
                    _id: id_from_json_block(&json)?,
                    content: InlineBlockType::from_json(&json)?,
                    parent: parent_from_json_block(&json)?,
                    marks: marks_from_json_block(&json)?,
                }))
            },
            "root" => {
                Ok(Block::Root(RootBlock {
                    _id: id_from_json_block(&json)?,
                    children: children_from_json_block(&json)?,
                }))
            },
            _ => Err(StepError(format!("Block kind {} not found", kind)))
        }
    }
    /// Some example json of blocks:
    /// let inline_block = json!({
    ///     "_id": inline_block_id.to_string(),
    ///     "kind": "inline",
    ///     "_type": "text",
    ///     "content": {
    ///         "text": "Hello World"
    ///     },
    ///     "marks": [],
    ///     "parent": paragraph_block_id.to_string()
    /// });
    /// let block = json!({
    ///     "_id": paragraph_block_id.to_string(),
    ///     "kind": "standard",
    ///     "_type": "paragraph",
    ///     "content": {
    ///         "inline_blocks": [inline_block_id.to_string()]
    ///     },
    ///     "children": [],
    ///     "marks": [],
    ///     "parent": root_block_id.to_string()
    /// });
    /// let root_block = json!({
    ///     "_id": root_block_id.to_string(),
    ///     "kind": "root",
    ///     "children": [paragraph_block_id.to_string()]
    /// });
    pub fn to_json(self) -> Result<Value, StepError> {
        match self {
            Block::StandardBlock(block) => {
                let standard_block_type_json = block.content.to_json();
                let _type = standard_block_type_json.get("_type").ok_or(StepError("Standard block type does not have _type field".to_string()))?.as_str().ok_or(StepError("Standard block type _type field is not a string".to_string()))?;
                let block_content = standard_block_type_json.get("content").ok_or(StepError("Standard block type does not have block field".to_string()))?.clone();
                return Ok(json!({
                    "_id": block._id.to_string(),
                    "kind": "standard",
                    "_type": _type,
                    "content": block_content,
                    "children": block.children.into_iter().map(|children| children.to_string()).collect::<Vec<String>>(),
                    "parent": block.parent.to_string(),
                    "marks": block.marks.into_iter().map(|mark| mark.to_string()).collect::<Vec<String>>(),
                }))
            },
            Block::InlineBlock(block) => {
                let inline_block_type_json = block.content.to_json();
                let _type = inline_block_type_json.get("_type").ok_or(StepError("Inline block type does not have _type field".to_string()))?.as_str().ok_or(StepError("Inline block type _type field is not a string".to_string()))?;
                let block_content = inline_block_type_json.get("content").ok_or(StepError("Inline block type does not have block field".to_string()))?.clone();
                return Ok(json!({
                    "_id": block.id(),
                    "kind": "inline",
                    "_type": _type,
                    "content": block_content,
                    "parent": block.parent.to_string(),
                    "marks": block.marks.into_iter().map(|mark| mark.to_string()).collect::<Vec<String>>(),
                }))
            },
            Block::Root(block) => {
                return Ok(json!({
                    "_id": block.id(),
                    "kind": "root",
                    "children": block.children.into_iter().map(|children| children.to_string()).collect::<Vec<String>>(),
                }))
            }
        }
    }

    pub fn id(&self) -> String {
        match self {
            Block::StandardBlock(block) => block._id.clone(),
            Block::InlineBlock(block) => block._id.clone(),
            Block::Root(block) => block._id.clone()
        }
    }

    pub fn marks<'a>(&'a self) -> Result<&'a Vec<Mark>, StepError> {
        match self {
            Block::StandardBlock(block) => Ok(&block.marks),
            Block::InlineBlock(block) => Ok(&block.marks),
            Block::Root(_) => Err(StepError("RootBlock does not have marks".to_string()))
        }
    }

    pub fn children(&self) -> Result<&Vec<String>, StepError> {
        match self {
            Block::StandardBlock(block) => Ok(&block.children),
            Block::InlineBlock(_) => Err(StepError("InlineBlock does not have children".to_string())),
            Block::Root(block) => Ok(&block.children)
        }
    }

    pub fn update_children(&mut self, children: Vec<String>) -> Result<(), StepError> {
        match self {
            Block::StandardBlock(block) => {
                block.children = children;
                Ok(())
            },
            Block::InlineBlock(_) => Err(StepError("InlineBlock does not have children".to_string())),
            Block::Root(block) => {
                block.children = children;
                Ok(())
            }
        }
    }

    pub fn index_of_child(&self, id: &str) -> Result<usize, StepError> {
        match self {
            Block::StandardBlock(block) => block.index_of_child(id),
            Block::Root(block) => block.index_of_child(id),
            Block::InlineBlock(_) => Err(StepError("InlineBlock does not have children".to_string()))
        }
    }

    pub fn get_child_from_index(&self, index: usize) -> Result<String, StepError> {
        match self {
            Block::StandardBlock(block) => block.get_child_from_index(index),
            Block::Root(block) => block.get_child_from_index(index),
            Block::InlineBlock(_) => Err(StepError("InlineBlock does not have children".to_string()))
        }
    }

    pub fn new_std_block_json(id: String, parent_id: String) -> Value {
        json!({
            "_id": id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": parent_id.to_string(),
            "children": []
        })
    }

    pub fn splice_children(&mut self, from: usize, to: usize, insert: Vec<String>) -> Result<(), StepError> {
        match self {
            Block::StandardBlock(block) => block.children.splice(
                from..to,
                insert
            ),
            Block::Root(block) => block.children.splice(from..to, insert),
            Block::InlineBlock(_) => return Err(StepError("InlineBlock does not have children".to_string()))
        };
        Ok(())
    }
}

pub fn id_from_json_block(json: &Value) -> Result<String, StepError> {
    let _id = json.get("_id").ok_or(StepError(format!("Block json does not have _id field: {}", json)))?
        .as_str().ok_or(StepError("Block _id field is not a string".to_string()))?;
    String::from_str(_id).map_err(|_| StepError("Block _id field is not a valid String".to_string()))
}

pub fn children_from_json_block(json: &Value) -> Result<Vec<String>, StepError> {
    let children = json.get("children").ok_or(StepError("Block does not have children field".to_string()))?.as_array().ok_or(StepError("Block children field is not an array".to_string()))?;
    children.iter().map(|child| {
        let child_id = child.as_str().ok_or(StepError("Block children field is not an array of strings".to_string()))?;
        String::from_str(child_id).map_err(|_| StepError("Block children field is not an array of valid Strings".to_string()))
    }).collect()
}

pub fn parent_from_json_block(json: &Value) -> Result<String, StepError> {
    let parent = json.get("parent").ok_or(StepError("Block does not have parent field".to_string()))?.as_str().ok_or(StepError("Block parent field is not a string".to_string()))?;
    String::from_str(parent).map_err(|_| StepError("Block parent field is not a valid String".to_string()))
}

pub fn marks_from_json_block(json: &Value) -> Result<Vec<Mark>, StepError> {
    let marks = json.get("marks").ok_or(StepError("Block does not have marks field".to_string()))?.as_array().ok_or(StepError("Block marks field is not an array".to_string()))?;
    marks.iter().map(|mark| {
        let mark = mark.as_str().ok_or(StepError("Block marks field is not an array of strings".to_string()))?;
        Mark::from_str(mark).map_err(|_| StepError("Block marks field is not an array of valid Mark strings".to_string()))
    }).collect()
}

#[derive(Debug, PartialEq, Clone)]
pub struct RootBlock {
    pub _id: String,
    pub children: Vec<String>,
}

impl RootBlock {
    pub fn from(_id: String, children: Vec<String>) -> Self {
        Self {
            _id,
            children
        }
    }

    pub fn id(&self) -> String {
        return self._id.clone()
    }

    pub fn index_of_child(&self, child_id: &str) -> Result<usize, StepError> {
        match self.children.iter().position(|id| *id == *child_id) {
            Some(index) => Ok(index),
            None => Err(StepError("Child not found".to_string()))
        }
    }

    pub fn get_child_from_index(&self, index: usize) -> Result<String, StepError> {
        match self.children.get(index) {
            Some(block_id) => Ok(block_id.clone()),
            None => Err(StepError("Block not found".to_string()))
        }
    }

    pub fn json_from(_id: String, children: Vec<String>) -> Value {
        json!({
            "_id": _id.to_string(),
            "kind": "root",
            //collect as vec string
            "children": children.into_iter().map(|id| id.to_string()).collect::<Vec<String>>()
        })
    }
}

/// HashMap<Id, JSON (as str)>
#[derive(Serialize, Deserialize)]
pub struct BlockMap(pub HashMap<String, String>);

impl BlockMap {
    pub fn from(blocks: Vec<String>) -> Result<Self, StepError> {
        let mut map = HashMap::new();
        for block_json_str in blocks {
            let block = match serde_json::Value::from_str(&block_json_str) {
                Ok(block) => block,
                Err(_) => return Err(StepError(format!("Failed to parse json from str for block: {}", block_json_str)))
            };
            let id = match block.get("_id") {
                Some(id) => id.as_str().ok_or(StepError("Block _id field is not a string".to_string())),
                None => Err(StepError("Block does not have _id field".to_string()))
            }?;
            map.insert(String::from_str(id).unwrap(), block_json_str);
        }
        Ok(Self(map))
    }

    pub fn get_block(&self, id: &str) -> Result<Block, StepError> {
        match self.0.get(id) {
            Some(block) => Block::from_json(block),
            None => Err(StepError(format!("Block with id {} does not exist", id)))
        }
    }

    pub fn get_standard_block(&self, id: &str) -> Result<StandardBlock, StepError> {
        let block = self.get_block(id)?;
        match block {
            Block::StandardBlock(block) => Ok(block),
            Block::InlineBlock(_) => Err(StepError(format!("Block with id {} is an inline block, not a standard block", id))),
            Block::Root(_) => Err(StepError(format!("Block with id {} is a root block, not a standard block", id)))
        }
    }

    pub fn get_inline_block(&self, id: &String) -> Result<InlineBlock, StepError> {
        let block = self.get_block(id)?;
        match block {
            Block::StandardBlock(_) => Err(StepError(format!("Block with id {} is a standard block, not an inline block", id))),
            Block::InlineBlock(block) => Ok(block),
            Block::Root(_) => Err(StepError(format!("Block with id {} is a root block, not an inline block", id)))
        }
    }

    pub fn get_root_block(&self, id: &String) -> Result<RootBlock, StepError> {
        let block = self.get_block(id)?;
        match block {
            Block::StandardBlock(_) => Err(StepError(format!("Block with id {} is a standard block, not a root block", id))),
            Block::InlineBlock(_) => Err(StepError(format!("Block with id {} is an inline block, not a root block", id))),
            Block::Root(block) => Ok(block)
        }
    }

    pub fn ids_to_blocks(&self, ids: &Vec<String>) -> Result<Vec<Block>, StepError> {
        ids.iter().map(|id| self.get_block(id)).collect()
    }

    pub fn get_nearest_ancestor_standard_block_incl_self(&self, id: &String) -> Result<StandardBlock, StepError> {
        let from_block = self.get_block(id)?;
        return match from_block {
            Block::StandardBlock(block) => Ok(block),
            Block::InlineBlock(block) => {
                let parent_block = self.get_block(&block.parent)?;
                match parent_block {
                    Block::StandardBlock(block) => Ok(block),
                    _ => return Err(StepError("Invalid block structure".to_string()))
                }
            },
            Block::Root(_) => return Err(StepError("Cannot enter on root block".to_string())),
        }
    }

    pub fn update_block(&mut self, block: Block) -> Result<Option<String>, StepError> {
        let id = block.id();
        let json = block.to_json()?.to_string();
        return Ok(self.0.insert(id, json))
    }

    pub fn remove_block(&mut self, id: &String) -> Result<Option<Block>, StepError> {
        let returned_block_as_json = self.0.remove(id);
        match returned_block_as_json {
            Some(block_as_json) => {
                let block = Block::from_json(&block_as_json)?;
                Ok(Some(block))
            },
            None => Ok(None)
        }
    }
}