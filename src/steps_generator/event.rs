use serde::{Serialize, Deserialize};

use crate::mark::{Color, Mark};

use super::StepError;

pub enum Event {
    KeyPress(KeyPress),
    FormatBar(FormatBarEvent)
}

impl Event {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, StepError> {
        let _type = json.get("_type")
            .ok_or(StepError("Event json should contain '_type' field".to_string()))?
            .as_str();
        return match _type {
            Some("keypress") => Ok(Event::KeyPress(KeyPress::from_json(json)?)),
            Some("formatbar") => Ok(Event::FormatBar(FormatBarEvent::from_json(json)?)),
            Some(_) => Err(StepError("Expected event[0] to be either 'keypress' or 'formatbar'".to_string())),
            None => Err(StepError("Expected event[0] to be a str".to_string()))
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

    pub fn from_json(json: &serde_json::Value) -> Result<Self, StepError> {
        let key = json.get("value")
            .ok_or(StepError("Event json should contain 'value' field".to_string()))?
            .as_str();
        let key = match key {
            Some(key) => Key::from_json_str(key)?,
            None => return Err(StepError("Expected json 'value' (key pressed) to be a string".to_string()))
        };

        let metadata_json = json.get("metadata")
            .ok_or(StepError("Event json should contain 'metadata' field".to_string()))?;
        let metadata: KeyPressMetadata = match serde_json::from_str(&metadata_json.to_string()) {
            Ok(metadata)  => metadata,
            Err(_) => return Err(StepError("Keypress metadata could not be parsed from json".to_string()))
        };

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

pub enum Key {
    Backspace,
    Delete,
    Enter,
    Tab,
    Escape,
    Standard(char)
}

impl Key {
    pub fn from_json_str(key: &str) -> Result<Self, StepError> {
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
    pub fn from_json(json: &serde_json::Value) -> Result<Self, StepError> {
        let value = json.get("value").
            ok_or(StepError("FormatBarEvent json should contain 'value' field".to_string()))?
            .as_str();
        return match value {
            Some("bold") => Ok(FormatBarEvent::Bold),
            Some("italic") => Ok(FormatBarEvent::Italic),
            Some("underline") => Ok(FormatBarEvent::Underline),
            Some("strikethrough") => Ok(FormatBarEvent::Strikethrough),
            Some(event) => {
                let as_mark = Mark::color_mark_from_str(event)?;
                match as_mark {
                    Mark::ForeColor(color) => Ok(FormatBarEvent::ForeColor(color)),
                    Mark::BackColor(color) => Ok(FormatBarEvent::BackColor(color)),
                    _ => Err(StepError("Should parse as either a fore color or back color mark".to_string()))
                }
            },
            None => Err(StepError("Expected json arr[1] (mark type/value) to be a string".to_string()))
        }
    }
}

