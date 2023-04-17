use crate::{step::TurnInto, blocks::{BlockMap, Block, standard_blocks::StandardBlock}, steps_generator::{StepError, selection::{Selection, SubSelection}}};

use super::UpdatedState;

// This function takes in a
// - TurnInto step,
// - a BlockMap,
// - a vector of Strings representing the blocks to update.
//It returns a Result object with an UpdatedState as the Ok variant,
//or a StepError as the Err variant.

// The function first retrieves the standard block identified by the block_id field of the TurnInto step.
//Then, it uses the update_block_content method of the new_block_type field
//of the TurnInto step to update the content of the retrieved block with the new block type.

// Next, the function updates the block map with the updated block
//and updates the list of blocks to update.
//It then creates a subselection representing the end of the updated block
//and returns an UpdatedState object with
// - the updated block map,
// - the created subselection as the selection,
// - the updated list of blocks to update,
// - an empty list of blocks to remove,
// -  a None value for the copy field.
pub fn actualise_turn_into_step(
    turn_into_step: TurnInto,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let block = block_map.get_standard_block(&turn_into_step.block_id)?;
    let new_block_content = match block.content_block() {
        Ok(content_block) => turn_into_step.new_block_type.update_block_content(content_block.clone())?,
        Err(_) => turn_into_step.new_block_type.clone()
    };
    let block = StandardBlock {
        _id: block._id,
        content: new_block_content,
        children: block.children,
        parent: block.parent,
        marks: block.marks,
    };
    block_map.update_block(Block::StandardBlock(block), &mut blocks_to_update)?;
    let subselection = SubSelection::at_end_of_block(&turn_into_step.block_id, &block_map)?;
    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection { anchor: subselection.clone(), head: subselection }),
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}