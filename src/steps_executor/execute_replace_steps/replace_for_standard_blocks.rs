use crate::{blocks::{standard_blocks::{StandardBlock, content_block::ContentBlock}, BlockMap, inline_blocks::InlineBlock, Block},
steps_generator::{selection::{SubSelection, Selection}, StepError}, steps_executor::{UpdatedState, clean_block_after_transform}, step::{ReplaceSlice, ReplaceStep}};

use super::replace_for_inline_blocks::{update_from_inline_block_text, update_to_inline_block_text};

/// check parent is the same
/// -> replace children on "from" block with children on the "to" block
/// -> remove all inline blocks on "from" block after the "from" subselection offset/index
/// -> add to the end of the "from" block inline blocks:
///  all inline blocks on "to" block including & after the "to" subselection offset/index
/// -> remove text from "from subselection" block after the subselection offset 
/// -> remove text from "to subselection" block before the subselection offset
/// -> update "to subselection" block's parent to "from" block
/// -> remove all standard blocks between from & to
pub fn replace_selected_across_standard_blocks(
    mut from_block: StandardBlock,
    mut block_map: BlockMap,
    mut replace_step: ReplaceStep,
) -> Result<UpdatedState, StepError> {
    let to_block = block_map.get_standard_block(&replace_step.to.block_id)?;
    if from_block.parent != to_block.parent {
        return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
    }

    let number_of_from_layers = replace_step.from.count_layers();
    let number_of_to_layers = replace_step.to.count_layers();

    let mut parent_block = block_map.get_block(&from_block.parent)?;
    let to_block = block_map.get_standard_block(&replace_step.to.block_id)?;
    parent_block.splice_children(from_block.index(&block_map)? + 1, to_block.index(&block_map)? + 1, vec![])?;
    block_map.update_block(parent_block)?;

    //if number_of_from_layers != number_of_to_layers {
    //if number_of_from_layers != number_of_to_layers {
        replace_step.from = replace_step.from.get_two_deepest_layers()?;
        from_block = block_map.get_standard_block(&replace_step.from.block_id)?;
        replace_step.to = replace_step.to.get_two_deepest_layers()?;
    // }

    match &replace_step.from.subselection {
        Some(from_inner_subselection) => {
            let to_inner_subselection = replace_step.to.get_child_subselection()?;
            let inner_from_index = from_block.index_of(&from_inner_subselection.block_id)?;
            let inner_to_index = to_block.index_of(&to_inner_subselection.block_id)?;

            from_block.children = to_block.children.clone();
            from_block.set_new_parent_of_children(&mut block_map)?;

            let from_block_with_updated_text = merge_blocks_inline_blocks(from_block, to_block, inner_from_index, inner_to_index)?;
            let block_map = from_block_with_updated_text.set_as_parent_for_all_inline_blocks(block_map)?;

            let block_map = update_from_subselection_inline_block_text(block_map, &replace_step)?;
            let block_map = update_to_subselection_inline_block_text(block_map, &replace_step,&from_block_with_updated_text._id)?;
            let block_map = clean_block_after_transform(from_block_with_updated_text, block_map)?;

            return Ok(UpdatedState { block_map, selection: Some(Selection::update_selection_from(replace_step)) })
        },
        None => return replace_across_standard_blocks_no_subselection(from_block, block_map, replace_step)
    }
}

fn update_from_subselection_inline_block_text(
    block_map: BlockMap,
    replace_step: &ReplaceStep,
) -> Result<BlockMap, StepError> {
    let (from_subselection_block, offset) = get_subselection_inline_block(&block_map, &replace_step.from)?;
    let replace_with = match &replace_step.slice {
        ReplaceSlice::String(string) => string.clone(),
        _ => return Err(StepError("Replace slice should be string".to_string()))
    };
    return update_from_inline_block_text(from_subselection_block, block_map, offset, replace_with)
}

fn update_to_subselection_inline_block_text(
    block_map: BlockMap,
    replace_step: &ReplaceStep,
    new_parent_id: &str
) -> Result<BlockMap, StepError> {
    let (mut to_subselection_block, offset) = get_subselection_inline_block(&block_map, &replace_step.to)?;
    to_subselection_block.parent = new_parent_id.to_string();
    return update_to_inline_block_text(to_subselection_block, block_map, offset)
}

fn get_subselection_inline_block(
    block_map: &BlockMap,
    subselection: &SubSelection,
) -> Result<(InlineBlock, usize), StepError> {
    let inner_subselection = subselection.get_child_subselection()?;
    return Ok((block_map.get_inline_block(&inner_subselection.block_id)?, inner_subselection.offset))
}

fn replace_across_standard_blocks_no_subselection(
    from_block: StandardBlock,
    block_map: BlockMap,
    replace_step: ReplaceStep
) -> Result<UpdatedState, StepError> {
    if replace_step.from.block_id == replace_step.to.block_id { // same block
        if replace_step.from.offset == replace_step.to.offset { // same offset
            let updated_subselection = SubSelection::at_end_of_block(&from_block._id, &block_map)?;

            let mut inline_blocks = from_block.content_block()?.clone().inline_blocks;
            let blocks_to_add = match replace_step.slice {
                ReplaceSlice::Blocks(blocks) => blocks,
                _ => return Err(StepError("Replace slice should be blocks".to_string()))
            };
            if replace_step.from.offset == 0 { // add blocks at start of this blocks inline blocks
                inline_blocks.splice(0..0, blocks_to_add);
            } else { // add blocks at end of this blocks inline blocks
                inline_blocks = vec![inline_blocks, blocks_to_add].concat();
            }
            let updated_standard_block = from_block.update_block_content(ContentBlock { inline_blocks })?;
            let updated_standard_block_id = updated_standard_block.id();
            let block_map = clean_block_after_transform(updated_standard_block, block_map)?;
            let updated_standard_block = block_map.get_standard_block(&updated_standard_block_id)?;
            let block_map = updated_standard_block.set_as_parent_for_all_inline_blocks(block_map)?;
            return Ok(UpdatedState { block_map, selection: Some(Selection{ anchor: updated_subselection.clone(), head: updated_subselection } ) })
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }
}

fn merge_blocks_inline_blocks(from_block: StandardBlock, to_block: StandardBlock, inner_from_index: usize, inner_to_index: usize) -> Result<StandardBlock, StepError> {
    let updated_content_block = from_block.content_block()?.clone();
    let mut updated_inline_blocks = updated_content_block.inline_blocks[..inner_from_index + 1].to_vec();
    let to_content_block = to_block.content_block()?;
    let mut to_inline_blocks_after_deletion = to_content_block.inline_blocks[inner_to_index..].to_vec();
    updated_inline_blocks.append(&mut to_inline_blocks_after_deletion);
    return from_block.update_block_content(ContentBlock {
        inline_blocks: updated_inline_blocks
    });
}