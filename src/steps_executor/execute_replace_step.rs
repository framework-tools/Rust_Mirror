use mongodb::bson::oid::ObjectId;

use crate::{step::ReplaceStep, blocks::{BlockMap, Block}, steps_generator::StepError};

use super::clean_block_after_transform;

/// Apply replace step & update to block map
/// Update each block in "update block map"
/// For each "update" we need to:
/// -> merge adjacent inline blocks with same marks (unimplemented)
/// -> delete any text blocks with no text (unimplemented)
pub fn execute_replace_step(replace_step: ReplaceStep, mut block_map: BlockMap) -> Result<BlockMap, StepError> {
    let block = block_map.get_block(&replace_step.block_id)?;
    match block {
        Block::InlineBlock(_) => {
            return Err(StepError("Cannot execute replace step on inline block".to_string()))
        },
        Block::StandardBlock(standard_block) => {
            if from_and_to_are_inline_blocks(&replace_step, &block_map) {
                let mut content = standard_block.content_block()?.clone();
                // replace content's inline blocks from "from" offset to "to" offset with slice
                content.inline_blocks.splice(replace_step.from.offset..replace_step.to.offset, replace_step.slice);
                let updated_standard_block = standard_block.update_block_content(content)?;
                let updated_block = Block::StandardBlock(updated_standard_block);
                block_map.update_block(updated_block)?;
                let standard_block = block_map.get_standard_block(&replace_step.block_id)?;
                block_map = clean_block_after_transform(&standard_block, block_map)?;
            } else {
                return execute_replace_on_standard_blocks_fully_selected(replace_step, block_map)
                // let standard_block = execute_replace_on_blocks_children(
                //     Block::StandardBlock(standard_block),
                //     replace_step.from.offset,
                //     replace_step.to.offset,
                //     replace_step.slice,
                // )?;
                // block_map.update_block(standard_block)?;
                // let standard_block = block_map.get_standard_block(&replace_step.block_id)?;
                // block_map = clean_block_after_transform(&standard_block, block_map)?;
            }
        },
        Block::Root(root_block) => {
            return execute_replace_on_standard_blocks_fully_selected(replace_step, block_map)
            // let root_block = execute_replace_on_blocks_children(
            //     Block::Root(root_block),
            //     replace_step.from.offset,
            //     replace_step.to.offset,
            //     replace_step.slice,
            // )?;
            // block_map.update_block(root_block)?;
        },
    };

    for block in replace_step.blocks_to_update {
        match &block {
            Block::StandardBlock(std_block) => {
                block_map = clean_block_after_transform(std_block, block_map)?;
            },
            _ => {}
        };
        block_map.update_block(block)?;
    }
    return Ok(block_map)
}

// fn execute_replace_on_blocks_children(mut block: Block, from_index: usize, to_index: usize, slice: Vec<ObjectId>) -> Result<Block, StepError> {
//     block.splice_children(from_index, to_index, slice)?;
//     return Ok(block)
// }

fn from_and_to_are_inline_blocks(replace_step: &ReplaceStep, block_map: &BlockMap) -> bool {
    let from_block = block_map.get_inline_block(&replace_step.from.block_id);
    let to_block = block_map.get_inline_block(&replace_step.to.block_id);
    return from_block.is_ok() && to_block.is_ok()
}

fn execute_replace_on_standard_blocks_fully_selected(replace_step: ReplaceStep, mut block_map: BlockMap) -> Result<BlockMap, StepError> {
    let from_standard_block = block_map.get_standard_block(&replace_step.from.block_id)?;
    let mut parent_block = block_map.get_block(&from_standard_block.parent)?;
    if replace_step.from.subselection.is_some() {
        return Err(StepError("From subselection should be none for standard block".to_string()))
    }
    let mut children = parent_block.children()?.clone();
    children.splice(replace_step.from.offset..replace_step.to.offset + 1, vec![]);
    parent_block.update_children(children)?;
    block_map.update_block(parent_block)?;
    return Ok(block_map)
}