
use crate::{step::{MarkStep}, blocks::{BlockMap, Block, inline_blocks::{InlineBlock}}, steps_generator::{StepError, selection::{Selection, SubSelection}}, new_ids::NewIds};

use self::actualise_across_std_blocks::actualise_mark_step_on_standard_blocks;

use super::{clean_block_after_transform, UpdatedState};

pub mod actualise_across_std_blocks;


// This function appears to be used to apply a "mark" to a range of blocks in a document. 
//It takes a MarkStep struct as input, 
//which represents the mark to be applied and the range of blocks to which it should be applied. 
//It also takes a BlockMap which seems to be a map of block IDs to blocks, 
//a boolean add_mark which indicates whether the mark should be added or removed, 
//a mutable reference to a NewIds struct which seems to be used to generate new unique IDs for blocks as needed, 
//and a mutable vector of strings called blocks_to_update, 
//which appears to be used to track which blocks need to be updated in the document.
// The function first converts the from and to fields of the MarkStep struct 
//to "raw selections" using the to_raw_selection function. 
//It then retrieves the block specified in the from field of the MarkStep struct, 
//and dispatches on the type of the block. 
//If the block is an inline block, 
//it calls the actualise_mark_step_on_inline_blocks function to apply the mark to the inline block. 
//If the block is a standard block, 
//it calls the actualise_mark_step_on_standard_blocks function to apply the mark to a range of standard blocks. 
//If the block is the root block, it returns an error.
//The function converts the raw selections back to regular selections
// and returns an UpdatedState struct containing the updated BlockMap
// and a selection range from the modified from and to blocks.
pub fn actualise_mark_step(
    mark_step: MarkStep,
    mut block_map: BlockMap,
    add_mark: bool,
    new_ids: &mut NewIds,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let from_raw_selection = mark_step.from.to_raw_selection(&block_map)?;
    let to_raw_selection = mark_step.to.to_raw_selection(&block_map)?;

    let block = block_map.get_block(&mark_step.from.block_id)?;
    match block {
        Block::InlineBlock(from_block) => {
            let updated_state = actualise_mark_step_on_inline_blocks(mark_step, from_block, block_map, add_mark, new_ids, blocks_to_update)?;
            block_map = updated_state.block_map;
            blocks_to_update = updated_state.blocks_to_update;
        },
        Block::StandardBlock(_) => {
            let updated_state = actualise_mark_step_on_standard_blocks(mark_step, block_map, add_mark, new_ids, blocks_to_update)?;
            block_map = updated_state.block_map;
            blocks_to_update = updated_state.blocks_to_update;
        },
        Block::Root(_) => return Err(StepError("Cannot mark root block".to_string()))
    };

    let selection = Some(Selection {
        anchor: from_raw_selection.real_selection_from_raw(&block_map)?,
        head: to_raw_selection.real_selection_from_raw(&block_map)?
    });
    return Ok(UpdatedState { block_map, selection, blocks_to_update, blocks_to_remove: vec![], copy: None })
}

// This function appears to be used to apply a "mark" to a range of inline blocks in a document. 
//It takes a MarkStep struct as input, 
//which represents the mark to be applied and the range of blocks to which it should be applied, 
//an InlineBlock called from_block, a BlockMap, a boolean add_mark which indicates 
//whether the mark should be added or removed, 
//a mutable reference to a NewIds struct which seems to be used to generate new unique IDs for blocks as needed, 
//and a mutable vector of strings called blocks_to_update, 
//which appears to be used to track which blocks need to be updated in the document.

// The function first checks if the from and to fields of the MarkStep struct refer to the same block. 
//If they do, it calls the create_before_middle_after_blocks_with_new_text_and_mark function 
//to create three new blocks: 
// - one before the range to be marked, 
// - one containing the range to be marked, 
// - one after the range to be marked. 
//It then updates the parent block to include the new blocks, 
//and returns an UpdatedState struct containing the updated BlockMap and 
//a selection range covering the newly marked block.

// If the from and to fields refer to different blocks, 
//the function splits the from and to blocks at the specified offsets, 
//applies the mark to the appropriate blocks, and then updates the parent blocks to include the new blocks. 
//It then returns an UpdatedState struct containing the updated BlockMap
//and a selection range covering the marked blocks.
fn actualise_mark_step_on_inline_blocks(
    mark_step: MarkStep,
    from_block: InlineBlock,
    mut block_map: BlockMap,
    add_mark: bool,
    new_ids: &mut NewIds,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    if mark_step.from.block_id == mark_step.to.block_id {
        let (before_block, middle_block, after_block) = create_before_middle_after_blocks_with_new_text_and_mark(from_block, new_ids, mark_step, add_mark)?;

        let parent_block = block_map.get_standard_block(&before_block.parent)?;
        let original_block_index = parent_block.index_of(&before_block._id)?;

        let mut content_block = parent_block.content_block()?.clone();
        content_block.inline_blocks.splice(
            original_block_index..original_block_index+1,
            vec![before_block.id(), middle_block.id(), after_block.id()]
        );
        let updated_parent_block = parent_block.update_block_content(content_block)?;

        let new_selection = Selection {
            anchor: SubSelection { block_id: middle_block.id(), offset: 0, subselection: None },
            head: SubSelection { block_id: middle_block.id(), offset: middle_block.text()?.len(), subselection: None },
        };
        block_map.update_blocks(vec![
            Block::StandardBlock(updated_parent_block.clone()), Block::InlineBlock(before_block), Block::InlineBlock(middle_block), Block::InlineBlock(after_block),
        ], &mut blocks_to_update)?;
        block_map = clean_block_after_transform(updated_parent_block, block_map, &mut blocks_to_update)?;
        return Ok(UpdatedState { block_map, selection: Some(new_selection), blocks_to_update, blocks_to_remove: vec![], copy: None })
    } else {
        //split from block
        //split to block
        //apply marks to split blocks within the selection & all the blocks in between
        //replace in parent block with new blocks
        //update every block that was changed
        let from_parent_block = block_map.get_standard_block(&from_block.parent)?;
        let to_block = block_map.get_inline_block(&mark_step.to.block_id)?;

        let (first_half_of_from_block, second_half_of_from_block) = from_block.split(mark_step.from.offset, new_ids)?;
        let first_half_of_from_block_id = first_half_of_from_block.id();
        let second_half_of_from_block_id = second_half_of_from_block.id();
        let (first_half_of_to_block, second_half_of_to_block) = to_block.split(mark_step.to.offset, new_ids)?;
        let first_half_of_to_block_id = first_half_of_to_block.id();
        let second_half_of_to_block_id = second_half_of_to_block.id();

        let second_half_of_from_block = second_half_of_from_block.apply_mark(mark_step.mark.clone(), add_mark);
        let first_half_of_to_block = first_half_of_to_block.apply_mark(mark_step.mark.clone(), add_mark);

        let new_subselection = updated_selection_after_apply_mark(second_half_of_from_block_id.clone(), &first_half_of_to_block)?;

        block_map.update_blocks(vec![
            Block::InlineBlock(first_half_of_from_block), Block::InlineBlock(second_half_of_from_block),
            Block::InlineBlock(first_half_of_to_block), Block::InlineBlock(second_half_of_to_block)
        ], &mut blocks_to_update)?;

        let mut content_block = from_parent_block.content_block()?.clone();
        // for_each_block_between_from_and_to_apply_mark
        let mut i = content_block.index_of(&first_half_of_from_block_id)? + 1;
        let j = content_block.index_of(&first_half_of_to_block_id)?;
        while i < j {
            let block = block_map.get_inline_block(&content_block.inline_blocks[i])?;
            let block = block.apply_mark(mark_step.mark.clone(), add_mark);
            block_map.update_block(Block::InlineBlock(block), &mut blocks_to_update)?;
            i += 1;
        }

        // splice for "from" block
        content_block.inline_blocks.splice(
            content_block.index_of(&first_half_of_from_block_id)?..content_block.index_of(&first_half_of_from_block_id)?+1,
            vec![first_half_of_from_block_id, second_half_of_from_block_id]
        );

        //splice for "to" block
        content_block.inline_blocks.splice(
            content_block.index_of(&first_half_of_to_block_id)?..content_block.index_of(&first_half_of_to_block_id)?+1,
            vec![first_half_of_to_block_id, second_half_of_to_block_id]
        );
        let updated_parent_block = from_parent_block.update_block_content(content_block)?;
        block_map.update_block(Block::StandardBlock(updated_parent_block.clone()), &mut blocks_to_update)?;
        block_map = clean_block_after_transform(updated_parent_block, block_map, &mut blocks_to_update)?;

        return Ok(UpdatedState { block_map, selection: Some(new_subselection), blocks_to_update, blocks_to_remove: vec![], copy: None })
    }
}

//This function appears to be used to create three new inline blocks based on a given input block. 
//It takes an InlineBlock called from_block, 
//a mutable reference to a NewIds struct which seems to be used to generate new unique IDs for blocks as needed, 
//a MarkStep struct which represents the mark to be applied and the range of blocks to which it should be applied, 
//and a boolean add_mark which indicates whether the mark should be added or removed.

// The function first retrieves the text of the input block, 
//and then calls the split_before_middle_after method on the text to split it into three parts: 
// - one before the range to be marked, 
// - one containing the range to be marked, 
// - one after the range to be marked. 
//t then creates three new inline blocks based on the input block, 
//using the update_text method to set the text of each block to the corresponding part of the split text. 
//It applies the mark specified in the MarkStep struct to the middle block, 
//and then returns the three blocks as a tuple.

pub fn create_before_middle_after_blocks_with_new_text_and_mark(
    from_block: InlineBlock,
    new_ids: &mut NewIds,
    mark_step: MarkStep,
    add_mark: bool
) -> Result<(InlineBlock, InlineBlock, InlineBlock), StepError> {
    let text = from_block.text()?.clone();
    let (before_text, middle_text, after_text) = text.split_before_middle_after(mark_step.from.offset, mark_step.to.offset);

    let before_block = from_block.clone().update_text(before_text)?;
    let mut middle_block = from_block.clone().to_new_block(new_ids)?.update_text(middle_text)?;
    let after_block = from_block.to_new_block(new_ids)?.update_text(after_text)?;

    middle_block = middle_block.apply_mark(mark_step.mark, add_mark);
    return Ok((before_block, middle_block, after_block))
}

// This function appears to be used to create a selection range based on the IDs and lengths of two inline blocks. 
//It takes the ID of the first block as a string and a reference to the second block. 
//It then returns a Selection struct with the anchor set
// - to the start of the first block and 
// - the head set to the end of the second block.
pub fn updated_selection_after_apply_mark(
    second_half_of_from_block_id: String,
    first_half_of_to_block: &InlineBlock,
) -> Result<Selection, StepError> {
    return Ok(Selection {
        anchor: SubSelection { block_id: second_half_of_from_block_id, offset: 0, subselection: None },
        head: SubSelection { block_id: first_half_of_to_block.id(), offset: first_half_of_to_block.text()?.len(), subselection: None },
    })
}
