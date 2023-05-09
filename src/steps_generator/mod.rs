
use crate::{blocks::{BlockMap}, step::{Step}, mark::Mark, custom_copy::CustomCopy};

use self::{event::{Event, FormatBarEvent, ContextMenuEvent}, keypress_step_generator::{generate_keyboard_event_steps}, selection::{Selection}, mark_steps::generate_mark_steps, slash_scrim::generate_slash_scrim_steps, turn_into::generate_turn_into_step, clipboard_steps::{generate_cut_steps, generate_paste_steps}};

pub mod keypress_step_generator;
pub mod selection;
pub mod event;
pub mod generate_replace_selected_steps;
pub mod mark_steps;
pub mod slash_scrim;
pub mod turn_into;
pub mod clipboard_steps;

#[derive(Debug, PartialEq)]
pub struct StepError (pub String);

pub fn generate_steps(event: &Event, block_map: &BlockMap, selection: Selection, copy: &CustomCopy) -> Result<Vec<Step>, StepError> {
    let (from, to) = selection.get_from_to(block_map)?;
    return match event {
        Event::KeyPress(key_press) => generate_keyboard_event_steps(key_press, block_map, from, to, copy),
        Event::FormatBar(event) => match event {
            FormatBarEvent::Bold => generate_mark_steps(Mark::Bold, from, to, block_map),
            FormatBarEvent::Italic => generate_mark_steps(Mark::Italic, from, to, block_map),
            FormatBarEvent::Underline => generate_mark_steps(Mark::Underline, from, to, block_map),
            FormatBarEvent::Strikethrough => generate_mark_steps(Mark::Strikethrough, from, to, block_map),
            FormatBarEvent::ForeColor(color) => generate_mark_steps(Mark::ForeColor(color.clone()), from, to, block_map),
            FormatBarEvent::BackColor(color) => generate_mark_steps(Mark::BackColor(color.clone()), from, to, block_map),
            FormatBarEvent::TurnInto(new_block_type) => generate_turn_into_step(new_block_type, from, block_map),
        },
        Event::ContextMenu(context_menu_event) => match context_menu_event {
            ContextMenuEvent::Copy => Ok(vec![Step::Copy(from, to)]),
            ContextMenuEvent::Cut => generate_cut_steps(from, to, block_map),
            ContextMenuEvent::Paste => generate_paste_steps(from, to, block_map, copy.clone()),
        },
        Event::SlashScrim(slash_scrim_event) => generate_slash_scrim_steps(slash_scrim_event, from, to, block_map),
        Event::ToggleCompleted(_id) => Ok(vec![Step::ToggleCompleted(_id.clone())]),
        Event::DropBlock(drop_block_event) => Ok(vec![Step::DropBlock(drop_block_event.clone())]),
        Event::DeleteBlock(block_id) => Ok(vec![Step::DeleteBlock(block_id.clone())]),
        Event::Duplicate(block_id) => Ok(vec![Step::Duplicate(block_id.clone())]),
        Event::ReplaceWithChildren(replace_with_children_event) => Ok(vec![Step::ReplaceWithChildren(replace_with_children_event.clone())]),
        Event::AddParagraphAtBottom(root_block_id) => Ok(vec![Step::AddParagraphAtBottom(root_block_id.clone())])
    }
}

