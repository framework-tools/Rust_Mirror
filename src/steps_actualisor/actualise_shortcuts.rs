use std::ops::Index;

use crate::{utilities::{BlocksBetween, get_blocks_between, BlockStructure, Tree, get_all_blocks, update_state_tools}, custom_copy::CustomCopy, steps_generator::{StepError, selection::{SubSelection, Selection}}, blocks::{BlockMap, standard_blocks::{StandardBlock, content_block::ContentBlock}, Block}, new_ids::NewIds};

use super::{UpdatedState, clean_block_after_transform};


pub fn actualise_copy(
    mut copy: CustomCopy,
    from: SubSelection,
    to: SubSelection,
    block_map: BlockMap,
    new_ids: &mut NewIds,
    blocks_to_update: Vec<String>
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
        selection: Some(Selection { anchor: from, head: to }),
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: Some(copy)
    })
}

/// -> first blocks inline blocks get inserted at "from" selection standard block -> similar to inline case
/// -> any children of first block are inserted at start of insertion block's children
/// -> Rest of the blocks get inserted into the "from" std block's parents underneath it
pub fn actualise_paste(
    copy: CustomCopy,
    from: SubSelection,
    mut block_map: BlockMap,
    new_ids: &mut NewIds,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let mut copy_tree = copy.to_tree()?;
    copy_tree.reassign_ids(new_ids)?;
    let last_block = copy_tree.get_last_block()?;
    
    block_map.add_block_map(copy_tree.block_map)?;
    
    let mut selection = None;
    if copy_tree.top_blocks.len() > 0 {
        selection = Some(Selection { 
            anchor: SubSelection::at_end_of_block(&last_block._id, &block_map)?, 
            head: SubSelection::at_end_of_block(&last_block._id, &block_map)? 
        });
        
        let insertion_std_block = block_map.get_nearest_ancestor_standard_block_incl_self(&from.block_id)?;
        
        let mut insertion_std_block = paste_inline_blocks(
            insertion_std_block, 
            from.get_deepest_subselection().clone(), 
            copy_tree.top_blocks[0].clone(),
            &mut block_map, 
            new_ids, 
            &mut blocks_to_update
        )?;
        update_state_tools::splice_children_on_std_block(
            &mut insertion_std_block, 
            0..0, 
            copy_tree.top_blocks[0].children.clone(), 
            &mut blocks_to_update, 
            &mut block_map
        )?;
        let parent = insertion_std_block.get_parent(&block_map)?;
        copy_tree.top_blocks.remove(0);
        update_state_tools::splice_children(
            parent, 
            insertion_std_block.index(&block_map)? + 1..insertion_std_block.index(&block_map)? + 1,
            copy_tree.top_blocks.iter().map(|b| b._id.clone()).collect(),
            &mut blocks_to_update, 
            &mut block_map
        )?;

        block_map = clean_block_after_transform(insertion_std_block, block_map, &mut blocks_to_update)?;
    }

    return Ok(UpdatedState {
        block_map,
        selection,
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}

/// Split inline block at selection
/// Splice new inline blocks at selection
fn paste_inline_blocks(
    mut insertion_block: StandardBlock,
    deepest_subselection: SubSelection,
    first_copied_block: StandardBlock,
    block_map: &mut BlockMap,
    new_ids: &mut NewIds,
    blocks_to_update: &mut Vec<String>
) -> Result<StandardBlock, StepError> {
    let insertion_block_id = insertion_block.get_inline_block_from_index(insertion_block.index_of(&deepest_subselection.block_id)?)?;
    let insertion_inline_block = block_map.get_inline_block(&insertion_block_id)?;
    let (_first_half, second_half) = update_state_tools::split_inline_block(insertion_inline_block, deepest_subselection.offset, blocks_to_update, block_map, new_ids)?;
    
    let inline_blocks_to_paste = first_copied_block.get_inline_blocks(&block_map)?;
    let mut new_inline_blocks_ids: Vec<String> = inline_blocks_to_paste.iter().map(|b| b._id.clone()).collect();
    new_inline_blocks_ids.push(second_half._id.clone());

    let index = insertion_block.index_of(&deepest_subselection.block_id)? + 1;
    insertion_block = update_state_tools::splice_inline_blocks(
        insertion_block, 
        index..index,
        new_inline_blocks_ids, 
        blocks_to_update,
        block_map
    )?;

    return Ok(insertion_block)
}