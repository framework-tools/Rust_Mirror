use crate::{step::SplitStep, blocks::{BlockMap, standard_blocks::{StandardBlockType, content_block::ContentBlock, StandardBlock, list_block::ListBlock}, Block}, steps_generator::{StepError, selection::{Selection, SubSelection}}, new_ids::NewIds};

use super::{UpdatedState, clean_block_after_transform};

/// If current standard block is Paragraph, H1, H2, or H3 -> new block should be a paragraph
/// If current standard block is some type of List block -> new block should be same type of list block

/// -> split inline block at point of subselection -> creating a new inline block
/// -> split parent's inline blocks after the index of the subselection index block
/// -> create a new standard block with the new inline block & all inline blocks after the index
/// -> move children from "from" block to new block
pub fn execute_split_step(split_step: SplitStep, mut block_map: BlockMap, new_ids: &mut NewIds) -> Result<UpdatedState, StepError> {
    let inline_block = block_map.get_inline_block(&split_step.subselection.block_id)?;
    let (first_half_inline_block, second_half_inline_block) = inline_block.split(split_step.subselection.offset, new_ids)?;
    let parent = first_half_inline_block.get_parent(&block_map)?;
    let new_block_content = get_new_enter_block_type(&parent.content)?;
    let new_block_content = new_block_content.push_to_content(vec![second_half_inline_block.id()])?;
    let (updated_standard_block, new_standard_block) = parent.split(first_half_inline_block.index(&block_map)? + 1, new_block_content, new_ids)?;
    new_standard_block.set_new_parent_of_children(&mut block_map)?;

    let mut parents_parent = block_map.get_block(&updated_standard_block.parent())?;
    let parent_index = updated_standard_block.index(&block_map)?;
    parents_parent.splice_children(parent_index + 1, parent_index + 1, vec![new_standard_block.id()])?;

    block_map.update_blocks(vec![Block::InlineBlock(first_half_inline_block), Block::InlineBlock(second_half_inline_block), parents_parent])?;
    let block_map = new_standard_block.set_as_parent_for_all_inline_blocks(block_map)?;
    let block_map = clean_block_after_transform(updated_standard_block, block_map)?;
    let new_standard_block_id = new_standard_block.id();
    let block_map = clean_block_after_transform(new_standard_block, block_map)?;

    let new_standard_block = block_map.get_standard_block(&new_standard_block_id)?;
    let first_inline_block_id = new_standard_block.content_block()?.inline_blocks[0].clone();
    let updated_subselection = SubSelection { block_id: first_inline_block_id, offset: 0, subselection: None };
    return Ok(UpdatedState { block_map, selection: Some(Selection { anchor: updated_subselection.clone(), head: updated_subselection.clone() }) })
}

fn get_new_enter_block_type(block_type: &StandardBlockType) -> Result<StandardBlockType, StepError> {
    return match block_type {
        StandardBlockType::Paragraph(_) | StandardBlockType::H1(_) | StandardBlockType::H2(_) | StandardBlockType::H3(_)
            => Ok(StandardBlockType::Paragraph(ContentBlock { inline_blocks: vec![] })),
        StandardBlockType::TodoList(_) => Ok(StandardBlockType::TodoList(ListBlock { content: ContentBlock { inline_blocks: vec![] }, completed: false })),
        StandardBlockType::DotPointList(_) => Ok(StandardBlockType::DotPointList(ListBlock { content: ContentBlock { inline_blocks: vec![] }, completed: false })),
        StandardBlockType::NumberedList(_) => Ok(StandardBlockType::NumberedList(ListBlock { content: ContentBlock { inline_blocks: vec![] }, completed: false })),
        StandardBlockType::ArrowList(_) => Ok(StandardBlockType::ArrowList(ListBlock { content: ContentBlock { inline_blocks: vec![] }, completed: false })),
        //block_type => return Err(StepError(format!("Cannot enter on block type {:?}", block_type)))
    }
}

