use std::{collections::HashMap};

use crate::{steps_generator::{selection::SubSelection, StepError},
    blocks::{BlockMap, standard_blocks::{StandardBlock, content_block::ContentBlock}, Block,
    inline_blocks::{InlineBlock}},
    steps_actualisor::actualise_mark_steps::{actualise_across_std_blocks::split_edge_inline_blocks,
            create_before_middle_after_blocks_with_new_text_and_mark},
            new_ids::NewIds};
pub mod update_state_tools;

#[derive(PartialEq)]
pub enum BlockStructure {
    Flat,
    Tree
}

pub enum BlocksBetween {
    Flat(Vec<StandardBlock>),
    Tree(Tree)
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub top_blocks: Vec<StandardBlock>,
    pub block_map: BlockMap
}

impl Tree {
    pub fn get_last_block(&self) -> Result<InlineBlock, StepError> {
        let mut current_block = self.top_blocks[self.top_blocks.len() - 1].clone();
        if current_block.children.len() > 0{
            current_block = self.block_map.get_standard_block(&current_block.children[0])?;
        } else {
            return current_block.get_last_inline_block(&self.block_map)
        }
        loop {
            let next_sibling = current_block.next_sibling(&self.block_map)?;
            if next_sibling.is_some() {
                current_block = next_sibling.unwrap();
            } else if current_block.children.len() > 0 {
                current_block = self.block_map.get_standard_block(&current_block.children[0])?;
            } else {
                return current_block.get_last_inline_block(&self.block_map)
            }
        }
    }

    /// We need to reassign every id in the blockmap
    /// We need to map top blocks to also contain the new top blocks
    /// We need to ensure the each inline block is changed on it's standard block, and each inline block's
    /// parent is changed to the new id
    /// We need to ensure children are changed both on the parent and the child's parent is updated
    pub fn reassign_ids(&mut self, new_ids: &mut NewIds, blocks_to_update: &mut Vec<String>) -> Result<(), StepError> {
        let all_new_std_blocks = get_all_blocks(
            &self.top_blocks,
            &self.block_map
        )?;

        reassign_ids(all_new_std_blocks, &mut self.top_blocks, &mut self.block_map, new_ids, blocks_to_update)?;

        return Ok(())
    }

}

/// Goes through and gets every standard block that is selected (even partially)
///
/// Gets them in either a tree structure or flat.
///
/// The first and last blocks are edge cases, where they may only have a partial selection
/// on their inline blocks.
///
/// Steps:
/// -> Create a list nodes to store the selected nodes.
/// -> Set currentNode to node1.
/// -> While currentNode is not equal to node2:
/// a. Add currentNode to the nodes list.
/// b. If currentNode has children, set currentNode to the first child.
/// c. If currentNode does not have children, set currentNode to its next sibling.
/// d. If currentNode does not have a next sibling, set currentNode to its parent's next sibling and repeat step c.
/// -> Return the tree
pub fn get_blocks_between(
    block_structure: BlockStructure,
    from: &SubSelection,
    to: &SubSelection,
    block_map: &BlockMap,
    new_ids: &mut NewIds
) -> Result<BlocksBetween, StepError> {
    let mut blocks = Vec::new();
    let mut new_block_map = BlockMap::Rust(HashMap::new());
    let from_second_deepest = from.clone().get_two_deepest_layers();
    let mut current_node = match from_second_deepest {
        Ok(sub_selection) => block_map.get_standard_block(&sub_selection.block_id),
        Err(_) => block_map.get_inline_block(&from.block_id)?.get_parent(block_map),
    }?;
    let depth_from_root = current_node.depth_from_root(block_map);
    let mut depth_from_root = match depth_from_root {
        Ok(v) => v,
        _ => 0
    };

    let mut first = true;
    loop {
        if first && block_structure == BlockStructure::Tree {
            if from.get_deepest_subselection().block_id != to.get_deepest_subselection().block_id {
                current_node = split_edge_block_inline_blocks(
                    true,
                    from,
                    block_map,
                    current_node,
                    &mut new_block_map,
                    new_ids
                )?;
                first = false;
            }
        }

        let to_second_deepest = to.clone().get_two_deepest_layers();
        if to_second_deepest.is_ok() {
            if current_node.id() == to_second_deepest.unwrap().block_id {
                break;
            }
        } else {
            break;
        }

        let next_node = get_next_block_in_tree(&current_node, block_map, &mut depth_from_root)?;

        if block_structure == BlockStructure::Tree {
            add_block_and_inline_blocks_to_new_block_map(block_map, &mut new_block_map, current_node.clone())?;
        }

        if should_add_block(&current_node, &block_structure, block_map, depth_from_root)? {
            blocks.push(current_node);
        }

        current_node = next_node;
    }

    if block_structure == BlockStructure::Tree {
        current_node.children = vec![];
        let deepest_from = from.get_deepest_subselection();
        let deepest_to = to.get_deepest_subselection();
        if deepest_from.block_id == deepest_to.block_id {
            let mut inline_block_selected = block_map.get_inline_block(&deepest_from.block_id)?;
            let (_, middle, _) = inline_block_selected.text()?.split_before_middle_after(deepest_from.offset, deepest_to.offset);
            inline_block_selected = inline_block_selected.update_text(middle)?;
            new_block_map.update_block(Block::InlineBlock(inline_block_selected), &mut Vec::new())?;
        } else {
            current_node = split_edge_block_inline_blocks(
                false,
                to,
                block_map,
                current_node,
                &mut new_block_map,
                new_ids
            )?;
        }
        add_block_and_inline_blocks_to_new_block_map(block_map, &mut new_block_map, current_node.clone())?;
    }

    if should_add_block(&current_node, &block_structure, block_map, depth_from_root)? {
        blocks.push(current_node);
    }

    return match block_structure {
        BlockStructure::Tree => Ok(BlocksBetween::Tree(Tree { top_blocks: blocks, block_map: new_block_map })),
        BlockStructure::Flat => Ok(BlocksBetween::Flat(blocks))
    }
}

pub fn get_next_block_in_tree(current_node: &StandardBlock, block_map: &BlockMap, depth_from_root: &mut usize) -> Result<StandardBlock, StepError> {
    if current_node.children.len() > 0 { // has children
        return block_map.get_standard_block(&current_node.children[0])
    } else if current_node.next_sibling(block_map)?.is_some() {
        return Ok(current_node.next_sibling(block_map)?.unwrap())
    } else {
        let mut next_node = current_node.clone();
        loop {
            match next_node.parents_next_sibling(block_map)? {
                Some(parents_sib) => {
                    next_node = parents_sib;
                    break;
                },
                None => {}
            };
            next_node = match next_node.get_parent(block_map)? {
                Block::StandardBlock(block) => block,
                _ => return Err(StepError("No next node but have not yet reached final node".to_string()))
            };
        }
        if current_node.depth_from_root(block_map)? < *depth_from_root {
            *depth_from_root = current_node.depth_from_root(block_map)?;
        }
        return Ok(next_node)
    }
}

fn should_add_block(
    current_node: &StandardBlock,
    block_structure: &BlockStructure,
    block_map: &BlockMap,
    depth_from_root: usize
) -> Result<bool, StepError> {
    Ok(*block_structure == BlockStructure::Flat ||
    current_node.parent_is_root(block_map) ||
    current_node.depth_from_root(block_map)? <= depth_from_root)
}

fn add_block_and_inline_blocks_to_new_block_map(block_map: &BlockMap, new_block_map: &mut BlockMap, block: StandardBlock) -> Result<(), StepError> {
    let content_block = block.content_block()?;
    for id in &content_block.inline_blocks {
        if !new_block_map.contains(id) {
            let inline_block = block_map.get_inline_block(id)?;
            new_block_map.update_block(Block::InlineBlock(inline_block), &mut Vec::new())?;
        }
    }
    new_block_map.update_block(Block::StandardBlock(block), &mut Vec::new())?;
    return Ok(())
}

fn split_edge_block_inline_blocks(
    for_from: bool,
    subselection: &SubSelection,
    block_map: &BlockMap,
    current_node: StandardBlock,
    new_block_map: &mut BlockMap,
    new_ids: &mut NewIds
) -> Result<StandardBlock, StepError> {
    let deepest_subselection = subselection.get_deepest_subselection();
    let from_inline_block = block_map.get_inline_block(&deepest_subselection.block_id)?;
    let (first_half, second_half) = from_inline_block.split(deepest_subselection.offset, new_ids)?;

    let inline_index = current_node.index_of(&deepest_subselection.block_id)?;
    let mut inline_blocks = current_node.content_block()?.inline_blocks.clone();
    if for_from {
        inline_blocks.splice(0..inline_index + 1, vec![second_half.id()]);
        new_block_map.update_block(Block::InlineBlock(second_half), &mut Vec::new())?;
    } else {
        inline_blocks.splice(inline_index.., vec![first_half.id()]);
        new_block_map.update_block(Block::InlineBlock(first_half), &mut Vec::new())?;
    }
    return Ok(current_node.update_block_content(ContentBlock { inline_blocks })?)
}

/// If the top blocks are all the blocks on the root, this will get you every block in the tree,
/// flat and in order.
///
/// However, we have generalised it so you can give it a limited "top blocks" to get only and all the top blocks
/// and all their children.
pub fn get_all_blocks(top_blocks: &Vec<StandardBlock>, block_map: &BlockMap) -> Result<Vec<StandardBlock>, StepError> {
    let mut standard_blocks: Vec<StandardBlock> = Vec::new();

    let mut current_node = top_blocks[0].clone();
    let mut current_top_block_i = 0;

    loop {
        let next_node;
        if current_node.children.len() > 0 { // has children
            next_node = block_map.get_standard_block(&current_node.children[0])?;
        } else {
            match current_node.next_sibling(block_map) {
                Ok(Some(sibling)) => {
                    next_node = sibling;
                    if top_blocks.contains(&current_node) && top_blocks.contains(&next_node) {
                        current_top_block_i += 1;
                    } else if top_blocks.contains(&current_node) {
                        if !standard_blocks.contains(&current_node) {
                            standard_blocks.push(current_node);
                        }
                        break;
                    }
                },
                _ if !top_blocks.contains(&current_node) => {
                    match current_node.parents_next_sibling(block_map) {
                        Ok(Some(parents_sib)) => {
                            next_node = parents_sib;
                            if top_blocks.contains(&current_node) && top_blocks.contains(&next_node) {
                                current_top_block_i += 1;
                            } else if top_blocks.contains(&block_map.get_standard_block(&current_node.parent)?) {
                                if !standard_blocks.contains(&current_node) {
                                    standard_blocks.push(current_node);
                                }
                                break;
                            }
                        },
                        _ => match current_node.get_parent(block_map) {
                            Ok(Block::StandardBlock(block)) if !standard_blocks.contains(&block) =>{
                                if !standard_blocks.contains(&current_node) {
                                    standard_blocks.push(current_node);
                                }
                                current_node = block;
                                continue;
                            },
                            _ => {
                                current_top_block_i += 1;
                                match top_blocks.get(current_top_block_i) {
                                    Some(_) => next_node = top_blocks[current_top_block_i].clone(),
                                    None => {
                                        if !standard_blocks.contains(&current_node) {
                                            standard_blocks.push(current_node);
                                        }
                                        break;
                                    },
                                };
                            }
                        }
                    };
                },
                _ => {
                    if !standard_blocks.contains(&current_node) {
                        standard_blocks.push(current_node);
                    };
                    break;
                }
            };
        }

        if !standard_blocks.contains(&current_node) {
            standard_blocks.push(current_node);
        }
        current_node = next_node;
    }

    return Ok(standard_blocks)
}

pub fn caret_is_at_start_of_block(from: &SubSelection, to: &SubSelection, block_map: &BlockMap) -> Result<bool, StepError> {
    if from != to {
        return Ok(false)
    }

    if from.offset != 0 {
        return Ok(false)
    }

    let block = block_map.get_inline_block(&from.block_id);
    if block.is_err() {
        return Ok(false)
    }

    let block = block.unwrap();
    if block.index(block_map)? == 0 {
        return Ok(true)
    } else {
        return Ok(false)
    }
}

pub fn reassign_ids(
    blocks: Vec<StandardBlock>,
    top_blocks: &mut Vec<StandardBlock>,
    block_map: &mut BlockMap,
    new_ids: &mut NewIds,
    blocks_to_update: &mut Vec<String>
) -> Result<(), StepError> {
    // return Err(StepError(format!("Blocks: {:#?}", blocks)));

    let mut new_blocks: HashMap<String, Block> = HashMap::new();
    let mut new_top_blocks = Vec::new();
    for mut block in blocks {
        let new_std_block_id = new_ids.get_id()?;
        let old_id = block.id();
        block._id = new_std_block_id.clone();
        if top_blocks.iter().any(|x| x.id() == old_id) {
            new_top_blocks.push(block.clone());
        } else { // else must be a child
            let old_parent_id = block.parent.clone();
            let parent = new_blocks.get(&old_parent_id);
            let mut parent = match parent {
                Some(parent) => parent.clone(),
                None => return Err(StepError(format!("Should never happen as parent should already be added
                to new blocks. Block where this occured: {:#?}", block)))
            };
            block.parent = parent.id();
            let parent_children = parent.children()?.iter().map(|x| {
                if x == &old_id {
                    return new_std_block_id.clone()
                }
                return x.clone()
            }).collect();
            parent.update_children(parent_children)?;
            new_blocks.insert(old_parent_id, parent);
        }

        let inline_blocks = &block.content_block()?.inline_blocks;
        let mut new_inline_blocks = Vec::new();
        for old_inline_id in inline_blocks {
            let new_inline_block_id = new_ids.get_id()?;
            let mut inline_block = block_map.get_inline_block(old_inline_id)?;
            inline_block._id = new_inline_block_id.clone();
            inline_block.parent = new_std_block_id.clone();
            new_inline_blocks.push(new_inline_block_id);
            new_blocks.insert(old_inline_id.clone(), Block::InlineBlock(inline_block));
        }

        block = block.update_block_content(ContentBlock { inline_blocks: new_inline_blocks })?;


        new_blocks.insert(old_id, Block::StandardBlock(block));
    }

    for (_id, block) in new_blocks.into_iter() {
        block_map.update_block(block, blocks_to_update)?;
    }

    let mut updated_new_top_blocks = Vec::new();
    for block in new_top_blocks{
        updated_new_top_blocks.push(block_map.get_standard_block(&block.id())?);
    }
    *top_blocks = updated_new_top_blocks;

    return Ok(())
}