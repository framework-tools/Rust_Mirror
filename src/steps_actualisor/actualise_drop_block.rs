use crate::{steps_generator::{StepError, event::{DropBlockEvent, Side}}, blocks::{BlockMap, Block, standard_blocks::{StandardBlock, StandardBlockType, layout_block::LayoutBlock}}, new_ids::NewIds, utilities::update_state_tools, step::Step};

use super::UpdatedState;

/// -> Remove drag block from current place
/// -> If top or bottom -> add to parent at insertion point & change drag block's parent to new parent
pub fn actualise_drop_block(
    drop_block_event: DropBlockEvent,
    mut block_map: BlockMap,
    mut blocks_to_update: Vec<String>,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {
    // remove drag block from current place
    let mut drag_block = block_map.get_standard_block(&drop_block_event.drag_block_id)?;
    let mut drag_parent = drag_block.get_parent(&block_map)?;
    drag_parent = drag_parent.remove_child_from_id(&drag_block._id)?;
    block_map.update_block(drag_parent, &mut blocks_to_update)?;

    let mut drop_block = block_map.get_standard_block(&drop_block_event.drop_block_id)?;
    let mut drop_parent = drop_block.get_parent(&block_map)?;
    let drop_parent_id = drop_parent.id();
    match drop_block_event.side_dropped {
        Side::Top | Side::Bottom => {
            // add dragged block to new position 
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
        Side::Left | Side::Right => {
            if is_layout_block_or_is_inside_layout_block(&drop_block, &drop_parent)? {
                add_drag_block_to_layout_block(
                    &mut drop_block, 
                    &mut drop_parent, 
                    &drop_block_event.side_dropped, 
                    &drag_block._id, 
                    &block_map
                )?;
                
                drag_block.parent = drop_parent_id;
                block_map.update_blocks(vec![
                    Block::StandardBlock(drag_block), 
                    drop_parent
                ], &mut blocks_to_update)?;
            } else {
                // create new layout block
                // insert the dragged block and the block we dropped on inside this layout block
                let blocks = match drop_block_event.side_dropped {
                    Side::Left => vec![drag_block.id(), drop_block.id()],
                    Side::Right => vec![drop_block.id(), drag_block.id()],
                    _ => unreachable!()
                };
                let new_layout_block = StandardBlock { 
                    _id: new_ids.get_id()?, 
                    content: StandardBlockType::Layout(LayoutBlock { blocks, horizontal: true }),
                    children: vec![], 
                    parent: drop_block.parent.clone(), 
                    marks: vec![]
                };
                let new_layout_block_id = new_layout_block.id();
                block_map.update_block(Block::StandardBlock(new_layout_block), &mut blocks_to_update)?;
                // replace drop block with new layout block
                update_state_tools::splice_children(
                    drop_parent,
                    drop_block.index(&block_map)?..drop_block.index(&block_map)? + 1,
                    vec![new_layout_block_id.clone()],
                    &mut blocks_to_update,
                    &mut block_map
                )?;
                drag_block.parent = new_layout_block_id.clone();
                drop_block.parent = new_layout_block_id;
                block_map.update_blocks(vec![
                    Block::StandardBlock(drag_block), 
                    Block::StandardBlock(drop_block)
                ], &mut blocks_to_update)?;
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

fn is_layout_block_or_is_inside_layout_block(
    drop_block: &StandardBlock,
    drop_parent: &Block
) -> Result<bool, StepError> {
    match drop_block.content {
        StandardBlockType::Layout(_) => return Ok(true),
        _ => {}
    };
    match drop_parent{
        Block::StandardBlock(parent) => {
            match &parent.content {
                StandardBlockType::Layout(block) => return Ok(block.blocks.contains(&drop_block._id)),
                _ => return Ok(false)
            }
        }
        _ => return Ok(false)
    }
}

fn add_drag_block_to_layout_block(
    drop_block: StandardBlock, 
    drop_parent: Block,
    side_dropped: &Side,
    drag_block_id: &str,
    block_map: &BlockMap
) -> Result<((StandardBlock, Block)), StepError> {
    let edited_drop_block = false;
    match &drop_block.content {
        StandardBlockType::Layout(mut layout_block) => {
            edited_drop_block = true;
            if *side_dropped == Side::Left {
                layout_block.blocks.insert(0, drag_block_id.to_string());
                layout_block.blocks.clone()
            } else {
                layout_block.blocks.push(drag_block_id.to_string());
                layout_block.blocks.clone()
            }
            
        },
        _ => {
            match &drop_parent {
                Block::StandardBlock(StandardBlock { content: StandardBlockType::Layout(mut layout_block), .. }) => {
                    let mut index = drop_block.index(&block_map)?;
                    if *side_dropped == Side::Right {
                        index += 1;
                    }
                    layout_block.blocks.splice(index..index, vec![drop_block.id()]);
                    layout_block.blocks.clone()
                },
                _ => unreachable!()
            }
        }
    };
    return Ok((drop_block, drop_parent))
}