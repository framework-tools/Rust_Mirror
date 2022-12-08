use crate::{blocks::{BlockMap, standard_blocks::StandardBlockType, Block}, steps_generator::StepError};

use super::UpdatedState;

pub fn actualise_toggle_completed(_id: String, mut block_map: BlockMap) -> Result<UpdatedState, StepError> {
    let mut block = block_map.get_standard_block(&_id)?;
    return match block.content {
        StandardBlockType::TodoList(mut list_block) => {
            list_block.completed = !list_block.completed;
            block.content = StandardBlockType::TodoList(list_block);
            block_map.update_block(Block::StandardBlock(block))?;
            Ok(UpdatedState { block_map, selection: None })
        },
        t => Err(StepError(format!("Cannot toggle completed on any block other than a to-do list. Got block: {:#?}", t)))
    }
}