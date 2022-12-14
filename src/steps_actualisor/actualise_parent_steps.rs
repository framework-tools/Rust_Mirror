use crate::{blocks::{BlockMap, Block}, step::TurnToParent, steps_generator::StepError};

use super::UpdatedState;

/// -> Remove itself from it's current parent
/// -> split children in half -> push all it's previous siblings that came after it as to it's children
///
/// -> new parent is it's previous parent's parent
/// -> should be inserted 1 below previous parent as sibling
// ------------------------------------------------

//This function appears to be responsible for handling the TurnToParent operation, which involves taking a block and making it the sibling of its parent block. It does this by updating the parent of the current block, as well as updating the children of the parent block and the previous grandparent block.

// It does this by first getting a reference to the current block 
//and its previous parent block from the block_map, 
//using the block_id provided in the turn_to_parent_step parameter. 
//It then gets the index of the current block within the parent's children list. 
//It separates the children list of the parent into two lists, 
//one containing the children before the current block and one containing the children after it.

// The children after the current block are added to the current block's children list, 
//and the parent field of each of those blocks is updated to be the current block's ID. 
//The children before the current block are kept as the children of the previous parent.

// The previous grandparent block is then retrieved from the block_map, 
//and the current block is inserted into its children list after the previous parent block. 
//The parent field of the current block is updated to be the ID of the previous grandparent block. 
//Finally, all of the modified blocks are updated in the block_map and an UpdatedState object is returned.
pub fn actualise_parent_steps(
    mut block_map: BlockMap,
    turn_to_parent_step: TurnToParent,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let mut current_block = block_map.get_standard_block(&turn_to_parent_step.block_id)?;

    let mut previous_parent = block_map.get_standard_block(&current_block.parent)?;
    let current_block_index = current_block.index(&block_map)?;
    let first_half = &previous_parent.children[..current_block_index];
    let second_half = &previous_parent.children[current_block_index + 1..];
    current_block.children = vec![current_block.children, second_half.to_vec()].concat();
    for id in second_half {
        let mut block = block_map.get_standard_block(id)?;
        block.parent = current_block.id();
        block_map.update_block(Block::StandardBlock(block), &mut blocks_to_update)?;
    }
    previous_parent.children = first_half.to_vec();

    let mut previous_grandparent = block_map.get_block(&previous_parent.parent())?;
    current_block.parent = previous_grandparent.id();
    previous_grandparent.insert_child(current_block.id(), previous_parent.index(&block_map)? + 1)?;
    block_map.update_blocks(vec![
        Block::StandardBlock(current_block), Block::StandardBlock(previous_parent), previous_grandparent
    ], &mut blocks_to_update)?;
    return Ok(UpdatedState { block_map, selection: None, blocks_to_update, blocks_to_remove: vec![], copy: None })
}