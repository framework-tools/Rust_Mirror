use std::ops::Range;

use crate::{blocks::{Block, BlockMap, inline_blocks::{self, InlineBlock, text_block::StringUTF16}, standard_blocks::{StandardBlock, content_block::ContentBlock}}, steps_generator::StepError, mark::Mark, new_ids};

pub fn splice_children(
    mut block: Block, 
    range: Range<usize>, 
    new_children: Vec<String>, 
    blocks_to_update: &mut Vec<String>,
    block_map: &mut BlockMap,
) -> Result<(), StepError> {
    for child in &new_children{
        let mut child = block_map.get_standard_block(child)?;
        child.parent = block.id();
        block_map.update_block(Block::StandardBlock(child), blocks_to_update)?;
    }
    let mut children = block.children()?.clone();
    children.splice(range, new_children);
    block.update_children(children)?;
    block_map.update_block(block.clone(), blocks_to_update)?;
    return Ok(())    
}

pub fn splice_children_on_std_block(
    mut block: &mut StandardBlock, 
    range: Range<usize>, 
    new_children: Vec<String>, 
    blocks_to_update: &mut Vec<String>,
    block_map: &mut BlockMap,
) -> Result<(), StepError> {
    for child in &new_children{
        let mut child = block_map.get_standard_block(child)?;
        child.parent = block.id();
        block_map.update_block(Block::StandardBlock(child), blocks_to_update)?;
    }
    let mut children = block.children.clone();
    children.splice(range, new_children);
    block.children = children;
    block_map.update_block(Block::StandardBlock(block.clone()), blocks_to_update)?;
    return Ok(())    
}

pub fn splice_inline_blocks(
    mut block: StandardBlock, 
    range: Range<usize>, 
    new_inline_blocks: Vec<String>, 
    blocks_to_update: &mut Vec<String>,
    block_map: &mut BlockMap,
) -> Result<StandardBlock, StepError> {
    for inline_block in &new_inline_blocks{
        let mut inline_block = block_map.get_inline_block(inline_block)?;
        inline_block.parent = block.id();
        block_map.update_block(Block::InlineBlock(inline_block), blocks_to_update)?;
    }
    let mut inline_blocks = block.content_block()?.clone().inline_blocks;
    inline_blocks.splice(range, new_inline_blocks);
    block = block.update_block_content(ContentBlock{inline_blocks})?;
    block_map.update_block(Block::StandardBlock(block.clone()), blocks_to_update)?;
    return Ok(block)
}

pub fn update_text(
    mut inline_block: InlineBlock, 
    new_text: StringUTF16, 
    blocks_to_update: &mut Vec<String>,
    block_map: &mut BlockMap,
) -> Result<(), StepError> {
    inline_block = inline_block.update_text(new_text)?;
    block_map.update_block(Block::InlineBlock(inline_block.clone()), blocks_to_update)?;
    return Ok(())    
}

pub fn update_mark(
    mut inline_block: InlineBlock, 
    mark: Mark, 
    add_mark: bool,
    blocks_to_update: &mut Vec<String>,
    block_map: &mut BlockMap,
) -> Result<(), StepError> {
    inline_block = inline_block.apply_mark(mark, add_mark);
    block_map.update_block(Block::InlineBlock(inline_block.clone()), blocks_to_update)?;
    return Ok(())    
}

pub fn split_inline_block(
    inline_block: InlineBlock, 
    offset: usize, 
    blocks_to_update: &mut Vec<String>,
    block_map: &mut BlockMap,
    new_ids: &mut new_ids::NewIds,
) -> Result<(InlineBlock, InlineBlock), StepError> {
    let (left, right) = inline_block.split(offset, new_ids)?;
    block_map.update_block(Block::InlineBlock(left.clone()), blocks_to_update)?;
    block_map.update_block(Block::InlineBlock(right.clone()), blocks_to_update)?;
    return Ok((left, right))  
}
