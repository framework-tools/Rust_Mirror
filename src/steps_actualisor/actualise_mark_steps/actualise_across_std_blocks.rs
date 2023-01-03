use crate::{step::MarkStep, blocks::{standard_blocks::{StandardBlock, content_block::ContentBlock}, BlockMap, Block}, steps_actualisor::{UpdatedState, clean_block_after_transform}, steps_generator::{StepError, selection::SubSelection}, mark::Mark, new_ids::NewIds, utilities::{get_blocks_between, BlockStructure, BlocksBetween}};



/// -> apply mark for "from" std block -> from "inner from" to end of inline blocks
/// -> apply mark for "to" std block -> from start of inline blocks to "inner to"
/// -> for each standard block between "from" & "to" -> assign mark to each of their inline blocks
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

    match get_blocks_between(&mark_step.from, &mark_step.to, BlockStructure::Flat, &block_map)? {
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

fn split_edge_inline_blocks(
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