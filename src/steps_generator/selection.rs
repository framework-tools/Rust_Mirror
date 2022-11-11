use std::{str::FromStr};



use crate::{blocks::{Block, BlockMap}, step::{ReplaceStep, ReplaceSlice}};

use super::StepError;
use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Selection {
	pub from: SubSelection,
	pub to: SubSelection
}

impl Selection {
	pub fn from(from: SubSelection, to: SubSelection) -> Self {
		Self {
			from,
			to
		}
	}

	pub fn get_from_to(self) -> Result<(SubSelection, SubSelection), StepError> {
        return Ok((self.from, self.to))
		// match self.from.block_id == self.to.block_id {
		// 	true => {
		// 		match self.from.offset < self.to.offset {
		// 			true => Ok((self.from, self.to)),
		// 			false => Ok((self.to, self.from))
		// 		}
		// 	},
		// 	false => {
		// 		let from_block = block_map.get_block(&self.from.block_id)?;
		// 		let to_block = block_map.get_block(&self.to.block_id)?;
		// 		match from_block {
		// 			Block::InlineBlock(from_block) => {
		// 				match to_block {
		// 					Block::InlineBlock(to_block) => {
		// 						match from_block.parent == to_block.parent {
		// 							true => {
		// 								let parent = block_map.get_block(&from_block.parent)?;
		// 								match parent {
		// 									Block::StandardBlock(parent) => {
		// 										let content_block = parent.content_block()?;
		// 										let from_index = content_block.index_of(&self.from.block_id)?;
		// 										let to_index = content_block.index_of(&self.to.block_id)?;
		// 										match from_index < to_index {
		// 											true => Ok((self.from, self.to)),
		// 											false => Ok((self.to, self.from))
		// 										}
		// 									},
		// 									_ => Err(StepError("Parent block is not a StandardBlock".to_string()))
		// 								}
		// 							},
		// 							false => unimplemented!()
		// 						}
		// 					},
		// 					_ => unimplemented!()
		// 				}
		// 			},
		// 			Block::StandardBlock(std_block) => {
        //                 // which block comes first in their parent
        //                 let parent = block_map.get_block(&std_block.parent)?;
        //                 match &parent {
        //                     Block::StandardBlock(parent) => {
        //                         let from_index = parent.index_of_child(&self.from.block_id)?;
        //                         let to_index = parent.index_of_child(&self.to.block_id)?;
        //                         match from_index < to_index {
        //                             true => Ok((self.from, self.to)),
        //                             false => Ok((self.to, self.from))
        //                         }
        //                     },
        //                     Block::Root(parent) => {
        //                         let from_index = parent.index_of_child(&self.from.block_id)?;
        //                         let to_index = parent.index_of_child(&self.to.block_id)?;
        //                         match from_index < to_index {
        //                             true => Ok((self.from, self.to)),
        //                             false => Ok((self.to, self.from))
        //                         }
        //                     },
        //                     Block::InlineBlock(_) => Err(StepError("Parent block is an inline block which should never exist".to_string()))
        //                 }
        //             },
		// 			Block::Root(_) => return Err(StepError("Cannot get selection from root block".to_string()))
		// 		}
		// 	}
		// }
	}

    pub fn update_selection_from(replace_step: ReplaceStep) -> Self {
        match replace_step.slice {
            ReplaceSlice::String(replace_slice) => {
                let subselection = SubSelection {
                    block_id: replace_step.from.block_id,
                    offset: replace_step.from.offset + replace_slice.len(),
                    subselection: None
                };
                return Selection { from: subselection.clone(), to: subselection }
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
}

