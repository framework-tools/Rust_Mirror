use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use wasm_bindgen::JsValue;

use crate::{mark::{Color, Mark}, frontend_interface::{get_js_field_as_string, get_js_field, get_js_field_as_bool}, blocks::standard_blocks::{StandardBlockType, content_block::ContentBlock, list_block::ListBlock}};

use super::StepError;

pub enum Event {
    KeyPress(KeyPress),
    FormatBar(FormatBarEvent),
    SlashScrim(SlashScrimEvent),
    ToggleCompleted(String), //block id
    ContextMenu(ContextMenuEvent),
    DropBlock(DropBlockEvent),
    DeleteBlock(String),
    Duplicate(String),
    ReplaceWithChildren(ReplaceWithChildrenEvent),
    AddParagraphAtBottom(String) // (root block id)
}

impl Event {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let _type = get_js_field_as_string(&JsValue::from(&obj), "_type")?;
        return match _type.as_str() {
            "keypress" => Ok(Event::KeyPress(KeyPress::from_js_obj(obj)?)),
            "formatbar" => Ok(Event::FormatBar(FormatBarEvent::from_js_obj(obj)?)),
            "slash_scrim" => Ok(Event::SlashScrim(SlashScrimEvent::from_js_obj(obj)?)),
            "toggle_completed" => Ok(Event::ToggleCompleted(get_js_field_as_string(&JsValue::from(&obj), "value")?)),
            "context_menu" => Ok(Event::ContextMenu(ContextMenuEvent::from_js_obj(obj)?)),
            "drop_block" => Ok(Event::DropBlock(DropBlockEvent::from_js_obj(obj)?)),
            "delete_block" => Ok(Event::DeleteBlock(get_js_field_as_string(&obj, "value")?)),
            "replace_with_children" => Ok(Event::ReplaceWithChildren(ReplaceWithChildrenEvent::from_js_obj(obj)?)),
            "duplicate_block" => Ok(Event::Duplicate(get_js_field_as_string(&obj, "value")?)),
            "add_paragraph_at_bottom" => Ok(Event::AddParagraphAtBottom(get_js_field_as_string(&obj, "value")?)),
            _type => Err(StepError(format!("Expected event _type. Got: {}", _type)))
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
        "turn_into(to-do list)" => return Ok(StandardBlockType::TodoList(ListBlock { content: ContentBlock::new(vec![]), completed: false } )),
        "turn_into(dotpoint list)" => return Ok(StandardBlockType::DotPointList(ListBlock { content: ContentBlock::new(vec![]), completed: false } )),
        "turn_into(numbered list)" => return Ok(StandardBlockType::NumberedList(ListBlock { content: ContentBlock::new(vec![]), completed: false } )),
        "turn_into(arrow list)" => return Ok(StandardBlockType::ArrowList(ListBlock { content: ContentBlock::new(vec![]), completed: false } )),
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

pub enum ContextMenuEvent {
    Copy,
    Cut,
    Paste
}
impl ContextMenuEvent {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        return match get_js_field_as_string(&obj, "value")?.as_str() {
            "copy" => Ok(Self::Copy),
            "cut" => Ok(Self::Cut),
            "paste" => Ok(Self::Paste),
            value => Err(StepError(format!("Expected valid context menu event. Got: {}", value))),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DropBlockEvent {
    pub drag_block_id: String,
    pub drop_block_id: String,
    pub side_dropped: Side
}
impl DropBlockEvent {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let value_obj = get_js_field(&obj, "value")?;
        let drag_block_id = get_js_field_as_string(&value_obj, "drag_block_id")?;
        let drop_block_id = get_js_field_as_string(&value_obj, "drop_block_id")?;
        let side_dropped = get_js_field_as_string(&value_obj, "side_dropped")?;
        let side_dropped = Side::from_str(&side_dropped)?;
        return Ok(Self {
            drag_block_id,
            drop_block_id,
            side_dropped
        })
    }

    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "drag_block_id": self.drag_block_id,
            "drop_block_id": self.drop_block_id,
            "side_dropped": match self.side_dropped {
                Side::Top => "top",
                Side::Bottom => "bottom",
                Side::Left => "left",
                Side::Right => "right"
            }
        }))
    }

    pub fn from_json(data_json: Value) -> Result<Self, StepError> {
        let drag_block_id = data_json.get("drag_block_id")
            .ok_or(StepError("Could not get drag_block_id from json".to_string()))?
            .as_str().ok_or(StepError("Could not get drag_block_id as str".to_string()))?.to_string();
        let drop_block_id = data_json.get("drop_block_id")
            .ok_or(StepError("Could not get drop_block_id from json".to_string()))?
            .as_str().ok_or(StepError("Could not get drop_block_id as str".to_string()))?.to_string();
        let side_dropped = data_json.get("side_dropped")
            .ok_or(StepError("Could not get side_dropped from json".to_string()))?
            .as_str().ok_or(StepError("Could not get side_dropped as str".to_string()))?.to_string();
        let side_dropped = Side::from_str(&side_dropped)?;
        return Ok(Self {
            drag_block_id,
            drop_block_id,
            side_dropped
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right
}

impl Side {
    pub fn from_str(string: &str) -> Result<Self, StepError> {
        return match string {
            "top" => Ok(Self::Top),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "bottom" => Ok(Self::Bottom),
            side => Err(StepError(format!("Not a valid side: {}", side))),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReplaceWithChildrenEvent {
    pub block_id: String
}

impl ReplaceWithChildrenEvent {
    pub fn from_js_obj(obj: js_sys::Object) -> Result<Self, StepError> {
        let block_id = get_js_field_as_string(&obj, "value")?;
        return Ok(Self {
            block_id
        })
    }

    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id
        }))
    }

    pub fn from_json(data_json: Value) -> Result<Self, StepError> {
        let block_id = data_json.get("block_id")
            .ok_or(StepError("Could not get block_id from json".to_string()))?
            .as_str().ok_or(StepError("Could not get block_id as str".to_string()))?.to_string();
        return Ok(Self {
            block_id
        })
    }
}