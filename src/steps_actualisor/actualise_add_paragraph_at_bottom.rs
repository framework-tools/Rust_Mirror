use crate::{blocks::{BlockMap, inline_blocks::InlineBlock, standard_blocks::StandardBlock, Block}, new_ids::NewIds, steps_generator::{StepError, selection::{SubSelection, Selection}}, utilities::update_state_tools};
use super::UpdatedState;

pub fn actualise_add_paragraph_at_bottom(
    root_block_id: String,
    mut block_map: BlockMap,
    new_ids: &mut NewIds,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let new_paragraph_id = new_ids.get_id()?;
    let new_inline_block = InlineBlock::new(new_ids, new_paragraph_id.clone())?;
    let inline_id = new_inline_block.id();
    let new_paragraph = StandardBlock::new_paragraph_block(
        new_paragraph_id.clone(),
        vec![new_inline_block.id()],
        vec![],
        vec![],
        root_block_id.clone()
    );
    block_map.update_blocks(vec![
        Block::StandardBlock(new_paragraph),
        Block::InlineBlock(new_inline_block)
    ], &mut blocks_to_update)?;

    let root = block_map.get_root_block(&root_block_id)?;
    let children_len = root.children.len();
    update_state_tools::splice_children(
        Block::Root(root),
        children_len..children_len,
        vec![new_paragraph_id.clone()],
        &mut blocks_to_update,
        &mut block_map
    )?;

    let new_subselection = SubSelection { block_id: inline_id, offset: 0, subselection: None };
    return Ok(UpdatedState {
        block_map,
        selection: Some(Selection {
            anchor: new_subselection.clone(),
            head: new_subselection
        }),
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}