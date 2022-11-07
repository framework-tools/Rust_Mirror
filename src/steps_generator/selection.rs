use std::{collections::HashMap, str::FromStr};

use mongodb::bson::oid::ObjectId;

use crate::blocks::{Block, BlockMap};

use super::StepError;



#[derive(Debug, PartialEq)]
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

    pub fn from_json(json: serde_json::Value) -> Result<Self, StepError> {
        let anchor = json.get("anchor")
            .ok_or(StepError("Selection json should contain 'anchor' field".to_string()))?;
        let anchor = SubSelection::from_json(anchor)?;
        let head = json.get("head")
            .ok_or(StepError("Selection json should contain 'head' field".to_string()))?;
        let head = SubSelection::from_json(head)?;
        return Ok(Self {
            anchor,
            head,
        })
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
												let anchor_index = content_block.index_of(self.anchor.block_id)?;
												let head_index = content_block.index_of(self.head.block_id)?;
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
                                let anchor_index = parent.index_of_child(self.anchor.block_id)?;
                                let head_index = parent.index_of_child(self.head.block_id)?;
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct SubSelection {
	pub block_id: ObjectId,
	pub offset: usize,
	pub subselection: Option<Box<SubSelection>>
}

impl SubSelection {
	pub fn from(block_id: ObjectId, offset: usize, subselection: Option<Box<SubSelection>>) -> Self {
		Self {
			block_id,
			offset,
			subselection
		}
	}

    pub fn from_json(json: &serde_json::Value) -> Result<Self, StepError> {
        let block_id = json.get("block_id")
            .ok_or(StepError("Subselection json should contain 'block id' field".to_string()))?
            .as_str().ok_or(StepError("Block id value should be able to be passed as str".to_string()))?;
        let block_id = ObjectId::from_str(block_id);
        let block_id = match block_id {
            Ok(block_id) => block_id,
            Err(_) => return Err(StepError("Block id should be an object id".to_string()))
        };
        let offset = json.get("offset")
            .ok_or(StepError("Subselection json should contain 'offset' field".to_string()))?
            .as_u64().ok_or(StepError("Offset value should be able to be passed as u64".to_string()))? as usize;

        let subselection = json.get("subselection")
            .ok_or(StepError("Subselection json should contain 'subselection' field".to_string()))?;
        match subselection.as_null() {
            Some(_) => return Ok(Self {
                block_id,
                offset,
                subselection: None,
            }),
            None => return Ok(Self {
                block_id,
                offset,
                subselection: Some(Box::new(Self::from_json(subselection)?)),
            })
        }
    }
}

