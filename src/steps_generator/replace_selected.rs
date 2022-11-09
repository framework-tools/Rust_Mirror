
use crate::{step::{Step, ReplaceStep}, blocks::{Block, BlockMap, inline_blocks::{InlineBlock},
standard_blocks::{StandardBlock, content_block::ContentBlock}}};

use super::{selection::SubSelection, StepError};

pub fn replace_selected(
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection,
    replace_with: String
) -> Result<Vec<Step>, StepError> {
    let block = block_map.get_block(&from.block_id)?;
    match block {
        Block::InlineBlock(inline_block) => replace_selected_across_inline_blocks(
            inline_block,
            block_map,
            from,
            to,
            replace_with
        ),
        Block::StandardBlock(_) => {unreachable!()}
        // replace_selected_across_standard_blocks(
        //     from_block,
        //     block_map,
        //     from,
        //     to,
        //     replace_with
        // ),
        Block::Root(_) => unimplemented!()
    }
}

/// replace "from" index of from block to index of "to" block on their parent
/// replace with slice that consists of:
///  -> "from" block with chars removed from offset to end of text
///  -> "to" block with chars removed from start of text to offset
pub fn replace_selected_across_inline_blocks(
    from_block: InlineBlock,
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection,
    replace_with: String
) -> Result<Vec<Step>, StepError> {
    let to_block = BlockMap::get_inline_block(block_map, &to.block_id)?;
    if from_block.parent != to_block.parent {
        return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
    }
    let parent_block = BlockMap::get_standard_block(block_map, &from_block.parent)?;
    let content_block = parent_block.content_block()?;
    if from.block_id == to.block_id {
        let text = from_block.text()?.clone();
        // create new block with text from replace_with inserted
        let updated_text = format!("{}{}{}", &text[0..from.offset], replace_with, &text[to.offset..]);
        let updated_block = from_block.update_text(updated_text)?;
        let parent_content_block = parent_block.content_block()?;
        return Ok(vec![
            Step::ReplaceStep(ReplaceStep {
                block_id: parent_block.id(),
                from: SubSelection { block_id: parent_block.id(), offset: parent_content_block.index_of(&updated_block._id)?, subselection: None },
                to: SubSelection { block_id: parent_block.id(), offset: parent_content_block.index_of(&updated_block._id)? + 1, subselection: None },
                slice: vec![updated_block.id()],
                blocks_to_update: vec![Block::InlineBlock(updated_block)]
            })
        ])
    } else {
        let first_block_updated_text = format!("{}{}", &from_block.text()?.clone()[0..from.offset], replace_with);
        let block1 = from_block.update_text(first_block_updated_text)?;
        let second_block_updated_text = to_block.text()?.clone()[to.offset..].to_string();
        let block2 = to_block.update_text(second_block_updated_text)?;
        return Ok(vec![Step::ReplaceStep(ReplaceStep {
            block_id: parent_block.id(),
            from: SubSelection { block_id: parent_block.id(), offset: content_block.index_of(&from.block_id)?, subselection: None },
            to: SubSelection { block_id: parent_block.id(), offset: content_block.index_of(&to.block_id)? + 1, subselection: None },
            slice: vec![block1.id(), block2.id()],
            blocks_to_update: vec![Block::InlineBlock(block1), Block::InlineBlock(block2)]
        })])
    }
}

/// check parent is the same
/// -> replace children on "from" block with children on the "to" block
/// -> remove all inline blocks on "from" block after the "from" subselection offset/index
/// -> add to the end of the "from" block inline blocks:
///  all inline blocks on "to" block including & after the "to" subselection offset/index
/// -> remove text from "from subselection" block after the subselection offset 
/// -> remove text from "to subselection" block before the subselection offset
/// -> update "to subselection" block's parent to "from" block
fn replace_selected_across_standard_blocks(
    mut from_block: StandardBlock,
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection,
    replace_with: String
) -> Result<Vec<Step>, StepError> {
    let to_block = BlockMap::get_standard_block(block_map, &to.block_id)?;
    match from_block.parent == to_block.parent {
        true => {
            from_block.children = to_block.children.clone();
            let updated_content_block = from_block.content_block()?;
            let mut updated_inline_blocks = updated_content_block.inline_blocks[..from.offset + 1].to_vec();
            let to_content_block = to_block.content_block()?;
            let mut to_inline_blocks_after_deletion = to_content_block.inline_blocks[to.offset..].to_vec();
            updated_inline_blocks.append(&mut to_inline_blocks_after_deletion);
            let from_block = from_block.update_block_content(ContentBlock {
                inline_blocks: updated_inline_blocks
            })?;

            let from_subselection = match from.subselection {
                Some(subselection) => *subselection,
                None => return Err(StepError("Expected from.subselection to be Some".to_string()))
            };

            let to_subselection = match to.subselection {
                Some(subselection) => *subselection,
                None => return Err(StepError("Expected to.subselection to be Some".to_string()))
            };
            let from_subselection_block = block_map.get_inline_block(&from_subselection.block_id)?;
            let to_subselection_block = block_map.get_inline_block(&to_subselection.block_id)?;
            let from_subselection_updated_text = format!("{}{}", &from_subselection_block.text()?.clone()[..from_subselection.offset], replace_with);
            let to_subselection_updated_text = to_subselection_block.text()?.clone()[to_subselection.offset..].to_string();
            let from_subselection_updated_block = from_subselection_block.update_text(from_subselection_updated_text)?;
            let mut to_subselection_updated_block = to_subselection_block.update_text(to_subselection_updated_text)?;
            to_subselection_updated_block.parent = from_block.id();
            let from_block_parent_id = from_block.parent();
            let parent_block = BlockMap::get_block(block_map, &from_block.parent)?;
            return Ok(vec![
                Step::ReplaceStep(ReplaceStep {
                    block_id: from_block_parent_id.clone(),
                    from: SubSelection {
                        block_id: from_block_parent_id.clone(),
                        offset: parent_block.index_of_child(&from_block._id)?,
                        subselection: None
                    },
                    to: SubSelection {
                        block_id: from_block_parent_id.clone(),
                        offset: parent_block.index_of_child(&to_block._id)? + 1,
                        subselection: None
                    },
                    slice: vec![from_block.id()],
                    blocks_to_update: vec![
                        Block::StandardBlock(from_block),
                        Block::InlineBlock(from_subselection_updated_block),
                        Block::InlineBlock(to_subselection_updated_block)
                    ]
                }),
            ])

        },
        false => return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
    };
}