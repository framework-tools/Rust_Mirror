use crate::{mark::Mark, step::{Step, MarkStep}, blocks::{BlockMap, Block, standard_blocks::StandardBlock}};

use super::{selection::SubSelection, StepError};

/// If inline block / blocks
/// -> if all the blocks have an identical mark with same values -> remove mark
/// -> else -> add mark
pub fn generate_mark_steps(mark: Mark, from: SubSelection, to: SubSelection, block_map: &BlockMap) -> Result<Vec<Step>, StepError> {
        let from_block = block_map.get_block(&from.block_id)?;
        match from_block {
            Block::InlineBlock(from_block) => {
                let parent_block = block_map.get_standard_block(&from_block.parent)?;
                let from_block_index = parent_block.index_of(&from.block_id)?;
                let to_block_index = parent_block.index_of(&to.block_id)?;
                if parent_block.all_inline_blocks_in_range_have_identical_mark(&mark, from_block_index, to_block_index, block_map)? {
                    return Ok(vec![Step::RemoveMarkStep(MarkStep {
                        block_id: parent_block.id(),
                        from,
                        to,
                        mark
                    })])
                } else {
                    return Ok(vec![Step::AddMarkStep(MarkStep {
                        block_id: parent_block.id(),
                        from,
                        to,
                        mark
                    })])
                }
            },
            Block::StandardBlock(from_block) => {
                let parent = block_map.get_block(&from_block.parent)?;
                if all_standard_blocks_have_identical_mark(&parent, &mark, block_map, from.clone(), to.clone())? {
                    return Ok(vec![Step::RemoveMarkStep(MarkStep {
                        block_id: parent.id(),
                        from,
                        to,
                        mark
                    })])
                } else {
                    return Ok(vec![Step::AddMarkStep(MarkStep {
                        block_id: parent.id(),
                        from,
                        to,
                        mark
                    })])
                }
            },
            Block::Root(_) => return Err(StepError("Cannot generate mark steps for a root block".to_string()))
        }
}

/// NEED TO ADD SAME DESCENDANT CASE AFTER FINISHED HANDLING THESE CASES
/// For "from" and "to":
/// -> start at 2nd deepest layer (standard block before inline block with subselection)
/// -> for "from"
///     => in 2nd deepest layer check all inline blocks from "from" (deepest layer) to end
///     => check all blocks physically lower
/// -> for "to"
///     => in 2nd deepest layer check all inline blocks from start to "to" (deepest layer)
///     => check all ancestors & older siblings
/// For each block between from & to (top level siblings)
///     => check each block & all descendants
fn all_standard_blocks_have_identical_mark(
    highest_level_parent: &Block,
    mark: &Mark,
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection
) -> Result<bool, StepError> {
    // for "from"
    let from_second_deepest_layer = from.clone().get_two_deepest_layers()?;
    let from_deepest_layer = from.get_deepest_subselection();
    let from_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;
    if from_block.all_inline_blocks_in_range_have_identical_mark(
        mark, 
        from_block.index_of(&from_deepest_layer.block_id)?, 
        from_block.content_block()?.inline_blocks.len() - 1,
        block_map
    )? == false {
        return Ok(false)
    }
    if all_descendants_inline_blocks_have_identical_mark(&from_block, mark, block_map, None)? == false
    || all_lower_relatives_have_identical_mark(&from_block, mark, block_map)? == false {
        return Ok(false)
    }

    let to_second_deepest_layer = to.clone().get_two_deepest_layers()?;
    let to_deepest_layer = to.get_deepest_subselection();
    let to_block = block_map.get_standard_block(&to_second_deepest_layer.block_id)?;
    if to_block.all_inline_blocks_in_range_have_identical_mark(
        mark, 
        0, 
        to_block.index_of(&to_deepest_layer.block_id)?,
        block_map
    )? == false {
        return Ok(false)
    }
    if all_blocks_above_have_identical_marks(&to_block, mark, block_map)? == false {
        return Ok(false)
    }

    let from_block_index = highest_level_parent.index_of_child(&from.block_id)?;
    let to_block_index = highest_level_parent.index_of_child(&to.block_id)?;

    for block_index in  from_block_index..=to_block_index {
        let block = highest_level_parent.child(block_index)?;
        if block.all_inline_blocks_have_identical_mark(mark, block_map)? == false
        || all_descendants_inline_blocks_have_identical_mark(&block, mark, block_map, None)? == false {
            return Ok(false)
        }
    }

    return Ok(true)
}

fn all_descendants_inline_blocks_have_identical_mark(
    block: &StandardBlock, 
    mark: &Mark, 
    block_map: &BlockMap,
    stop_at_id: Option<&String> // block id
) -> Result<bool, StepError> {
    for id in &block.children {
        let child = block_map.get_standard_block(id)?;
        if child.all_inline_blocks_have_identical_mark(mark, block_map)? == false {
            return Ok(false)
        }
        for id in &child.children {
            let grandchild = block_map.get_standard_block(id)?;
            if all_descendants_inline_blocks_have_identical_mark(&grandchild, mark, block_map, stop_at_id)? == false {
                return Ok(false)
            }
        }
    }

    return Ok(true)
}

fn all_lower_relatives_have_identical_mark(
    block: &StandardBlock,
    mark: &Mark, 
    block_map: &BlockMap
) -> Result<bool, StepError> {
    let parent_as_block = block.get_parent(block_map)?;
    let parent;
    if parent_as_block.is_root() {
        return Ok(true)
    } else {
        parent = block_map.get_standard_block(&parent_as_block.id())?;
    }
    let mut i = block.index(block_map)? + 1;
    while i < parent.children.len() {
        let younger_sibling = block_map.get_standard_block(&parent.children[i])?;
        if all_descendants_inline_blocks_have_identical_mark(&younger_sibling, mark, block_map, None)? == false {
            return Ok(false)
        }
        i += 1;
    }


    if all_lower_relatives_have_identical_mark(&parent, mark, block_map)? == false {
        return Ok(false)
    } else {
        return Ok(true)
    }
}

fn all_blocks_above_have_identical_marks(
    block: &StandardBlock,
    mark: &Mark,
    block_map: &BlockMap
) -> Result<bool, StepError> {
    let parent_as_block = block.get_parent(block_map)?;
    let parent;
    if parent_as_block.is_root() {
        return Ok(true)
    } else {
        parent = block_map.get_standard_block(&parent_as_block.id())?;
    }
    let mut i = 0;
    while i < block.index(block_map)? {
        let older_sibling = block_map.get_standard_block(&parent.children[i])?;
        if all_descendants_inline_blocks_have_identical_mark(&older_sibling, mark, block_map, None)? == false {
            return Ok(false)
        }
        i += 1;
    }

    if all_blocks_above_have_identical_marks(&parent, mark, block_map)? == false {
        return Ok(false)
    } else {
        return Ok(true)
    }
}
