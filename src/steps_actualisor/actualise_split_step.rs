use crate::{step::SplitStep, blocks::{BlockMap, standard_blocks::{StandardBlockType, content_block::ContentBlock, StandardBlock, list_block::ListBlock}, Block}, steps_generator::{StepError, selection::{Selection, SubSelection}}, new_ids::NewIds};

use super::{UpdatedState, clean_block_after_transform};

/// If current standard block is Paragraph, H1, H2, or H3 -> new block should be a paragraph
/// If current standard block is some type of List block -> new block should be same type of list block

/// -> split inline block at point of subselection -> creating a new inline block
/// -> split parent's inline blocks after the index of the subselection index block
/// -> create a new standard block with the new inline block & all inline blocks after the index
/// -> move children from "from" block to new block
//-----------------------------------------------------

// This function seems to be implementing a "split block" operation.
//It takes a SplitStep struct as input,
//which includes a SubSelection field that specifies where in the block the split should occur.

// The function starts by retrieving the InlineBlock specified by the SubSelection,
//and then splits it into two blocks at the specified offset using the split method.
//If the second half of the split is empty (i.e., has no text), it removes any marks from it.

// Next, the function retrieves the parent of the first half of the split InlineBlock.
//It creates a new block of the same type as the parent,
//and pushes the second half of the split InlineBlock into it.
//Then, it uses the split method on the parent to split it into two blocks
//at the position of the first half of the split InlineBlock,
//resulting in an updated parent block and a new block.
//The new block has its children's parent fields updated to point to it,
//and the new block is inserted into the parent block's list of children after the updated parent block.

// Finally, the function updates the blocks in the block_map and returns an UpdatedState struct
//with the modified block_map, a new selection (at the start of the new block),
//and updated lists of blocks to update and remove.
pub fn actualise_split_step(
    split_step: SplitStep,
    mut block_map: BlockMap,
    new_ids: &mut NewIds,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let inline_block = block_map.get_inline_block(&split_step.subselection.block_id)?;
    let (first_half_inline_block, mut second_half_inline_block) = inline_block.split(split_step.subselection.offset, new_ids)?;
    if second_half_inline_block.text()?.len() == 0 {
        second_half_inline_block.marks = vec![];
    }
    let parent = first_half_inline_block.get_parent(&block_map)?;
    let new_block_content = get_new_enter_block_type(&parent.content)?;
    let new_block_content = new_block_content.push_to_content(vec![second_half_inline_block.id()])?;
    let (updated_standard_block, new_standard_block) = parent.split(first_half_inline_block.index(&block_map)? + 1, new_block_content, new_ids)?;
    new_standard_block.set_new_parent_of_children(&mut block_map, &mut blocks_to_update)?;

    let mut parents_parent = block_map.get_block(&updated_standard_block.parent())?;
    let parent_index = updated_standard_block.index(&block_map)?;
    parents_parent.splice_children(parent_index + 1, parent_index + 1, vec![new_standard_block.id()])?;

    block_map.update_blocks(vec![Block::InlineBlock(first_half_inline_block), Block::InlineBlock(second_half_inline_block), parents_parent]
    , &mut blocks_to_update)?;
    let block_map = new_standard_block.set_as_parent_for_all_inline_blocks(block_map, &mut blocks_to_update)?;
    let block_map = clean_block_after_transform(updated_standard_block, block_map, &mut blocks_to_update)?;
    let new_standard_block_id = new_standard_block.id();
    let block_map = clean_block_after_transform(new_standard_block, block_map, &mut blocks_to_update)?;

    let new_standard_block = block_map.get_standard_block(&new_standard_block_id)?;
    let first_inline_block_id = new_standard_block.content_block()?.inline_blocks[0].clone();
    let updated_subselection = SubSelection { block_id: first_inline_block_id, offset: 0, subselection: None };
    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection { anchor: updated_subselection.clone(), head: updated_subselection.clone() }),
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}

// This function appears to be creating a new block of the same type as block_type,
//with an empty inline_blocks field in its ContentBlock.
//If block_type is not a block that can be "entered", it returns an error.

// The get_new_enter_block_type function is used in the actualise_split_step function
//to create a new block to place the second half of a split block into.
//This allows the user to continue editing after they have split a block.
fn get_new_enter_block_type(block_type: &StandardBlockType) -> Result<StandardBlockType, StepError> {
    return match block_type {
        StandardBlockType::Paragraph(_) | StandardBlockType::H1(_) | StandardBlockType::H2(_) | StandardBlockType::H3(_)
            => Ok(StandardBlockType::Paragraph(ContentBlock { inline_blocks: vec![] })),
        StandardBlockType::TodoList(_) => Ok(StandardBlockType::TodoList(ListBlock { content: ContentBlock { inline_blocks: vec![] }, completed: false })),
        StandardBlockType::DotPointList(_) => Ok(StandardBlockType::DotPointList(ListBlock { content: ContentBlock { inline_blocks: vec![] }, completed: false })),
        StandardBlockType::NumberedList(_) => Ok(StandardBlockType::NumberedList(ListBlock { content: ContentBlock { inline_blocks: vec![] }, completed: false })),
        StandardBlockType::ArrowList(_) => Ok(StandardBlockType::ArrowList(ListBlock { content: ContentBlock { inline_blocks: vec![] }, completed: false })),
        block_type => return Err(StepError(format!("Cannot enter on block type {:?}", block_type)))
    }
}

