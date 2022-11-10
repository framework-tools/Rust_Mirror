use crate::{blocks::BlockMap, steps_generator::{selection::SubSelection, StepError, generate_replace_selected_steps::generate_replace_selected_steps}, step::{Step, ReplaceStep}};


pub fn generate_steps_for_backspace(
    block_map: &BlockMap,
    mut from: SubSelection,
    to: SubSelection
) -> Result<Vec<Step>, StepError> {
    let from_standard_block = block_map.get_standard_block(&from.block_id);
    if from_standard_block.is_ok() {
        unimplemented!()
        // let from_standard_block = from_standard_block.unwrap();
        // return Ok(vec![Step::ReplaceStep(ReplaceStep {
        //     block_id: from_standard_block.parent,
        //     from,
        //     to,
        //     slice: vec![],
        //     blocks_to_update: vec![]
        // })])
    }else {
        if from == to && from.offset != 0 {
            from.offset -= 1;
        }
        return generate_replace_selected_steps(block_map, from, to, "".to_string())
    }
}