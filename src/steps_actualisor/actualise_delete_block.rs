use crate::{blocks::BlockMap, steps_generator::StepError, utilities::update_state_tools};

use super::UpdatedState;


pub fn actualise_delete_block(
    block_id: String,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {

    if !block_map.only_one_std_block(&block_id)? {
        let block = block_map.get_standard_block(&block_id)?;
        let parent = block.get_parent(&block_map)?;

        let block_index = block.index(&block_map)?;
        update_state_tools::splice_children(
            parent,
            block_index..block_index+1,
            vec![],
            &mut blocks_to_update,
            &mut block_map
        )?;
    }

    return Ok(UpdatedState {
        block_map,
        selection: None,
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}