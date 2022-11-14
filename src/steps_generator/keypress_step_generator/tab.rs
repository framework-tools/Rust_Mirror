use crate::{step::{Step, TurnToChild}, steps_generator::{StepError, selection::SubSelection}, blocks::{BlockMap, Block}};


pub fn generate_steps_for_tab(block_map: &BlockMap, from: SubSelection, to: SubSelection) -> Result<Vec<Step>, StepError> {
    let from_block = block_map.get_block(&from.block_id)?;
    match from_block {

        Block::InlineBlock(inline_block) => {
        
            let parent_block = inline_block.get_parent(block_map)?;

            if parent_block.index(block_map)? == 0 {
                return Ok(vec![])
            } else {
                return Ok(vec![
                    Step::TurnToChild(TurnToChild { block_id: inline_block.parent })
                ])        
            }
        },

        Block::StandardBlock(_) => unimplemented!(),
        Block::Root(_) => unimplemented!(),
    }
}