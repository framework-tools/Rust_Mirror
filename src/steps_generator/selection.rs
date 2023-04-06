


use crate::{
    blocks::{standard_blocks::StandardBlock, Block, BlockMap, inline_blocks::text_block::StringUTF16},
    step::{ReplaceSlice, ReplaceStep}, frontend_interface::{get_js_field, get_js_field_as_string, get_js_field_as_f64},
};

use super::{StepError};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Selection {
    pub anchor: SubSelection,
    pub head: SubSelection,
}

impl Selection {
    /// Check if both deepest anchor and head are inline blocks
    /// if true start creating selection
    /// else if standard_block for either add an extra layer underneath with the inline block
    /// -------
    /// Make deepest layer of selection the anchor and head
    /// keep adding to the selection in the loop by bubbling up to its parent
    /// until the next parent is the root block or common shared parent for both anchor and head
    /// after this, remove each layer from the top down until the last shared common parent is reached
    pub fn from_frontend_data(
        anchor_id: String,
        head_id: String,
        anchor_offset: usize,
        head_offset: usize,
        block_map: &BlockMap,
        anchor_is_above: bool
    ) -> Result<Self, StepError> {
        let anchor_block = block_map.get_block(&anchor_id);
        let head_block = block_map.get_block(&head_id);

        let mut anchor_subselection: SubSelection = SubSelection::new();
        let mut anchor = get_deepest_subselection(
            anchor_block,
            anchor_offset,
            anchor_is_above,
            &mut anchor_subselection,
            block_map
        )?;

        let mut head_subselection: SubSelection = SubSelection::new();
        let mut head = get_deepest_subselection(
            head_block,
            head_offset,
            anchor_is_above,
            &mut head_subselection,
            block_map
        )?;

        build_up_selection_from_base(
            &mut anchor,
            &mut head,
            &mut anchor_subselection,
            &mut head_subselection,
            block_map
        )?;

        return remove_excess_from_selection(anchor_subselection, head_subselection)
    }

    pub fn from(anchor: SubSelection, head: SubSelection) -> Self {
        Self { anchor, head }
    }

    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let anchor_obj = get_js_field(&JsValue::from(&obj), "anchor")?;
        let head_obj = get_js_field(&JsValue::from(obj), "head")?;
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
                    offset: deepest_from_subselection.offset + StringUTF16::from_str(replace_slice.as_str()).len(),
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

    pub fn to_js_obj(self) -> Result<JsValue, StepError> {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("anchor"), &JsValue::from(self.anchor.to_js_obj()?)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("head"), &JsValue::from(self.head.to_js_obj()?)).unwrap();
        return Ok(obj.into())
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SubSelection {
    pub block_id: String,
    pub offset: usize,
    pub subselection: Option<Box<SubSelection>>,
}

impl SubSelection {
    pub fn new() -> Self {
        return Self {
            block_id: "".to_string(),
            offset: 0,
            subselection: None
        }
    }

    pub fn from(block_id: String, offset: usize, subselection: Option<Box<SubSelection>>) -> Self {
        Self {
            block_id,
            offset,
            subselection,
        }
    }

    pub fn from_js_obj(obj: JsValue) -> Result<Self, StepError> {
        let block_id = get_js_field_as_string(&obj, "block_id")?;
        let offset = get_js_field_as_f64(&obj, "offset")? as usize;
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

    /// Gives the selection based on the raw length of all inline blocks text combined
    /// up to the current deepest subselection
    pub fn to_raw_selection(&self, block_map: &BlockMap) -> Result<Self, StepError> {
        match self.subselection {
            Some(_) => self.get_deepest_subselection().to_raw_selection(block_map),
            None => {
                let inline_block = block_map.get_inline_block(&self.block_id)?;
                let parent = &inline_block.get_parent(block_map)?;
                let inline_blocks = &parent.content_block()?.inline_blocks;
                let mut i = 0;
                let mut raw_offset = 0;
                while i < inline_block.index(block_map)? {
                    let inline_block = block_map.get_inline_block(&inline_blocks[i])?;
                    raw_offset += inline_block.text()?.len();

                    i += 1;
                }
                raw_offset += self.offset;
                return Ok(SubSelection {
                    block_id: parent.id(),
                    offset: raw_offset,
                    subselection: None
                })
            }
        }
    }
    pub fn real_selection_from_raw(self, block_map: &BlockMap) -> Result<Self, StepError> {
        let std_block = &block_map.get_standard_block(&self.block_id)?;
        let inline_blocks = &std_block.content_block()?.inline_blocks;
        let mut current_count = 0;
        for id in inline_blocks {
            let inline_block = block_map.get_inline_block(&id)?;
            if inline_block.text()?.len() + current_count >= self.offset {
                return Ok(Self {
                    block_id: id.clone(),
                    offset: self.offset - current_count,
                    subselection: None
                })
            } else {
                current_count += inline_block.text()?.len();
            }
        }

        return Err(StepError("Raw offset is greater than total text length of this standard block!".to_string()))
    }

    pub fn to_js_obj(self) -> Result<JsValue, StepError> {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("block_id"), &JsValue::from_str(self.block_id.as_str()))
            .map_err(|_| StepError("Failed to set block id on subselection js obj".to_string()))?;
        js_sys::Reflect::set(&obj, &JsValue::from_str("offset"), &JsValue::from_f64(self.offset as f64))
            .map_err(|_| StepError("Failed to set offset on subselection js obj".to_string()))?;
        let js_subselection = match self.subselection {
            Some(subselection) => subselection.to_js_obj()?,
            None => JsValue::null()
        };
        js_sys::Reflect::set(&obj, &JsValue::from_str("subselection"), &js_subselection)
            .map_err(|_| StepError("Failed to set offset on subselection js obj".to_string()))?;
        return Ok(obj.into())
    }

    // pub fn adjust_deepest_subselection_for_marks(&mut self, for_from: bool, block_map: &BlockMap) -> Result<(), StepError> {
    //     match &mut self.subselection {
    //         Some(subselection) => return subselection.adjust_deepest_subselection_for_marks(for_from, block_map),
    //         None if for_from => {
    //             let block = block_map.get_inline_block(&self.block_id)?;
    //             if self.offset == block.text()?.len() {
    //                 let next_block = block.next_block(block_map)?;
    //                 self.block_id = next_block.id();
    //                 self.offset = 0;
    //             }
    //         },
    //         None =>     {
    //             if self.offset == 0 {
    //                 let block = block_map.get_inline_block(&self.block_id)?;
    //                 let previous = block.previous_block(block_map)?;
    //                 self.block_id = previous.id();
    //                 self.offset = previous.text()?.len();
    //             }
    //         }
    //     };
    //     return Ok(())
    // }
}

pub fn get_deepest_subselection(
    block: Result<Block, StepError>,
    offset: usize,
    anchor_is_above: bool,
    subselection: &mut SubSelection,
    block_map: &BlockMap
) -> Result<Block, StepError> {
    match block {
        Ok(Block::InlineBlock(inline_block)) => {
            *subselection = SubSelection {
                block_id: inline_block._id.clone(),
                offset: offset,
                subselection: None,
            };
            return Ok(Block::InlineBlock(inline_block))
        },
        Ok(Block::StandardBlock(standard_block)) => {
            // we want to add a layer above the deepest layer where the selection is at the start of the inline block
            if anchor_is_above {
                let inline = block_map.get_inline_block(&standard_block.content_block()?.inline_blocks[0])?;
                *subselection = SubSelection {
                    block_id: inline.id(),
                    offset: 0,
                    subselection: None,
                };
                return Ok(Block::InlineBlock(inline))
            } else {
                let inline = block_map.get_inline_block(&standard_block.content_block()?.inline_blocks[standard_block.content_block()?.inline_blocks.len() - 1])?;
                *subselection = SubSelection {
                    block_id: inline.id(),
                    offset: inline.text()?.len(),
                    subselection: None,
                };
                return Ok(Block::InlineBlock(inline))
            }
        },
        Err(_) | Ok(Block::Root(_)) => {
            return Err(StepError("Anchor block not found or is root".to_string()))
        }
    }
}

pub fn build_up_selection_from_base(
    anchor: &mut Block,
    head: &mut Block,
    anchor_subselection: &mut SubSelection,
    head_subselection: &mut SubSelection,
    block_map: &BlockMap,
) -> Result<(), StepError> {
    loop {
        if !anchor.is_root() {
            *anchor = block_map.get_block(&anchor.parent()?)?;
        }
        if !head.is_root() {
            *head = block_map.get_block(&head.parent()?)?;
        }
        if !anchor.is_root() {
            *anchor_subselection = SubSelection {
                block_id: anchor.id().to_string(),
                offset: 0,
                subselection: Some(Box::new(anchor_subselection.clone())),
            };
        }
        if !head.is_root() {
            *head_subselection = SubSelection {
                block_id: head.id().to_string(),
                offset: 0,
                subselection: Some(Box::new(head_subselection.clone())),
            };
        }

        if anchor.is_root() && head.is_root() {
            return Ok(())
        }
    }
}

pub fn remove_excess_from_selection(
    mut anchor: SubSelection,
    mut head: SubSelection,
) -> Result<Selection, StepError> {
    if anchor.subselection.is_none() && head.subselection.is_none() {
        return Ok(Selection { anchor, head })
    }

    while anchor.block_id == head.block_id {
        match &anchor.subselection {
            Some(subselection) => match &subselection.subselection {
                Some(subselection) => anchor = *subselection.clone(),
                None => return Ok(Selection { anchor, head })
            },
            None => return Ok(Selection { anchor, head })
        };
        match &head.subselection {
            Some(subselection) => match &subselection.subselection {
                Some(subselection) => head = *subselection.clone(),
                None => return Ok(Selection { anchor, head })
            },
            None => return Ok(Selection { anchor, head })
        };
    }

    return Ok(Selection { anchor, head })
}