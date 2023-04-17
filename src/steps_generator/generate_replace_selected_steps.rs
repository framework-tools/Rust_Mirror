use crate::{blocks::{BlockMap, Block, standard_blocks::{StandardBlockType, content_block::ContentBlock}}, step::{Step, ReplaceStep, ReplaceSlice, TurnInto}};

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
        Block::StandardBlock(standard_block) => {
            if &from.block_id == &to.block_id {
                return Ok(vec![
                    Step::TurnInto(TurnInto {
                        block_id: from.block_id,
                        new_block_type: StandardBlockType::Paragraph(ContentBlock::new(vec![]))
                    })
                ])
            } else {
                Ok(vec![
                    Step::ReplaceStep(ReplaceStep {
                        block_id: standard_block.parent,
                        from,
                        to,
                        slice: ReplaceSlice::String(replace_with)
                    })
                ])
            }
        },
        Block::Root(_) => return Err(StepError("Cannot perform replace step on root".to_string()))
    }
}
