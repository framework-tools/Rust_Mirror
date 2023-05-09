use crate::{step::{Step, PasteStep}, blocks::BlockMap, custom_copy::CustomCopy};

use super::{selection::SubSelection, StepError, keypress_step_generator::backspace::generate_steps_for_backspace};


pub fn generate_cut_steps(from: SubSelection, to: SubSelection, block_map: &BlockMap) -> Result<Vec<Step>, StepError> {
    if from == to {
        return Ok(vec![Step::Copy(from.clone(), to.clone())])
    } else {
        return Ok(vec![
            vec![Step::Copy(from.clone(), to.clone())],
            generate_steps_for_backspace(block_map, from, to)?,
        ].into_iter().flatten().collect())
    }
}

pub fn generate_paste_steps(from: SubSelection, to: SubSelection, block_map: &BlockMap, copy: CustomCopy) -> Result<Vec<Step>, StepError> {
    if from == to {
        return Ok(vec![Step::Paste(PasteStep {
            from,
            to,
            copy_tree: copy.to_tree()?
        })])
    } else {
        return Ok(vec![
            generate_steps_for_backspace(block_map, from.clone(), to.clone())?,
            vec![Step::Paste(PasteStep {
                from,
                to,
                copy_tree: copy.to_tree()?
            })]
        ].into_iter().flatten().collect())
    }
}