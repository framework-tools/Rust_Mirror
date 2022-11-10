use crate::{blocks::{inline_blocks::InlineBlock, BlockMap, Block}, step::{ReplaceStep, ReplaceSlice}, steps_generator::{StepError, selection::Selection}, steps_executor::UpdatedState};


/// replace "from" index of from block to index of "to" block on their parent
/// replace with slice that consists of:
///  -> "from" block with chars removed from offset to end of text
///  -> "to" block with chars removed from start of text to offset
pub fn replace_selected_across_inline_blocks(
    from_block: InlineBlock,
    block_map: BlockMap,
    replace_step: ReplaceStep,
) -> Result<UpdatedState, StepError> {
    let to_block = block_map.get_inline_block(&replace_step.to.block_id)?;
    if from_block.parent != to_block.parent {
        return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
    }
    // let parent_block = BlockMap::get_standard_block(block_map, &from_block.parent)?;
    // let content_block = parent_block.content_block()?;
    if from_block._id == to_block._id {
        replace_across_single_inline_block(from_block, block_map, replace_step)
    } else {
        unimplemented!()
        // let first_block_updated_text = format!("{}{}", &from_block.text()?.clone()[0..from.offset], replace_with);
        // let block1 = from_block.update_text(first_block_updated_text)?;
        // let second_block_updated_text = to_block.text()?.clone()[to.offset..].to_string();
        // let block2 = to_block.update_text(second_block_updated_text)?;
        // return Ok(vec![Step::ReplaceStep(ReplaceStep {
        //     block_id: parent_block.id(),
        //     from: SubSelection { block_id: parent_block.id(), offset: content_block.index_of(&from.block_id)?, subselection: None },
        //     to: SubSelection { block_id: parent_block.id(), offset: content_block.index_of(&to.block_id)? + 1, subselection: None },
        //     slice: vec![block1.id(), block2.id()],
        //     blocks_to_update: vec![Block::InlineBlock(block1), Block::InlineBlock(block2)]
        // })])
    }
}

fn replace_across_single_inline_block(
    from_block: InlineBlock,
    mut block_map: BlockMap,
    replace_step: ReplaceStep,
) -> Result<UpdatedState, StepError> {
    let text = from_block.text()?.clone();
    // create new block with text from replace_with inserted
    let replace_with = match &replace_step.slice {
        ReplaceSlice::String(string) => string,
        ReplaceSlice::Blocks(_) => return Err(StepError("Expected string replace slice for inline blocks. Got vec of blocks".to_string()))
    };
    let updated_text = format!("{}{}{}", &text[0..replace_step.from.offset], replace_with, &text[replace_step.to.offset..]);
    let updated_block = from_block.update_text(updated_text)?;
    block_map.update_block(Block::InlineBlock(updated_block))?;

    return Ok(UpdatedState {
        block_map,
        selection: Selection::update_selection_from(replace_step)
    })
}