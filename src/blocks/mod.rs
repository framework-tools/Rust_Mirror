

use std::{collections::HashMap, str::FromStr};
use js_sys::{Map, JsString};
use serde_json::{Value, json};
use wasm_bindgen::JsValue;

use crate::{mark::Mark, steps_generator::StepError, frontend_interface::{get_js_field_as_string, get_js_field}};

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
                    content: StandardBlockType::from_json_block(&json)?,
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

    pub fn parent(&self) -> Result<String, StepError> {
        match self {
            Block::StandardBlock(block) => Ok(block.parent.clone()),
            Block::InlineBlock(block) => Ok(block.parent.clone()),
            Block::Root(_) => Err(StepError("Root block does not have a parent".to_string()))
        }
    }


    pub fn from_js_obj(obj: &JsValue) -> Result<Self, StepError> {
        let kind = get_js_field_as_string(obj, "kind")?;

        return match kind.as_str() {
            "standard" => {
                Ok(Block::StandardBlock(StandardBlock {
                    _id: id_from_js_block(obj)?,
                    content: StandardBlockType::from_js_block(obj)?,
                    children: children_from_js_block(obj)?,
                    parent: parent_from_js_block(obj)?,
                    marks: marks_from_js_block(obj)?,
                }))
            },
            "inline" => {
                Ok(Block::InlineBlock(InlineBlock {
                    _id: id_from_js_block(obj)?,
                    content: InlineBlockType::from_js_block(obj)?,
                    parent: parent_from_js_block(obj)?,
                    marks: marks_from_js_block(obj)?,
                }))
            },
            "root" => {
                Ok(Block::Root(RootBlock {
                    _id: id_from_js_block(obj)?,
                    children: children_from_js_block(obj)?,
                }))
            },
            _ => Err(StepError(format!("Block kind {} not found", kind)))
        }
    }

    pub fn to_js_block(self) -> Result<JsValue, StepError> {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("_id"), &JsValue::from_str(self.id().as_str())).unwrap();
        match self {
            Block::InlineBlock(inline_block) => {
                js_sys::Reflect::set(&obj, &JsValue::from_str("kind"), &JsValue::from_str("inline")).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("_type"), &JsValue::from_str(inline_block.content._type_as_string()?.as_str())).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("content"), &inline_block.content.to_js_content()?).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("marks"), &marks_to_js_arr(inline_block.marks)?).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("parent"), &JsValue::from_str(inline_block.parent.as_str())).unwrap();
            },
            Block::StandardBlock(std_block) => {
                js_sys::Reflect::set(&obj, &JsValue::from_str("kind"), &JsValue::from_str("standard")).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("_type"), &JsValue::from_str(std_block.content._type_as_string().as_str())).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("content"), &std_block.content.to_js()?).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("children"), &vec_string_to_arr(&std_block.children)?).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("marks"), &marks_to_js_arr(std_block.marks)?).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("parent"), &JsValue::from_str(std_block.parent.as_str())).unwrap();
            },
            Block::Root(root) => {
                js_sys::Reflect::set(&obj, &JsValue::from_str("kind"), &JsValue::from_str("root")).unwrap();
                js_sys::Reflect::set(&obj, &JsValue::from_str("children"), &vec_string_to_arr(&root.children)?).unwrap();
            }
        }
        return Ok(obj.into())
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

    pub fn index(&self, block_map: &BlockMap) -> Result<usize, StepError> {
        match self {
            Block::InlineBlock(block) => return block.index(block_map),
            Block::StandardBlock(block) => return block.index(block_map),
            Block::Root(_) => return Err(StepError("Should not try to get index of the root block".to_string()))
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
                "inline_blocks": []
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

    pub fn remove_child_from_id(mut self, id: &str) -> Result<Self, StepError> {
        let child_index = self.index_of_child(id)?;
        self.splice_children(child_index, child_index + 1, vec![])?;
        return Ok(self)
    }

    pub fn set_children(&mut self, new_children: Vec<String>) -> Result<(), StepError> {
        match self {
            Self::StandardBlock(block) => block.children = new_children,
            Self::Root(block) => block.children = new_children,
            Self::InlineBlock(_) => return Err(StepError("Inline blocks cannot have children.".to_string()))
        };
        return Ok(())
    }

    pub fn insert_child(&mut self, child_id: String, index: usize)  -> Result<(), StepError> {
        match self {
            Self::StandardBlock(block) => block.children.insert(index, child_id),
            Self::Root(block) => block.children.insert(index, child_id),
            Self::InlineBlock(_) => return Err(StepError("Inline blocks cannot have children..".to_string()))
        };
        return Ok(())
    }

    pub fn set_new_parent_of_children(&self, block_map: &mut BlockMap, blocks_to_update: &mut Vec<String>) -> Result<(), StepError> {
        match self {
            Self::StandardBlock(std_block) => return std_block.set_new_parent_of_children(block_map, blocks_to_update),
            Self::Root(root) => {
                for id in &root.children {
                    let mut block = block_map.get_standard_block(id)?;
                    block.parent = self.id();
                    block_map.update_block(Block::StandardBlock(block), blocks_to_update)?;
                }
            },
            Self::InlineBlock(_) => return Err(StepError("Inline blocks do not contain children".to_string()))
        };
        return Ok(())
    }

    pub fn is_root(&self) -> bool {
        match self {
            Self::Root(_) => true,
            _ => false
        }
    }
}

pub fn id_from_js_block(obj: &JsValue) -> Result<String, StepError> {
    return get_js_field_as_string(obj, "_id")
}

pub fn children_from_js_block(obj: &JsValue) -> Result<Vec<String>, StepError> {
    let children = js_sys::Array::from(&get_js_field(obj, "children")?);
    let children: Vec<String> = children.iter().map(|child| {
        child.as_string().ok_or(StepError("Block children field is not an array of strings".to_string()))
    }).collect::<Result<Vec<String>, StepError>>()?;
    return Ok(children)
}

pub fn parent_from_js_block(obj: &JsValue) -> Result<String, StepError> {
    return get_js_field_as_string(obj, "parent")
}

pub fn marks_from_js_block(obj: &JsValue) -> Result<Vec<Mark>, StepError> {
    let marks = js_sys::Array::from(&get_js_field(obj, "marks")?);
    let marks: Vec<Mark> = marks.iter().map(|mark| {
        let mark = mark.as_string().ok_or(StepError("Block marks field is not an array of strings".to_string()))?;
        Mark::from_str(&mark).map_err(|_| StepError("Block marks field is not an array of valid Mark strings".to_string()))
    }).collect::<Result<Vec<Mark>, StepError>>()?;
    return Ok(marks)
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
#[derive(Debug, Clone)]
pub enum BlockMap {
    Js(Map),
    Rust(HashMap<String, String>)
}

impl BlockMap {
    pub fn from_js_map(js_map: Map) -> Self {
        return BlockMap::Js(js_map)
    }

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
        Ok(Self::Rust(map))
    }

    pub fn get_block(&self, id: &str) -> Result<Block, StepError> {
        match self {
            Self::Rust(rust_map) => match rust_map.get(id) {
                Some(block) => return Block::from_json(block),
                None => Err(StepError(format!("Block with id {} does not exist", id)))
            },
            Self::Js(js_map) => {
                let opt_block = js_map.get(&JsString::from(id));
                match opt_block.is_null() {
                    true => Err(StepError(format!("Block with id {} does not exist", id))),
                    false => return Block::from_js_obj(&opt_block)
                }
            }
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

    pub fn update_block(&mut self, block: Block, blocks_to_update: &mut Vec<String>) -> Result<Option<String>, StepError> {
        let id = block.id();
        blocks_to_update.push(id.clone());
        match self {
            Self::Rust(rust_map) => {
                let json = block.to_json()?.to_string();
                return Ok(rust_map.insert(id, json))
            },
            Self::Js(js_map) => {
                js_map.set(&JsValue::from_str(&id), &block.to_js_block()?);
                return Ok(None)
            }
        }
    }

    pub fn update_blocks(&mut self, blocks: Vec<Block>, blocks_to_update: &mut Vec<String>) -> Result<Vec<Option<String>>, StepError> {
        let mut return_values = vec![];
        for block in blocks {
            return_values.push(self.update_block(block, blocks_to_update)?)
        }
        return Ok(return_values)
    }

    pub fn remove_block(&mut self, id: &String) -> Result<Option<Block>, StepError> {
        match self {
            Self::Rust(rust_map) => {
                let returned_block_as_json = rust_map.remove(id);
                match returned_block_as_json {
                    Some(block_as_json) => {
                        let block = Block::from_json(&block_as_json)?;
                        Ok(Some(block))
                    },
                    None => Ok(None)
                }
            },
            Self::Js(_js_map) => {
                // let returned_block_as_json = js_map.remove(id);
                // match returned_block_as_json {
                //     Some(block_as_json) => {
                //         let block = Block::from_json(&block_as_json)?;
                //         Ok(Some(block))
                //     },
                //     None => Ok(None)
                // }
                unimplemented!()
            }
        }
    }

    /// iterate through block map to add and use "update block" on self for each block
    pub fn add_block_map(&mut self, block_map_to_add: BlockMap) -> Result<(), StepError> {
        match block_map_to_add {
            BlockMap::Rust(rust_map) => {
                for (_, block_as_json) in rust_map {
                    let block = Block::from_json(&block_as_json)?;
                    self.update_block(block, &mut vec![])?;
                }
            },
            BlockMap::Js(js_map) => {
                for block_as_js in js_map.values() {
                    let block = Block::from_js_obj(&block_as_js.unwrap())?;
                    self.update_block(block, &mut vec![])?;
                }
            }
        }

        return Ok(())
    }

    /// Utility for tests
    pub fn get_newly_added_blocks(&self, previously_used_ids: Vec<String>) -> Result<Vec<Block>, StepError> {
        let mut newly_added_blocks = vec![];
        match self {
            Self::Rust(rust_map) => {
                for (id, _) in rust_map {
                    if !previously_used_ids.contains(id) {
                        newly_added_blocks.push(self.get_block(id)?)
                    }
                }
            }
            Self::Js(_) => return Err(StepError("This function is only used for tests".to_string()))
        };
        return Ok(newly_added_blocks)
    }

    /// Utility for tests
    pub fn get_newly_added_standard_blocks(&self, previously_used_ids: Vec<String>) -> Result<Vec<StandardBlock>, StepError> {
        let mut newly_added_standard_blocks = vec![];
        let newly_added_blocks = self.get_newly_added_blocks(previously_used_ids)?;
        for block in newly_added_blocks {
            match block {
                Block::StandardBlock(std_block) => newly_added_standard_blocks.push(std_block),
                _ => {}
            };
        }
        return Ok(newly_added_standard_blocks)
    }

    pub fn to_js_map(self) -> Result<Map, StepError> {
        match self {
            Self::Js(js_map) => Ok(js_map),
            Self::Rust(rust_map) => {
                let js_map = js_sys::Map::new();
                for (id, json) in rust_map {
                    js_map.set(&JsValue::from_str(&id) ,&js_sys::JSON::parse(&json).unwrap());
                }
                return Ok(js_map)
            }
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        match self {
            Self::Js(js_map) => js_map.has(&JsValue::from_str(key)),
            Self::Rust(rs_map) => rs_map.contains_key(key)
        }
    }

    pub fn only_one_std_block(&self, any_std_block_id: &str) -> Result<bool, StepError> {
        let block = self.get_standard_block(any_std_block_id)?;
        let parent = block.get_parent(self)?;
        match parent {
            Block::Root(root_block) => {
                return Ok(root_block.children.len() == 1 && block.children.len() == 0)
            },
            _ => return Ok(false)
        }
    }
}

pub fn id_from_json_block(json: &Value) -> Result<String, StepError> {
    let _id = json.get("_id").ok_or(StepError(format!("Block json does not have _id field: {}", json)))?
        .as_str().ok_or(StepError("Block _id field is not a string".to_string()))?;
    String::from_str(_id).map_err(|_| StepError("Block _id field is not a valid String".to_string()))
}

pub fn children_from_json_block(json: &Value) -> Result<Vec<String>, StepError> {
    let children = json.get("children")
        .ok_or(StepError("Block does not have children field".to_string()))?
        .as_array().ok_or(StepError("Block children field is not an array".to_string()))?;
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

pub fn marks_to_js_arr(marks: Vec<Mark>) -> Result<JsValue, StepError> {
    let arr = js_sys::Array::new();
    for mark in marks {
        arr.push(&JsValue::from_str(mark.to_string().as_str()));
    }
    return Ok(JsValue::from(arr))
}

pub fn vec_string_to_arr(vec: &Vec<String>) -> Result<JsValue, StepError> {
    let arr = js_sys::Array::new();
    for s in vec {
        arr.push(&JsValue::from_str(s.as_str()));
    }
    return Ok(JsValue::from(arr))
}