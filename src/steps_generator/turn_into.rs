use crate::{blocks::{standard_blocks::{StandardBlockType, content_block::ContentBlock}, BlockMap}, step::{Step, TurnInto}};

use super::{selection::SubSelection, StepError};



pub fn generate_turn_into_step(new_block_type: &StandardBlockType, from: SubSelection, block_map: &BlockMap) -> Result<Vec<Step>, StepError> {
    let inline_block = block_map.get_inline_block(&from.block_id)?;
    return Ok(vec![Step::TurnInto(TurnInto { block_id: inline_block.parent, new_block_type: new_block_type.clone() })])
}

pub fn turn_into_paragraph_step(block_id: String) -> Result<Vec<Step>, StepError> {
    return Ok(vec![Step::TurnInto(TurnInto {
        block_id,
        new_block_type: StandardBlockType::Paragraph(ContentBlock::new(vec![]))
    })])
}