use crate::{blocks::{standard_blocks::{StandardBlock, content_block::ContentBlock}, BlockMap, inline_blocks::InlineBlock, Block},
steps_generator::{selection::{SubSelection, Selection}, StepError}, steps_actualisor::{UpdatedState, clean_block_after_transform}, step::{ReplaceSlice, ReplaceStep}};

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
// -----------------------------------------------------------

//This function replaces a selected range of text with a new string in a document.
//The from_block and to_block parameters represent the start and end blocks of the selected range, respectively.
//The replace_step parameter holds the new string to be inserted and the updated selection after the replacement.
//The blocks_to_update parameter is a list of blocks that will be updated in the block map
//as a result of the replacement.

// The function first checks if the from_block and to_block have the same parent block.
//If they do not, an error is returned.
//Next, the function checks if the from_block and to_block
//are the same block or if they are different blocks.
//If they are the same block, it calls the replace_across_single_standard_block function to perform the replacement.
//If they are different blocks, it calls the replace_across_multiple_standard_blocks function to perform the replacement.

// The replace_across_single_standard_block function updates the text of the from_block
//with the new string and updates the block map with the modified block.
//The replace_across_multiple_standard_blocks function removes all blocks between the from_block and to_block,
//updates the text of the from_block and to_block with the new string,
//and updates the block map with the modified blocks.

// The function returns a Result object with an UpdatedState variant containing
//the updated block map,
// - updated selection,
// - list of blocks to update,
// - list of blocks to remove,
// - copy information.
//If an error occurs, the Result object will contain a StepError variant
pub fn replace_selected_across_standard_blocks(
    from_block: StandardBlock,
    mut block_map: BlockMap,
    mut replace_step: ReplaceStep,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let to_block = block_map.get_standard_block(&replace_step.to.block_id)?;
    if from_block.parent != to_block.parent {
        return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
    }

    remove_any_blocks_between_from_and_to_incl_to(&mut block_map, &from_block, &to_block, &mut blocks_to_update)?;
    let (mut from_block, to_block) = get_deepest_std_blocks_in_selection(&mut replace_step, &block_map)?;

    if !to_block.parent_is_root(&block_map) {
        move_to_blocks_younger_siblings_after_from_block(&from_block, &to_block, &mut block_map, &mut blocks_to_update)?;
    }

    match &replace_step.from.subselection {
        Some(_) => {
            from_block.children = to_block.children.clone();
            from_block.set_new_parent_of_children(&mut block_map, &mut blocks_to_update)?;

            let block_map = replace_inline_blocks_text(&replace_step, from_block, to_block, block_map, &mut blocks_to_update)?;

            return Ok(UpdatedState {
                block_map,
                selection: Some(Selection::update_selection_from(replace_step)),
                blocks_to_update,
                blocks_to_remove: vec![],
                copy: None
            })
        },
        None => return replace_across_standard_blocks_no_subselection(from_block, block_map, replace_step, blocks_to_update)
    }
}

fn remove_any_blocks_between_from_and_to_incl_to(block_map: &mut BlockMap, from_block: &StandardBlock, to_block: &StandardBlock, mut blocks_to_update: &mut Vec<String>) -> Result<(), StepError> {
    let mut parent_block = block_map.get_block(&from_block.parent)?;
    parent_block.splice_children(from_block.index(&block_map)? + 1, to_block.index(&block_map)? + 1, vec![])?;
    block_map.update_block(parent_block, &mut blocks_to_update)?;
    return Ok(())
}

fn get_deepest_std_blocks_in_selection(replace_step: &mut ReplaceStep, block_map: &BlockMap) -> Result<(StandardBlock, StandardBlock), StepError> {
    replace_step.from = replace_step.from.clone().get_two_deepest_layers()?;
    replace_step.to = replace_step.to.clone().get_two_deepest_layers()?;
    return Ok((block_map.get_standard_block(&replace_step.from.block_id)?, block_map.get_standard_block(&replace_step.to.block_id)?))
}

fn move_to_blocks_younger_siblings_after_from_block(
    from_block: &StandardBlock,
    to_block: &StandardBlock,
    block_map: &mut BlockMap,
    blocks_to_update: &mut Vec<String>
) -> Result<(), StepError> {
    let siblings_after_to_block = to_block.get_siblings_after(&block_map)?;
    let mut from_parent = from_block.get_parent(&block_map)?;
    let mut from_siblings = from_parent.children()?.clone();
    // if from_parent.is_root() {
        from_siblings.splice(from_block.index(&block_map)? + 1..from_block.index(&block_map)? + 1, siblings_after_to_block);
    // } else {
        // from_siblings.splice(from_block.index(&block_map)? + 1.., siblings_after_to_block);
    // }
    from_parent.set_children(from_siblings)?;
    from_parent.set_new_parent_of_children(block_map, blocks_to_update)?;
    block_map.update_block(from_parent, blocks_to_update)?;
    return Ok(())
}

fn replace_inline_blocks_text(
    replace_step: &ReplaceStep,
    from_block: StandardBlock,
    to_block: StandardBlock,
    block_map: BlockMap,
    blocks_to_update: &mut Vec<String>
) -> Result<BlockMap, StepError> {
    let inner_from_index = from_block.index_of(&replace_step.from.get_child_subselection()?.block_id)?;
    let inner_to_index = to_block.index_of(&replace_step.to.get_child_subselection()?.block_id)?;

    let from_block_with_updated_text = merge_blocks_inline_blocks(from_block, to_block, inner_from_index, inner_to_index)?;
    let block_map = from_block_with_updated_text.set_as_parent_for_all_inline_blocks(block_map, blocks_to_update)?;

    let block_map = update_from_subselection_inline_block_text(block_map, &replace_step, blocks_to_update)?;
    let block_map = update_to_subselection_inline_block_text(block_map, &replace_step,&from_block_with_updated_text._id, blocks_to_update)?;
    let block_map = clean_block_after_transform(from_block_with_updated_text, block_map, blocks_to_update)?;
    return Ok(block_map)
}

// This function updates the text of the inline block
//that corresponds to the from position of the replace_step operation.

//It does this by:

// 1. Retrieving the inline block and the offset within it that corresponds to the from position. This is done using the get_subselection_inline_block function.
// 2. Extracting the string to be used as the replacement from the replace_step operation.
// 3. Updating the text of the inline block by replacing a slice of it with the replacement string, starting at the specified offset. This is done using the update_from_inline_block_text function.
// 4. Finally, the updated block map is returned.

// Note that this function expects the replace_step to contain a subselection
//(i.e., the from and to positions should have an inner selection).
//If this is not the case, an error is returned.
fn update_from_subselection_inline_block_text(
    block_map: BlockMap,
    replace_step: &ReplaceStep,
    mut blocks_to_update: &mut Vec<String>
) -> Result<BlockMap, StepError> {
    let (from_subselection_block, offset) = get_subselection_inline_block(&block_map, &replace_step.from)?;
    let replace_with = match &replace_step.slice {
        ReplaceSlice::String(string) => string.clone(),
        _ => return Err(StepError("Replace slice should be string".to_string()))
    };
    return update_from_inline_block_text(from_subselection_block, block_map, offset, replace_with, &mut blocks_to_update)
}

// The update_to_subselection_inline_block_text function updates
//the text of the to_subselection_block inline block in the given block_map.
//The to_subselection_block is obtained by calling the get_subselection_inline_block function
//with the replace_step.to argument. The offset value is also obtained
//from the get_subselection_inline_block function.

// The to_subselection_block inline block's parent is set to new_parent_id,
//and the update_to_inline_block_text function is called with the
// - to_subselection_block,
// - block_map,
// - offset,
// - blocks_to_update arguments.
//The update_to_inline_block_text function creates a new text string
//by splicing the current text of to_subselection_block from the start index to the offset index.
//Then, the function updates the to_subselection_block inline block with the new text
//and updates the block_map with the updated block.
// The updated block_map is returned by the update_to_subselection_inline_block_text function.
fn update_to_subselection_inline_block_text(
    block_map: BlockMap,
    replace_step: &ReplaceStep,
    new_parent_id: &str,
    mut blocks_to_update: &mut Vec<String>
) -> Result<BlockMap, StepError> {
    let (mut to_subselection_block, offset) = get_subselection_inline_block(&block_map, &replace_step.to)?;
    to_subselection_block.parent = new_parent_id.to_string();
    return update_to_inline_block_text(to_subselection_block, block_map, offset, &mut blocks_to_update)
}

// This function appears to be used to get an InlineBlock and an offset based on a given SubSelection.
//The SubSelection is used to get a "child" SubSelection,
//which is then used to get an InlineBlock from the BlockMap.
//The offset is taken from the child SubSelection.
//This InlineBlock and offset are then returned as a tuple
fn get_subselection_inline_block(
    block_map: &BlockMap,
    subselection: &SubSelection,
) -> Result<(InlineBlock, usize), StepError> {
    let inner_subselection = subselection.get_child_subselection()?;
    return Ok((block_map.get_inline_block(&inner_subselection.block_id)?, inner_subselection.offset))
}

// This function appears to be a helper function for replacing
//a selection of text with new text or blocks in a document.
//If the "from" and "to" positions of the selection are within the same block,
//the function will update the contents of that block by
//splicing in the new text or blocks as specified in the replace_step argument.
//If the "from" and "to" positions are not within the same block,
//it looks like the function will attempt to remove all blocks between
//the "from" and "to" blocks and then splice in the new text or blocks.


//However, the code for this scenario has not been implemented yet, as indicated by the unimplemented! macros.

fn replace_across_standard_blocks_no_subselection(
    from_block: StandardBlock,
    block_map: BlockMap,
    replace_step: ReplaceStep,
    mut blocks_to_update: Vec<String>
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
            let block_map = clean_block_after_transform(updated_standard_block, block_map, &mut blocks_to_update)?;
            let updated_standard_block = block_map.get_standard_block(&updated_standard_block_id)?;
            let block_map = updated_standard_block.set_as_parent_for_all_inline_blocks(block_map, &mut blocks_to_update)?;
            return Ok(UpdatedState {
                block_map,
                selection: Some(Selection{ anchor: updated_subselection.clone(), head: updated_subselection }),
                blocks_to_update,
                blocks_to_remove: vec![],
                copy: None
            })
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }
}
// This function looks like it merges two inline blocks within two different standard blocks together.

// It first creates a copy of the from_block's content block,
//and then creates a vector of the first inner_from_index + 1 inline blocks in the new content block.
//It then gets the to_block's content block,
//and gets a vector of the inline blocks after inner_to_index.
//These two vectors are then appended together to form a new vector of inline blocks
//for the new content block.
//Finally, the from_block's content block is updated with this new vector of inline blocks,
//and the modified from_block is returned.
fn merge_blocks_inline_blocks(
    from_block: StandardBlock,
    to_block: StandardBlock,
    inner_from_index: usize,
    inner_to_index: usize
) -> Result<StandardBlock, StepError> {
    let updated_content_block = from_block.content_block()?.clone();
    let mut updated_inline_blocks = updated_content_block.inline_blocks[..inner_from_index + 1].to_vec();
    let to_content_block = to_block.content_block()?;
    let mut to_inline_blocks_after_deletion = to_content_block.inline_blocks[inner_to_index..].to_vec();
    updated_inline_blocks.append(&mut to_inline_blocks_after_deletion);
    return from_block.update_block_content(ContentBlock {
        inline_blocks: updated_inline_blocks
    });
}