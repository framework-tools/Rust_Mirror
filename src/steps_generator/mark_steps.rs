use crate::{mark::Mark, step::{Step, MarkStep}, blocks::{BlockMap, Block}, utilities::{get_blocks_between, BlockStructure, BlocksBetween}, new_ids::NewIds};

use super::{selection::SubSelection, StepError};

#[derive(PartialEq, Debug)]
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
        Block::StandardBlock(_) => {
            match get_blocks_between(BlockStructure::Flat, &from, &to, block_map, &mut NewIds::new())? {
                BlocksBetween::Flat(blocks) => {
                    let mut i = 0;
                    for block in &blocks {
                        if i == 0 {
                            if block.all_inline_blocks_in_range_have_identical_mark(
                                &mark,
                                block.index_of(&from.get_deepest_subselection().block_id)?,
                                block.content_block()?.inline_blocks.len() - 1,
                                ForSelection::From(from.get_deepest_subselection().offset),
                                block_map
                            )? == false {
                                should_add_mark = true;
                                break;
                            }
                        } else if i == blocks.len() - 1 {
                            if block.all_inline_blocks_in_range_have_identical_mark(
                                &mark,
                                0,
                                block.index_of(&to.get_deepest_subselection().block_id)?,
                                ForSelection::To(to.get_deepest_subselection().offset),
                                block_map
                            )? == false {
                                should_add_mark = true;
                                break;
                            }
                        } else {
                            if block.all_inline_blocks_have_identical_mark(&mark, block_map)? == false {
                                should_add_mark = true;
                                break;
                            }
                        }

                        i += 1;
                    }
                },
                _ => unreachable!()
            }
        },
        Block::Root(_) => return Err(StepError("Cannot generate mark steps for a root block".to_string()))
    };


    let mark_step = MarkStep { block_id: parent_block_id, from, to, mark };
    if should_add_mark {
        return Ok(vec![Step::AddMarkStep(mark_step)])
    } else {
        return Ok(vec![Step::RemoveMarkStep(mark_step)])
    }
}