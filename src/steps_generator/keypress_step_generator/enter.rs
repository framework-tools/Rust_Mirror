



use crate::{step::{Step, SplitStep, AddBlockStep}, blocks::{BlockMap, standard_blocks::{StandardBlockType, content_block::ContentBlock}}, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps, turn_into::turn_into_paragraph_step}, utilities::caret_is_at_start_of_block};

pub fn generate_steps_for_enter(block_map: &BlockMap, from: SubSelection, to: SubSelection) -> Result<Vec<Step>, StepError> {
    let mut steps = vec![];
    if from != to {
        let delete_selected_steps = generate_replace_selected_steps(block_map, from.clone(), to, "".to_string())?;
        for step in delete_selected_steps {
            steps.push(step);
        }
    }  else {
        let std_block = block_map.get_inline_block(&from.block_id)?.get_parent(block_map)?;
        if std_block.is_list() && std_block.text_is_empty(block_map)? {
            return turn_into_paragraph_step(std_block.id())
        } else if caret_is_at_start_of_block(&from, &to, block_map)? {
            let inline_block = block_map.get_inline_block(&from.block_id)?;
            let std_block = inline_block.get_parent(block_map)?;
            let parent = std_block.get_parent(block_map)?;
            return Ok(vec![
                Step::AddBlock(AddBlockStep {
                    block_id: parent.id(),
                    child_offset: std_block.index(block_map)?,
                    block_type: StandardBlockType::Paragraph(ContentBlock::new(vec![])),
                    focus_block_below: true
                })
            ])
        }
    }

    steps.push(Step::SplitStep(SplitStep { subselection: from.get_deepest_subselection().clone() }));
    return Ok(steps)
}