use crate::{blocks::{standard_blocks::{StandardBlock, content_block::ContentBlock}, BlockMap, inline_blocks::InlineBlock},
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
pub fn replace_selected_across_standard_blocks(
    mut from_block: StandardBlock,
    mut block_map: BlockMap,
    replace_step: ReplaceStep,
) -> Result<UpdatedState, StepError> {
    let to_block = block_map.get_standard_block(&replace_step.to.block_id)?;
    match from_block.parent == to_block.parent {
        true => {
            from_block.children = to_block.children.clone();
            let updated_content_block = from_block.content_block()?;
            let mut updated_inline_blocks = updated_content_block.inline_blocks[..replace_step.from.offset + 1].to_vec();
            let to_content_block = to_block.content_block()?;
            let mut to_inline_blocks_after_deletion = to_content_block.inline_blocks[replace_step.to.offset..].to_vec();
            updated_inline_blocks.append(&mut to_inline_blocks_after_deletion);
            let from_block = from_block.update_block_content(ContentBlock {
                inline_blocks: updated_inline_blocks
            })?;

            let parent_block = block_map.get_block(&from_block.parent)?;
            let parent_block = parent_block.remove_child_from_id(&to_block._id)?;
            block_map.update_block(parent_block)?;

            let block_map = update_from_subselecton_inline_block_text(block_map, &replace_step)?;
            let block_map = update_to_subselecton_inline_block_text(block_map, &replace_step,&from_block._id)?;
            let block_map = clean_block_after_transform(&from_block, block_map)?;

            return Ok(UpdatedState { block_map, selection: Selection::update_selection_from(replace_step) })
        },
        false => return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
    };
}

fn update_from_subselecton_inline_block_text(
    block_map: BlockMap,
    replace_step: &ReplaceStep,
) -> Result<BlockMap, StepError> {
    let (from_subselection_block, offset) = get_subselection_inline_block(&block_map, &replace_step.from)?;
    let replace_with = match &replace_step.slice {
        ReplaceSlice::String(string) => string.clone(),
        ReplaceSlice::Blocks(_) => unimplemented!()
    };
    return update_from_inline_block_text(from_subselection_block, block_map, offset, replace_with)
}

fn update_to_subselecton_inline_block_text(
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
    let inner_subselection = match &subselection.subselection {
        Some(inner_subselection) => *inner_subselection.clone(),
        None => return Err(StepError("Expected to.subselection to be Some".to_string()))
    };
    return Ok((block_map.get_inline_block(&inner_subselection.block_id)?, inner_subselection.offset))
}
