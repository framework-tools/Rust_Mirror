
use std::ops::Index;

use serde_json::json;
use wasm_bindgen::JsValue;

use crate::{mark::Mark, steps_generator::{StepError, mark_steps::ForSelection, selection::SubSelection}, new_ids::NewIds, frontend_interface::get_js_field_as_string, step::AddBlockStep, steps_actualisor::clean_block_after_transform};

use self::{content_block::ContentBlock, list_block::ListBlock};

use super::{inline_blocks::InlineBlock, BlockMap, Block, vec_string_to_arr};

pub mod content_block;
pub mod list_block;

#[derive(Debug, PartialEq, Clone)]
pub struct StandardBlock {
    pub _id: String,
    pub content: StandardBlockType,
    pub children: Vec<String>, //Vec<StandardBlock>
    pub parent: String, //StandardBlock
    pub marks: Vec<Mark>
}

impl StandardBlock {
    pub fn id(&self) -> String {
        return self._id.clone()
    }

    pub fn parent(&self) -> String {
        return self.parent.clone()
    }

    pub fn from(content: StandardBlockType, parent: String, new_ids: &mut NewIds) -> Result<Self, StepError> {
        return Ok(Self {
            _id: new_ids.get_id()?,
            content,
            children: vec![],
            parent,
            marks: vec![]
        })
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
            StandardBlockType::TodoList(content) | StandardBlockType::ArrowList(content) |
            StandardBlockType::DotPointList(content) | StandardBlockType::NumberedList(content) => Ok(&content.content),
            _ => Err(StepError("Block does not have a content block".to_string()))
        }
    }

    pub fn remove_blocks_between_offsets(self, from_offset: usize, to_offset: usize) -> Result<Self, StepError> {
        let content_block = self.content_block()?.clone();
        let new_inline_blocks = content_block.inline_blocks[from_offset + 1..to_offset].to_vec();
        return self.update_block_content(ContentBlock { inline_blocks: new_inline_blocks })
    }

    pub fn all_inline_blocks_in_range_have_identical_mark(
        &self,
        mark: &Mark,
        from: usize,
        to: usize,
        _for: ForSelection,
        block_map: &BlockMap
    ) -> Result<bool, StepError> {
        let inline_blocks = self.content_block()?.inline_blocks.clone();
        let mut i = from;
        match _for {
            ForSelection::Both(offset, _) | ForSelection::From(offset) => {
                let from_block = block_map.get_inline_block(&inline_blocks[i])?;
                if offset == from_block.text()?.len() {
                    i += 1;
                }
            },
            _ => {}
        };
        while i < to + 1 && i < inline_blocks.len() {
            let inline_block = block_map.get_inline_block(&inline_blocks[i])?;

            if i == to {
                match _for {
                    ForSelection::To(0) => {
                        return Ok(true)
                    },
                    _ => {}
                };
            }

            if !inline_block.marks.contains(mark) {
                return Ok(false)
            }
            i += 1;
        }
        return Ok(true)
    }

    pub fn all_inline_blocks_have_identical_mark(
        &self,
        mark: &Mark,
        block_map: &BlockMap
    ) -> Result<bool, StepError> {
        let inline_blocks = self.content_block()?.inline_blocks.clone();
        let mut i = 0;
        while i < inline_blocks.len() {
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

    pub fn split(mut self, index: usize, mut new_block_content: StandardBlockType, new_ids: &mut NewIds) -> Result<(Self, Self), StepError> {
        let inline_blocks = &self.content_block()?.inline_blocks;
        if index > inline_blocks.len() {
            return Err(StepError(format!("Inline blocks length: {}, is less than index: {}", inline_blocks.len(), index)))
        }

        let first_half = inline_blocks[..index].to_vec();
        let second_half = inline_blocks[index..].to_vec();
        self = self.update_block_content(ContentBlock { inline_blocks: first_half })?;
        let mut new_block = StandardBlock::from(new_block_content, self.parent.clone(), new_ids)?.push_to_content(second_half)?;
        new_block.children = self.children;
        self.children = vec![];
        return Ok((self, new_block))
    }

    fn push_to_content(mut self, new_inline_blocks: Vec<String>) -> Result<Self, StepError> {
        self.content = self.content.push_to_content(new_inline_blocks)?;
        return Ok(self)
    }

    pub fn set_as_parent_for_all_inline_blocks(
        &self,
        mut block_map: BlockMap,
        blocks_to_update: &mut Vec<String>
    ) -> Result<BlockMap, StepError> {
        for id in &self.content_block()?.inline_blocks {
            let mut inline_block = block_map.get_inline_block(&id)?;
            inline_block.parent = self.id();
            block_map.update_block(Block::InlineBlock(inline_block), blocks_to_update)?;
        }
        return Ok(block_map)
    }

    pub fn parent_is_root(&self, block_map: &BlockMap) -> bool {
        return block_map.get_root_block(&self.parent).is_ok()
    }

    pub fn set_new_parent_of_children(
        &self,
        block_map: &mut BlockMap,
        blocks_to_update: &mut Vec<String>
    ) -> Result<(), StepError> {
        for id in &self.children {
            let mut block = block_map.get_standard_block(id)?;
            block.parent = self.id();
            block_map.update_block(Block::StandardBlock(block), blocks_to_update)?;
        }
        return Ok(())
    }

    /// Getting the last child on the deepest layer
    pub fn get_youngest_descendant(self, block_map: &BlockMap) -> Result<Self, StepError> {
        match self.children.len() > 0 {
            true => {
                let youngest_child = block_map.get_standard_block(&self.children[self.children.len() - 1])?;
                return youngest_child.get_youngest_descendant(block_map)
            },
            false => return Ok(self)
        }
    }

    pub fn next_sibling(&self, block_map: &BlockMap) -> Result<Option<StandardBlock>, StepError> {
        let parent = self.get_parent(block_map)?;
        let next_sibling = parent.get_child_from_index(self.index(block_map)? + 1);
        return match next_sibling {
            Ok(id) => Ok(Some(block_map.get_standard_block(&id)?)),
            Err(_) => Ok(None)
        }
    }

    pub fn get_siblings_after(&self, block_map: &BlockMap) -> Result<Vec<String>, StepError> {
        let parent = self.get_parent(block_map)?;
        let children_after = &parent.children()?[self.index(block_map)? + 1 ..];
        return Ok(children_after.to_vec())
    }

    pub fn parents_next_sibling(&self, block_map: &BlockMap) -> Result<Option<StandardBlock>, StepError> {
        let parent = block_map.get_standard_block(&self.parent)?;
        let grand_parent = parent.get_parent(block_map)?;
        let next_sibling = grand_parent.get_child_from_index(parent.index(block_map)? + 1);
        return match next_sibling {
            Ok(id) => Ok(Some(block_map.get_standard_block(&id)?)),
            Err(_) => Ok(None)
        }
    }

    pub fn apply_mark_to_all_inline_blocks_in_range(
        &self,
        mark: Mark,
        from: usize,
        to: usize,
        block_map: &mut BlockMap,
        add_mark: bool,
        blocks_to_update: &mut Vec<String>
    ) -> Result<(), StepError> {
        let inline_blocks = &self.content_block()?.inline_blocks;
        let mut i = from;
        while i < to + 1 && i < inline_blocks.len() {
            let mut inline_block = block_map.get_inline_block(&inline_blocks[i])?;
            inline_block = inline_block.apply_mark(mark.clone(), add_mark);
            block_map.update_block(Block::InlineBlock(inline_block), blocks_to_update)?;
            i += 1;
        }
        return Ok(())
    }

    pub fn apply_mark_to_all_inline_blocks(
        &self,
        mark: Mark,
        block_map: &mut BlockMap,
        add_mark: bool,
        blocks_to_update: &mut Vec<String>
    ) -> Result<(), StepError> {
        let inline_blocks = &self.content_block()?.inline_blocks;
        let mut i = 0;
        while i < inline_blocks.len() {
            let mut inline_block = block_map.get_inline_block(&inline_blocks[i])?;
            inline_block = inline_block.apply_mark(mark.clone(), add_mark);
            block_map.update_block(Block::InlineBlock(inline_block), blocks_to_update)?;
            i += 1;
        }
        return Ok(())
    }

    pub fn is_list(&self) -> bool {
        return match self.content {
            StandardBlockType::TodoList(_) | StandardBlockType::DotPointList(_)
            | StandardBlockType::ArrowList(_) | StandardBlockType::NumberedList(_) => true,
            _ => false
        }
    }

    pub fn text_is_empty(&self, block_map: &BlockMap) -> Result<bool, StepError> {
        let content = self.content_block()?;
        if content.inline_blocks.len() == 1 {
            let inline_block = block_map.get_inline_block(&content.inline_blocks[0])?;
            return Ok(inline_block.text()?.len() == 0)
        } else {
            return Ok(false)
        }
    }

    pub fn depth_from_root(&self, block_map: &BlockMap) -> Result<usize, StepError> {
        let mut i = 0;
        let mut current_block = self.clone();
        loop  {
            current_block = match current_block.parent_is_root(block_map) {
                true => break,
                false => block_map.get_standard_block(&current_block.id())?
            };
            i += 1;
            current_block = block_map.get_standard_block(&current_block.parent)?;
        }
        return Ok(i)
    }

    pub fn get_inline_blocks(&self, block_map: &BlockMap) -> Result<Vec<InlineBlock>, StepError> {
        let inline_blocks_ids = &self.content_block()?.inline_blocks;
        let mut inline_blocks = Vec::new();
        for id in inline_blocks_ids {
            inline_blocks.push(block_map.get_inline_block(id)?);
        }
        return Ok(inline_blocks)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StandardBlockType {
    Paragraph(ContentBlock),
    H1(ContentBlock),
    H2(ContentBlock),
    H3(ContentBlock),
    TodoList(ListBlock),
    DotPointList(ListBlock),
    NumberedList(ListBlock),
    ArrowList(ListBlock),
}

impl StandardBlockType {
    pub fn from_js_block(obj: &JsValue) -> Result<Self, StepError> {
        let _type = get_js_field_as_string(obj, "_type")?;

        match _type.as_str() {
            "paragraph" => Ok(StandardBlockType::Paragraph(ContentBlock::from_js_block(obj)?)),
            "h1" => Ok(StandardBlockType::H1(ContentBlock::from_js_block(obj)?)),
            "h2" => Ok(StandardBlockType::H2(ContentBlock::from_js_block(obj)?)),
            "h3" => Ok(StandardBlockType::H3(ContentBlock::from_js_block(obj)?)),
            "to-do list" => Ok(StandardBlockType::TodoList(ListBlock::from_js_block(obj)?)),
            "dotpoint list" => Ok(StandardBlockType::DotPointList(ListBlock::from_js_block(obj)?)),
            "numbered list" => Ok(StandardBlockType::NumberedList(ListBlock::from_js_block(obj)?)),
            "arrow list" => Ok(StandardBlockType::ArrowList(ListBlock::from_js_block(obj)?)),
            _type => Err(StepError(format!("Block type '{}' not found", _type)))
        }
    }

    pub fn from_json_block(json: &serde_json::Value) -> Result<Self, StepError> {
        let block_type = json.get("_type").ok_or(StepError("Block does not have _type field".to_string()))?.as_str().ok_or(StepError("Block _type field is not a string".to_string()))?;
        match block_type {
            "paragraph" => Ok(StandardBlockType::Paragraph(ContentBlock::from_json(json)?)),
            "h1" => Ok(StandardBlockType::H1(ContentBlock::from_json(json)?)),
            "h2" => Ok(StandardBlockType::H2(ContentBlock::from_json(json)?)),
            "h3" => Ok(StandardBlockType::H3(ContentBlock::from_json(json)?)),
            "to-do list" => Ok(StandardBlockType::TodoList(ListBlock::from_json(json)?)),
            "dotpoint list" => Ok(StandardBlockType::DotPointList(ListBlock::from_json(json)?)),
            "numbered list" => Ok(StandardBlockType::NumberedList(ListBlock::from_json(json)?)),
            "arrow list" => Ok(StandardBlockType::ArrowList(ListBlock::from_json(json)?)),
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
            },
            StandardBlockType::TodoList(block) => {
                json!({
                    "_type": "to-do list",
                    "content": {
                        "inline_blocks": block.content.inline_blocks.iter().map(|inline_block| inline_block.to_string()).collect::<Vec<String>>(),
                        "completed": block.completed
                    }
                })
            },
            StandardBlockType::DotPointList(block) => {
                json!({
                    "_type": "dotpoint list",
                    "content": {
                        "inline_blocks": block.content.inline_blocks.iter().map(|inline_block| inline_block.to_string()).collect::<Vec<String>>(),
                        "completed": block.completed
                    }
                })
            },
            StandardBlockType::NumberedList(block) => {
                json!({
                    "_type": "numbered list",
                    "content": {
                        "inline_blocks": block.content.inline_blocks.iter().map(|inline_block| inline_block.to_string()).collect::<Vec<String>>(),
                        "completed": block.completed
                    }
                })
            },
            StandardBlockType::ArrowList(block) => {
                json!({
                    "_type": "arrow list",
                    "content": {
                        "inline_blocks": block.content.inline_blocks.iter().map(|inline_block| inline_block.to_string()).collect::<Vec<String>>(),
                        "completed": block.completed
                    }
                })
            },
        }
    }

    pub fn _type_as_string(&self) -> Result<String, StepError> {
        match self {
            StandardBlockType::Paragraph(_) => return Ok("paragraph".to_string()),
            StandardBlockType::H1(_) => return Ok("h1".to_string()),
            StandardBlockType::H2(_) => return Ok("h2".to_string()),
            StandardBlockType::H3(_) => return Ok("h3".to_string()),
            StandardBlockType::TodoList(_) => return Ok("to-do list".to_string()),
            StandardBlockType::DotPointList(_) => return Ok("dotpoint list".to_string()),
            StandardBlockType::NumberedList(_) => return Ok("numbered list".to_string()),
            StandardBlockType::ArrowList(_) => return Ok("arrow list".to_string()),
        }
    }

    pub fn to_js(&self) -> Result<JsValue, StepError> {
        let content = js_sys::Object::new();
        match self {
            StandardBlockType::Paragraph(content_block) | StandardBlockType::H1(content_block) |
            StandardBlockType::H2(content_block) | StandardBlockType::H3(content_block)
                => {
                    js_sys::Reflect::set(&content, &JsValue::from_str("inline_blocks"), &vec_string_to_arr(&content_block.inline_blocks)?.into()).unwrap();
            },
            StandardBlockType::TodoList(list_block) | StandardBlockType::DotPointList(list_block) |
            StandardBlockType::NumberedList(list_block) | StandardBlockType::ArrowList(list_block)
                => {
                    js_sys::Reflect::set(&content, &JsValue::from_str("inline_blocks"), &vec_string_to_arr(&list_block.content.inline_blocks)?.into()).unwrap();
                    js_sys::Reflect::set(&content, &JsValue::from_str("completed"), &JsValue::from(list_block.completed)).unwrap();
            }
        }
        return Ok(content.into())
    }

    pub fn update_block_content(&self, content_block: ContentBlock) -> Result<Self, StepError> {
        match self {
            StandardBlockType::Paragraph(_) => Ok(StandardBlockType::Paragraph(content_block)),
            StandardBlockType::H1(_) => Ok(StandardBlockType::H1(content_block)),
            StandardBlockType::H2(_) => Ok(StandardBlockType::H2(content_block)),
            StandardBlockType::H3(_) => Ok(StandardBlockType::H3(content_block)),
            StandardBlockType::TodoList(list_block) => Ok(StandardBlockType::TodoList(ListBlock {
                content: content_block,
                completed: list_block.completed
            })),
            StandardBlockType::DotPointList(list_block) => Ok(StandardBlockType::DotPointList(ListBlock {
                content: content_block,
                completed: list_block.completed
            })),
            StandardBlockType::NumberedList(list_block) => Ok(StandardBlockType::NumberedList(ListBlock {
                content: content_block,
                completed: list_block.completed
            })),
            StandardBlockType::ArrowList(list_block) => Ok(StandardBlockType::ArrowList(ListBlock {
                content: content_block,
                completed: list_block.completed
            })),
        }
    }

    pub fn push_to_content(self, new_inline_blocks: Vec<String>) -> Result<Self, StepError> {
        match self {
            StandardBlockType::Paragraph(ContentBlock { inline_blocks } ) => {
                let updated_inline_blocks = vec![inline_blocks, new_inline_blocks].concat();
                return Ok(StandardBlockType::Paragraph(ContentBlock { inline_blocks: updated_inline_blocks } ))
            },
            StandardBlockType::H1(ContentBlock { inline_blocks }) => {
                let updated_inline_blocks = vec![inline_blocks, new_inline_blocks].concat();
                return Ok(StandardBlockType::H1(ContentBlock { inline_blocks: updated_inline_blocks } ))
            },
            StandardBlockType::H2(ContentBlock { inline_blocks }) => {
                let updated_inline_blocks = vec![inline_blocks, new_inline_blocks].concat();
                return Ok(StandardBlockType::H2(ContentBlock { inline_blocks: updated_inline_blocks } ))
            },
            StandardBlockType::H3(ContentBlock { inline_blocks }) => {
                let updated_inline_blocks = vec![inline_blocks, new_inline_blocks].concat();
                return Ok(StandardBlockType::H3(ContentBlock { inline_blocks: updated_inline_blocks } ))
            },
            StandardBlockType::TodoList(list_block) => {
                let updated_inline_blocks = vec![list_block.content.inline_blocks, new_inline_blocks].concat();
                return Ok(Self::TodoList(ListBlock {
                    content: ContentBlock { inline_blocks: updated_inline_blocks },
                    completed: list_block.completed
                }))
            },
            StandardBlockType::DotPointList(list_block) => {
                let updated_inline_blocks = vec![list_block.content.inline_blocks, new_inline_blocks].concat();
                return Ok(Self::DotPointList(ListBlock {
                    content: ContentBlock { inline_blocks: updated_inline_blocks },
                    completed: list_block.completed
                }))
            },
            StandardBlockType::NumberedList(list_block) => {
                let updated_inline_blocks = vec![list_block.content.inline_blocks, new_inline_blocks].concat();
                return Ok(Self::NumberedList(ListBlock {
                    content: ContentBlock { inline_blocks: updated_inline_blocks },
                    completed: list_block.completed
                }))
            },
            StandardBlockType::ArrowList(list_block) => {
                let updated_inline_blocks = vec![list_block.content.inline_blocks, new_inline_blocks].concat();
                return Ok(Self::ArrowList(ListBlock {
                    content: ContentBlock { inline_blocks: updated_inline_blocks },
                    completed: list_block.completed
                }))
            },
        }
    }
}