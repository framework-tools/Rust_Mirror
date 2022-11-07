

use crate::{step::{Step}, blocks::{BlockMap}, steps_generator::{selection::SubSelection, replace_selected::{replace_selected}}};

use super::{StepError};


/// if the from is an inline block
///    -> return a replace step with the updated text (insert char at correct position in text)
///    from it's parent from the index of the first block to the index of the next block
/// else
///     -> replace from the first block to the second block with a single updated standard block in the block that is there parent
pub fn generate_step_for_standard_key(
    key: char,
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection
) -> Result<Vec<Step>, StepError> {
    return replace_selected(block_map, from, to, key.to_string())
}