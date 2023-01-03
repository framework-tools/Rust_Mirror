use std::collections::HashMap;

use crate::{steps_generator::{selection::SubSelection, StepError}, blocks::{BlockMap, standard_blocks::StandardBlock, Block}};

#[derive(PartialEq)]
pub enum BlockStructure {
    Flat,
    Tree
}

pub enum BlocksBetween {
    Flat(Vec<StandardBlock>),
    Tree {
        top_blocks: Vec<StandardBlock>,
        block_map: BlockMap
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
    from: &SubSelection,
    to: &SubSelection,
    block_structure: BlockStructure,
    block_map: &BlockMap
) -> Result<BlocksBetween, StepError> {
    let mut blocks = Vec::new();
    let mut new_block_map = BlockMap::Rust(HashMap::new());
    let current_node = block_map.get_standard_block(&from.clone().get_two_deepest_layers()?.block_id);
    let mut current_node = match current_node {
        Ok(current_node) => Ok(current_node),
        Err(_) => Err(StepError("Expected from block to be std block. Got inline block. Not allowed in this fn".to_string()))
    }?;

    let mut depth_from_root = current_node.depth_from_root(block_map)?;

    let to_second_deepest = to.clone().get_two_deepest_layers()?;
    while current_node.id() != to_second_deepest.block_id {
        let next_node;
        if current_node.children.len() > 0 { // has children
            next_node = block_map.get_standard_block(&current_node.children[0])?;
        } else if current_node.next_sibling(block_map)?.is_some() {
            next_node = current_node.next_sibling(block_map)?.unwrap();
        } else {
            next_node = match current_node.parents_next_sibling(block_map)? {
                Some(sib) => sib,
                None => return Err(StepError("Reached no end of nodes before we found the to block".to_string()))
            };
            if current_node.depth_from_root(block_map)? == 0 {
                depth_from_root = 0;
            }
        }

        if block_structure == BlockStructure::Tree {
            add_block_and_inline_blocks_to_new_block_map(block_map, &mut new_block_map, current_node.clone())?;
        }

        if should_add_block(&current_node, &block_structure, block_map, depth_from_root)? {
            blocks.push(current_node);
        }

        current_node = next_node;
    }

    current_node.children = vec![];
    if block_structure == BlockStructure::Tree {
        add_block_and_inline_blocks_to_new_block_map(block_map, &mut new_block_map, current_node.clone())?;
    }

    if should_add_block(&current_node, &block_structure, block_map, depth_from_root)? {
        blocks.push(current_node);
    }

    return match block_structure {
        BlockStructure::Tree => Ok(BlocksBetween::Tree { top_blocks: blocks, block_map: new_block_map }),
        BlockStructure::Flat => Ok(BlocksBetween::Flat(blocks))
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
        let inline_block = block_map.get_inline_block(id)?;
        new_block_map.update_block(Block::InlineBlock(inline_block), &mut Vec::new())?;
    }
    new_block_map.update_block(Block::StandardBlock(block), &mut Vec::new())?;
    return Ok(())
}