



use crate::{step::{Step, SplitStep, TurnInto}, blocks::{BlockMap, standard_blocks::{StandardBlockType, content_block::ContentBlock}, Block}, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps, turn_into::turn_into_paragraph_step}, new_ids::NewIds};

pub fn generate_steps_for_enter(block_map: &BlockMap, from: SubSelection, to: SubSelection) -> Result<Vec<Step>, StepError> {
    let mut steps = vec![];
    if from != to {
        let delete_selected_steps = generate_replace_selected_steps(block_map, from.clone(), to, "".to_string())?;
        for step in delete_selected_steps {
            steps.push(step);
        }
    } else {
        let std_block = block_map.get_inline_block(&from.block_id)?.get_parent(block_map)?;
        if std_block.is_list() && std_block.text_is_empty(block_map)? {
            return turn_into_paragraph_step(std_block.id())
        }
    }

    steps.push(Step::SplitStep(SplitStep { subselection: from.get_deepest_subselection().clone() }));
    return Ok(steps)
}