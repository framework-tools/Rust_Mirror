use crate::{mark::Mark, step::{Step, MarkStep}, blocks::{BlockMap, Block, standard_blocks::StandardBlock}};

use super::{selection::SubSelection, StepError};

#[derive(PartialEq)]
pub enum ForSelection {
    From(usize),
    To(usize),
    Both(usize, usize)
}

/// If inline block / blocks
/// -> if all the blocks have an identical mark with same values -> remove mark
/// -> else -> add mark
pub fn generate_mark_steps(mark: Mark, from: SubSelection, to: SubSelection, block_map: &BlockMap) -> Result<Vec<Step>, StepError> {
    let from_block = block_map.get_block(&from.block_id)?;
    let parent_block_id = from_block.parent()?;
    let mut should_add_mark = false;
    match from_block {
        Block::InlineBlock(from_block) => {
            let parent_block = block_map.get_standard_block(&from_block.parent)?;
            if parent_block.all_inline_blocks_in_range_have_identical_mark(
                &mark,
                parent_block.index_of(&from.block_id)?,
                parent_block.index_of(&to.block_id)?,
                ForSelection::Both(from.offset, to.offset),
                block_map
            )? == false {
                should_add_mark = true;
            }
        },
        Block::StandardBlock(from_block) => {
            let parent = block_map.get_block(&from_block.parent)?;
            if all_standard_blocks_have_identical_mark(&parent, &mark, block_map, from.clone(), to.clone())? == false {
                should_add_mark = true;
            }
        },
        Block::Root(_) => return Err(StepError("Cannot generate mark steps for a root block".to_string()))
    };

    // from.adjust_deepest_subselection_for_marks(true, block_map)?;
    // to.adjust_deepest_subselection_for_marks(false, block_map)?;

    let mark_step = MarkStep { block_id: parent_block_id, from, to, mark };
    if should_add_mark {
        return Ok(vec![Step::AddMarkStep(mark_step)])
    } else {
        return Ok(vec![Step::RemoveMarkStep(mark_step)])
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
    let top_from_block = block_map.get_standard_block(&from.block_id)?;
    if !top_from_block.parent_is_root(block_map) || from.block_id == to.block_id {
        let from_second_deepest_layer = from.clone().get_two_deepest_layers()?;
        let from_deepest_layer = from.get_deepest_subselection();
        let from_deepest_std_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;
        let to_second_deepest_layer = to.clone().get_two_deepest_layers()?;
        let to_deepest_layer = to.get_deepest_subselection();
        let to_deepest_std_block = block_map.get_standard_block(&to_second_deepest_layer.block_id)?;
        if from_deepest_std_block.all_inline_blocks_in_range_have_identical_mark(
            mark,
            from_deepest_std_block.index_of(&from_deepest_layer.block_id)?,
            from_deepest_std_block.content_block()?.inline_blocks.len() - 1,
            ForSelection::From(from_deepest_layer.offset),
            block_map
        )? == false
        || descendants_inline_blocks_have_identical_mark(
            &from_deepest_std_block,
            mark,
            block_map,
            Some((&to_deepest_layer.block_id, to_deepest_std_block.index_of(&to_deepest_layer.block_id)?, to_deepest_layer.offset))
        )? == false {
            return Ok(false)
        }

        if top_from_block.parent_is_root(block_map) {
            let parent_block = block_map.get_standard_block(&top_from_block.parent)?;
            let from_index = parent_block.index_of_child(&from.block_id)?;
            let to_index = parent_block.index_of_child(&to.block_id)?;
            for block_id in parent_block.children[from_index + 1..=to_index].iter() {
                let child = block_map.get_standard_block(block_id)?;
                if child.id() == to_deepest_std_block.id() {
                    if child.all_inline_blocks_in_range_have_identical_mark(
                        mark,
                        0,
                        to_deepest_std_block.index_of(&to_deepest_layer.block_id)?,
                        ForSelection::To(to_deepest_layer.offset),
                        block_map
                    )? == false {
                        return Ok(false)
                    };
                    break;
                } else {
                    if child.all_inline_blocks_have_identical_mark(mark, block_map)? == false
                    || descendants_inline_blocks_have_identical_mark(
                        &child,
                        mark,
                        block_map,
                        Some((&to_deepest_layer.block_id, to_deepest_std_block.index_of(&to_deepest_layer.block_id)?, to_deepest_layer.offset))
                    )? == false {
                        return Ok(false)
                    }
                }
            }
        }

    } else {
        // for "from"
        let from_second_deepest_layer = from.clone().get_two_deepest_layers()?;
        let from_deepest_layer = from.get_deepest_subselection();
        let from_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;
        if from_block.all_inline_blocks_in_range_have_identical_mark(
            mark,
            from_block.index_of(&from_deepest_layer.block_id)?,
            from_block.content_block()?.inline_blocks.len() - 1,
            ForSelection::From(from_deepest_layer.offset),
            block_map
        )? == false {
            return Ok(false)
        }
        if descendants_inline_blocks_have_identical_mark(&from_block, mark, block_map, None)? == false
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
            ForSelection::To(to_deepest_layer.offset),
            block_map
        )? == false {
            return Ok(false)
        }

        if &to_block._id != &to.block_id {
            let to_block_highest = block_map.get_standard_block(&to.block_id)?;
            if to_block_highest.all_inline_blocks_have_identical_mark(mark, block_map)? == false
            || descendants_inline_blocks_have_identical_mark(
                &to_block_highest,
                mark,
                block_map,
                Some((&to_block._id, to_block.index_of(&to_deepest_layer.block_id)?, to_deepest_layer.offset))
            )? == false {
                return Ok(false)
            }
        }

        let to_block_index = highest_level_parent.index_of_child(&to.block_id)?;

        let from_block_index = highest_level_parent.index_of_child(&from.block_id)?;
        let highest_parent_children = highest_level_parent.children()?;

        for id in highest_parent_children[from_block_index + 1..to_block_index].iter() {
            let block = block_map.get_standard_block(id)?;
            if block.all_inline_blocks_have_identical_mark(mark, block_map)? == false
            || descendants_inline_blocks_have_identical_mark(&block, mark, block_map, None)? == false {
                return Ok(false)
            }
        }

    }

    return Ok(true)
}

/// This function is recursive
fn descendants_inline_blocks_have_identical_mark(
    block: &StandardBlock,
    mark: &Mark,
    block_map: &BlockMap,
    to_block_to_stop_at: Option<(&String, usize, usize)> // (block id, end of to selection index (of inline block), offset)
) -> Result<bool, StepError> {
    for id in &block.children {
        let child = block_map.get_standard_block(id)?;

        match to_block_to_stop_at {
            Some(to_block_to_stop_at) => {
                if id == to_block_to_stop_at.0 {
                    if child.all_inline_blocks_in_range_have_identical_mark(
                        mark,
                        0,
                        to_block_to_stop_at.1,
                        ForSelection::To(to_block_to_stop_at.2),
                        block_map
                    )? == false {
                        return Ok(false)
                    } else {
                        return Ok(true)
                    }
                } else if child.all_inline_blocks_have_identical_mark(mark, block_map)? == false
                || descendants_inline_blocks_have_identical_mark(&child, mark, block_map, Some(to_block_to_stop_at))? == false {
                    return Ok(false)
                }
            },
            None if child.all_inline_blocks_have_identical_mark(mark, block_map)? == false
            || descendants_inline_blocks_have_identical_mark(&child, mark, block_map, to_block_to_stop_at)? == false => return Ok(false),
            _ => {}
        };
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
        if descendants_inline_blocks_have_identical_mark(&younger_sibling, mark, block_map, None)? == false {
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
