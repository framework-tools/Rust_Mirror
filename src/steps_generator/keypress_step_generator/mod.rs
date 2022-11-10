
use crate::{blocks::{BlockMap}, step::Step, mark::Mark, new_ids::NewIds};

use self::{backspace::generate_steps_for_backspace, enter::generate_steps_for_enter, };

use super::{event::{KeyPress, Key}, selection::{SubSelection}, StepError, mark_steps::generate_mark_steps, generate_replace_selected_steps::generate_replace_selected_steps};

pub mod backspace;
pub mod enter;

pub fn generate_keyboard_event_steps(
    key_press: &KeyPress,
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection,
    new_ids: &mut NewIds
) -> Result<Vec<Step>, StepError> {
    return match key_press.key {
        //Shortcuts
        Key::Standard('b') | Key::Standard('B') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Bold, from, to, block_map),
        Key::Standard('i') | Key::Standard('I') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Italic, from, to, block_map),
        Key::Standard('u') | Key::Standard('U') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Underline, from, to, block_map),
        //standard press
        Key::Standard(key) => generate_replace_selected_steps(block_map, from, to, key.to_string()),
        Key::Backspace => generate_steps_for_backspace(block_map, from, to),
        Key::Enter => generate_steps_for_enter(block_map, from, to, new_ids),
        _ => unimplemented!(),
    }
}

