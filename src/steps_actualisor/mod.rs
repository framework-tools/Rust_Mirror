use crate::blocks::Block;
use crate::blocks::inline_blocks::InlineBlockType;
use crate::blocks::standard_blocks::StandardBlock;
use crate::blocks::standard_blocks::content_block::ContentBlock;
use crate::custom_copy::CustomCopy;
use crate::mark::Mark;
use crate::new_ids::NewIds;
use crate::steps_generator::selection::{Selection};
use crate::{step::Step, blocks::BlockMap, steps_generator::StepError};
use crate::steps_actualisor::actualise_mark_steps::actualise_mark_step;

use self::actualise_delete_block::actualise_delete_block;
use self::actualise_drop_block::actualise_drop_block;
use self::actualise_replace_with_children::actualise_replace_with_children;
use self::actualise_shortcuts::{actualise_copy, actualise_paste};
use self::actualise_toggle_completed::actualise_toggle_completed;
use self::actualise_turn_into::actualise_turn_into_step;
use self::actualise_add_block::actualise_add_block;
use self::actualise_child_steps::actualise_child_steps;
use self::actualise_replace_steps::actualise_replace_step;
use self::actualise_split_step::actualise_split_step;
use crate::steps_actualisor::actualise_parent_steps::actualise_parent_steps;
use crate::steps_actualisor::actualise_add_paragraph_at_bottom::actualise_add_paragraph_at_bottom;

pub mod actualise_replace_steps;
pub mod actualise_mark_steps;
pub mod actualise_split_step;
pub mod actualise_child_steps;
pub mod actualise_parent_steps;
pub mod actualise_add_block;
pub mod actualise_turn_into;
pub mod actualise_toggle_completed;
pub mod actualise_shortcuts;
pub mod actualise_drop_block;
pub mod actualise_delete_block;
pub mod actualise_duplicate;
pub mod actualise_replace_with_children;
pub mod actualise_add_paragraph_at_bottom;

pub struct UpdatedState {
    pub block_map: BlockMap,
    pub selection: Option<Selection>,
    pub blocks_to_update: Vec<String>, // Vec<ID>
    pub blocks_to_remove: Vec<String>, // Vec<ID>
    pub copy: Option<CustomCopy>
}

impl UpdatedState {
    pub fn new(block_map: BlockMap) -> Self {
        return Self {
            block_map,
            selection: None,
            blocks_to_update: vec![],
            blocks_to_remove: vec![],
            copy: None
        }
    }
}


// actualise_steps is a function that takes a vector of Steps
//and updates a BlockMap according to those steps.
//It returns an UpdatedState struct which contains the updated BlockMap and other related information.

// For each step in the input vector,
//the function matches on the type of step and calls the corresponding function to perform the update.
//The possible step types are:

// Step::ReplaceStep(replace_step): calls actualise_replace_step to replace text in an inline block with new text.
// Step::SplitStep(split_step): calls actualise_split_step to split an inline block at a given offset and insert a new standard block in between.
// Step::AddMarkStep(mark_step): calls actualise_mark_step to add a mark to an inline block.
// Step::RemoveMarkStep(mark_step): calls actualise_mark_step to remove a mark from an inline block.
// Step::TurnToChild(turn_to_child_step): calls actualise_child_steps to turn a standard block into a child of another block.
// Step::TurnToParent(turn_to_parent_step): calls actualise_parent_steps to turn a standard block into a parent of another block.
// Step::AddBlock(add_block_step): calls actualise_add_block to insert a new standard block at a given position.
// Step::TurnInto(turn_into_step): calls actualise_turn_into_step to change the type of a standard block.
// Step::ToggleCompleted(_id): calls actualise_toggle_completed to toggle the "completed" state of a to-do list block.
// Step::Copy(from, to) and Step::Paste(from, to): currently not implemented.

// Finally, the function calls clean_block_after_transform to clean up the block map
//after all the updates have been performed.
//This function merges inline blocks with
// - identical marks,
// - removes empty inline blocks,
// - updates the block map with the cleaned blocks
pub fn actualise_steps(steps: Vec<Step>, block_map: BlockMap, new_ids: &mut NewIds, mut copy: CustomCopy) -> Result<UpdatedState, StepError> {
    let mut updated_state = UpdatedState::new(block_map);
    for step in steps {
        updated_state = match step {
            Step::ReplaceStep(replace_step) => actualise_replace_step(replace_step, updated_state.block_map, updated_state.selection, updated_state.blocks_to_update, new_ids)?,
            Step::SplitStep(split_step) => actualise_split_step(split_step, updated_state.block_map, new_ids, updated_state.blocks_to_update)?,
            Step::AddMarkStep(mark_step) => actualise_mark_step(mark_step, updated_state.block_map, true, new_ids, updated_state.blocks_to_update)?,
            Step::RemoveMarkStep(mark_step) => actualise_mark_step(mark_step, updated_state.block_map, false, new_ids, updated_state.blocks_to_update)?,
            Step::TurnToChild(turn_to_child_step) => actualise_child_steps(updated_state.block_map, turn_to_child_step, updated_state.blocks_to_update)?,
            Step::TurnToParent(turn_to_parent_step) => actualise_parent_steps(updated_state.block_map, turn_to_parent_step, updated_state.blocks_to_update)?,
            Step::AddBlock(add_block_step) => actualise_add_block(add_block_step, updated_state.block_map, new_ids, updated_state.blocks_to_update)?,
            Step::TurnInto(turn_into_step) => actualise_turn_into_step(turn_into_step, updated_state.block_map, updated_state.blocks_to_update)?,
            Step::ToggleCompleted(_id) => actualise_toggle_completed(_id, updated_state.block_map, updated_state.blocks_to_update)?,
            Step::Copy(from, to) => {
                updated_state = actualise_copy(copy, from, to, updated_state.block_map, new_ids, updated_state.blocks_to_update)?;
                copy = updated_state.copy.unwrap();
                updated_state.copy = None;
                updated_state
            },
            Step::Paste(from, _to) => actualise_paste(copy.clone(), from, updated_state.block_map, new_ids, updated_state.blocks_to_update)?,
            Step::DropBlock(drop_block_event) => actualise_drop_block(drop_block_event, updated_state.block_map, updated_state.blocks_to_update, new_ids)?,
            Step::DeleteBlock(block_id) => actualise_delete_block(block_id, updated_state.block_map, updated_state.blocks_to_update)?,
            Step::Duplicate(block_id) => actualise_duplicate(block_id, updated_state.block_map, new_ids, updated_state.blocks_to_update)?,
            Step::ReplaceWithChildren(replace_with_children_event) => actualise_replace_with_children(replace_with_children_event, updated_state.block_map, updated_state.blocks_to_update)?,
            Step::AddParagraphAtBottom(root_block_id) => actualise_add_paragraph_at_bottom(root_block_id, updated_state.block_map, new_ids, updated_state.blocks_to_update)?,
        };
    }
    updated_state.copy = Some(copy);
    return Ok(updated_state)
}

// This function is used to clean up a StandardBlock after
//it has undergone some kind of transformation.
//It does this by first updating the given block in the block_map,
//then checking if there are more than one InlineBlocks within the StandardBlock.
//If so, it merges any InlineBlocks with identical marks and types, and removes any empty InlineBlocks.
// It returns the updated block_map or an error if something goes wrong during the process.
pub fn clean_block_after_transform(block: StandardBlock, mut block_map: BlockMap, blocks_to_update: &mut Vec<String>) -> Result<BlockMap, StepError> {
    block_map.update_block(Block::StandardBlock(block.clone()), blocks_to_update)?;
    if block.content_block()?.inline_blocks.len() > 1 {
        block_map = merge_inline_blocks_with_identical_marks(&block, block_map, blocks_to_update)?;
        let block = block_map.get_standard_block(&block.id())?; // need to get newly updated block
        block_map = remove_empty_inline_blocks(
            &block,
            block_map,
            &block.content_block()?.inline_blocks[0],
            blocks_to_update
        )?;
    }
    return Ok(block_map)
}

// This function appears to be merging two adjacent inline blocks
//that have the same marks and type.
//The function loops through each inline block in the StandardBlock's ContentBlock
//and checks if the previous inline block has the same marks and type.
//If they do, the previous block is merged with the current block,
//the current block is removed from the ContentBlock,
//and the function is called recursively to check for further merges.
//If the blocks do not have the same marks and type,
//the current block is set as the previous block for the next iteration.
//If no blocks are merged, the function simply returns the original BlockMap.

// It's worth noting that the function returns a Result with an Err variant of StepError
//in case any errors occur during the block update process.

pub fn merge_inline_blocks_with_identical_marks(
    standard_block: &StandardBlock,
    mut block_map: BlockMap,
    blocks_to_update: &mut Vec<String>
) -> Result<BlockMap, StepError> {
    let content_block = standard_block.content_block()?;
    let mut previous_marks: Vec<Mark> = vec![];
    let mut previous_type: Option<InlineBlockType> = None; // CAN only be None for first round
    let mut i = 0;
    for id in &content_block.inline_blocks {
        let inline_block = block_map.get_inline_block(id)?;
        if previous_type.is_some() {
            let previous_type = previous_type.unwrap();
            if all_marks_are_identical(&inline_block.marks, &previous_marks) && inline_block.is_same_type(&previous_type) {
                let previous_inline_block = block_map.get_inline_block(&content_block.inline_blocks[i - 1])?;
                let new_inline_block = previous_inline_block.merge(inline_block)?;
                block_map.update_block(Block::InlineBlock(new_inline_block), blocks_to_update)?;

                let mut content_block = content_block.clone();
                content_block.inline_blocks.remove(i);
                let standard_block = standard_block.clone().update_block_content(content_block)?;
                block_map.update_block(Block::StandardBlock(standard_block.clone()), blocks_to_update)?;
                return merge_inline_blocks_with_identical_marks(&standard_block, block_map, blocks_to_update)
            }
        }
        previous_marks = inline_block.marks.clone();
        previous_type = Some(inline_block.content);
        i += 1;
    }
    return Ok(block_map)
}

// This function appears to be part of a series of functions that are used to clean up a block
//after it has been transformed in some way.

// remove_empty_inline_blocks takes
// - a StandardBlock,
// - a BlockMap,
// - the id of the first inline block in the StandardBlock,
// - a mutable vector of Strings called blocks_to_update as arguments,
//and returns a Result containing a BlockMap.

// The function first retrieves the ContentBlock contained in the StandardBlock,
//and then iterates over the inline_blocks contained in it.
//For each inline block, it checks if the text contained in it is empty.
//If it is, the inline block is removed from the ContentBlock,
//and the StandardBlock is updated with the new ContentBlock.
//If the ContentBlock becomes empty as a result,
//the first inline block is added back to the ContentBlock,
//and the StandardBlock is updated with the modified ContentBlock.
//Finally, the updated BlockMap is returned.
pub fn remove_empty_inline_blocks(
    standard_block: &StandardBlock,
    mut block_map: BlockMap,
    first_block_id: &str,
    blocks_to_update: &mut Vec<String>
) -> Result<BlockMap, StepError> {
    let content_block = standard_block.content_block()?;
    let mut i = 0;
    for id in &content_block.inline_blocks {
        let inline_block = block_map.get_inline_block(&id)?;
        if inline_block.text()?.len() == 0 {
            let mut content_block = content_block.clone();
            content_block.inline_blocks.remove(i);
            let standard_block = standard_block.clone().update_block_content(content_block)?;
            block_map.update_block(Block::StandardBlock(standard_block.clone()), blocks_to_update)?;
            return remove_empty_inline_blocks(&standard_block, block_map, first_block_id, blocks_to_update)
        }
        i += 1;
    }

    // if content block is empty -> readd the first inline block
    if content_block.inline_blocks.len() == 0 {
        let updated_content_block = ContentBlock { inline_blocks: vec![first_block_id.to_string()]};
        block_map.update_block(Block::StandardBlock(standard_block.clone().update_block_content(updated_content_block)?), blocks_to_update)?;
    }
    return Ok(block_map)
}

// This function checks whether the two input lists of Marks are identical.
//It does this by first checking that the lists have the same number of elements,
//and then checking that each element in marks is contained in the other_marks.
//If either of these conditions is not met, the function returns false.
//Otherwise, it returns true.

pub fn all_marks_are_identical(marks: &Vec<Mark>, other_marks: &Vec<Mark>) -> bool {
    if marks.len() != other_marks.len() {
        return false
    }
    for mark in marks {
        if !other_marks.contains(mark) {
            return false
        }
    }
    return true
}