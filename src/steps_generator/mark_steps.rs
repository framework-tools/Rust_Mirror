use crate::{mark::Mark, step::{Step, MarkStep}, blocks::{BlockMap, Block}};

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
                if all_standard_blocks_have_identical_mark(&parent, &mark, block_map, from.offset, to.offset)? {
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

/// For "from" and "to":
/// -> start at 2nd deepest layer (standard block before inline block with subselection)
/// -> for "from"
///     => check all inline blocks from "from" to end
///     => check all descendants inline blocks
/// -> for "to"
///     => check all inline blocks from start to "to"
///     => check all ancestors & older siblings
/// For each block between from & to (top level siblings)
///     => check each block & all descendants
fn all_standard_blocks_have_identical_mark(
    parent: &Block,
    mark: &Mark,
    block_map: &BlockMap,
    from: &SubSelection,
    to: &SubSelection
) -> Result<bool, StepError> {
    let children = parent.children()?;
    let mut i = from;
    while i < to + 1 {
        let block = block_map.get_standard_block(&children[i])?;
        let content_block = block.content_block()?;
        for inline_id in &content_block.inline_blocks {
            let inline_block = block_map.get_inline_block(&inline_id)?;
            if !inline_block.marks.contains(mark) {
                return Ok(false)
            }
        }
        i += 1;
    }
    Ok(true)
}
