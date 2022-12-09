use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;

use crate::{mark::{Color, Mark}, frontend_interface::{get_js_field_as_string, get_js_field, get_js_field_as_bool}, blocks::standard_blocks::{StandardBlockType, content_block::ContentBlock}};

use super::StepError;

pub enum Event {
    KeyPress(KeyPress),
    FormatBar(FormatBarEvent),
    SlashScrim(SlashScrimEvent),
    ToggleCompleted(String), //block id
}

impl Event {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let _type = get_js_field_as_string(&JsValue::from(&obj), "_type")?;
        return match _type.as_str() {
            "keypress" => Ok(Event::KeyPress(KeyPress::from_js_obj(obj)?)),
            "formatbar" => Ok(Event::FormatBar(FormatBarEvent::from_js_obj(obj)?)),
            "slash_scrim" => Ok(Event::SlashScrim(SlashScrimEvent::from_js_obj(obj)?)),
            "toggle_completed" => Ok(Event::ToggleCompleted(get_js_field_as_string(&JsValue::from(&obj), "value")?)),
            _type => Err(StepError(format!("Expected event _type to be either 'keypress' or 'formatbar'. Got: {}", _type)))
        }
    }
}

pub struct KeyPress {
    pub key: Key,
    pub metadata: KeyPressMetadata,
}

impl KeyPress {
    pub fn new(key: Key, metadata: Option<KeyPressMetadata>) -> Self {
        return match metadata {
            Some(metadata) => KeyPress {
                key,
                metadata
            },
            None => KeyPress {
                key,
                metadata: KeyPressMetadata {
                    shift_down: false,
                    meta_down: false,
                    ctrl_down: false,
                    alt_down: false,
                }
            }
        }
    }

    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let key = Key::from_str(&get_js_field_as_string(&JsValue::from(&obj), "value")?)?;
        let metadata = get_js_field(&JsValue::from(obj), "metadata")?;
        let metadata = KeyPressMetadata::from_js_obj(&metadata)?;

        return Ok(KeyPress {
            key,
            metadata
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct KeyPressMetadata {
    pub shift_down: bool,
    pub meta_down: bool,
    pub ctrl_down: bool,
    pub alt_down: bool,
}

impl KeyPressMetadata {
    pub fn from_js_obj(obj: &JsValue) -> Result<Self, StepError> {
        let shift_down = get_js_field_as_bool(obj, "shift_down")?;
        let meta_down = get_js_field_as_bool(obj, "meta_down")?;
        let ctrl_down = get_js_field_as_bool(obj, "ctrl_down")?;
        let alt_down = get_js_field_as_bool(obj, "alt_down")?;

        return Ok(Self { shift_down, meta_down, ctrl_down, alt_down })
    }
}

pub enum Key {
    Backspace,
    Delete,
    Enter,
    Tab,
    Escape,
    Standard(char)
}

impl Key {
    pub fn from_str(key: &str) -> Result<Self, StepError> {
        match key {
            "Backspace" => return Ok(Key::Backspace),
            "Delete" => return Ok(Key::Delete),
            "Enter" => return Ok(Key::Enter),
            "Tab" => return Ok(Key::Tab),
            "Escape" => return Ok(Key::Escape),
            _ => {
                let chars: Vec<char> = key.chars().collect();
                if chars.len() > 1 {
                    return Err(StepError("Standard key should only contain a single char".to_string()))
                } else {
                    return Ok(Key::Standard(chars[0]))
                }
            }
        }
    }
}

pub enum FormatBarEvent {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    ForeColor(Color),
    BackColor(Color),
    TurnInto(StandardBlockType)
}

impl FormatBarEvent {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let value = get_js_field_as_string(&JsValue::from(obj), "value")?;

        return match value.as_str() {
            "bold" => Ok(FormatBarEvent::Bold),
            "italic" => Ok(FormatBarEvent::Italic),
            "underline" => Ok(FormatBarEvent::Underline),
            "strikethrough" => Ok(FormatBarEvent::Strikethrough),
            value if value.contains("turn_into") => Ok(FormatBarEvent::TurnInto(parse_turn_into_str(value)?)),
            value => {
                let as_mark = Mark::color_mark_from_str(value)?;
                match as_mark {
                    Mark::ForeColor(color) => Ok(FormatBarEvent::ForeColor(color)),
                    Mark::BackColor(color) => Ok(FormatBarEvent::BackColor(color)),
                    _ => Err(StepError("Should parse as either a fore color or back color mark".to_string()))
                }
            }
        }
    }
}

fn parse_turn_into_str(value: &str) -> Result<StandardBlockType, StepError> {
    match value {
        "turn_into(paragraph)" => return Ok(StandardBlockType::Paragraph(ContentBlock::new(vec![]))),
        "turn_into(heading 1)" => return Ok(StandardBlockType::H1(ContentBlock::new(vec![]))),
        "turn_into(heading 2)" => return Ok(StandardBlockType::H2(ContentBlock::new(vec![]))),
        "turn_into(heading 3)" => return Ok(StandardBlockType::H3(ContentBlock::new(vec![]))),
        value => return Err(StepError(format!("Not a valid turn into statement. Got: {}", value)))
    }
}

pub struct SlashScrimEvent {
    pub block_type: String,
}

impl SlashScrimEvent {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        return Ok(SlashScrimEvent {
            block_type: get_js_field_as_string(&obj, "value")?
        })
    }
}