use std::{collections::HashMap, str::FromStr};

use js_sys::JsString;

use crate::{blocks::{inline_blocks::InlineBlock, BlockMap, Block, standard_blocks::{content_block::ContentBlock, StandardBlock}},
step::{ReplaceStep, ReplaceSlice}, steps_generator::{StepError, selection::Selection},
steps_executor::{UpdatedState, clean_block_after_transform}};


/// replace "from" index of from block to index of "to" block on their parent
/// replace with slice that consists of:
///  -> "from" block with chars removed from offset to end of text
///  -> "to" block with chars removed from start of text to offset
pub fn replace_selected_across_inline_blocks(
    from_block: InlineBlock,
    block_map: BlockMap,
    replace_step: ReplaceStep,
) -> Result<UpdatedState, StepError> {
    let replace_with = match &replace_step.slice {
        ReplaceSlice::String(string) => string.clone(),
        _ => return Err(StepError("Replace slice should be string".to_string()))
    };
    let to_block = block_map.get_inline_block(&replace_step.to.block_id)?;
    if from_block.parent != to_block.parent {
        return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
    }
    // let parent_block = BlockMap::get_standard_block(block_map, &from_block.parent)?;
    // let content_block = parent_block.content_block()?;
    if from_block._id == to_block._id {
        replace_across_single_inline_block(from_block, block_map, replace_step, replace_with)
    } else {
        replace_across_multiple_inline_blocks(from_block, block_map, replace_step, replace_with)
    }
}

fn replace_across_single_inline_block(
    from_block: InlineBlock,
    mut block_map: BlockMap,
    replace_step: ReplaceStep,
    replace_with: String
) -> Result<UpdatedState, StepError> {
    let text = from_block.text()?.clone();
    // create new block with text from replace_with inserted
    let first_half = text.substring(0, replace_step.from.offset as u32);
    let second_half = text.substring(0, replace_step.to.offset as u32);
    let updated_text = first_half.concat(&js_sys::JsString::from_str(replace_with.as_str()).unwrap()).concat(&second_half);
    let updated_block = from_block.update_text(updated_text)?;
    block_map.update_block(Block::InlineBlock(updated_block))?;

    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection::update_selection_from(replace_step))
    })
}

/// from {
///     block_id: T1
///     offset: 2
///}
/// to {
///     block_id: T3
///     offset: 2
///}
/// }
/// <p>
///     <T1>He\llo</T><T2> New<T/><T3> W\orld<T/>
/// </p>
/// -> Remove any inline blocks between the "from" & "to" index on the parent block
/// -> Remove "from" block text from "from" offset to end & add replace with
/// -> Remove "to" block text from start to offset
fn replace_across_multiple_inline_blocks(
    from_block: InlineBlock,
    block_map: BlockMap,
    replace_step: ReplaceStep,
    replace_with: String
) -> Result<UpdatedState, StepError> {
    let to_block = block_map.get_inline_block(&replace_step.to.block_id)?;
    let updated_parent_block = remove_inline_blocks_between_from_and_to(
        &block_map,
        &from_block,
        &to_block._id,
    )?;
    let block_map = update_from_inline_block_text(from_block, block_map, replace_step.from.offset, replace_with)?;
    let block_map = update_to_inline_block_text(to_block, block_map, replace_step.to.offset)?;
    let block_map = clean_block_after_transform(updated_parent_block, block_map)?;
    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection::update_selection_from(replace_step))
    })
}

fn remove_inline_blocks_between_from_and_to(
    block_map: &BlockMap,
    from_block: &InlineBlock,
    to_block_id: &str,
) -> Result<StandardBlock, StepError> {
    let parent_block = from_block.get_parent(block_map)?;
    let from_index = parent_block.index_of(&from_block._id)?;
    let to_index = parent_block.index_of(to_block_id)?;
    let mut content_block = parent_block.content_block()?.clone();
    content_block.inline_blocks.splice(from_index + 1..to_index, []);
    return parent_block.update_block_content(content_block)
}

pub fn update_from_inline_block_text(
    from_block: InlineBlock,
    block_map: BlockMap,
    offset: usize,
    replace_with: String
) -> Result<BlockMap, StepError> {
    let updated_text = from_block.text()?.substring(0, offset as u32).concat(&js_sys::JsString::from_str(replace_with.as_str()).unwrap());
    return update_inline_block_with_new_text_in_block(from_block, block_map, updated_text)
}

pub fn update_to_inline_block_text(to_block: InlineBlock, block_map: BlockMap, offset: usize) -> Result<BlockMap, StepError> {
    let updated_text = &to_block.text()?.substring(offset as u32, to_block.text()?.length());
    return update_inline_block_with_new_text_in_block(to_block, block_map, updated_text.to_owned())
}

fn update_inline_block_with_new_text_in_block(
    inline_block: InlineBlock,
    mut block_map: BlockMap,
    updated_text: JsString
) -> Result<BlockMap, StepError> {
    let updated_inline_block = inline_block.update_text(updated_text)?;
    block_map.update_block(Block::InlineBlock(updated_inline_block))?;
    return Ok(block_map)
}