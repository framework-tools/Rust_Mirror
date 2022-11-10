



use crate::{step::{Step, ReplaceStep}, blocks::{BlockMap, standard_blocks::{StandardBlock, StandardBlockType, content_block::ContentBlock}, Block}, steps_generator::{selection::SubSelection, StepError}, new_ids::NewIds};

/// If current standard block is Paragraph, H1, H2, or H3 -> new block should be a paragraph
/// If current standard block is some type of List block -> new block should be same type of list block
///
/// -> remove all inline blocks after subselection offset from "from" standard block
/// -> remove text after offset in "from" subselection block
/// -> use this removed text to create a new inline block
/// -> create a new standard block with this new inline block & all removed inline blocks from previous block
/// -> move children from "from" block to new block
pub fn generate_steps_for_enter(block_map: &BlockMap, from: SubSelection, to: SubSelection, new_ids: &mut NewIds) -> Result<Vec<Step>, StepError> {
    unimplemented!()
//     let mut from_standard_block = block_map.get_nearest_ancestor_standard_block_incl_self(&from.block_id)?;
//     let to_standard_block = block_map.get_nearest_ancestor_standard_block_incl_self(&from.block_id)?;
//     let mut from_inline_block = block_map.get_inline_block(&from.block_id)?;

//     if from_standard_block.id() == to_standard_block.id() {
//         if from.block_id != to.block_id {
//             let from_index = from_standard_block.index_of_child(&from.block_id())?;
//             let to_index = from_standard_block.index_of_child(&to.block_id)?;
//             from_standard_block = from_standard_block.remove_blocks_between_offsets(from_index, to_index)?;
//         } else if from.offset != to.offset {
//             let from_text = from_inline_block.text()?;
//             let updated_text = from_text[from.offset..to.offset].to_string();
//             from_inline_block = from_inline_block.update_text(updated_text)?;
//         }
//         let content_block = from_standard_block.content_block()?;
//         let index_of_selection = content_block.index_of(&from.block_id)?;
//         let blocks_up_to_including_selection = content_block.inline_blocks[0..index_of_selection + 1].to_vec();
//         let mut blocks_after_selection = content_block.inline_blocks[index_of_selection + 1..].to_vec();
//         let updated_from_block_content_block = ContentBlock {
//             inline_blocks: blocks_up_to_including_selection
//         };
//         let new_block_type = get_new_enter_block_type(&from_standard_block.content)?;
//         let new_inline_block_id = new_ids.get_id()?;
//         blocks_after_selection.insert(0, new_inline_block_id.clone());
//         let new_block_type = new_block_type.update_block_content(ContentBlock {
//             inline_blocks: blocks_after_selection
//         })?;
//         let updated_from_block = Block::StandardBlock(StandardBlock {
//             _id: from_standard_block.id(),
//             content: from_standard_block.content.update_block_content(updated_from_block_content_block)?,
//             children: vec![],
//             parent: from_standard_block.parent.clone(),
//             marks: from_standard_block.marks.clone()
//         });
//         let from_standard_block_parent = from_standard_block.parent();
//         let new_block = Block::StandardBlock(StandardBlock {
//             _id: new_ids.get_id()?,
//             content: new_block_type,
//             children: from_standard_block.children,
//             parent: from_standard_block_parent.clone(),
//             marks: vec![],
//         });
//         let parent_block = block_map.get_block(&from_standard_block.parent)?;
//         let index_of_std_block = parent_block.index_of_child(&from_standard_block._id)?;
//         let from_inline_block_clone = from_inline_block.clone();
//         let from_inline_block_text = from_inline_block.text()?.clone();
//         let from_inline_block_text_before_offset = from_inline_block_text[0..from.offset].to_string();
//         let updated_subselection_block = from_inline_block.update_text(from_inline_block_text_before_offset)?;
//         let from_inline_block_text_after_offset = from_inline_block_text[from.offset..].to_string();
//         let mut new_inline_block = from_inline_block_clone.update_text(from_inline_block_text_after_offset)?;
//         new_inline_block._id = new_inline_block_id;
//         new_inline_block.parent = new_block.id();
//         return Ok(vec![Step::ReplaceStep(ReplaceStep {
//             block_id: from_standard_block_parent.clone(),
//             from: SubSelection {
//                 block_id: from_standard_block_parent.clone(),
//                 offset: index_of_std_block.clone(),
//                 subselection: None
//             },
//             to: SubSelection {
//                 block_id: from_standard_block_parent.clone(),
//                 offset: index_of_std_block + 1,
//                 subselection: None
//             },
//             slice: vec![updated_from_block.id(), new_block.id()],
//             blocks_to_update: vec![
//                 updated_from_block,
//                 Block::InlineBlock(updated_subselection_block),
//                 new_block,
//                 Block::InlineBlock(new_inline_block)
//             ]
//         })])
//     } else {
//         return enter_across_standard_blocks(
//             from_standard_block,
//             block_map,
//             from,
//             to,
//             "".to_string()
//         );
//     }
// }

// fn get_new_enter_block_type(block_type: &StandardBlockType) -> Result<StandardBlockType, StepError> {
//     return match block_type {
//         StandardBlockType::Paragraph(_) | StandardBlockType::H1(_) | StandardBlockType::H2(_) | StandardBlockType::H3(_)
//             => Ok(StandardBlockType::Paragraph(ContentBlock { inline_blocks: vec![] })),
//         //block_type => return Err(StepError(format!("Cannot enter on block type {:?}", block_type)))
//     }
// }

// /// replace slice contains id of the "from" & the "to" block
// /// -> update blocks should contain:
// ///   -> updated "from" block with inline blocks removed that got deleted (all blocks after subselection offset)
// ///   -> updated "to" block with inline blocks removed that got deleted (all blocks before subselection offset)
// ///   -> updated "from" subselection block with chars removed from offset to end of text
// ///   -> updated "to" subselection block with chars removed from start of text to offset
// fn enter_across_standard_blocks(
//     from_block: StandardBlock,
//     block_map: &BlockMap,
//     from: SubSelection,
//     to: SubSelection,
//     replace_with: String
// ) -> Result<Vec<Step>, StepError> {
//     let to_block = BlockMap::get_standard_block(block_map, &to.block_id)?;
//     match from_block.parent == to_block.parent {
//         true => {
//             let updated_from_content_block = from_block.content_block()?.clone().remove_blocks_after_offset(from.offset)?;
//             let updated_from_block = from_block.update_block_content(updated_from_content_block)?;
//             let updated_to_content_block = to_block.content_block()?.clone().remove_blocks_before_offset(to.offset)?;
//             let updated_to_block = to_block.update_block_content(updated_to_content_block)?;

//             let from_subselection = match from.subselection {
//                 Some(subselection) => *subselection,
//                 None => return Err(StepError("Expected from.subselection to be Some".to_string()))
//             };
//             let from_subselection_block = BlockMap::get_inline_block(block_map, &from_subselection.block_id)?;
//             let from_subselection_text = from_subselection_block.text()?.clone();
//             let updated_from_subselection_text = format!("{}{}", &from_subselection_text[0..from_subselection.offset], replace_with);
//             let updated_from_subselection_block = from_subselection_block.update_text(updated_from_subselection_text)?;

//             let to_subselection = match to.subselection {
//                 Some(subselection) => *subselection,
//                 None => return Err(StepError("Expected to.subselection to be Some".to_string()))
//             };
//             let to_subselection_block = BlockMap::get_inline_block(block_map, &to_subselection.block_id)?;
//             let to_subselection_text = to_subselection_block.text()?.clone();
//             let updated_to_subselection_text = to_subselection_text[to_subselection.offset..].to_string();
//             let updated_to_subselection_block = to_subselection_block.update_text(updated_to_subselection_text)?;

//             let parent_block = block_map.get_block(&updated_from_block.parent)?;
//             return Ok(vec![Step::ReplaceStep(ReplaceStep {
//                 block_id: updated_from_block.parent(),
//                 from: SubSelection {
//                     block_id: updated_from_block.parent(),
//                     offset: parent_block.index_of_child(&from.block_id)?,
//                     subselection: None
//                 },
//                 to: SubSelection {
//                     block_id: updated_from_block.parent(),
//                     offset: parent_block.index_of_child(&to.block_id)? + 1,
//                     subselection: None
//                 },
//                 slice: vec![
//                     updated_from_block.id(),
//                     updated_to_block.id()
//                 ],
//                 blocks_to_update: vec![
//                     Block::StandardBlock(updated_from_block),
//                     Block::InlineBlock(updated_from_subselection_block),
//                     Block::StandardBlock(updated_to_block),
//                     Block::InlineBlock(updated_to_subselection_block)
//                 ]
//             })])
//         },
//         false => return Err(StepError("Expected from_block and to_block to have the same parent".to_string()))
//     };
}