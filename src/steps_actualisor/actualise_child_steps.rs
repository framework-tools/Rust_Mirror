use crate::{steps_generator::StepError, blocks::{BlockMap, Block}, step::TurnToChild};

use super::UpdatedState;


/// remove itself as a child from it's previous parent
/// Block above -> parent of the block we're turning to a child
/// set block's parent to block above
/// -> update all these blocks in the block map
// ------------------------------------------------

// It looks like this function is intended to actualize a TurnToChild step, 
//which involves changing the parent-child relationship of a block. 
//The function first retrieves the new_child_block, 
//which is the block that will become a child, 
//and then retrieves its current parent block. 
//It removes the new_child_block from the parent's list of children, 
//and then retrieves the block immediately above the new_child_block. 
//If this block exists, the function sets the new_child_block as its child 
//and updates the blocks in the block map. 
//If the block above the new_child_block does not exist, 
//the function returns an error indicating that it is not possible to turn the block into a child 
//if it is the first block in the document. 
//Finally, the function returns an UpdatedState object with the updated block map 
//and an empty list of blocks to update and remove.
pub fn actualise_child_steps(
    mut block_map: BlockMap,
    turn_to_child_step: TurnToChild,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
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
            ], &mut blocks_to_update)?;
        },
        None => return Err(StepError("Should not be able to turn into child if first index".to_string()))
    }
    return Ok(UpdatedState {
        block_map,
        selection: None,
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}