use crate::{step::MarkStep, blocks::{standard_blocks::{StandardBlock, content_block::ContentBlock}, BlockMap, Block}, steps_executor::{UpdatedState, clean_block_after_transform}, steps_generator::{StepError, selection::SubSelection}, mark::Mark, new_ids::NewIds};



/// -> apply mark for "from" std block -> from "inner from" to end of inline blocks
/// -> apply mark for "to" std block -> from start of inline blocks to "inner to"
/// -> for each standard block between "from" & "to" -> assign mark to each of their inline blocks
pub fn execute_mark_step_on_standard_blocks(
    mark_step: MarkStep,
    mut block_map: BlockMap,
    add_mark: bool,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {

    let top_from_block = block_map.get_standard_block(&mark_step.from.block_id)?;

    let parent_block = block_map.get_block(&top_from_block.parent)?;
    let from_index = parent_block.index_of_child(&mark_step.from.block_id)?;
    let to_index = parent_block.index_of_child(&mark_step.to.block_id)?;
    let from_second_deepest_layer = mark_step.from.clone().get_two_deepest_layers()?;
    let from_deepest_layer = mark_step.from.get_deepest_subselection();
    let from_deepest_std_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;
    let to_second_deepest_layer = mark_step.to.clone().get_two_deepest_layers()?;
    let to_deepest_layer = mark_step.to.get_deepest_subselection();
    let to_deepest_std_block = block_map.get_standard_block(&to_second_deepest_layer.block_id)?;
    split_edge_inline_blocks(&mut block_map, new_ids, from_deepest_layer, from_deepest_std_block)?;
    split_edge_inline_blocks(&mut block_map, new_ids, to_deepest_layer, to_deepest_std_block)?;
    let from_deepest_std_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;
    let to_deepest_std_block = block_map.get_standard_block(&to_second_deepest_layer.block_id)?;
    if !top_from_block.parent_is_root(&block_map) {
        let parent_block = block_map.get_standard_block(&top_from_block.parent)?;
        from_deepest_std_block.apply_mark_to_all_inline_blocks_in_range(
            mark_step.mark.clone(),
            from_deepest_std_block.index_of(&from_deepest_layer.block_id)? + 1,
            from_deepest_std_block.content_block()?.inline_blocks.len() - 1,
            &mut block_map,
            add_mark,
        )?;
        
        apply_mark_to_descendants_inline_blocks(
            &from_deepest_std_block, 
            &mark_step.mark,
            &mut block_map, 
            Some((&to_deepest_layer.block_id, to_deepest_std_block.index_of(&to_deepest_layer.block_id)?)),
            add_mark
        )?;

        for block_id in parent_block.children[from_index + 1..=to_index].iter() {
            let child = block_map.get_standard_block(block_id)?;
            if child.id() == to_deepest_std_block.id() {
                child.apply_mark_to_all_inline_blocks_in_range(
                    mark_step.mark.clone(),
                    0,
                    to_deepest_std_block.index_of(&to_deepest_layer.block_id)?,
                    &mut block_map,
                    add_mark,
                )?;
                break;
            } else {
                child.apply_mark_to_all_inline_blocks(mark_step.mark.clone(), &mut block_map, add_mark)?;
                apply_mark_to_descendants_inline_blocks(
                    &child,
                    &mark_step.mark,
                    &mut block_map,
                    Some((&to_deepest_layer.block_id, to_deepest_std_block.index_of(&to_deepest_layer.block_id)?)),
                    add_mark
                )?;
            }
        }
    } else {
        // for "from"
        let from_second_deepest_layer = mark_step.from.clone().get_two_deepest_layers()?;
        let from_deepest_layer = mark_step.from.get_deepest_subselection();
        let from_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;

        from_block.apply_mark_to_all_inline_blocks_in_range(
            mark_step.mark.clone(), 
            from_block.index_of(&from_deepest_layer.block_id)? + 1,
            from_block.content_block()?.inline_blocks.len() - 1,
            &mut block_map,
            add_mark,
        )?;
        
        apply_mark_to_descendants_inline_blocks(
            &from_block,
            &mark_step.mark,
            &mut block_map,
            None,
            add_mark
        )?;
        apply_mark_to_all_lower_relatives(&from_block, &mark_step.mark, &mut block_map, add_mark)?;
        let to_second_deepest_layer = mark_step.to.clone().get_two_deepest_layers()?;
        let to_deepest_layer = mark_step.to.get_deepest_subselection();
        let to_block = block_map.get_standard_block(&to_second_deepest_layer.block_id)?;
        to_block.apply_mark_to_all_inline_blocks_in_range(
            mark_step.mark.clone(), 
            0, 
            to_block.index_of(&to_deepest_layer.block_id)?,
            &mut block_map,
            add_mark
        )?;
        
        if &to_block._id != &mark_step.to.block_id {
            let to_block_highest = block_map.get_standard_block(&mark_step.to.block_id)?;
            to_block_highest.apply_mark_to_all_inline_blocks_in_range(mark_step.mark.clone(), 0, to_block_highest.content_block()?.inline_blocks.len() - 1, &mut block_map, add_mark)?;
            apply_mark_to_descendants_inline_blocks(
                &to_block_highest, 
                &mark_step.mark.clone(),
                &mut block_map, 
                Some((&to_block._id, to_block.index_of(&to_deepest_layer.block_id)?)),
                add_mark
            )?;
        }
        
        let highest_level_parent = block_map.get_block(&mark_step.block_id)?;
        let to_block_index = highest_level_parent.index_of_child(&mark_step.to.block_id)?;
        
        let from_block_index = highest_level_parent.index_of_child(&mark_step.from.block_id)?;
        let highest_parent_children = highest_level_parent.children()?;
        
        for id in highest_parent_children[from_block_index + 1..to_block_index].iter() {
            let block = block_map.get_standard_block(id)?;
            block.apply_mark_to_all_inline_blocks(mark_step.mark.clone(), &mut block_map, add_mark)?;
            apply_mark_to_descendants_inline_blocks(&block, &mark_step.mark, &mut block_map, None, add_mark)?;
        }
        
    }
    let from_second_deepest_layer = mark_step.from.clone().get_two_deepest_layers()?;
    let from_deepest_std_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;
    let to_second_deepest_layer = mark_step.to.clone().get_two_deepest_layers()?;
    let to_deepest_std_block = block_map.get_standard_block(&to_second_deepest_layer.block_id)?;
    block_map = clean_block_after_transform(from_deepest_std_block, block_map)?;
    block_map = clean_block_after_transform(to_deepest_std_block, block_map)?;

    return Ok(UpdatedState { block_map, selection: None })
}

/// This function is recursive
fn apply_mark_to_descendants_inline_blocks(
    block: &StandardBlock,
    mark: &Mark,
    block_map: &mut BlockMap,
    to_block_to_stop_at: Option<(&String, usize)>, // (block id, end of to selection index (of inline block))
    add_mark: bool
) -> Result<(), StepError> {
    for id in &block.children {
        let child = block_map.get_standard_block(id)?;

        match to_block_to_stop_at {
            Some(to_block_to_stop_at) => {
                if id == to_block_to_stop_at.0 {
                    return child.apply_mark_to_all_inline_blocks_in_range(mark.clone(), 0, to_block_to_stop_at.1, block_map, add_mark)
                } else {
                    child.apply_mark_to_all_inline_blocks(mark.clone(), block_map, add_mark)?;
                    apply_mark_to_descendants_inline_blocks(&child, mark, block_map, None, add_mark)?;
                }
            },
            None => {
                child.apply_mark_to_all_inline_blocks(mark.clone(), block_map, add_mark)?;
                apply_mark_to_descendants_inline_blocks(&child, mark, block_map, None, add_mark)?;
            }
        };
    }

    return Ok(())
}

fn apply_mark_to_all_lower_relatives(
    block: &StandardBlock,
    mark: &Mark,
    block_map: &mut BlockMap,
    add_mark: bool
) -> Result<(), StepError> {
    let parent_as_block = block.get_parent(block_map)?;
    let parent;
    if parent_as_block.is_root() {
        return Ok(())
    } else {
        parent = block_map.get_standard_block(&parent_as_block.id())?;
    }
    let mut i = block.index(block_map)? + 1;
    while i < parent.children.len() {
        let younger_sibling = block_map.get_standard_block(&parent.children[i])?;
        younger_sibling.apply_mark_to_all_inline_blocks(mark.clone(), block_map, add_mark)?;
        apply_mark_to_descendants_inline_blocks(&younger_sibling, mark, block_map, None, add_mark)?;
        i += 1;
    }

    apply_mark_to_all_lower_relatives(&parent, mark, block_map, add_mark)?;
    return Ok(())
}

fn split_edge_inline_blocks(
    block_map: &mut BlockMap,
    new_ids: &mut NewIds,
    deepest_layer: &SubSelection,
    deepest_std_block: StandardBlock,
) -> Result<(), StepError> {
    let from_inline_block = block_map.get_inline_block(&deepest_layer.block_id)?;
    let (first_half, second_half) = from_inline_block.split(deepest_layer.offset, new_ids)?;
    let mut inline_blocks = deepest_std_block.content_block()?.clone().inline_blocks;
    inline_blocks.insert(first_half.index(&block_map)? + 1, second_half.id());
    let deepest_std_block = deepest_std_block.update_block_content(ContentBlock { inline_blocks })?;
    block_map.update_blocks(vec![Block::InlineBlock(first_half), Block::InlineBlock(second_half), Block::StandardBlock(deepest_std_block)])?;
    return Ok(())
}