use crate::{steps_generator::{StepError, event::{DropBlockEvent, Side}}, blocks::{BlockMap, Block}, new_ids::NewIds, utilities::update_state_tools};

use super::UpdatedState;

/// -> Remove drag block from current place
/// -> If top or bottom -> add to parent at insertion point & change drag block's parent to new parent
pub fn actualise_drop_block(
    drop_block_event: DropBlockEvent,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let mut drag_block = block_map.get_standard_block(&drop_block_event.drag_block_id)?;
    let mut drag_parent = drag_block.get_parent(&block_map)?;
    drag_parent = drag_parent.remove_child_from_id(&drag_block._id)?;
    block_map.update_block(drag_parent, &mut blocks_to_update)?;

    let drop_block = block_map.get_standard_block(&drop_block_event.drop_block_id)?;
    let drop_parent = drop_block.get_parent(&block_map)?;
    let drop_parent_id = drop_parent.id();
    match drop_block_event.side_dropped {
        Side::Top | Side::Bottom => {
            let mut insertion_index = drop_block.index(&block_map)?;
            if drop_block_event.side_dropped == Side::Bottom {
                insertion_index += 1;
            }
            update_state_tools::splice_children(
                drop_parent,
                insertion_index..insertion_index,
                vec![drag_block.id()],
                &mut blocks_to_update,
                &mut block_map
            )?;
            drag_block.parent = drop_parent_id;
            block_map.update_block(Block::StandardBlock(drag_block), &mut blocks_to_update)?;
        },
        Side::Left => unimplemented!(),
        Side::Right => unimplemented!()
    };

    return Ok(UpdatedState {
        block_map,
        selection: None,
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}