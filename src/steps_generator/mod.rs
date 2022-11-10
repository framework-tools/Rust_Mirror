

use crate::{blocks::{BlockMap}, step::Step, mark::Mark, new_ids::NewIds};

use self::{event::{Event, FormatBarEvent}, keypress_step_generator::{generate_keyboard_event_steps}, selection::Selection, mark_steps::generate_mark_steps};
pub mod keypress_step_generator;
pub mod selection;
pub mod event;
pub mod generate_replace_selected_steps;
pub mod mark_steps;

#[derive(Debug, PartialEq)]
pub struct StepError (pub String);

pub fn generate_steps(event: &Event, block_map: &BlockMap, selection: Selection, new_ids: &mut NewIds) -> Result<Vec<Step>, StepError> {
    let (from, to) = selection.get_from_to(block_map)?;
    let from = from.conform_illegal_start_offset_selection(block_map)?;
    let to = to.conform_illegal_start_offset_selection(block_map)?;

    return match event {
        Event::KeyPress(key_press) => generate_keyboard_event_steps(key_press, block_map, from, to, new_ids),
        Event::FormatBar(event) => match event {
            FormatBarEvent::Bold => generate_mark_steps(Mark::Bold, from, to, block_map),
            FormatBarEvent::Italic => generate_mark_steps(Mark::Italic, from, to, block_map),
            FormatBarEvent::Underline => generate_mark_steps(Mark::Underline, from, to, block_map),
            FormatBarEvent::Strikethrough => generate_mark_steps(Mark::Strikethrough, from, to, block_map),
            FormatBarEvent::ForeColor(color) => generate_mark_steps(Mark::ForeColor(color.clone()), from, to, block_map),
            FormatBarEvent::BackColor(color) => generate_mark_steps(Mark::BackColor(color.clone()), from, to, block_map),
        },
    }
}



