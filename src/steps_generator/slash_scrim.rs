use crate::{step::{Step, ReplaceStep, ReplaceSlice, AddBlockStep, TurnInto}, blocks::{BlockMap, Block, standard_blocks::{StandardBlockType, content_block::ContentBlock, list_block::ListBlock, StandardBlock, page_block::PageBlock}}};

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
            let mut char = "".to_string();
            while &char != "/" {
                char = String::from_utf16(&[text.0[i - 1]]).unwrap();
                i -= 1;
                replace_slash_scrim_text_step = Some(ReplaceStep {
                    block_id: inline_block.parent.clone(),
                    from: SubSelection { block_id: inline_block.id(), offset: i, subselection: None },
                    to: SubSelection { block_id: inline_block.id(), offset: to.offset, subselection: None },
                    slice: ReplaceSlice::String("".to_string())
                });
            }
        },
        Block::StandardBlock(_) => {},
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
        "inline page" => StandardBlockType::InlinePage(PageBlock::new()),
        "square page" => StandardBlockType::SquarePage(PageBlock::new()),
        "link page" => StandardBlockType::LinkBlock(PageBlock::new()),
        block_type => return Err(StepError(format!("There is no valid block type: {}", block_type)))
    };

    let mut steps = vec![];
    let nearest_standard_block = block_map.get_nearest_ancestor_standard_block_incl_self(&from.block_id)?;
    let mut block_is_being_replaced = false;
    if replace_slash_scrim_text_step.is_some() {
        let replace_step = replace_slash_scrim_text_step.unwrap();
        if block_is_empty_other_than_slash_and_search(&nearest_standard_block, block_map, &replace_step)? && new_block_type.has_content()  {
            return Ok(vec![
                Step::DeleteBlock(nearest_standard_block.id()),
                Step::AddBlock(AddBlockStep {
                    block_id:  nearest_standard_block.parent(),
                    child_offset: nearest_standard_block.index(block_map)?,
                    block_type: new_block_type,
                    focus_block_below: false,
                })
            ])
        } else if block_is_empty_other_than_slash_and_search(&nearest_standard_block, block_map, &replace_step)? {
            return Ok(vec![
                Step::DeleteBlock(nearest_standard_block.id()),
                Step::AddBlock(AddBlockStep {
                    block_id:  nearest_standard_block.parent(),
                    child_offset: nearest_standard_block.index(block_map)?,
                    block_type: new_block_type,
                    focus_block_below: false,
                }),
                Step::AddBlock(AddBlockStep {
                    block_id:  nearest_standard_block.parent(),
                    child_offset: nearest_standard_block.index(block_map)? + 1,
                    block_type: StandardBlockType::Paragraph(ContentBlock::new(vec![])),
                    focus_block_below: false,
                })
            ])
        } else {
            steps.push(Step::ReplaceStep(replace_step));
        }
    }

    let add_paragraph_block_below_new_block = !new_block_type.has_content();

    let mut offset_to_add_at = nearest_standard_block.index(block_map)?;
    if !block_is_being_replaced {
        offset_to_add_at += 1;
    }
    steps.push(Step::AddBlock(AddBlockStep {
        block_id: nearest_standard_block.parent.clone(),
        child_offset: offset_to_add_at,
        block_type: new_block_type,
        focus_block_below: false
    }));
    if add_paragraph_block_below_new_block {
        steps.push(Step::AddBlock(AddBlockStep {
            block_id: nearest_standard_block.parent.clone(),
            child_offset: offset_to_add_at + 1,
            block_type: StandardBlockType::Paragraph(ContentBlock::new(vec![])),
            focus_block_below: false
        }));
    }

    return Ok(steps)
}

fn block_is_empty_other_than_slash_and_search(
    nearest_standard_block: &StandardBlock,
    block_map: &BlockMap,
    replace_step: &ReplaceStep
) -> Result<bool, StepError> {
    let content_block = nearest_standard_block.content_block()?;
    if content_block.inline_blocks.len() == 1 {
        let inline_block = block_map.get_inline_block(&content_block.inline_blocks[0])?;
        return Ok(replace_step.from.offset == 0 && replace_step.to.offset == inline_block.text()?.len())
    } else {
        return Ok(false)
    }
}