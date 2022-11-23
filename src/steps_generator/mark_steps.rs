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
                if parent_block.all_blocks_have_identical_mark(&mark, from_block_index, to_block_index, block_map)? {
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
        // let from_block = block_map.get_standard_block(&from.block_id)?;
        // if all_blocks_in_selection_have_identical_marks(&mark, &from, &to, block_map)? {
        //     return Ok(vec![Step::RemoveMarkStep(MarkStep {
        //         block_id: from_block.parent,
        //         from,
        //         to,
        //         mark
        //     })])
        // } else {
        //     return Ok(vec![Step::AddMarkStep(MarkStep {
        //         block_id: from_block.parent,
        //         from,
        //         to,
        //         mark
        //     })])
        // }
}

fn all_standard_blocks_have_identical_mark(parent: &Block, mark: &Mark, block_map: &BlockMap, from: usize, to: usize) -> Result<bool, StepError> {
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

// fn all_blocks_in_selection_have_identical_mark(
//     mark: &Mark,
//     from: &SubSelection,
//     to: &SubSelection,
//     block_map: &BlockMap
// ) -> Result<bool, StepError> {
//     let from_block = block_map.get_standard_block(&from.block_id)?;
//     let from_subselection = match &from.subselection {
//         Some(subselection) => *subselection.clone(),
//         None => return Err(StepError("Subselection not found".to_string()))
//     };
//     let index_of_subselection = from_block.index_of(from_subselection.block_id)?;
//     if !from_block.all_blocks_have_identical_mark(&mark, index_of_subselection, from_block.inline_blocks_length()? - 1, block_map)? {
//         return Ok(false)
//     }
//     let to_block = block_map.get_standard_block(&to.block_id)?;
//     let to_subselection = match &to.subselection {
//         Some(subselection) => *subselection.clone(),
//         None => return Err(StepError("Subselection not found".to_string()))
//     };
//     let index_of_subselection = to_block.index_of(to_subselection.block_id)?;
//     if !to_block.all_blocks_have_identical_mark(&mark, index_of_subselection, to_block.inline_blocks_length()? - 1, block_map)? {
//         return Ok(false)
//     }
//     let parent_block = block_map.get_block(&from_block.parent)?;
//     let mut i = parent_block.index_of_child(from_block.id())?;
//     while i < parent_block.index_of_child(to_block.id())? {
//         let block = block_map.get_standard_block(&parent_block.get_child_from_index(i)?)?;
//         if !block.all_blocks_have_identical_mark(&mark, 0, block.inline_blocks_length()? - 1, block_map)? {
//             return Ok(false)
//         }
//         i += 1;
//     }
//     return Ok(true)
// }

