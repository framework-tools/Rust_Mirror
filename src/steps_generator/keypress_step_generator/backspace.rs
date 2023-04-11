use crate::{blocks::{BlockMap, Block, inline_blocks::InlineBlock, standard_blocks::{StandardBlock, StandardBlockType, content_block::ContentBlock}}, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps, turn_into::turn_into_paragraph_step}, step::{Step, ReplaceStep, ReplaceSlice, TurnToParent}};


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
                        let std_block = block_map.get_inline_block(&from.block_id)?.get_parent(block_map)?;
                        if std_block.is_list() {
                            return turn_into_paragraph_step(std_block.id())
                        } else {
                            return caret_at_start_of_parent_block_steps(from_block, block_map)
                        }
                    } else { // caret at start of inline block that is not the first inline in it's parent
                        let previous_inline_block = from_block.previous_block(block_map)?;
                        from = SubSelection {
                            block_id: previous_inline_block.id(),
                            offset: previous_inline_block.text()?.len() as usize - 1,
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
    if !parent_block.parent_is_root(block_map) {
        return Ok(vec![Step::TurnToParent(TurnToParent { block_id: parent_block.id() })])
    } else {
        let block_before_parent: Option<StandardBlock> = parent_block.get_previous(block_map)?;
        return match block_before_parent {
            Some(block_before_parent) => {
                Ok(vec![
                    Step::ReplaceStep(ReplaceStep {
                        block_id: parent_block.parent.clone(),
                        from: SubSelection::at_end_of_youngest_descendant(&block_before_parent, block_map)?,
                        to: SubSelection {
                            block_id: parent_block.id(),
                            offset: 0,
                            subselection: Some(Box::new(SubSelection {
                                block_id: from_block.id(),
                                offset: 0,
                                subselection: None
                            }))
                        },
                        slice: ReplaceSlice::String("".to_string())
                    })
                ])
            },
            None => Ok(vec![])
        }
    }
}