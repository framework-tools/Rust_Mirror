use std::ops::Index;

use crate::{utilities::{BlocksBetween, get_blocks_between, BlockStructure, Tree, get_all_blocks}, custom_copy::CustomCopy, steps_generator::{StepError, selection::{SubSelection, Selection}}, blocks::{BlockMap, standard_blocks::{StandardBlock, content_block::ContentBlock}, Block}, new_ids::NewIds};

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
    let copy_tree = copy.to_tree()?;
    if copy_tree.top_blocks.len() > 0 {
        let insertion_std_block = block_map.get_nearest_ancestor_standard_block_incl_self(&from.block_id)?;
        let new_inline_blocks = copy_tree.top_blocks[0].get_inline_blocks(&copy_tree.block_map)?;

        let mut insertion_std_block = insertion_std_block.insert_inline_blocks(
            new_inline_blocks,
            from.get_deepest_subselection().clone(),
            &mut block_map,
            new_ids,
            &mut blocks_to_update
        )?;
        if insertion_std_block.children.len() > 0 {
            insertion_std_block = add_children_to_start_of_blocks_children(
                insertion_std_block,
                copy_tree.top_blocks[0].clone(),
                &copy_tree,
                &mut block_map,
                new_ids,
                &mut blocks_to_update
            )?;
        }
        // let insertion_std_block = add_all_other_blocks_below(insertion_std_block, copy_tree, &mut block_map, new_ids, &mut blocks_to_update)?;

        block_map = clean_block_after_transform(insertion_std_block, block_map, &mut blocks_to_update)?;
    }

    return Ok(UpdatedState {
        block_map,
        selection: None,
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}

/// -> All children of first block gets spliced at start of insertion block's children
/// -> Set top level children added new parent to insertion block
/// -> Set ALL descendants added to have new ids and add them to our block map
fn add_children_to_start_of_blocks_children(
    mut insertion_block: StandardBlock,
    mut first_copied_block: StandardBlock,
    copy_tree: &Tree,
    block_map: &mut BlockMap,
    new_ids: &mut NewIds,
    blocks_to_update: &mut Vec<String>
) -> Result<StandardBlock, StepError> {
    let all_new_std_blocks = get_all_blocks(
        vec![first_copied_block.id()],
        &copy_tree.block_map
    )?;
    for mut block in all_new_std_blocks {
        let new_std_id = new_ids.get_id()?;
        let index_of_child = first_copied_block.children.iter().position(|x| x == &block._id);
        if index_of_child.is_some() {
            first_copied_block.children[index_of_child.unwrap()] = new_std_id.clone();
        }
        block._id = new_std_id.clone();
        let mut new_inline_ids = Vec::new();
        for id in  &block.content_block()?.inline_blocks {
            let new_id = new_ids.get_id()?;
            new_inline_ids.push(new_id.clone());
            let mut inline_block = copy_tree.block_map.get_inline_block(&id)?;
            inline_block._id = new_id;
            inline_block.parent = block.id();
            block_map.update_block(Block::InlineBlock(inline_block), blocks_to_update)?;
        }

        block = block.update_block_content(ContentBlock { inline_blocks: new_inline_ids })?;
        block_map.update_block(Block::StandardBlock(block), blocks_to_update)?;
    }

    insertion_block.children.splice(0..0, first_copied_block.children);
    return Ok(insertion_block)
}

fn add_all_other_blocks_below(
    insertion_block: StandardBlock,
    mut copy_tree: Tree,
    block_map: &mut BlockMap,
    new_ids: &mut NewIds,
    blocks_to_update: &mut Vec<String>
) -> Result<StandardBlock, StepError> {
    let mut parent_insertion_block = insertion_block.get_parent(block_map)?;
    let index_of_insertion_block = insertion_block.index(block_map)?;

    copy_tree.top_blocks.remove(0);
    let all_new_std_blocks = get_all_blocks(
        copy_tree.top_blocks.iter().map(|x| x.id()).collect(),
        &copy_tree.block_map
    )?;

    let mut new_top_blocks_ids = Vec::new();
    for mut block in all_new_std_blocks {
        let new_std_id = new_ids.get_id()?;
        let top_block_index = copy_tree.top_blocks.iter().position(|x| &x._id == &block._id);
        if top_block_index.is_some() {
            new_top_blocks_ids.push(new_std_id.clone());
        }
        block._id = new_std_id.clone();
        let mut new_inline_ids = Vec::new();
        for id in  &block.content_block()?.inline_blocks {
            let new_id = new_ids.get_id()?;
            new_inline_ids.push(new_id.clone());
            let mut inline_block = copy_tree.block_map.get_inline_block(&id)?;
            inline_block._id = new_id;
            inline_block.parent = block.id();
            block_map.update_block(Block::InlineBlock(inline_block), blocks_to_update)?;
        }

        block = block.update_block_content(ContentBlock { inline_blocks: new_inline_ids })?;
        block_map.update_block(Block::StandardBlock(block), blocks_to_update)?;
    }

    let mut children = parent_insertion_block.children()?.clone();
    children.splice(index_of_insertion_block..index_of_insertion_block, new_top_blocks_ids);
    parent_insertion_block.update_children(children)?;
    block_map.update_block(parent_insertion_block, blocks_to_update)?;

    return Ok(insertion_block)
}