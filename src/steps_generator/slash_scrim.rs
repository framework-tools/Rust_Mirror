use crate::{step::{Step, ReplaceStep, ReplaceSlice, AddBlockStep}, blocks::{BlockMap, Block, standard_blocks::{StandardBlockType, content_block::ContentBlock, list_block::ListBlock}}};

use super::{StepError, event::SlashScrimEvent, selection::SubSelection};


pub fn generate_slash_scrim_steps(
    slash_scrim_event: &SlashScrimEvent,
    from: SubSelection,
    to: SubSelection,
    block_map: &BlockMap
) -> Result<Vec<Step>, StepError> {
    let mut replace_slash_scrim_text_step: Option<ReplaceStep> = None;
    let from_block = block_map.get_block(&from.block_id)?;
    match from_block {
        Block::InlineBlock(inline_block) => {
            let text = inline_block.text()?;
            let mut i = to.offset;
            loop {
                let char;
                if from.block_id == to.block_id && from.offset == to.offset && from.offset == text.len() {
                    char = String::from_utf16(&[text.0[i - 1]]).unwrap();
                    replace_slash_scrim_text_step = Some(ReplaceStep {
                        block_id: inline_block.parent.clone(),
                        from: SubSelection { block_id: inline_block.id(), offset: from.offset - 1, subselection: None },
                        to: SubSelection { block_id: inline_block.id(), offset: to.offset, subselection: None },
                        slice: ReplaceSlice::String("".to_string())
                    })
                } else {
                    char = String::from_utf16(&[text.0[i]]).unwrap();
                }
                if char == "/".to_string() {
                    break;
                }
                i -= 1;
                replace_slash_scrim_text_step = Some(ReplaceStep {
                    block_id: inline_block.parent.clone(),
                    from: SubSelection { block_id: inline_block.id(), offset: i, subselection: None },
                    to: SubSelection { block_id: inline_block.id(), offset: to.offset, subselection: None },
                    slice: ReplaceSlice::String("".to_string())
                })
            }
        },
        Block::StandardBlock(std_block) => {},
        Block::Root(_) => return Err(StepError("Cannot perform slash scrim event directly on root block".to_string()))
    }

    let new_block_type = match slash_scrim_event.block_type.as_str() {
        "paragraph" => StandardBlockType::Paragraph(ContentBlock::new(vec![])),
        "heading 1" => StandardBlockType::H1(ContentBlock::new(vec![])),
        "heading 2" => StandardBlockType::H2(ContentBlock::new(vec![])),
        "heading 3" => StandardBlockType::H3(ContentBlock::new(vec![])),
        "to-do list" => StandardBlockType::TodoList(ListBlock::new()),
        "numbered list" => StandardBlockType::NumberedList(ListBlock::new()),
        "dotpoint list" => StandardBlockType::DotPointList(ListBlock::new()),
        "arrow list" => StandardBlockType::ArrowList(ListBlock::new()),
        block_type => return Err(StepError(format!("There is no valid block type: {}", block_type)))
    };

    let mut steps = vec![];
    if replace_slash_scrim_text_step.is_some() {
        steps.push(Step::ReplaceStep(replace_slash_scrim_text_step.unwrap()));
    }

    let nearest_standard_block = block_map.get_nearest_ancestor_standard_block_incl_self(&from.block_id)?;
    steps.push(Step::AddBlock(AddBlockStep {
        block_id: nearest_standard_block.parent.clone(),
        child_offset: nearest_standard_block.index(block_map)? + 1,
        block_type: new_block_type
    }));
    return Ok(steps)
}