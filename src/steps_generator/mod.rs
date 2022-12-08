
use crate::{blocks::{BlockMap, standard_blocks::StandardBlockType}, step::{Step, TurnInto}, mark::Mark};

use self::{event::{Event, FormatBarEvent}, keypress_step_generator::{generate_keyboard_event_steps}, selection::{Selection, SubSelection}, mark_steps::generate_mark_steps, slash_scrim::generate_slash_scrim_steps};

pub mod keypress_step_generator;
pub mod selection;
pub mod event;
pub mod generate_replace_selected_steps;
pub mod mark_steps;
pub mod slash_scrim;

#[derive(Debug, PartialEq)]
pub struct StepError (pub String);

pub fn generate_steps(event: &Event, block_map: &BlockMap, selection: Selection) -> Result<Vec<Step>, StepError> {
    let (from, to) = selection.get_from_to(block_map)?;
    return match event {
        Event::KeyPress(key_press) => generate_keyboard_event_steps(key_press, block_map, from, to),
        Event::FormatBar(event) => match event {
            FormatBarEvent::Bold => generate_mark_steps(Mark::Bold, from, to, block_map),
            FormatBarEvent::Italic => generate_mark_steps(Mark::Italic, from, to, block_map),
            FormatBarEvent::Underline => generate_mark_steps(Mark::Underline, from, to, block_map),
            FormatBarEvent::Strikethrough => generate_mark_steps(Mark::Strikethrough, from, to, block_map),
            FormatBarEvent::ForeColor(color) => generate_mark_steps(Mark::ForeColor(color.clone()), from, to, block_map),
            FormatBarEvent::BackColor(color) => generate_mark_steps(Mark::BackColor(color.clone()), from, to, block_map),
            FormatBarEvent::TurnInto(new_block_type) => generate_turn_into_step(new_block_type, from, block_map),
        },
        Event::SlashScrim(slash_scrim_event) => generate_slash_scrim_steps(slash_scrim_event, from, to, block_map),
        Event::ToggleCompleted(_id) => Ok(vec![Step::ToggleCompleted(_id.clone())])
    }
}

fn generate_turn_into_step(new_block_type: &StandardBlockType, from: SubSelection, block_map: &BlockMap) -> Result<Vec<Step>, StepError> {
    let inline_block = block_map.get_inline_block(&from.block_id)?;
    return Ok(vec![Step::TurnInto(TurnInto { block_id: inline_block.parent, new_block_type: new_block_type.clone() })])
}