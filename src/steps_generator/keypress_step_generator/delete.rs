use crate::{blocks::{BlockMap, Block}, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps}, step::Step};

pub fn generate_steps_for_delete(
    block_map: &BlockMap,
    mut from: SubSelection,
    mut to: SubSelection
) -> Result<Vec<Step>, StepError>{
    let from_block = block_map.get_block(&from.block_id)?;
    match from_block {
        Block::InlineBlock(from_block) => {
            if from == to { // caret selection
                to.offset += 1;
            } 
            return generate_replace_selected_steps(block_map, from, to, "".to_string())
        },
        Block::StandardBlock(_) => return generate_replace_selected_steps(block_map, from, to, "".to_string()),
        Block::Root(_) => return Err(StepError("Cannot perform a delete operation on a root block".to_string()))
    }
}

