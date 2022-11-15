use crate::{blocks::{BlockMap, Block}, step::TurnToParent, steps_generator::StepError};

use super::UpdatedState;

/// -> Remove itself from it's current parent
/// -> split children in half -> set all it's previous siblings that came after it as it's new children
///
/// -> new parent is it's previous parent's parent
/// -> should be inserted 1 below previous parent as sibling
pub fn execute_parent_steps(mut block_map: BlockMap, turn_to_parent_step: TurnToParent) -> Result<UpdatedState, StepError> {
    let mut current_block = block_map.get_standard_block(&turn_to_parent_step.block_id)?;

    let mut previous_parent = block_map.get_standard_block(&current_block.parent)?;
    let current_block_index = current_block.index(&block_map)?;
    let first_half = &previous_parent.children[..current_block_index];
    let second_half = &previous_parent.children[current_block_index + 1..];
    current_block.children = second_half.to_vec();
    previous_parent.children = first_half.to_vec();

    let mut previous_grandparent = block_map.get_block(&previous_parent.parent())?;
    current_block.parent = previous_grandparent.id();
    previous_grandparent.insert_child(current_block.id(), previous_parent.index(&block_map)? + 1)?;
    block_map.update_blocks(vec![
        Block::StandardBlock(current_block), Block::StandardBlock(previous_parent), previous_grandparent
    ])?;
    return Ok(UpdatedState { block_map, selection: None })
}