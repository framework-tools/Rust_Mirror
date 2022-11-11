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

