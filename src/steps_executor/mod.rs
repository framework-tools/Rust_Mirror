use crate::blocks::Block;
use crate::blocks::inline_blocks::InlineBlockType;
use crate::blocks::standard_blocks::StandardBlock;
use crate::blocks::standard_blocks::content_block::ContentBlock;
use crate::mark::Mark;
use crate::new_ids::NewIds;
use crate::steps_generator::selection::{SubSelection, Selection};
use crate::{step::Step, blocks::BlockMap, steps_generator::StepError};
use crate::steps_executor::execute_mark_step::execute_mark_step;

use self::execute_child_steps::execute_child_steps;
use self::execute_replace_steps::execute_replace_step;
use self::execute_split_step::execute_split_step;
use crate::steps_executor::execute_parent_steps::execute_parent_steps;

pub mod execute_replace_steps;
pub mod execute_mark_step;
pub mod execute_split_step;
pub mod execute_child_steps;
pub mod execute_parent_steps;

pub struct UpdatedState {
    pub block_map: BlockMap,
    pub selection: Option<Selection>
}

impl UpdatedState {
    pub fn new(block_map: BlockMap) -> Self {
        return Self {
            block_map,
            selection: None
        }
    }
}



pub fn execute_steps(steps: Vec<Step>, block_map: BlockMap, new_ids: &mut NewIds) -> Result<UpdatedState, StepError> {
    let mut updated_state = UpdatedState::new(block_map);
    for step in steps {
        updated_state = match step {
            Step::ReplaceStep(replace_step) => execute_replace_step(replace_step, updated_state.block_map, updated_state.selection)?,
            Step::SplitStep(split_step) => execute_split_step(split_step, updated_state.block_map, new_ids)?,
            Step::AddMarkStep(mark_step) => execute_mark_step(mark_step, updated_state.block_map, true, new_ids)?, // execute_mark_step(mark_step, block_map, true, new_ids)?,
            Step::RemoveMarkStep(mark_step) => execute_mark_step(mark_step, updated_state.block_map, false, new_ids)?, // execute_mark_step(mark_step, block_map, false, new_ids)?
            Step::TurnToChild(turn_to_child_step) => execute_child_steps(updated_state.block_map, turn_to_child_step)?,
            Step::TurnToParent(turn_to_parent_step) => execute_parent_steps(updated_state.block_map, turn_to_parent_step)?
        };
    }
    return Ok(updated_state)
}

pub fn clean_block_after_transform(block: StandardBlock, mut block_map: BlockMap) -> Result<BlockMap, StepError> {
    block_map.update_block(Block::StandardBlock(block.clone()))?;
    if block.content_block()?.inline_blocks.len() > 1 {
        block_map = merge_inline_blocks_with_identical_marks(&block, block_map)?;
        let block = block_map.get_standard_block(&block.id())?; // need to get newly updated block
        block_map = remove_empty_inline_blocks(&block, block_map, &block.content_block()?.inline_blocks[0])?;
    }
    return Ok(block_map)
}

pub fn merge_inline_blocks_with_identical_marks(standard_block: &StandardBlock, mut block_map: BlockMap) -> Result<BlockMap, StepError> {
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
                block_map.update_block(Block::InlineBlock(new_inline_block))?;
                //block_map.remove_block(&id)?;

                let mut content_block = content_block.clone();
                content_block.inline_blocks.remove(i);
                let standard_block = standard_block.clone().update_block_content(content_block)?;
                block_map.update_block(Block::StandardBlock(standard_block.clone()))?;
                return merge_inline_blocks_with_identical_marks(&standard_block, block_map)
            }
        }
        previous_marks = inline_block.marks.clone();
        previous_type = Some(inline_block.content);
        i += 1;
    }
    return Ok(block_map)
}

pub fn remove_empty_inline_blocks(
    standard_block: &StandardBlock,
    mut block_map: BlockMap,
    first_block_id: &str
) -> Result<BlockMap, StepError> {
    let content_block = standard_block.content_block()?;
    let mut i = 0;
    for id in &content_block.inline_blocks {
        let inline_block = block_map.get_inline_block(&id)?;
        if inline_block.text()?.length() == 0 {
            let mut content_block = content_block.clone();
            content_block.inline_blocks.remove(i);
            let standard_block = standard_block.clone().update_block_content(content_block)?;
            block_map.update_block(Block::StandardBlock(standard_block.clone()))?;
            return remove_empty_inline_blocks(&standard_block, block_map, first_block_id)
        }
        i += 1;
    }

    // if content block is empty -> readd the first inline block
    if content_block.inline_blocks.len() == 0 {
        let updated_content_block = ContentBlock { inline_blocks: vec![first_block_id.to_string()]};
        block_map.update_block(Block::StandardBlock(standard_block.clone().update_block_content(updated_content_block)?))?;
    }
    return Ok(block_map)
}

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