use std::{str::FromStr};

use crate::{blocks::{Block, BlockMap}, step::{ReplaceStep, ReplaceSlice}};

use super::StepError;
use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
	}

    pub fn update_selection_from(replace_step: ReplaceStep) -> Self {
        match replace_step.slice {
            ReplaceSlice::String(replace_slice) => {
                let deepest_from_subselection = replace_step.from.get_deepest_subselection();
                let subselection = SubSelection {
                    block_id: deepest_from_subselection.block_id,
                    offset: deepest_from_subselection.offset + replace_slice.len(),
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

    pub fn get_child_subselection(&self) -> Result<&SubSelection, StepError> {
        return match &self.subselection {
            Some(inner_subselection) => Ok(&*inner_subselection),
            None => return Err(StepError("Expected subselection to be Some".to_string()))
        }
    }

    pub fn get_deepest_subselection(self) -> Self {
        match self.subselection {
            Some(subselection) => subselection.get_deepest_subselection(),
            None => self,
        }
    }
}
