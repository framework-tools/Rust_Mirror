
use crate::{blocks::{BlockMap}, step::Step, mark::Mark};

use self::{standard_key::generate_step_for_standard_key, backspace::generate_steps_for_backspace, enter::generate_steps_for_enter, };

use super::{event::{KeyPress, Key}, selection::{SubSelection}, StepError, mark_steps::generate_mark_steps};

pub mod standard_key;
pub mod backspace;
pub mod enter;

pub fn generate_keyboard_event_steps(key_press: &KeyPress, block_map: &BlockMap, from: SubSelection, to: SubSelection) -> Result<Vec<Step>, StepError> {
    return match key_press.key {
        //Shortcuts
        Key::Standard('b') | Key::Standard('B') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Bold, from, to, block_map),
        Key::Standard('i') | Key::Standard('I') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Italic, from, to, block_map),
        Key::Standard('u') | Key::Standard('U') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Underline, from, to, block_map),
        //standard press
        Key::Standard(key) => generate_step_for_standard_key(key, block_map, from, to),
        Key::Backspace => generate_steps_for_backspace(block_map, from, to),
        Key::Enter => generate_steps_for_enter(block_map, from, to),
        _ => unimplemented!(),
    }
}

