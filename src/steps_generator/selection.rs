use std::{ops::Sub, str::FromStr};

use crate::{
    blocks::{standard_blocks::StandardBlock, Block, BlockMap},
    step::{ReplaceSlice, ReplaceStep},
};

use super::StepError;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Selection {
    pub anchor: SubSelection,
    pub head: SubSelection,
}

impl Selection {
    pub fn from(anchor: SubSelection, head: SubSelection) -> Self {
        Self { anchor, head }
    }

    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let anchor_obj =
            match js_sys::Reflect::get(&obj, &JsValue::from_str("anchor")) {
                Ok(anchor_obj) => anchor_obj,
                Err(_) => {
                    return Err(StepError(
                        "Failed to get anchor from selection js obj".to_string(),
                    ))
                }
            };
        let head_obj = match js_sys::Reflect::get(&obj, &JsValue::from_str("head")) {
            Ok(head_obj) => head_obj,
            Err(_) => {
                return Err(StepError(
                    "Failed to get head from selection js obj".to_string(),
                ))
            }
        };
        return Ok(Self {
            anchor: SubSelection::from_js_obj(anchor_obj)?,
            head: SubSelection::from_js_obj(head_obj)?,
        });
    }

    /// Converts anchor & head to "from" & "to"
    ///
    /// Check if top layer blocks are different
    /// -> if true => block with lower index is the from
    /// -> else =>
    ///     Check if inline blocks (deepest layer) are same
    ///     -> if true => compare offsets & return where lowest offset is the from
    ///     Check if anchor has more layers
    ///     -> if true => anchor is from
    pub fn get_from_to(
        self,
        block_map: &BlockMap,
    ) -> Result<(SubSelection, SubSelection), StepError> {
        if &self.anchor.block_id != &self.head.block_id {
            let anchor_block = block_map.get_block(&self.anchor.block_id)?;
            let head_block = block_map.get_block(&self.head.block_id)?;
            if anchor_block.index(block_map)? < head_block.index(block_map)? {
                return Ok((self.anchor, self.head));
            } else {
                return Ok((self.head, self.anchor));
            }
        } else {
            let deepest_anchor = self.anchor.get_deepest_subselection();
            let deepest_head = self.head.get_deepest_subselection();

            if deepest_anchor.block_id == deepest_head.block_id {
                // same inline block -> check offset
                if deepest_anchor.offset < deepest_head.offset {
                    return Ok((self.anchor, self.head));
                } else {
                    return Ok((self.head, self.anchor));
                }
            } else {
                if self.anchor.count_layers() < self.head.count_layers() {
                    return Ok((self.anchor, self.head));
                } else {
                    return Ok((self.head, self.anchor));
                }
            }
        }
    }

    pub fn update_selection_from(replace_step: ReplaceStep) -> Self {
        match replace_step.slice {
            ReplaceSlice::String(replace_slice) => {
                let deepest_from_subselection =
                    replace_step.from.get_deepest_subselection().clone();
                let subselection = SubSelection {
                    block_id: deepest_from_subselection.block_id,
                    offset: deepest_from_subselection.offset + replace_slice.len(),
                    subselection: None,
                };
                return Selection {
                    anchor: subselection.clone(),
                    head: subselection,
                };
            }
            ReplaceSlice::Blocks(blocks) => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SubSelection {
    pub block_id: String,
    pub offset: usize,
    pub subselection: Option<Box<SubSelection>>,
}

impl SubSelection {
    pub fn from(block_id: String, offset: usize, subselection: Option<Box<SubSelection>>) -> Self {
        Self {
            block_id,
            offset,
            subselection,
        }
    }

    pub fn from_js_obj(obj: JsValue) -> Result<Self, StepError> {
        let block_id = match js_sys::Reflect::get(&obj, &JsValue::from_str("block_id")) {
            Ok(block_id) => match block_id.as_string() {
                Some(block_id) => block_id,
                None => {
                    return Err(StepError(
                        "Block id on js obj subselection is not a string".to_string(),
                    ))
                }
            },
            Err(_) => {
                return Err(StepError(
                    "Failed to get block id from subselection js obj".to_string(),
                ))
            }
        };
        let offset = match js_sys::Reflect::get(&obj, &JsValue::from_str("offset")) {
            Ok(offset) => match offset.as_f64() {
                Some(offset) => offset as usize,
                None => {
                    return Err(StepError(
                        "Offset on js obj subselection is not a number".to_string(),
                    ))
                }
            },
            Err(_) => {
                return Err(StepError(
                    "Failed to get offset from subselection js obj".to_string(),
                ))
            }
        };
        let subselection = match js_sys::Reflect::get(&obj, &JsValue::from_str("subselection")) {
            Ok(subselection) => match subselection.is_null() {
                true => None,
                false => Some(Box::new(SubSelection::from_js_obj(subselection)?)),
            },
            Err(_) => {
                return Err(StepError(
                    "Failed to get subselection from subselection js obj".to_string(),
                ))
            }
        };
        return Ok(Self {
            block_id,
            offset,
            subselection,
        });
    }

    pub fn block_id(&self) -> String {
        return self.block_id.clone();
    }

    pub fn get_child_subselection(&self) -> Result<&SubSelection, StepError> {
        return match &self.subselection {
            Some(inner_subselection) => Ok(&*inner_subselection),
            None => return Err(StepError("Expected subselection to be Some".to_string())),
        };
    }

    pub fn get_deepest_subselection<'a>(&'a self) -> &'a Self {
        let mut subselection = self;
        loop {
            subselection = match &subselection.subselection {
                Some(subselection) => subselection,
                None => return &subselection,
            };
        }
    }

    pub fn get_two_deepest_layers(self) -> Result<Self, StepError> {
        let mut subselection = &self;
        match &subselection.subselection {
            Some(_) => {}
            None => return Err(StepError("Subselection only has one layer".to_string())),
        };

        loop {
            subselection = match &subselection.subselection {
                Some(sub_subselection) => match &sub_subselection.subselection {
                    Some(_) => &**sub_subselection,
                    None => return Ok(subselection.clone()),
                },
                None => return Ok(subselection.clone()),
            };
        }
    }

    pub fn count_layers(&self) -> usize {
        let mut subselection = self;
        let mut layers = 0;
        loop {
            layers += 1;
            subselection = match &subselection.subselection {
                Some(subselection) => subselection,
                None => return layers,
            };
        }
    }

    pub fn at_end_of_block(block_id: &str, block_map: &BlockMap) -> Result<Self, StepError> {
        let block = block_map.get_block(&block_id)?;
        match block {
            Block::InlineBlock(inline_block) => {
                return Ok(SubSelection {
                    block_id: inline_block.id(),
                    offset: inline_block.text()?.len(),
                    subselection: None,
                })
            }
            Block::StandardBlock(standard_block) => {
                let last_inline_block = standard_block.get_last_inline_block(block_map)?;
                return SubSelection::at_end_of_block(&last_inline_block._id, block_map);
            }
            Block::Root(root_block) => {
                return SubSelection::at_end_of_block(
                    &root_block.children[root_block.children.len() - 1],
                    block_map,
                )
            }
        }
    }

    pub fn at_end_of_youngest_descendant(
        standard_block: &StandardBlock,
        block_map: &BlockMap,
    ) -> Result<SubSelection, StepError> {
        match standard_block.children.len() > 0 {
            true => {
                let youngest_child = block_map.get_standard_block(
                    &standard_block.children[standard_block.children.len() - 1],
                )?;
                return Ok(SubSelection {
                    block_id: standard_block.id(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection::at_end_of_youngest_descendant(
                        &youngest_child,
                        block_map,
                    )?)),
                });
            }
            false => {
                let inline_blocks = &standard_block.content_block()?.inline_blocks;
                let last_inline_block_in_block_before =
                    block_map.get_inline_block(&inline_blocks[inline_blocks.len() - 1])?;
                return Ok(SubSelection {
                    block_id: standard_block.id(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: last_inline_block_in_block_before.id(),
                        offset: last_inline_block_in_block_before.text()?.len(),
                        subselection: None,
                    })),
                });
            }
        }
    }
}
