use crate::{blocks::{standard_blocks::{StandardBlock, content_block::ContentBlock, StandardBlockType}, BlockMap, inline_blocks::InlineBlock, Block},
steps_generator::{selection::{SubSelection, Selection}, StepError}, steps_actualisor::{UpdatedState, clean_block_after_transform}, step::{ReplaceSlice, ReplaceStep}, utilities::{get_blocks_between, BlockStructure, BlocksBetween, update_state_tools}, new_ids::{NewIds}};

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
    mut blocks_to_update: Vec<String>,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {
    let to_block = block_map.get_standard_block(&replace_step.to.block_id)?;
    if from_block.parent != to_block.parent {
        return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
    }

    remove_all_selected_blocks_between_from_and_to(&mut block_map, &replace_step, &mut blocks_to_update, new_ids)?;
    let highest_from = replace_step.from.clone();
    let highest_to = replace_step.to.clone();
    let highest_from_block = block_map.get_standard_block(&highest_from.block_id)?;
    let highest_to_block = block_map.get_standard_block(&highest_to.block_id)?;

    let selection_is_inside_single_std_block = !(
        highest_from_block.parent_is_root(&block_map) && highest_to_block.parent_is_root(&block_map)
        && highest_from_block._id != highest_to_block._id
    );

    let (mut from_block, to_block) = get_deepest_std_blocks_in_selection(&mut replace_step, &block_map)?;
    move_to_block_children_to_from_block(&mut from_block, to_block, &mut block_map, &mut blocks_to_update)?;

    let from_block = block_map.get_standard_block(&replace_step.from.block_id)?;
    let to_block = block_map.get_standard_block(&replace_step.to.block_id)?;

    if !to_block.parent_is_root(&block_map) && !(selection_is_inside_single_std_block && from_block.children.contains(&to_block._id)) {
        let to_block_parent = to_block.get_parent(&block_map)?;
        let parent_is_layout_block = match to_block_parent {
            Block::StandardBlock(StandardBlock { content: StandardBlockType::Layout(_), .. }) => true,
            _ => false
        };

        if !parent_is_layout_block {
            move_to_block_siblings_after_from_block(&from_block, &to_block, &mut block_map, &mut blocks_to_update)?;
        }
    }
    to_block.drop(&mut block_map, &mut blocks_to_update)?;
    // need to re-get from block as it to_block drop may have removed it from "from" block
    let from_block = block_map.get_standard_block(&replace_step.from.block_id)?;

    if !selection_is_inside_single_std_block {
        let highest_from_block = block_map.get_standard_block(&highest_from.block_id)?;
        let highest_from_parent = block_map.get_block(&highest_from_block.parent)?;
        let highest_to_block = block_map.get_standard_block(&highest_to.block_id)?;

        let one_of_highest_blocks_is_a_layout_block = highest_from_block.is_horizontal_layout() || highest_to_block.is_horizontal_layout();
        if !one_of_highest_blocks_is_a_layout_block {
            update_state_tools::splice_children( // move any remaining children from highest "to" block to root
                highest_from_parent,
                highest_from_block.index(&block_map)? + 1..highest_from_block.index(&block_map)? + 1,
                highest_to_block.children,
                &mut blocks_to_update,
                &mut block_map
            )?;
        }
    }

    let block_map = replace_inline_blocks_text(&replace_step, from_block, to_block, block_map, &mut blocks_to_update)?;

    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection::update_selection_from(replace_step)),
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}

fn remove_all_selected_blocks_between_from_and_to(
    block_map: &mut BlockMap,
    replace_step: &ReplaceStep,
    blocks_to_update: &mut Vec<String>,
    new_ids: &mut NewIds
) -> Result<(), StepError> {
    let selected_blocks = match get_blocks_between(
        BlockStructure::Flat,
        &replace_step.from,
        &replace_step.to,
        block_map,
        new_ids
    )? {
        BlocksBetween::Flat(blocks) => blocks,
        _ => return Err(StepError("Expected BlocksBetween::Flat".to_string()))
    };
    let mut i = 0;
    let len = selected_blocks.len();
    for block in selected_blocks {
        match block.content {
            StandardBlockType::Layout(_) => {},
            _ if i != 0 && i != len - 1 => {
                block.drop(block_map, blocks_to_update)?;
            },
            _ => {}
        };
        i += 1;
    }
    return Ok(())
}

fn get_deepest_std_blocks_in_selection(replace_step: &mut ReplaceStep, block_map: &BlockMap) -> Result<(StandardBlock, StandardBlock), StepError> {
    replace_step.from = replace_step.from.clone().get_two_deepest_layers()?;
    replace_step.to = replace_step.to.clone().get_two_deepest_layers()?;
    return Ok((block_map.get_standard_block(&replace_step.from.block_id)?, block_map.get_standard_block(&replace_step.to.block_id)?))
}

fn move_to_block_siblings_after_from_block(
    from_block: &StandardBlock,
    to_block: &StandardBlock,
    block_map: &mut BlockMap,
    blocks_to_update: &mut Vec<String>,
) -> Result<(), StepError> {
    let siblings_after_to_block = to_block.get_siblings_after(&block_map)?;
    to_block.remove_siblings_after(block_map, blocks_to_update)?;

    let from_parent = from_block.get_parent(&block_map)?;
    update_state_tools::splice_children(
        from_parent,
        from_block.index(&block_map)? + 1..from_block.index(&block_map)? + 1,
        siblings_after_to_block,
        blocks_to_update,
        block_map
    )?;

    return Ok(())
}

fn move_to_block_children_to_from_block(
    from_block: &mut StandardBlock,
    mut to_block: StandardBlock,
    block_map: &mut BlockMap,
    blocks_to_update: &mut Vec<String>,
) -> Result<(), StepError> {
    update_state_tools::splice_children_on_std_block(
        from_block,
        0..0,
        to_block.children,
        blocks_to_update,
        block_map
    )?;
    from_block.set_new_parent_of_children(block_map, blocks_to_update)?;
    to_block.children = vec![];
    block_map.update_blocks(vec![Block::StandardBlock(to_block)], blocks_to_update)?;
    return Ok(())
}

// fn move_descendant_blocks_in_top_to_std_block_to_root_block(
//     to_block: &StandardBlock,
//     block_map: &BlockMap,
// ) {
//     let block_after_to_block = get_next_block_in_tree(to_block, block_map, &mut 0)?;
//     let blocks_after_selection = get_blocks_between(
//         BlockStructure::Tree,
//         block_after_to_block, // from is next block after "to block" in selection
//         to, // to is last block inside parent of "to block"
//         block_map,
//         new_ids
//     )?;
// }

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