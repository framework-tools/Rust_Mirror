

use crate::{step::{ReplaceStep, ReplaceSlice}, blocks::{BlockMap, Block, inline_blocks::InlineBlock}, steps_generator::{StepError, selection::{Selection, SubSelection}}};

use self::{replace_for_inline_blocks::replace_selected_across_inline_blocks, replace_for_standard_blocks::replace_selected_across_standard_blocks};

use super::{UpdatedState};

pub mod replace_for_inline_blocks;
pub mod replace_for_standard_blocks;

/// Apply replace step & update to block map
/// Update each block in "update block map"
/// For each "update" we need to:
/// -> merge adjacent inline blocks with same marks (unimplemented)
/// -> delete any text blocks with no text (unimplemented)
pub fn execute_replace_step(replace_step: ReplaceStep, block_map: BlockMap) -> Result<UpdatedState, StepError> {
    let from_block = block_map.get_block(&replace_step.from.block_id)?;
    return match from_block {
        Block::InlineBlock(from_block) => replace_selected_across_inline_blocks(from_block, block_map, replace_step),
        Block::StandardBlock(from_block) => replace_selected_across_standard_blocks(from_block, block_map, replace_step),
        Block::Root(root_block) => replace_selected_across_blocks_children(
            Block::Root(root_block),
            block_map,
            replace_step.from,
            replace_step.to,
            replace_step.slice
        ),
    }
}


pub fn replace_selected_across_blocks_children(
    mut block: Block,
    mut block_map: BlockMap,
    from: SubSelection,
    to: SubSelection,
    slice: ReplaceSlice,
) -> Result<UpdatedState, StepError> {
    let blocks_to_add = match slice {
        ReplaceSlice::Blocks(blocks) => blocks,
        ReplaceSlice::String(_) => return Err(StepError("Cannot replace with string when replacing across blocks children".to_string()))
    };
    block.splice_children(from.offset, to.offset, blocks_to_add);
    let block_before_first_child_deleted_id = block.get_child_from_index(from.offset - 1)?;
    block_map.update_block(block)?;
    let updated_subselection = SubSelection::at_end_of_block(&block_before_first_child_deleted_id, &block_map)?;
    return Ok(UpdatedState { block_map, selection: Some(Selection{ from: updated_subselection.clone(), to: updated_subselection } ) })
}

// fn from_and_to_are_inline_blocks(replace_step: &ReplaceStep, block_map: &BlockMap) -> bool {
//     let from_block = block_map.get_inline_block(&replace_step.from.block_id);
//     let to_block = block_map.get_inline_block(&replace_step.to.block_id);
//     return from_block.is_ok() && to_block.is_ok()
// }

// fn execute_replace_on_standard_blocks_fully_selected(replace_step: ReplaceStep, mut block_map: BlockMap) -> Result<BlockMap, StepError> {
//     let from_standard_block = block_map.get_standard_block(&replace_step.from.block_id)?;
//     let mut parent_block = block_map.get_block(&from_standard_block.parent)?;
//     if replace_step.from.subselection.is_some() {
//         return Err(StepError("From subselection should be none for standard block".to_string()))
//     }
//     let mut children = parent_block.children()?.clone();
//     children.splice(replace_step.from.offset..replace_step.to.offset + 1, vec![]);
//     parent_block.update_children(children)?;
//     block_map.update_block(parent_block)?;
//     return Ok(block_map)
// }