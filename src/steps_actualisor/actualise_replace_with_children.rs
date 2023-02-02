use crate::{steps_generator::{StepError, event::ReplaceWithChildrenEvent}, blocks::BlockMap, utilities::update_state_tools};

use super::UpdatedState;

pub fn actualise_replace_with_children(
    replace_with_children_event: ReplaceWithChildrenEvent,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>,
) -> Result<UpdatedState, StepError> {
    // let block = block_map.get_standard_block(&replace_with_children_event.block_id)?;
    // let parent = block.get_parent(&block_map)?;
    // let block_index = block.index(&block_map)?;

    // update_state_tools::splice_children(
    //     parent,
    //     block_index..block_index + 1,
    //     block.children,
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