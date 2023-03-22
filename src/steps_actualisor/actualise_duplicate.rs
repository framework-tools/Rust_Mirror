use crate::{blocks::BlockMap, steps_generator::StepError, utilities::{update_state_tools, get_all_blocks, reassign_ids}, new_ids::NewIds};

use super::UpdatedState;


pub fn actualise_duplicate(
    block_id: String,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {
    // let block = block_map.get_standard_block(&block_id)?;
    // let parent = block.get_parent(&block_map)?;
    // let original_block_index = block.index(&block_map)?;
    // let mut top_block = vec![block];
    // let block_and_all_descendants = get_all_blocks(&top_block, &block_map)?;
    // reassign_ids(block_and_all_descendants, &mut top_block, &mut block_map, new_ids, &mut blocks_to_update)?;

    // update_state_tools::splice_children(
    //     parent,
    //     original_block_index+1..original_block_index+1,
    //     vec![top_block[0].id()],
    //     &mut blocks_to_update,
    //     &mut block_map
    // )?;

    return Ok(UpdatedState {
        block_map,
        selection: None,
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}