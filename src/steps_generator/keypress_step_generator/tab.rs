
use crate::{step::{Step, TurnToChild, TurnToParent}, steps_generator::{StepError, selection::SubSelection, event::{KeyPressMetadata, KeyPress}}, blocks::{BlockMap, Block, RootBlock, standard_blocks::StandardBlock}};


pub fn generate_steps_for_tab(block_map: &BlockMap, from: SubSelection, to: SubSelection, key_press: KeyPressMetadata) -> Result<Vec<Step>, StepError> {
    let from_block = block_map.get_block(&from.block_id)?;
    match from_block {
        Block::InlineBlock(inline_block) => {
            let parent_block = inline_block.get_parent(block_map)?;
            let parents_parent_as_root = block_map.get_root_block(&parent_block.parent);

            if key_press.shift_down {
                if parents_parent_as_root.is_ok() {
                    return Ok(vec![])
                } else {
                return Ok(vec![
                    Step::TurnToParent(TurnToParent { block_id: inline_block.parent })
                ])
                }
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
            while i < to_block.index(block_map)? + 1 {
                let std_block = block_map.get_standard_block(&parents_children[i])?;
                let option_step = turn_to_child_step_generator(block_map, std_block)?;
                if option_step.len() > 0 {
                    steps.push(option_step[0].clone());
                }
                i += 1;
            }
            println!("got here");
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