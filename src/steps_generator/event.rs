use crate::mark::{Color, Mark};

use super::StepError;

pub enum Event {
    KeyPress(KeyPress),
    FormatBar(FormatBarEvent)
}

impl Event {
    pub fn from_json(json: serde_json::Value) -> Result<Self, StepError> {
        let event = json.get("event")
            .ok_or(StepError("Expected 'event' field".to_string()))?
            .as_array();
        return match event {
            Some(event) => {
                match event[0].as_str() {
                    Some("keypress") => Ok(Event::KeyPress(KeyPress::from_json_array(event)?)),
                    Some("formatbar") => Ok(Event::FormatBar(FormatBarEvent::from_json_array(event)?)),
                    Some(_) => Err(StepError("Expected event[0] to be either 'keypress' or 'formatbar'".to_string())),
                    None => Err(StepError("Expected event[0] to be a str".to_string()))
                }
            }
            None => Err(StepError("Expected event json field to contain an array".to_ascii_lowercase()))
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

    pub fn from_json_array(json_arr: &Vec<serde_json::Value>) -> Result<Self, StepError> {
        let key = json_arr[1].as_str();
        let key = match key {
            Some(key) => Key::from_json_str(key)?,
            None => return Err(StepError("Expected json_arr[1] (key) to be a string".to_string()))
        };

        let metadata = KeyPressMetadata::from_json(&json_arr[2])?;
        return Ok(KeyPress {
            key,
            metadata
        })
    }
}

pub struct KeyPressMetadata {
    pub shift_down: bool,
    pub meta_down: bool,
    pub ctrl_down: bool,
    pub alt_down: bool,
}

impl KeyPressMetadata {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, StepError> {
        let shift_down = json.get("shift_down")
            .ok_or(StepError("Expected 'shift_down' field".to_string()))?
            .as_bool();
        let shift_down = match shift_down {
            Some(shift_down) => shift_down,
            None => return Err(StepError("Expected shift_down to be a bool".to_string()))
        };
        let meta_down = json.get("meta_down")
            .ok_or(StepError("Expected 'meta_down' field".to_string()))?
            .as_bool();
        let meta_down = match meta_down {
            Some(meta_down) => meta_down,
            None => return Err(StepError("Expected meta_down to be a bool".to_string()))
        };
        let ctrl_down = json.get("ctrl_down")
            .ok_or(StepError("Expected 'ctrl_down' field".to_string()))?
            .as_bool();
        let ctrl_down = match ctrl_down {
            Some(ctrl_down) => ctrl_down,
            None => return Err(StepError("Expected ctrl_down to be a bool".to_string()))
        };
        let alt_down = json.get("alt_down")
            .ok_or(StepError("Expected 'alt_down' field".to_string()))?
            .as_bool();
        let alt_down = match alt_down {
            Some(alt_down) => alt_down,
            None => return Err(StepError("Expected alt_down to be a bool".to_string()))
        };
        return Ok(Self {
            shift_down,
            meta_down,
            ctrl_down,
            alt_down,
        })
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
    pub fn from_json_array(json_arr: &Vec<serde_json::Value>) -> Result<Self, StepError> {
        return match json_arr[1].as_str() {
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

