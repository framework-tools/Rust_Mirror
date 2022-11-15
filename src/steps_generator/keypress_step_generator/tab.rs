
use crate::{step::{Step, TurnToChild, TurnToParent}, steps_generator::{StepError, selection::SubSelection, event::{KeyPressMetadata, KeyPress}}, blocks::{BlockMap, Block, RootBlock, standard_blocks::StandardBlock, inline_blocks::InlineBlock}};


pub fn generate_steps_for_tab(block_map: &BlockMap, from: SubSelection, to: SubSelection, key_press_metadata: KeyPressMetadata) -> Result<Vec<Step>, StepError> {
    let from_block = block_map.get_block(&from.block_id)?;
    match from_block {
        Block::InlineBlock(inline_block) => {
            let parent_block = inline_block.get_parent(block_map)?;
            let parents_parent_as_root = block_map.get_root_block(&parent_block.parent);

            if key_press_metadata.shift_down {
                return turn_to_parent_step_generator(block_map, parent_block, parents_parent_as_root.is_ok())
            } else {
                return turn_to_child_step_generator(block_map, parent_block)
            }
        },
        Block::StandardBlock(from_block) => {
            let to_block = block_map.get_standard_block(&to.block_id)?;
            let parent = from_block.get_parent(block_map)?;
            let parents_children = parent.children()?;
            let mut i = from_block.index(block_map)?;
            let mut steps = vec![];
            let j = to_block.index(block_map)? + 1;
            while i < j {
                let std_block = block_map.get_standard_block(&parents_children[i])?;
                let option_step;
                if key_press_metadata.shift_down {
                    let parent_as_root = block_map.get_root_block(&std_block.parent);
                    option_step = turn_to_parent_step_generator(block_map, std_block, parent_as_root.is_ok())?;
                } else {
                    option_step = turn_to_child_step_generator(block_map, std_block)?;
                }

                if option_step.len() > 0 {
                    steps.push(option_step[0].clone());
                }
                i += 1;
            }
            return Ok(steps)
        },
        Block::Root(_) => return Err(StepError("Cannot tab on root block".to_string()))
    }
}

fn turn_to_child_step_generator(block_map: &BlockMap, std_block: StandardBlock) -> Result<Vec<Step>, StepError> {
    if std_block.index(block_map)? == 0 {
        return Ok(vec![])
    } else {
        return Ok(vec![
            Step::TurnToChild(TurnToChild { block_id: std_block.id() })
        ])
    }
}

fn turn_to_parent_step_generator(block_map: &BlockMap, std_block: StandardBlock, parents_parent_as_root_is_ok: bool) -> Result<Vec<Step>, StepError> {
    if parents_parent_as_root_is_ok {
        return Ok(vec![])
    } else {
        return Ok(vec![
            Step::TurnToParent(TurnToParent { block_id: std_block._id })
        ])
    }
}