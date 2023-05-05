

use crate::{step::{ReplaceStep, ReplaceSlice}, blocks::{BlockMap, Block}, steps_generator::{StepError, selection::{Selection, SubSelection}}, new_ids::{NewIds}};

use self::{replace_for_inline_blocks::replace_selected_across_inline_blocks, replace_for_standard_blocks::replace_selected_across_standard_blocks};

use super::{UpdatedState};

pub mod replace_for_inline_blocks;
pub mod replace_for_standard_blocks;

/// Apply replace step & update to block map
/// Update each block in "update block map"
/// For each "update" we need to:
/// -> merge adjacent inline blocks with same marks (unimplemented)
/// -> delete any text blocks with no text (unimplemented)
// ------------------------------------------------------
// This function appears to be used to replace a selected range of blocks with new content. 
//It takes a ReplaceStep struct which contains the range to be replaced and the new content, 
//as well as a BlockMap which is a map of all the blocks in the document and a vector of block IDs to be updated. 
//It also takes an optional current_updated_selection which is the current selection after the replacement.

// The function first gets the block specified by the from field in the ReplaceStep struct
// and determines the type of block it is. 
//It then calls one of three functions depending on the type of block: 
// - replace_selected_across_inline_blocks, 
// - replace_selected_across_standard_blocks, 
// - replace_selected_across_blocks_children. 
//These functions are responsible for replacing the selected range with the new content 
//and returning an UpdatedState struct which contains the updated BlockMap, 
//a new selection range if applicable, and vectors of block IDs to be updated and removed.

pub fn actualise_replace_step(
    replace_step: ReplaceStep,
    block_map: BlockMap,
    current_updated_selection: Option<Selection>,
    blocks_to_update: Vec<String>,
    new_ids: &mut NewIds,
) -> Result<UpdatedState, StepError> {
    let from_block = block_map.get_block(&replace_step.from.block_id)?;
    return match from_block {
        Block::InlineBlock(from_block) => replace_selected_across_inline_blocks(from_block, block_map, replace_step, blocks_to_update),
        Block::StandardBlock(from_block) => replace_selected_across_standard_blocks(from_block, block_map, replace_step, blocks_to_update, new_ids),
        Block::Root(root_block) => replace_selected_across_blocks_children(
            Block::Root(root_block),
            block_map,
            replace_step.from,
            replace_step.to,
            replace_step.slice,
            current_updated_selection,
            blocks_to_update
        ),
    }
}

// This function looks like it handles the process of replacing blocks of content 
//within a document with a given set of new blocks. 
//It does this by first splicing the children of the block specified by from.offset and to.offset
//with the new blocks specified in slice. 
//It then updates the block in the block map with the modified version of the block.

// If there is a current updated selection specified in current_updated_selection, 
//the function returns this as the new selection. 
//Otherwise, it calculates the new selection by getting the id of the block 
//before the first child was deleted and finding the end of this block. 
//It then returns this as the new selection.
pub fn replace_selected_across_blocks_children(
    mut block: Block,
    mut block_map: BlockMap,
    from: SubSelection,
    to: SubSelection,
    slice: ReplaceSlice,
    current_updated_selection: Option<Selection>,
    mut blocks_to_update: Vec<String>,
) -> Result<UpdatedState, StepError> {
    let blocks_to_add = match slice {
        ReplaceSlice::Blocks(blocks) => blocks,
        _ => return Err(StepError("Replace slice should be blocks".to_string()))
    };
    block.splice_children(from.offset, to.offset, blocks_to_add)?;
    let block_before_first_child_deleted_id = block.get_child_from_index(from.offset - 1)?;
    block_map.update_block(block, &mut blocks_to_update)?;
    if current_updated_selection.is_some() {
        return Ok(UpdatedState { block_map, selection: current_updated_selection, blocks_to_update, blocks_to_remove: vec![], copy: None })
    } else {
        let updated_subselection = SubSelection::at_end_of_block(&block_before_first_child_deleted_id, &block_map)?;
        return Ok(UpdatedState {
            block_map,
            selection: Some(Selection{ anchor: updated_subselection.clone(), head: updated_subselection }),
            blocks_to_update,
            blocks_to_remove: vec![],
            copy: None
        })
    }
}

// fn from_and_to_are_inline_blocks(replace_step: &ReplaceStep, block_map: &BlockMap) -> bool {
//     let from_block = block_map.get_inline_block(&replace_step.from.block_id);
//     let to_block = block_map.get_inline_block(&replace_step.to.block_id);
//     return from_block.is_ok() && to_block.is_ok()
// }

// fn actualise_replace_on_standard_blocks_fully_selected(replace_step: ReplaceStep, mut block_map: BlockMap) -> Result<BlockMap, StepError> {
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