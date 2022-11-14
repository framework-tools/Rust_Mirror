use crate::{steps_generator::StepError, blocks::{BlockMap, Block}, step::TurnToChild};

use super::UpdatedState;


/// remove itself as a child from it's previous parent
/// Block above -> parent of the block we're turning to a child
/// set block's parent to block above
/// -> update all these blocks in the block map
pub fn execute_child_steps(mut block_map: BlockMap, turn_to_child_step: TurnToChild) -> Result<UpdatedState, StepError> {
    let mut new_child_block = block_map.get_standard_block(&turn_to_child_step.block_id)?;

    let mut parent = new_child_block.get_parent(&block_map)?;
    parent = parent.remove_child_from_id(&new_child_block._id)?;

    let block_above = new_child_block.get_previous(&block_map)?;
    match block_above {
        Some(mut block_above) => {
            block_above.children.push(new_child_block.id());
            new_child_block.parent = block_above.id();
            block_map.update_blocks(vec![
                Block::StandardBlock(new_child_block), Block::StandardBlock(block_above), parent
            ])?;
        },
        None => return Err(StepError("Should not be able to turn into child if first index".to_string()))
    }
    return Ok(UpdatedState { block_map, selection: None })
}