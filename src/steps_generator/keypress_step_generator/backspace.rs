use crate::{blocks::{BlockMap, Block, inline_blocks::InlineBlock, standard_blocks::StandardBlock}, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps}, step::{Step, ReplaceStep, ReplaceSlice}};


pub fn generate_steps_for_backspace(
    block_map: &BlockMap,
    mut from: SubSelection,
    mut to: SubSelection
) -> Result<Vec<Step>, StepError> {
    let from_block = block_map.get_block(&from.block_id)?;
    match from_block {
        Block::InlineBlock(from_block) => {
            if from == to { // caret selection
                if from.offset == 0 { // at start of block
                    if from_block.index(block_map)? == 0 { // caret at start of standard block
                        return caret_at_start_of_parent_block_steps(from_block, block_map)
                    } else { // caret at start of inline block that is not the first inline in it's parent
                        let from_block = block_map.get_inline_block(&from.block_id)?;
                        let previous_inline_block = from_block.previous_block(block_map)?;
                        from = SubSelection {
                            block_id: previous_inline_block.id(),
                            offset: previous_inline_block.text()?.len() - 1,
                            subselection: None
                        };
                        to = from.clone();
                        to.offset += 1;
                    }
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

fn caret_at_start_of_parent_block_steps(from_block: InlineBlock, block_map: &BlockMap) -> Result<Vec<Step>, StepError> {
    let parent_block = from_block.get_parent(block_map)?;
    let block_before_parent: Option<StandardBlock> = parent_block.get_previous(block_map)?;
    return match block_before_parent {
        Some(block_before_parent) => {
            Ok(vec![
                Step::ReplaceStep(ReplaceStep {
                    block_id: block_before_parent.id(),
                    from: SubSelection::from(block_before_parent.id(), 1, None),
                    to: SubSelection::from(block_before_parent.id(), 1, None),
                    slice: ReplaceSlice::Blocks(parent_block.content_block()?.clone().inline_blocks)
                }),
                Step::ReplaceStep(ReplaceStep {
                    block_id: parent_block.get_parent(block_map)?.id(),
                    from: SubSelection::from(parent_block.get_parent(block_map)?.id(), parent_block.index(block_map)?, None),
                    to: SubSelection::from(parent_block.get_parent(block_map)?.id(), parent_block.index(block_map)? + 1, None),
                    slice: ReplaceSlice::Blocks(vec![])
                }),
            ])
        },
        None => Ok(vec![])
    }
}