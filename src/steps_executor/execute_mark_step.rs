
use crate::{step::{MarkStep}, blocks::{BlockMap, Block, inline_blocks::{InlineBlock}, standard_blocks::StandardBlock}, steps_generator::{StepError, selection::{Selection, SubSelection}}, new_ids::NewIds};

use super::{clean_block_after_transform, UpdatedState};


pub fn execute_mark_step(
    mark_step: MarkStep,
    mut block_map: BlockMap,
    add_mark: bool,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {
    let from_raw_selection = mark_step.from.to_raw_selection(&block_map)?;
    let to_raw_selection = mark_step.to.to_raw_selection(&block_map)?;

    let block = block_map.get_block(&mark_step.from.block_id)?;
    match block {
        Block::InlineBlock(from_block) => {
            let updated_state = execute_mark_step_on_inline_blocks(mark_step, from_block, block_map, add_mark, new_ids)?;
            block_map = updated_state.block_map;
        },
        Block::StandardBlock(from_block) => {
            let updated_state = execute_mark_step_on_standard_blocks(mark_step, from_block, block_map, add_mark, new_ids)?;
            block_map = updated_state.block_map;
        },
        Block::Root(_) => return Err(StepError("Cannot mark root block".to_string()))
    };

    let selection = Some(Selection {
        anchor: from_raw_selection.real_selection_from_raw(&block_map)?,
        head: to_raw_selection.real_selection_from_raw(&block_map)?
    });
    return Ok(UpdatedState { block_map, selection })
}

fn execute_mark_step_on_inline_blocks(
    mark_step: MarkStep,
    from_block: InlineBlock,
    mut block_map: BlockMap,
    add_mark: bool,
    new_ids: &mut NewIds
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
            Block::StandardBlock(updated_parent_block.clone()), Block::InlineBlock(before_block), Block::InlineBlock(middle_block), Block::InlineBlock(after_block)
        ])?;
        block_map = clean_block_after_transform(updated_parent_block, block_map)?;
        return Ok(UpdatedState { block_map, selection: Some(new_selection) })
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
        ])?;

        let mut content_block = from_parent_block.content_block()?.clone();
        // for_each_block_between_from_and_to_apply_mark
        let mut i = content_block.index_of(&first_half_of_from_block_id)? + 1;
        let j = content_block.index_of(&first_half_of_to_block_id)?;
        while i < j {
            let block = block_map.get_inline_block(&content_block.inline_blocks[i])?;
            let block = block.apply_mark(mark_step.mark.clone(), add_mark);
            block_map.update_block(Block::InlineBlock(block))?;
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
        block_map.update_block(Block::StandardBlock(updated_parent_block.clone()))?;
        block_map = clean_block_after_transform(updated_parent_block, block_map)?;

        return Ok(UpdatedState { block_map, selection: Some(new_subselection) })
    }
}

fn create_before_middle_after_blocks_with_new_text_and_mark(
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

/// HAS BUGS DUE TO CLEANING SYSTEM -> NEEDS TO MAP
fn updated_selection_after_apply_mark(
    second_half_of_from_block_id: String,
    first_half_of_to_block: &InlineBlock,
) -> Result<Selection, StepError> {
    return Ok(Selection {
        anchor: SubSelection { block_id: second_half_of_from_block_id, offset: 0, subselection: None },
        head: SubSelection { block_id: first_half_of_to_block.id(), offset: first_half_of_to_block.text()?.len(), subselection: None },
    })
}


/// -> apply mark for "from" std block -> from "inner from" to end of inline blocks
/// -> apply mark for "to" std block -> from start of inline blocks to "inner to"
/// -> for each standard block between "from" & "to" -> assign mark to each of their inline blocks
fn execute_mark_step_on_standard_blocks(
    mark_step: MarkStep,
    from_block: StandardBlock,
    mut block_map: BlockMap,
    add_mark: bool,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {
    let deepest_from_subselection = mark_step.from.get_deepest_subselection();
    let deepest_from_subselection_block_id = deepest_from_subselection.block_id.clone();
    let from_mark_step = MarkStep {
        block_id: from_block.id(),
        from: deepest_from_subselection.clone(),
        to: SubSelection::at_end_of_block(&from_block._id, &block_map)?,
        mark: mark_step.mark.clone(),
    };
    let inline_block = block_map.get_inline_block(&deepest_from_subselection_block_id)?;
    let updated_state = execute_mark_step_on_inline_blocks(from_mark_step, inline_block, block_map, add_mark, new_ids)?;
    block_map = updated_state.block_map;

    let to_block = block_map.get_standard_block(&mark_step.to.block_id)?;
    let deepest_to_subselection = mark_step.to.get_deepest_subselection();
    let to_mark_step = MarkStep {
        block_id: to_block.id(),
        from: SubSelection { block_id: to_block.content_block()?.inline_blocks[0].clone(), offset: 0, subselection: None },
        to: deepest_to_subselection.clone(),
        mark: mark_step.mark.clone(),
    };
    let inline_block = block_map.get_inline_block(&to_block.content_block()?.inline_blocks[0])?;

    let updated_state = execute_mark_step_on_inline_blocks(to_mark_step, inline_block, block_map, add_mark, new_ids)?;
    block_map = updated_state.block_map;

    let parent = from_block.get_parent(&block_map)?;
    let parents_children = parent.children()?;
    let mut i = from_block.index(&block_map)? + 1;
    let j = to_block.index(&block_map)?;
    while i < j {
        let block = block_map.get_standard_block(&parents_children[i])?;
        let deepest_from_subselection = SubSelection { block_id: block.content_block()?.inline_blocks[0].clone(), offset: 0, subselection: None };
        let inline_block = block_map.get_inline_block(&deepest_from_subselection.block_id)?;
        let inner_mark_step = MarkStep {
            block_id: to_block.id(),
            from: deepest_from_subselection,
            to: SubSelection::at_end_of_block(&block._id, &block_map)?,
            mark: mark_step.mark.clone(),
        };
        let updated_state = execute_mark_step_on_inline_blocks(inner_mark_step, inline_block, block_map, add_mark, new_ids)?;
        block_map = updated_state.block_map;

        i += 1;
    }

    return Ok(UpdatedState { block_map, selection: None })

}