use crate::{blocks::{BlockMap, Block}, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps}, step::Step};

use super::backspace::generate_steps_for_backspace;

pub fn generate_steps_for_delete(
    block_map: &BlockMap,
    mut from: SubSelection,
    mut to: SubSelection
) -> Result<Vec<Step>, StepError>{
    let from_block = block_map.get_block(&from.block_id)?;
    match from_block {
        Block::InlineBlock(from_block) => {
            if from.offset == from_block.text()?.len() { // at end of block
                if from_block.is_last_inline_block(block_map)? { 
                    let std_block = block_map.get_inline_block(&from.block_id)?.get_parent(block_map)?;
                    let next_standard_sibling = std_block.next_sibling(block_map)?;
                    let block_below_no_content = match &next_standard_sibling {
                        Some(block) => !block.has_content(),
                        None => true
                    };
                    if block_below_no_content {
                        return Ok(vec![])
                    }
                    let new_subselection = SubSelection {
                        block_id: next_standard_sibling.unwrap().content_block()?.inline_blocks[0].clone(),
                        offset: 0,
                        subselection: None
                    };
                    return generate_steps_for_backspace(block_map, new_subselection.clone(), new_subselection)
                } else { // caret at end of inline block that is not the last inline in it's parent
                    let next_inline_block = from_block.next_block(block_map)?;
                    from = SubSelection {
                        block_id: next_inline_block.id(),
                        offset: 0,
                        subselection: None
                    };
                    to = from.clone();
                    to.offset += 1;
                }
            } else { // somewhere inside block
                to.offset += 1;
            }
            return generate_replace_selected_steps(block_map, from, to, "".to_string())
        },
        Block::StandardBlock(_) => return generate_replace_selected_steps(block_map, from, to, "".to_string()),
        Block::Root(_) => return Err(StepError("Cannot perform a delete operation on a root block".to_string()))
    }
}