use std::{collections::HashMap, str::FromStr};

use js_sys::JsString;

use crate::{blocks::{inline_blocks::{InlineBlock, text_block::StringUTF16}, BlockMap, Block, standard_blocks::{content_block::ContentBlock, StandardBlock}},
step::{ReplaceStep, ReplaceSlice}, steps_generator::{StepError, selection::Selection},
steps_actualisor::{UpdatedState, clean_block_after_transform}};


/// replace "from" index of from block to index of "to" block on their parent
/// replace with slice that consists of:
///  -> "from" block with chars removed from offset to end of text
///  -> "to" block with chars removed from start of text to offset
pub fn replace_selected_across_inline_blocks(
    from_block: InlineBlock,
    block_map: BlockMap,
    replace_step: ReplaceStep,
    mut blocks_to_update: Vec<String>
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
        replace_across_single_inline_block(from_block, block_map, replace_step, replace_with, blocks_to_update)
    } else {
        replace_across_multiple_inline_blocks(from_block, block_map, replace_step, replace_with, blocks_to_update)
    }
}

fn replace_across_single_inline_block(
    from_block: InlineBlock,
    mut block_map: BlockMap,
    replace_step: ReplaceStep,
    replace_with: String,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let mut text = from_block.text()?.clone();
    text.splice(replace_step.from.offset..replace_step.to.offset, StringUTF16::from_str(replace_with.as_str()));
    // create new block with text from replace_with inserted
    let updated_block = from_block.update_text(text)?;
    block_map.update_block(Block::InlineBlock(updated_block), &mut blocks_to_update)?;

    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection::update_selection_from(replace_step)),
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
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
    replace_with: String,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let to_block = block_map.get_inline_block(&replace_step.to.block_id)?;
    let updated_parent_block = remove_inline_blocks_between_from_and_to(
        &block_map,
        &from_block,
        &to_block._id,
    )?;
    let block_map = update_from_inline_block_text(from_block, block_map, replace_step.from.offset, replace_with, &mut blocks_to_update)?;
    let block_map = update_to_inline_block_text(to_block, block_map, replace_step.to.offset, &mut blocks_to_update)?;
    let block_map = clean_block_after_transform(updated_parent_block, block_map, &mut blocks_to_update)?;
    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection::update_selection_from(replace_step)),
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
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
    replace_with: String,
    blocks_to_update: &mut Vec<String>
) -> Result<BlockMap, StepError> {
    let mut text = from_block.text()?.clone();
    text.splice(offset..text.len(), StringUTF16::from_str(replace_with.as_str()));
    return update_inline_block_with_new_text_in_block(from_block, block_map, text.clone(), blocks_to_update)
}

pub fn update_to_inline_block_text(
    to_block: InlineBlock,
    block_map: BlockMap,
    offset: usize,
    blocks_to_update: &mut Vec<String>
) -> Result<BlockMap, StepError> {
    let mut text = to_block.text()?.clone();
    text.splice(0..offset, StringUTF16::new());
    return update_inline_block_with_new_text_in_block(to_block, block_map, text.clone(), blocks_to_update)
}

fn update_inline_block_with_new_text_in_block(
    inline_block: InlineBlock,
    mut block_map: BlockMap,
    updated_text: StringUTF16,
    blocks_to_update: &mut Vec<String>
) -> Result<BlockMap, StepError> {
    let updated_inline_block = inline_block.update_text(updated_text)?;
    block_map.update_block(Block::InlineBlock(updated_inline_block), blocks_to_update)?;
    return Ok(block_map)
}