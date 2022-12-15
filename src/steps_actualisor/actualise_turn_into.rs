use crate::{step::TurnInto, blocks::{BlockMap, Block, standard_blocks::StandardBlock}, steps_generator::{StepError, selection::{Selection, SubSelection}}};

use super::UpdatedState;


pub fn actualise_turn_into_step(
    turn_into_step: TurnInto,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let block = block_map.get_standard_block(&turn_into_step.block_id)?;
    let new_block_content = turn_into_step.new_block_type.update_block_content(block.content_block()?.clone())?;
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