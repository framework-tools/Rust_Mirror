use std::{collections::HashMap};

use crate::{blocks::BlockMap, step::Step, steps_generator::StepError, new_ids::NewIds, steps_actualisor::{actualise_steps, UpdatedState}, custom_copy::CustomCopy};



pub fn backend_actualise_mirror_steps(
    steps_as_json: Vec<(String, String)>,
    new_ids: Vec<String>,
    block_map_rust: HashMap<String, String>
) -> Result<HashMap<String, String>, StepError> {
    let mut block_map = BlockMap::Rust(block_map_rust);
    let mut new_ids = NewIds::Rust(new_ids);

    for (_type, data) in steps_as_json {
        let step = Step::from_json(&_type, &data)?;
        let updated_state = 
            actualise_steps(vec![step.clone()], block_map, &mut new_ids, CustomCopy::new())?;
        block_map = updated_state.block_map;
    }
    return match block_map {
        BlockMap::Rust(block_map) => Ok(block_map),
        BlockMap::Js(_) => unreachable!("block_map should be Rust")
    }
}