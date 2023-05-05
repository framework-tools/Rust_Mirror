use crate::{step::MarkStep, blocks::{standard_blocks::{StandardBlock, content_block::ContentBlock}, BlockMap, Block}, steps_actualisor::{UpdatedState, clean_block_after_transform}, steps_generator::{StepError, selection::SubSelection}, new_ids::NewIds, utilities::{get_blocks_between, BlockStructure, BlocksBetween}};

/// -> apply mark for "from" std block -> from "inner from" to end of inline blocks
/// -> apply mark for "to" std block -> from start of inline blocks to "inner to"
/// -> for each standard block between "from" & "to" -> assign mark to each of their inline blocks

///--------------------------------------------------

// This looks like a function that updates a document by applying a "mark" to a range of blocks. 
//It appears to first split any inline blocks at the start and end of the range to be marked, 
//then it applies the mark to all inline blocks in the range. Finally, 
//it cleans up the document by removing any unnecessary blocks.
// The function takes a MarkStep as input, 
//which represents the mark to be applied and the range of blocks to which it should be applied. 
//It also takes a BlockMap which seems to be a map of block IDs to blocks, 
//a boolean add_mark which indicates whether the mark should be added or removed, 
//and a mutable reference to a NewIds struct which seems to be used to generate new unique IDs for blocks as needed. 
//Finally, it takes a mutable vector of strings blocks_to_update, 
//which appears to be used to track which blocks need to be updated in the document.
// The function returns a Result object with an UpdatedState variant, 
//which appears to contain information about the updated document. 
//If an error occurs, a StepError variant is returned instead.

pub fn actualise_mark_step_on_standard_blocks(
    mark_step: MarkStep,
    mut block_map: BlockMap,
    add_mark: bool,
    new_ids: &mut NewIds,
    mut blocks_to_update: Vec<String>
) -> Result<UpdatedState, StepError> {
    let from_second_deepest_layer = mark_step.from.clone().get_two_deepest_layers()?;
    let from_deepest_layer = mark_step.from.get_deepest_subselection();
    let from_deepest_std_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;
    let to_second_deepest_layer = mark_step.to.clone().get_two_deepest_layers()?;
    let to_deepest_layer = mark_step.to.get_deepest_subselection();
    let to_deepest_std_block = block_map.get_standard_block(&to_second_deepest_layer.block_id)?;
    split_edge_inline_blocks(&mut block_map, new_ids, from_deepest_layer, from_deepest_std_block, &mut blocks_to_update)?;
    split_edge_inline_blocks(&mut block_map, new_ids, to_deepest_layer, to_deepest_std_block, &mut blocks_to_update)?;

    match get_blocks_between(BlockStructure::Flat, &mark_step.from, &mark_step.to, &block_map, new_ids)? {
        BlocksBetween::Flat(blocks) => {
            let mut i = 0;
            for block in &blocks {
                if i == 0 {
                    block.apply_mark_to_all_inline_blocks_in_range(
                        mark_step.mark.clone(),
                        block.index_of(&from_deepest_layer.block_id)? + 1,
                        block.content_block()?.inline_blocks.len() - 1,
                        &mut block_map,
                        add_mark,
                        &mut blocks_to_update
                    )?;
                }
                else if i == blocks.len() - 1 {
                    block.apply_mark_to_all_inline_blocks_in_range(
                        mark_step.mark.clone(),
                        0,
                        block.index_of(&to_deepest_layer.block_id)?,
                        &mut block_map,
                        add_mark,
                        &mut blocks_to_update
                    )?;
                }
                else  {
                    block.apply_mark_to_all_inline_blocks(mark_step.mark.clone(), &mut block_map, add_mark, &mut blocks_to_update)?
                }
                i += 1;
            }
        },
        _ => unreachable!()
    }

    let from_second_deepest_layer = mark_step.from.clone().get_two_deepest_layers()?;
    let from_deepest_std_block = block_map.get_standard_block(&from_second_deepest_layer.block_id)?;
    let to_second_deepest_layer = mark_step.to.clone().get_two_deepest_layers()?;
    let to_deepest_std_block = block_map.get_standard_block(&to_second_deepest_layer.block_id)?;
    block_map = clean_block_after_transform(from_deepest_std_block, block_map, &mut blocks_to_update)?;
    block_map = clean_block_after_transform(to_deepest_std_block, block_map, &mut blocks_to_update)?;

    return Ok(UpdatedState {
        block_map,
        selection: None,
        blocks_to_update,
        blocks_to_remove: vec![],
        copy: None
    })
}

// This function appears to be used to split an inline block at a given offset. 
//It takes a mutable reference to a BlockMap, 
//a mutable reference to a NewIds struct, 
//a reference to a SubSelection struct called deepest_layer, 
//and a StandardBlock called deepest_std_block. 
//It also takes a mutable reference to a vector of strings called blocks_to_update, 
//which appears to be used to track which blocks need to be updated in the document.
// The function first retrieves the inline block specified in deepest_layer, 
//and then splits it into two blocks at the given offset. 
//It then updates the list of inline blocks in the deepest_std_block to include the newly created block, 
//and updates the BlockMap with the new blocks and the updated standard block. 
//Finally, the function returns Ok(()) to indicate success, or Err(StepError) if an error occurred.

pub fn split_edge_inline_blocks(
    block_map: &mut BlockMap,
    new_ids: &mut NewIds,
    deepest_layer: &SubSelection,
    deepest_std_block: StandardBlock,
    blocks_to_update: &mut Vec<String>
) -> Result<(), StepError> {
    let inline_block = block_map.get_inline_block(&deepest_layer.block_id)?;
    let (first_half, second_half) = inline_block.split(deepest_layer.offset, new_ids)?;
    let mut inline_blocks = deepest_std_block.content_block()?.clone().inline_blocks;
    inline_blocks.insert(first_half.index(&block_map)? + 1, second_half.id());
    let deepest_std_block = deepest_std_block.update_block_content(ContentBlock { inline_blocks })?;
    block_map.update_blocks(vec![Block::InlineBlock(first_half), Block::InlineBlock(second_half), Block::StandardBlock(deepest_std_block)], blocks_to_update)?;
    return Ok(())
}