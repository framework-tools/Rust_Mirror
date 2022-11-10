use crate::{blocks::{BlockMap, Block}, step::{Step, ReplaceStep, ReplaceSlice}};

use super::{selection::SubSelection, StepError};

pub fn generate_replace_selected_steps(
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection,
    replace_with: String
) -> Result<Vec<Step>, StepError> {
    return match block_map.get_block(&from.block_id)? {
        Block::InlineBlock(inline_block) => Ok(vec![
            Step::ReplaceStep(ReplaceStep {
                block_id: inline_block.parent,
                from,
                to,
                slice: ReplaceSlice::String(replace_with)
            })
        ]),
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