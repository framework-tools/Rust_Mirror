use std::{str::FromStr};



use crate::{blocks::{Block, BlockMap}, step::{ReplaceStep, ReplaceSlice}};

use super::StepError;
use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Selection {
	pub anchor: SubSelection,
	pub head: SubSelection
}

impl Selection {
	pub fn from(anchor: SubSelection, head: SubSelection) -> Self {
		Self {
			anchor,
			head
		}
	}

	pub fn get_from_to(self, block_map: &BlockMap) -> Result<(SubSelection, SubSelection), StepError> {
		match self.anchor.block_id == self.head.block_id {
			true => {
				match self.anchor.offset < self.head.offset {
					true => Ok((self.anchor, self.head)),
					false => Ok((self.head, self.anchor))
				}
			},
			false => {
				let anchor_block = block_map.get_block(&self.anchor.block_id)?;
				let head_block = block_map.get_block(&self.head.block_id)?;
				match anchor_block {
					Block::InlineBlock(anchor_block) => {
						match head_block {
							Block::InlineBlock(head_block) => {
								match anchor_block.parent == head_block.parent {
									true => {
										let parent = block_map.get_block(&anchor_block.parent)?;
										match parent {
											Block::StandardBlock(parent) => {
												let content_block = parent.content_block()?;
												let anchor_index = content_block.index_of(&self.anchor.block_id)?;
												let head_index = content_block.index_of(&self.head.block_id)?;
												match anchor_index < head_index {
													true => Ok((self.anchor, self.head)),
													false => Ok((self.head, self.anchor))
												}
											},
											_ => Err(StepError("Parent block is not a StandardBlock".to_string()))
										}
									},
									false => unimplemented!()
								}
							},
							_ => unimplemented!()
						}
					},
					Block::StandardBlock(std_block) => {
                        // which block comes first in their parent
                        let parent = block_map.get_block(&std_block.parent)?;
                        match &parent {
                            Block::StandardBlock(parent) => {
                                let anchor_index = parent.index_of_child(&self.anchor.block_id)?;
                                let head_index = parent.index_of_child(&self.head.block_id)?;
                                match anchor_index < head_index {
                                    true => Ok((self.anchor, self.head)),
                                    false => Ok((self.head, self.anchor))
                                }
                            },
                            Block::Root(parent) => {
                                let anchor_index = parent.index_of_child(&self.anchor.block_id)?;
                                let head_index = parent.index_of_child(&self.head.block_id)?;
                                match anchor_index < head_index {
                                    true => Ok((self.anchor, self.head)),
                                    false => Ok((self.head, self.anchor))
                                }
                            },
                            Block::InlineBlock(_) => Err(StepError("Parent block is an inline block which should never exist".to_string()))
                        }
                    },
					Block::Root(_) => return Err(StepError("Cannot get selection from root block".to_string()))
				}
			}
		}
	}

    pub fn update_selection_from(replace_step: ReplaceStep) -> Self {
        match replace_step.slice {
            ReplaceSlice::String(replace_slice) => {
                let subselection = SubSelection {
                    block_id: replace_step.from.block_id,
                    offset: replace_step.from.offset + replace_slice.len(),
                    subselection: None
                };
                return Selection { anchor: subselection.clone(), head: subselection }
            },
            ReplaceSlice::Blocks(blocks) => unimplemented!()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SubSelection {
	pub block_id: String,
	pub offset: usize,
	pub subselection: Option<Box<SubSelection>>
}

impl SubSelection {
	pub fn from(block_id: String, offset: usize, subselection: Option<Box<SubSelection>>) -> Self {
		Self {
			block_id,
			offset,
			subselection
		}
	}

    pub fn block_id(&self) -> String {
        return self.block_id.clone()
    }

    /// If selection is inline block & offset is 0, & is not the first inline block in parent block
    /// -> change selection to the previous inline block, & set the offset is at the end of its text
    pub fn conform_illegal_start_offset_selection(self, block_map: &BlockMap) -> Result<Self, StepError> {
        match self.subselection {
            Some(subselection) => subselection.conform_illegal_start_offset_selection(block_map),
            None => {
                match block_map.get_inline_block(&self.block_id) {
                    Ok(inline_block) => {
                        if self.offset == 0 {
                            let parent_block = block_map.get_standard_block(&inline_block.parent)?;
                            let block_index = parent_block.index_of(&inline_block._id)?;
                            if block_index != 0 {
                                let previous_block_id = parent_block.get_inline_block_from_index(block_index - 1)?;
                                let previous_block = block_map.get_inline_block(&previous_block_id)?;
                                return Ok(Self {
                                    block_id: previous_block.id(),
                                    offset: previous_block.text()?.len(),
                                    subselection: None,
                                })
                            } else {
                                return Ok(self)
                            }
                        } else {
                            return Ok(self)
                        }
                    },
                    Err(_) => return Ok(self)
                }
            }
        }
    }
}

