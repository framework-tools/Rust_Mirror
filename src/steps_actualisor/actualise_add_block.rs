use crate::{step::AddBlockStep, blocks::{BlockMap, inline_blocks::InlineBlock, standard_blocks::{StandardBlock, content_block::ContentBlock}, Block}, steps_generator::{StepError, selection::{Selection, SubSelection}}, new_ids::NewIds};

use super::UpdatedState;

// This function appears to implement the logic for adding a new block to a document.

// The AddBlockStep struct specifies the type of block to add, 
//the ID of the parent block it should be added to, 
//and the offset of the new block within the parent block's list of children.

// The function first retrieves the parent block 
//from the block_map and generates a new ID for the new block. 
//Then, it creates a new inline block and sets the new standard block's content to be this inline block. 
//The new standard block is then inserted into the parent block's list of children at the specified offset. 
//Finally, the inline and standard blocks, as well as the updated parent block, 
//are added to the block_map and the function returns an UpdatedState object 
//with the updated block_map and a new selection.
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
        blocks_to_remove: vec![],
        copy: None
    })
}