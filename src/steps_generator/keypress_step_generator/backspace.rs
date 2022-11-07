use crate::{blocks::BlockMap, steps_generator::{selection::SubSelection, StepError, replace_selected::{replace_selected}}, step::{Step, ReplaceStep}};


pub fn generate_steps_for_backspace(
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection
) -> Result<Vec<Step>, StepError> {
    let from_standard_block = block_map.get_standard_block(&from.block_id);
    if from_standard_block.is_ok() {
        let from_standard_block = from_standard_block.unwrap();
        return Ok(vec![Step::ReplaceStep(ReplaceStep {
            block_id: from_standard_block.parent,
            from,
            to,
            slice: vec![],
            blocks_to_update: vec![]
        })])
    }else {
        return replace_selected(block_map, from, to, "".to_string())
    } 
    
}