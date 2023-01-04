use crate::{utilities::{BlocksBetween, get_blocks_between, BlockStructure}, custom_copy::CustomCopy, steps_generator::{StepError, selection::SubSelection}, blocks::BlockMap, new_ids::NewIds};

use super::UpdatedState;


pub fn actualise_copy(
    mut copy: CustomCopy,
    from: SubSelection,
    to: SubSelection,
    block_map: BlockMap,
    new_ids: &mut NewIds
) -> Result<UpdatedState, StepError> {
    let blocks_between = get_blocks_between(
        BlockStructure::Tree,
        &from,
        &to,
        &block_map,
        new_ids
    )?;
    match blocks_between {
        BlocksBetween::Tree(tree) => copy = copy.update(tree)?,
        BlocksBetween::Flat(_) => return Err(StepError("Should get blocks as tree".to_string())),
    };

    return Ok(UpdatedState {
        block_map,
        selection: None,
        blocks_to_update: vec![],
        blocks_to_remove: vec![],
        copy: Some(copy)
    })
}