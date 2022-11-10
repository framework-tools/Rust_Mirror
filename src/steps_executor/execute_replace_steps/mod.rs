

use crate::{step::{ReplaceStep, ReplaceSlice}, blocks::{BlockMap, Block, inline_blocks::InlineBlock}, steps_generator::{StepError, selection::Selection}};

use self::replace_for_inline_blocks::replace_selected_across_inline_blocks;

use super::{UpdatedState};

pub mod replace_for_inline_blocks;

/// Apply replace step & update to block map
/// Update each block in "update block map"
/// For each "update" we need to:
/// -> merge adjacent inline blocks with same marks (unimplemented)
/// -> delete any text blocks with no text (unimplemented)
pub fn execute_replace_step(replace_step: ReplaceStep, block_map: BlockMap) -> Result<UpdatedState, StepError> {
    let from_block = block_map.get_block(&replace_step.from.block_id)?;
    return match from_block {
        Block::InlineBlock(from_block) => replace_selected_across_inline_blocks(from_block, block_map, replace_step),
        Block::StandardBlock(standard_block) => {
            unimplemented!()
            // if Replacing only inline blocks within a single standard block
            // if replace_step.from.subselection.is_none() && replace_step.to.subselection.is_none() {
            //     let mut content = standard_block.content_block()?.clone();
            //     // replace content's inline blocks from "from" offset to "to" offset with slice
            //     content.inline_blocks.splice(replace_step.from.offset..replace_step.to.offset, replace_step.slice);
            //     let updated_standard_block = standard_block.update_block_content(content)?;
            //     let updated_block = Block::StandardBlock(updated_standard_block);
            //     block_map.update_block(updated_block)?;
            //     let standard_block = block_map.get_standard_block(&replace_step.block_id)?;
            //     block_map = clean_block_after_transform(&standard_block, block_map)?;
            //     let updated_subselection = replace_step.
                // updated_selection = Selection {
                //     anchor:
                // }

            //} else {
       //         return execute_replace_on_standard_blocks_fully_selected(replace_step, block_map)
                // let standard_block = execute_replace_on_blocks_children(
                //     Block::StandardBlock(standard_block),
                //     replace_step.from.offset,
                //     replace_step.to.offset,
                //     replace_step.slice,
                // )?;
                // block_map.update_block(standard_block)?;
                // let standard_block = block_map.get_standard_block(&replace_step.block_id)?;
                // block_map = clean_block_after_transform(&standard_block, block_map)?;
            //}
        },
        Block::Root(root_block) => {
            unimplemented!()
            //return execute_replace_on_standard_blocks_fully_selected(replace_step, block_map)
            // let root_block = execute_replace_on_blocks_children(
            //     Block::Root(root_block),
            //     replace_step.from.offset,
            //     replace_step.to.offset,
            //     replace_step.slice,
            // )?;
            // block_map.update_block(root_block)?;
        },
    }
}

// fn execute_replace_on_blocks_children(mut block: Block, from_index: usize, to_index: usize, slice: Vec<String>) -> Result<Block, StepError> {
//     block.splice_children(from_index, to_index, slice)?;
//     return Ok(block)
// }

fn from_and_to_are_inline_blocks(replace_step: &ReplaceStep, block_map: &BlockMap) -> bool {
    let from_block = block_map.get_inline_block(&replace_step.from.block_id);
    let to_block = block_map.get_inline_block(&replace_step.to.block_id);
    println!("from inline: {}, to inline: {}", from_block.is_ok(), to_block.is_ok());
    return from_block.is_ok() && to_block.is_ok()
}

fn execute_replace_on_standard_blocks_fully_selected(replace_step: ReplaceStep, mut block_map: BlockMap) -> Result<BlockMap, StepError> {
    let from_standard_block = block_map.get_standard_block(&replace_step.from.block_id)?;
    let mut parent_block = block_map.get_block(&from_standard_block.parent)?;
    if replace_step.from.subselection.is_some() {
        return Err(StepError("From subselection should be none for standard block".to_string()))
    }
    let mut children = parent_block.children()?.clone();
    children.splice(replace_step.from.offset..replace_step.to.offset + 1, vec![]);
    parent_block.update_children(children)?;
    block_map.update_block(parent_block)?;
    return Ok(block_map)
}

// /// check parent is the same
// /// -> replace children on "from" block with children on the "to" block
// /// -> remove all inline blocks on "from" block after the "from" subselection offset/index
// /// -> add to the end of the "from" block inline blocks:
// ///  all inline blocks on "to" block including & after the "to" subselection offset/index
// /// -> remove text from "from subselection" block after the subselection offset 
// /// -> remove text from "to subselection" block before the subselection offset
// /// -> update "to subselection" block's parent to "from" block
// fn replace_selected_across_standard_blocks(
//     mut from_block: StandardBlock,
//     block_map: &BlockMap,
//     from: SubSelection,
//     to: SubSelection,
//     replace_with: String
// ) -> Result<Vec<Step>, StepError> {
//     let to_block = BlockMap::get_standard_block(block_map, &to.block_id)?;
//     match from_block.parent == to_block.parent {
//         true => {
//             from_block.children = to_block.children.clone();
//             let updated_content_block = from_block.content_block()?;
//             let mut updated_inline_blocks = updated_content_block.inline_blocks[..from.offset + 1].to_vec();
//             let to_content_block = to_block.content_block()?;
//             let mut to_inline_blocks_after_deletion = to_content_block.inline_blocks[to.offset..].to_vec();
//             updated_inline_blocks.append(&mut to_inline_blocks_after_deletion);
//             let from_block = from_block.update_block_content(ContentBlock {
//                 inline_blocks: updated_inline_blocks
//             })?;

//             let from_subselection = match from.subselection {
//                 Some(subselection) => *subselection,
//                 None => return Err(StepError("Expected from.subselection to be Some".to_string()))
//             };

//             let to_subselection = match to.subselection {
//                 Some(subselection) => *subselection,
//                 None => return Err(StepError("Expected to.subselection to be Some".to_string()))
//             };
//             let from_subselection_block = block_map.get_inline_block(&from_subselection.block_id)?;
//             let to_subselection_block = block_map.get_inline_block(&to_subselection.block_id)?;
//             let from_subselection_updated_text = format!("{}{}", &from_subselection_block.text()?.clone()[..from_subselection.offset], replace_with);
//             let to_subselection_updated_text = to_subselection_block.text()?.clone()[to_subselection.offset..].to_string();
//             let from_subselection_updated_block = from_subselection_block.update_text(from_subselection_updated_text)?;
//             let mut to_subselection_updated_block = to_subselection_block.update_text(to_subselection_updated_text)?;
//             to_subselection_updated_block.parent = from_block.id();
//             let from_block_parent_id = from_block.parent();
//             let parent_block = BlockMap::get_block(block_map, &from_block.parent)?;
//             return Ok(vec![
//                 Step::ReplaceStep(ReplaceStep {
//                     block_id: from_block_parent_id.clone(),
//                     from: SubSelection {
//                         block_id: from_block_parent_id.clone(),
//                         offset: parent_block.index_of_child(&from_block._id)?,
//                         subselection: None
//                     },
//                     to: SubSelection {
//                         block_id: from_block_parent_id.clone(),
//                         offset: parent_block.index_of_child(&to_block._id)? + 1,
//                         subselection: None
//                     },
//                     slice: vec![from_block.id()],
//                     blocks_to_update: vec![
//                         Block::StandardBlock(from_block),
//                         Block::InlineBlock(from_subselection_updated_block),
//                         Block::InlineBlock(to_subselection_updated_block)
//                     ]
//                 }),
//             ])

//         },
//         false => return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
//     };
// }