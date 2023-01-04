use std::{collections::HashMap, str::FromStr};

use js_sys::JsString;

use crate::{blocks::{inline_blocks::{InlineBlock, text_block::StringUTF16}, BlockMap, Block, standard_blocks::{content_block::ContentBlock, StandardBlock}},
step::{ReplaceStep, ReplaceSlice}, steps_generator::{StepError, selection::Selection},
steps_actualisor::{UpdatedState, clean_block_after_transform}};


/// replace "from" index of from block to index of "to" block on their parent
/// replace with slice that consists of:
///  -> "from" block with chars removed from offset to end of text
///  -> "to" block with chars removed from start of text to offset
//-------------------------------------------------------------
// The function replace_selected_across_inline_blocks takes in an 
// - InlineBlock, 
// - aBlockMap, 
// - a ReplaceStep, 
// - a mutable vector of Strings representing blocks to update. 
//It returns a Result with an UpdatedState on success or a StepError on failure.

// The function first checks the type of the slice field of the ReplaceStep, 
//which should be a String, and returns an error if it is not. 
//It then retrieves the InlineBlock specified by the to field of the ReplaceStep. 
//If the parent of the from_block is not the same as the parent of the to_block, the function returns an error.

// If the _id field of the from_block is the same as the _id field of the to_block, 
//the function calls replace_across_single_inline_block with the 
// - from_block, 
// - block_map, 
// - replace_step, 
// - replace_with, 
// - blocks_to_update as arguments. 
//Otherwise, it calls replace_across_multiple_inline_blocks with the same arguments.
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

// This function updates a single inline block with new text 
//that has the specified string replaced within it. 
//The replace_with parameter is the string to insert, and replace_step is a struct
//containing information about the range of the text to replace (from and to fields).

// The function first retrieves the current text of the from_block and stores it in a text variable. 
//It then uses the splice method of the text variable to remove the specified range 
//and insert the replace_with string in its place.

// Next, the function creates an updated version of the from_block 
//with the modified text using the update_text method. 
//Finally, it updates the block in the block map and returns an UpdatedState struct 
//with the modified block map and an updated selection.

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
//--------------------------------------------------

//This function appears to be responsible for replacing the selected text across multiple inline blocks, 
//in a document represented as a BlockMap. 
//The replace_step argument is a ReplaceStep struct 
//that contains the start and end points of the text selection, 
//and the replace_with argument is a string that contains the text to replace the selected text with. 
//The function first retrieves the to_block from the block_map, 
//which is the inline block that the end of the selection is in. 
//It then calls remove_inline_blocks_between_from_and_to to remove the inline blocks 
//between the from_block and the to_block.

// After that, the function calls update_from_inline_block_text to update the text of the from_block, 
//replacing the selected text with the replace_with string. 
//It then calls update_to_inline_block_text to update the text of the to_block, 
//replacing the selected text with the replace_with string. 
//Finally, it calls clean_block_after_transform to clean up the updated_parent_block, 
//and then returns an UpdatedState struct that contains the 
// - updated block_map, 
// - an updated selection, 
// - lists of blocks to update and blocks to remove.

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
// This function appears to remove all of the inline blocks 
//that appear between the from_block and to_block in the parent_block.

// It does this by getting the parent block of from_block, 
//then finding the indices of from_block and to_block in the parent block's list of inline blocks. 
//It then splices the list of inline blocks by removing all of the blocks 
//from the index immediately after from_block to to_block, inclusive. 
//Finally, it updates the parent block's content with the modified list of inline blocks
//and returns the updated parent block.
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

//This function updates the text of an InlineBlock in a BlockMap. 
//It takes in 
// - an InlineBlock, 
// - a BlockMap, 
// - a new offset for the text, 
// - a replace_with string to replace the text from the offset, 
// - a mutable blocks_to_update vector. 
//It first retrieves the current text of the InlineBlock 
//and creates a new StringUTF16 object with the replace_with string. 
//It then splices the original text from the offset 
//until the end of the text with the new StringUTF16 object. 
//It then calls update_inline_block_with_new_text_in_block function with the updated 
// - InlineBlock, 
// - the original BlockMap, 
// - the new text, 
// - the blocks_to_update vector and returns the result.
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

// This function updates the given to_block by replacing its text with a new text 
//that has the first offset characters removed. 
//The updated block is then added to the block map and returned. 
//The blocks_to_update vector is used to track the blocks that have been updated 
//so that they can be passed to the frontend to be updated.

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

//It looks like update_inline_block_with_new_text_in_block is a function that takes an 
// - InlineBlock, 
// - a BlockMap, 
// - a StringUTF16, 
// - a mutable reference to a Vec<String>. 
//It updates the text of the InlineBlock with the provided StringUTF16 
//and updates the BlockMap to include this updated InlineBlock. 
//It also appends the id of the updated InlineBlock to the Vec<String> passed in as the blocks_to_update argument. 
//Finally, it returns the updated BlockMap.

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