
use crate::{blocks::{BlockMap}, step::Step, mark::Mark, new_ids::NewIds};

use self::{backspace::generate_steps_for_backspace, enter::generate_steps_for_enter, tab::generate_steps_for_tab, delete::generate_steps_for_delete, };

use super::{event::{KeyPress, Key, KeyPressMetadata}, selection::{SubSelection}, StepError, mark_steps::generate_mark_steps, generate_replace_selected_steps::generate_replace_selected_steps, clipboard_steps::{generate_cut_steps, generate_paste_steps}};

pub mod backspace;
pub mod enter;
pub mod tab;
pub mod delete;

pub fn generate_keyboard_event_steps(
    key_press: &KeyPress,
    block_map: &BlockMap,
    from: SubSelection,
    to: SubSelection
) -> Result<Vec<Step>, StepError> {
    return match key_press.key {
        //Shortcuts
        Key::Standard('b') | Key::Standard('B') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Bold, from, to, block_map),
        Key::Standard('i') | Key::Standard('I') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Italic, from, to, block_map),
        Key::Standard('u') | Key::Standard('U') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_mark_steps(Mark::Underline, from, to, block_map),
        Key::Standard('c') | Key::Standard('C') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            Ok(vec![Step::Copy(from, to)]),
        Key::Standard('x') | Key::Standard('X') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_cut_steps(from, to, block_map),
        Key::Standard('v') | Key::Standard('V') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            generate_paste_steps(from, to, block_map),
        Key::Standard('z') | Key::Standard('Z') if key_press.metadata.ctrl_down || key_press.metadata.meta_down =>
            unimplemented!(),
        //standard press
        Key::Standard(key) => generate_replace_selected_steps(block_map, from, to, key.to_string()),
        Key::Backspace => generate_steps_for_backspace(block_map, from, to),
        Key::Delete => generate_steps_for_delete(block_map, from, to),
        Key::Enter => generate_steps_for_enter(block_map, from, to),
        Key::Tab => generate_steps_for_tab(block_map, from, to, key_press.metadata.clone()),
        _ => unimplemented!(),
    }
}

