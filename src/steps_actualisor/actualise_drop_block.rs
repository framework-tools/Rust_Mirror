use crate::{steps_generator::{StepError, event::{DropBlockEvent, Side}},
blocks::{BlockMap, Block, standard_blocks::{StandardBlock, StandardBlockType, layout_block::LayoutBlock}},
new_ids::NewIds, utilities::update_state_tools};

use super::UpdatedState;

/// -> Remove drag block from current place
/// -> If top or bottom -> add to parent at insertion point & change drag block's parent to new parent
pub fn actualise_drop_block(
    drop_block_event: DropBlockEvent,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {
    let mut drag_block = block_map.get_standard_block(&drop_block_event.drag_block_id)?;
    if drag_block.get_parent(&block_map).is_ok() {
        remove_drag_block_from_current_place(&mut block_map, &drop_block_event, &mut blocks_to_update)?;
    }

    let drop_block = block_map.get_standard_block(&drop_block_event.drop_block_id)?;
    let drop_parent = drop_block.get_parent(&block_map)?;

    match drop_block_event.side_dropped {
        Side::Top | Side::Bottom => {
            drop_block_top_or_bottom(drag_block, drop_block, drop_parent, drop_block_event, &mut block_map, &mut blocks_to_update)?;
        },
        Side::Left | Side::Right => {
            if is_layout_block_or_is_inside_layout_block(&drop_block, &drop_parent) {
                let new_column_id = new_ids.get_id()?;
                let horizontal_layout_id = get_horizontal_layout_id(&drop_block, &block_map)?;
                let new_column_layout = StandardBlock::new_layout_block(
                    new_column_id.clone(),
                    false,
                    vec![drag_block.id()],
                    horizontal_layout_id.clone()
                )?;
                drag_block.parent = new_column_id.clone();
                block_map.update_blocks(vec![
                    Block::StandardBlock(drag_block), Block::StandardBlock(new_column_layout)
                ], &mut blocks_to_update)?;

                let new_column_index = get_index_of_new_layout_column(&drop_block, &drop_block_event.side_dropped, &block_map)?;
                let horizontal_layout_block = block_map.get_block(&horizontal_layout_id)?;
                update_state_tools::splice_children(
                    horizontal_layout_block,
                    new_column_index..new_column_index,
                    vec![new_column_id],
                    &mut blocks_to_update,
                    &mut block_map
                )?;
            } else {
                create_new_horizontal_layout_block(
                    drag_block,
                    drop_block,
                    drop_parent,
                    drop_block_event,
                    &mut block_map,
                    &mut blocks_to_update,
                    new_ids
                )?;
            }
        },
    };

    return Ok(UpdatedState {
        block_map,
        selection: None,
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}

fn remove_drag_block_from_current_place(
    block_map: &mut BlockMap,
    drop_block_event: &DropBlockEvent,
    blocks_to_update: &mut Vec<String>
) -> Result<(), StepError> {
    let drag_block = block_map.get_standard_block(&drop_block_event.drag_block_id)?;
    let mut drag_parent = drag_block.get_parent(&block_map)?;
    drag_parent = drag_parent.remove_child_from_id(&drag_block._id)?;
    block_map.update_block(drag_parent, blocks_to_update)?;
    return Ok(())
}

fn is_layout_block_or_is_inside_layout_block(
    drop_block: &StandardBlock,
    drop_parent: &Block
) -> bool {
    return match drop_block.content {
        StandardBlockType::Layout(_) => true,
        _ => match drop_parent{
            Block::StandardBlock(parent) => {
                match &parent.content {
                    StandardBlockType::Layout(_) => true,
                    _ => false
                }
            }
            _ => return false
        }
    }
}

fn drop_block_top_or_bottom(
    drag_block: StandardBlock,
    drop_block: StandardBlock,
    drop_parent: Block,
    drop_block_event: DropBlockEvent,
    block_map: &mut BlockMap,
    blocks_to_update: &mut Vec<String>
) -> Result<(), StepError> {
    // add dragged block to new position
    let mut insertion_index = drop_block.index(&block_map)?;
    if drop_block_event.side_dropped == Side::Bottom {
        insertion_index += 1;
    }
    update_state_tools::splice_children(
        drop_parent,
        insertion_index..insertion_index,
        vec![drag_block.id()],
        blocks_to_update,
        block_map
    )?;

    return Ok(())
}

fn create_new_horizontal_layout_block(
    drag_block: StandardBlock,
    drop_block: StandardBlock,
    drop_parent: Block,
    drop_block_event: DropBlockEvent,
    block_map: &mut BlockMap,
    blocks_to_update: &mut Vec<String>,
    new_ids: &mut NewIds
) -> Result<(), StepError> {
    let drop_block_old_index = drop_block.index(&block_map)?;

    let new_horizontal_layout_id = new_ids.get_id()?;
    let column_id1 = new_ids.get_id()?;
    let column_id2 = new_ids.get_id()?;
    let horizontal_layout_children = match drop_block_event.side_dropped {
        Side::Left => {
            create_horizontal_layout_children(
                new_horizontal_layout_id.clone(),
                drag_block,
                drop_block,
                column_id1,
                column_id2,
                block_map,
                blocks_to_update
            )?
        },
        Side::Right => {
            create_horizontal_layout_children(
                new_horizontal_layout_id.clone(),
                drop_block,
                drag_block,
                column_id1,
                column_id2,
                block_map,
                blocks_to_update
            )?
        },
        _ => unreachable!()
    };
    let new_horizontal_layout_block = StandardBlock {
        _id: new_horizontal_layout_id,
        content: StandardBlockType::Layout(LayoutBlock { horizontal: true }),
        children: horizontal_layout_children,
        parent: drop_parent.id(),
        marks: vec![]
    };
    let new_horizontal_layout_block_id = new_horizontal_layout_block.id();
    block_map.update_block( Block::StandardBlock(new_horizontal_layout_block), blocks_to_update)?;
    // replace drop block with new layout block
    update_state_tools::splice_children(
        drop_parent,
        drop_block_old_index..drop_block_old_index + 1,
        vec![new_horizontal_layout_block_id.clone()],
        blocks_to_update,
        block_map
    )?;

    return Ok(())
}

fn create_horizontal_layout_children(
    horizontal_layout_id: String,
    mut left_block: StandardBlock,
    mut right_block: StandardBlock,
    left_column_id: String,
    right_column_id: String,
    block_map: &mut BlockMap,
    blocks_to_update: &mut Vec<String>
) -> Result<Vec<String>, StepError> {
    left_block.parent = left_column_id.clone();
    right_block.parent = right_column_id.clone();
    let left_column = StandardBlock::new_layout_block(left_column_id.clone(), false, vec![left_block.id()], horizontal_layout_id.clone())?;
    let right_column = StandardBlock::new_layout_block(right_column_id.clone(), false, vec![right_block.id()], horizontal_layout_id.clone())?;
    block_map.update_blocks(vec![
        Block::StandardBlock(left_block),
        Block::StandardBlock(right_block),
        Block::StandardBlock(left_column),
        Block::StandardBlock(right_column),
    ], blocks_to_update)?;
    return Ok(vec![left_column_id, right_column_id])
}

fn get_horizontal_layout_id(drop_block: &StandardBlock, block_map: &BlockMap) -> Result<String, StepError> {
    match &drop_block.content {
        StandardBlockType::Layout(LayoutBlock { horizontal: true }) => return Ok(drop_block.id()),
        StandardBlockType::Layout(LayoutBlock { horizontal: false }) => return Ok(drop_block.parent.clone()),
        _ => {
            let parent = block_map.get_standard_block(&drop_block.parent)?;
            return Ok(parent.parent.clone())
        }
    }
}

/// 3 cases: horizontal layout, vertical layout (column), not a layout block (inside a layout block)
fn get_index_of_new_layout_column(drop_block: &StandardBlock, side_dropped: &Side, block_map: &BlockMap) -> Result<usize, StepError> {
    return match &drop_block.content {
        StandardBlockType::Layout(LayoutBlock { horizontal: true }) => {
            if *side_dropped == Side::Left {
                Ok(0)
            } else {
                Ok(drop_block.children.len())
            }
        },
        StandardBlockType::Layout(LayoutBlock { horizontal: false }) => {
            if *side_dropped == Side::Left {
                drop_block.index(&block_map)
            } else {
                Ok(drop_block.index(&block_map)? + 1)
            }
        },
        _ => {
            let parent = block_map.get_standard_block(&drop_block.parent)?;
            if *side_dropped == Side::Left {
                parent.index(&block_map)
            } else {
                Ok(parent.index(&block_map)? + 1)
            }
        }
    }
}