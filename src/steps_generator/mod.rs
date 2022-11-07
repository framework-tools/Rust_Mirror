

use crate::{blocks::{BlockMap}, step::Step, mark::Mark};

use self::{event::{Event, FormatBarEvent}, keypress_step_generator::{generate_keyboard_event_steps}, selection::Selection, mark_steps::generate_mark_steps};
pub mod keypress_step_generator;
pub mod selection;
pub mod event;
pub mod replace_selected;
pub mod mark_steps;

use std::str::FromStr;

pub fn execute_event(json: String) -> Result<String, StepError> {
    let rust_json = serde_json::Value::from_str(&json);
    let rust_json = match rust_json {
        Ok(rust_json) => rust_json,
        Err(_) => return Err(StepError("json argument should be valid json".to_string()))
    };
    let event_json = rust_json.get("event");
    let event_json = match event_json {
        Some(event_json) => event_json,
        None => return Err(StepError("json should contain 'event' field".to_string()))
    };
    let selection_json = rust_json.get("selection");
    let selection_json = match selection_json {
        Some(selection_json) => selection_json,
        None => return Err(StepError("json should contain 'selection' field".to_string()))
    };
    let block_map_json = rust_json.get("block_map");
    let block_map_json = match block_map_json {
        Some(block_map_json) => block_map_json,
        None => return Err(StepError("json should contain 'block_map' field".to_string()))
    };

    let event = Event::from_json(event_json)?;
    let selection = Selection::from_json(selection_json)?;

    return Ok("it worked".to_string())
}

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
        },
    }
}



