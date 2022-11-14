use crate::{steps_generator::StepError, blocks::BlockMap, step::TurnToChild};

use super::UpdatedState;



pub fn execute_child_steps(block_map: BlockMap, turn_to_child_step: TurnToChild) -> Result<UpdatedState, StepError> {

    return Ok(UpdatedState { block_map, selection: None })
}