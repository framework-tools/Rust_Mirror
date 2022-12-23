use crate::{steps_generator::{selection::SubSelection, StepError}, blocks::{BlockMap, standard_blocks::StandardBlock}};

#[derive(PartialEq)]
pub enum BlockStructure {
    Flat,
    Tree
}

/// Goes through and gets every standard block that is selected (even partially)
///
/// Gets them all in the tree structure.
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
pub fn get_std_blocks_between(
    from: &SubSelection,
    to: &SubSelection,
    block_structure: BlockStructure,
    block_map: &BlockMap
) -> Result<Vec<StandardBlock>, StepError> {
    let mut top_blocks = Vec::new();
    let current_node = block_map.get_standard_block(&from.block_id);
    let mut current_node = match current_node {
        Ok(current_node) => Ok(current_node),
        Err(_) => Err(StepError("Expected from block to be std block. Got inline block. Not allowed in this fn".to_string()))
    }?;

    let mut depth_from_root = current_node.depth_from_root(block_map)?;

    while current_node.id() != to.block_id {
        let next_node;
        if current_node.children.len() > 0 {
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

        if should_add_block(&current_node, &block_structure, block_map, depth_from_root)? {
            top_blocks.push(current_node);
        }

        current_node = next_node;
    }
    if should_add_block(&current_node, &block_structure, block_map, depth_from_root)? {
        top_blocks.push(current_node);
    }

    return Ok(top_blocks)
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