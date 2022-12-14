use crate::{step::AddBlockStep, blocks::{BlockMap, inline_blocks::InlineBlock, standard_blocks::{StandardBlock, content_block::ContentBlock}, Block}, steps_generator::{StepError, selection::{Selection, SubSelection}}, new_ids::NewIds};

use super::UpdatedState;


pub fn actualise_add_block(
    add_block_step: AddBlockStep,
    mut block_map: BlockMap,
    new_ids: &mut NewIds,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let mut parent = block_map.get_block(&add_block_step.block_id)?;
    let new_std_block_id = new_ids.get_id()?;
    let new_inline_block = InlineBlock::new(new_ids, new_std_block_id.clone())?;

    let new_block_type = add_block_step.block_type.update_block_content(
        ContentBlock { inline_blocks: vec![new_inline_block.id()] }
    )?;
    let new_std_block = StandardBlock {
        _id: new_std_block_id,
        content: new_block_type,
        children: vec![],
        parent: parent.id(),
        marks: vec![],
    };

    parent.insert_child(new_std_block.id(), add_block_step.child_offset)?;

    let new_inline_block_id = new_inline_block.id();
    block_map.update_blocks(vec![
        Block::InlineBlock(new_inline_block), Block::StandardBlock(new_std_block), parent
    ], &mut blocks_to_update)?;

    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection {
            anchor: SubSelection { block_id: new_inline_block_id.clone(), offset: 0, subselection: None },
            head: SubSelection { block_id: new_inline_block_id.clone(), offset: 0, subselection: None },
        }),
        blocks_to_update,
        blocks_to_remove: vec![]
    })
}