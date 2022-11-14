
use crate::{step::{Step, TurnToChild, TurnToParent}, steps_generator::{StepError, selection::SubSelection, event::{KeyPressMetadata, KeyPress}}, blocks::{BlockMap, Block, RootBlock}};


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
                if parent_block.index(block_map)? == 0 {
                    return Ok(vec![])
                } else {
                    return Ok(vec![
                        Step::TurnToChild(TurnToChild { block_id: inline_block.parent })
                    ])        
                }
            }
        },

        Block::StandardBlock(_) => unimplemented!(),
        Block::Root(_) => unimplemented!(),
    }
}