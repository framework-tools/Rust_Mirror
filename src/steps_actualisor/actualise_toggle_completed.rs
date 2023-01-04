use crate::{blocks::{BlockMap, standard_blocks::StandardBlockType, Block}, steps_generator::StepError};

use super::UpdatedState;

// This function appears to toggle the completed field of a TodoList block in a BlockMap. 
//It returns an UpdatedState object with the 
// - updated block map, 
// - an empty selection, 
// - list of updated blocks, 
// - list of removed blocks, 
// - no copied block. 
//If the block is not a TodoList block, 
//it returns an error with a message indicating that the block type is not supported.
pub fn actualise_toggle_completed(
    _id: String,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let mut block = block_map.get_standard_block(&_id)?;
    return match block.content {
        StandardBlockType::TodoList(mut list_block) => {
            list_block.completed = !list_block.completed;
            block.content = StandardBlockType::TodoList(list_block);
            block_map.update_block(Block::StandardBlock(block), &mut blocks_to_update)?;
            Ok(UpdatedState {
                block_map,
                selection: None,
                blocks_to_update,
                blocks_to_remove: vec![],
                copy: None
            })
        },
        t => Err(StepError(format!("Cannot toggle completed on any block other than a to-do list. Got block: {:#?}", t)))
    }
}