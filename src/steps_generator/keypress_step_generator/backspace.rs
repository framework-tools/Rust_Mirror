use crate::{blocks::{BlockMap, Block}, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps}, step::{Step, ReplaceStep}};


pub fn generate_steps_for_backspace(
    block_map: &BlockMap,
    mut from: SubSelection,
    mut to: SubSelection
) -> Result<Vec<Step>, StepError> {
    let from_block = block_map.get_block(&from.block_id)?;
    match from_block {
        Block::InlineBlock(_) => {
            if from == to { // caret selection
                if from.offset == 0 { // at start of block
                    let from_block = block_map.get_inline_block(&from.block_id)?;
                    let previous_inline_block = from_block.previous_block(block_map)?;
                    from = SubSelection {
                        block_id: previous_inline_block.id(),
                        offset: previous_inline_block.text()?.len() - 1,
                        subselection: None
                    };
                    to = from.clone();
                    to.offset += 1;
                } else { // somewhere inside block
                    from.offset -= 1;
                }
            }
            return generate_replace_selected_steps(block_map, from, to, "".to_string())
        },
        Block::StandardBlock(_) => return generate_replace_selected_steps(block_map, from, to, "".to_string()),
        Block::Root(_) => return Err(StepError("Cannot perform a backspace operation on a root block".to_string()))
    }
}