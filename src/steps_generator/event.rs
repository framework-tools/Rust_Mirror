use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;

use crate::{mark::{Color, Mark}, frontend_interface::{get_js_field_as_string, get_js_field, get_js_field_as_bool}};

use super::StepError;

pub enum Event {
    KeyPress(KeyPress),
    FormatBar(FormatBarEvent)
}

impl Event {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let _type = get_js_field_as_string(&JsValue::from(obj), "_type")?;
        return match _type.as_str() {
            "keypress" => Ok(Event::KeyPress(KeyPress::from_js_obj(obj)?)),
            "formatbar" => Ok(Event::FormatBar(FormatBarEvent::from_js_obj(obj)?)),
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
        let key = Key::from_str(&get_js_field_as_string(&JsValue::from(obj), "key")?)?;
        let metadata = get_js_field(&JsValue::from(obj), "metadata")?;
        let metadata = KeyPressMetadata::from_js_obj(&obj)?;

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
}

impl FormatBarEvent {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let value = get_js_field_as_string(&JsValue::from(obj), "value")?;

        return match value.as_str() {
            "bold" => Ok(FormatBarEvent::Bold),
            "italic" => Ok(FormatBarEvent::Italic),
            "underline" => Ok(FormatBarEvent::Underline),
            "strikethrough" => Ok(FormatBarEvent::Strikethrough),
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

