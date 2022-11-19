
use crate::{step::{MarkStep}, blocks::{BlockMap, Block, inline_blocks::InlineBlock}, steps_generator::{StepError, selection::{Selection, SubSelection}}, new_ids::NewIds};

use super::{clean_block_after_transform, UpdatedState};


pub fn execute_mark_step(mark_step: MarkStep, mut block_map: BlockMap, add_mark: bool, new_ids: &mut NewIds) -> Result<UpdatedState, StepError> {
    let block = block_map.get_block(&mark_step.from.block_id)?;
    match block {
        Block::InlineBlock(inline_block) => {
            return execute_mark_step_on_inline_block(mark_step, inline_block, block_map, add_mark, new_ids)
        },
        Block::StandardBlock(standard_block) => {
            unimplemented!()
            // match mark_step.from.subselection {

            // }
        },
        Block::Root(root_block) => return Err(StepError("Cannot mark root block".to_string()))
    };
}

fn execute_mark_step_on_inline_block(
    mark_step: MarkStep,
    from_block: InlineBlock,
    mut block_map: BlockMap,
    add_mark: bool,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {
    if mark_step.from.block_id == mark_step.to.block_id {
        //split block into 3 new inline blocks
        //add mark to middle block
        //replace in parent block with new blocks
        let text = from_block.text()?.clone();
        let before_text = text.slice(0..mark_step.from.offset);
        let middle_text = text.slice(mark_step.from.offset..mark_step.to.offset);
        let after_text = text.slice(mark_step.to.offset..text.len());
        let before_block = from_block.clone().update_text(before_text)?;
        let mut middle_block = from_block.clone().to_new_block(new_ids)?.update_text(middle_text)?;
        let after_block = from_block.to_new_block(new_ids)?.update_text(after_text)?;
        middle_block = middle_block.apply_mark(mark_step.mark, add_mark);
        let parent_block = block_map.get_standard_block(&before_block.parent)?;
        let original_block_index = parent_block.index_of(&mark_step.from.block_id)?;
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
        let mut new_subselection = Selection {
            anchor: SubSelection { block_id: "".to_string(), offset: 0, subselection: None },
            head: SubSelection { block_id: "".to_string(), offset: 0, subselection: None },
        };
        //split from block
        //split to block
        //apply marks to split blocks within the selection & all the blocks in between
        //replace in parent block with new blocks
        //update every block that was changed
        let from_parent_block = block_map.get_standard_block(&from_block.parent)?;
        let to_block = block_map.get_inline_block(&mark_step.to.block_id)?;
        let from_text = from_block.text()?.clone();
        let to_text = to_block.text()?.clone();
        let (before_from_text, after_from_text) = from_text.split(mark_step.from.offset);
        let (before_to_text, after_to_text) = to_text.split(mark_step.from.offset);
        let before_from_block = from_block.clone().update_text(before_from_text)?;
        let before_from_block_id = before_from_block.id();
        block_map.update_block(Block::InlineBlock(before_from_block))?;
        let after_from_block = from_block.to_new_block(new_ids)?.update_text(after_from_text)?;
        let after_from_block_id = after_from_block.id();
        let after_from_block = after_from_block.apply_mark(mark_step.mark.clone(), add_mark);
        new_subselection.anchor.block_id = after_from_block.id();
        block_map.update_block(Block::InlineBlock(after_from_block))?;
        let before_to_block = to_block.clone().to_new_block(new_ids)?.update_text(before_to_text)?;
        let before_to_block = before_to_block.apply_mark(mark_step.mark.clone(), add_mark);
        let before_to_block_id = before_to_block.id();
        new_subselection.head.block_id = before_to_block.id();
        new_subselection.head.offset = before_to_block.text()?.len();
        block_map.update_block(Block::InlineBlock(before_to_block))?;
        let after_to_block = to_block.update_text(after_to_text)?;
        let after_to_block_id = after_to_block.id();
        block_map.update_block(Block::InlineBlock(after_to_block))?;
        let mut content_block = from_parent_block.content_block()?.clone();

        let mut i = content_block.index_of(&before_from_block_id)? + 1;
        let j = content_block.index_of(&after_to_block_id)?;
        while i < j {
            let block = block_map.get_inline_block(&content_block.inline_blocks[i])?;
            let block = block.apply_mark(mark_step.mark.clone(), add_mark);
            block_map.update_block(Block::InlineBlock(block))?;
            i += 1;
        }
        //one splice for "from" block
        //one splice for "to" block
        content_block.inline_blocks.splice(
            content_block.index_of(&before_from_block_id)?..content_block.index_of(&before_from_block_id)?+1,
            vec![before_from_block_id, after_from_block_id]
        );
        content_block.inline_blocks.splice(
            content_block.index_of(&after_to_block_id)?..content_block.index_of(&after_to_block_id)?+1,
            vec![before_to_block_id, after_to_block_id]
        );
        let updated_parent_block = from_parent_block.update_block_content(content_block)?;
        block_map.update_block(Block::StandardBlock(updated_parent_block.clone()))?;
        block_map = clean_block_after_transform(updated_parent_block, block_map)?;

        return Ok(UpdatedState { block_map, selection: Some(new_subselection) })
    }
}