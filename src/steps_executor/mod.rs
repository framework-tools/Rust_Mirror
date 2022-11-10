use crate::blocks::Block;
use crate::blocks::inline_blocks::InlineBlockType;
use crate::blocks::standard_blocks::StandardBlock;
use crate::mark::Mark;
use crate::new_ids::NewIds;
use crate::steps_generator::selection::{SubSelection};
use crate::{step::Step, blocks::BlockMap, steps_generator::StepError};

pub mod execute_replace_step;
pub mod execute_mark_step;

use crate::steps_executor::execute_replace_step::execute_replace_step;
use crate::steps_executor::execute_mark_step::execute_mark_step;

pub fn execute_steps(steps: Vec<Step>, mut block_map: BlockMap, new_ids: &mut NewIds) -> Result<BlockMap, StepError> {
    for step in steps {
        block_map = match step {
            Step::ReplaceStep(replace_step) => execute_replace_step(replace_step, block_map)?,
            Step::AddMarkStep(mark_step) => execute_mark_step(mark_step, block_map, true, new_ids)?,
            Step::RemoveMarkStep(mark_step) => execute_mark_step(mark_step, block_map, false, new_ids)?
        };
    }
    return Ok(block_map)
}

pub fn clean_block_after_transform(block: &StandardBlock, mut block_map: BlockMap) -> Result<BlockMap, StepError> {
    if block.content_block()?.inline_blocks.len() > 1 {
        block_map = merge_inline_blocks_with_identical_marks(block, block_map)?;
        let block = block_map.get_standard_block(&block.id())?; // need to get newly updated block
        block_map = remove_empty_inline_blocks(&block, block_map)?;
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
                block_map.remove_block(&id)?;

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
) -> Result<BlockMap, StepError> {
    let content_block = standard_block.content_block()?;
    let mut i = 0;
    for id in &content_block.inline_blocks {
        let inline_block = block_map.get_inline_block(&id)?;
        if inline_block.text()?.is_empty() {
            let mut content_block = content_block.clone();
            block_map.remove_block(id)?;
            content_block.inline_blocks.remove(i);
            let standard_block = standard_block.clone().update_block_content(content_block)?;
            block_map.update_block(Block::StandardBlock(standard_block.clone()))?;
            return remove_empty_inline_blocks(&standard_block, block_map)
        }
        i += 1;
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