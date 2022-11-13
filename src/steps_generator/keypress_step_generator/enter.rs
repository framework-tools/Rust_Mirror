



use crate::{step::{Step, SplitStep}, blocks::{BlockMap, standard_blocks::{StandardBlock, StandardBlockType, content_block::ContentBlock}, Block}, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps}, new_ids::NewIds};

pub fn generate_steps_for_enter(block_map: &BlockMap, from: SubSelection, to: SubSelection) -> Result<Vec<Step>, StepError> {
    let mut steps = vec![];
    if from != to {
        let delete_selected_steps = generate_replace_selected_steps(block_map, from.clone(), to, "".to_string())?;
        for step in delete_selected_steps {
            steps.push(step);
        }
    }

    steps.push(Step::SplitStep(SplitStep { subselection: from }));
    return Ok(steps)
}