
use serde_json::{Value, json};
use wasm_bindgen::JsValue;

use crate::{steps_generator::{selection::{SubSelection}, event::{DropBlockEvent, ReplaceWithChildrenEvent}, StepError}, mark::Mark, blocks::{standard_blocks::StandardBlockType}};


#[derive(Debug, PartialEq, Clone)]
pub enum Step {
    ReplaceStep(ReplaceStep),
    SplitStep(SplitStep),
    AddMarkStep(MarkStep),
    RemoveMarkStep(MarkStep),
    TurnToChild(TurnToChild),
    TurnToParent(TurnToParent),
    AddBlock(AddBlockStep),
    TurnInto(TurnInto),
    ToggleCompleted(String), //block id
    Copy(SubSelection, SubSelection),
    Paste(SubSelection, SubSelection),
    DropBlock(DropBlockEvent),
    DeleteBlock(String), //ID
    Duplicate(String), //ID
    ReplaceWithChildren(ReplaceWithChildrenEvent),
    AddParagraphAtBottom(String) // (Root block id)
    //ReplaceAroundStep
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReplaceStep {
    pub block_id: String,
    pub from: SubSelection,
    pub to: SubSelection,
    pub slice: ReplaceSlice
}

impl ReplaceStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id,
            "from": self.from.to_json()?,
            "to": self.to.to_json()?,
            "slice": self.slice.to_json()?
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReplaceSlice {
    Blocks(Vec<String>), // Vec<Id>
    String(String)
}

impl ReplaceSlice {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "blocks": match self {
                Self::Blocks(blocks) => unimplemented!("Blocks not implemented yet"),
                Self::String(string) => string
            }
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkStep {
    pub block_id: String,
    pub from: SubSelection,
    pub to: SubSelection,
    pub mark: Mark,
}

impl MarkStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id,
            "from": self.from.to_json()?,
            "to": self.to.to_json()?,
            "mark": self.mark.to_json()?
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SplitStep {
    pub subselection: SubSelection
}

impl SplitStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "subselection": self.subselection.to_json()?
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TurnToChild {
    pub block_id: String
}

impl TurnToChild {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TurnToParent {
    pub block_id: String
}

impl TurnToParent {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddBlockStep {
    pub block_id: String,
    pub child_offset: usize,
    pub block_type: StandardBlockType,
    pub focus_block_below: bool
}

impl AddBlockStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id,
            "child_offset": self.child_offset,
            "block_type": self.block_type.to_json(),
            "focus_block_below": self.focus_block_below
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TurnInto {
    pub block_id: String,
    pub new_block_type: StandardBlockType
}

impl TurnInto {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id,
            "new_block_type": self.new_block_type.to_json()
        }))
    }
}

impl Step {
    pub fn to_js_obj(self) -> Result<JsValue, StepError> {
        let obj = js_sys::Object::new();

        let _type = match &self {
            Self::AddBlock(_) => "AddBlock",
            Self::AddMarkStep(_) => "AddMarkStep",
            Self::RemoveMarkStep(_) => "RemoveMarkStep",
            Self::ReplaceStep(_) => "ReplaceStep",
            Self::SplitStep(_) => "SplitStep",
            Self::TurnToChild(_) => "TurnToChild",
            Self::TurnToParent(_) => "TurnToParent",
            Self::TurnInto(_) => "TurnInto",
            Self::ToggleCompleted(_) => "ToggleCompleted",
            Self::Copy(_, _) => "Copy",
            Self::Paste(_, _) => "Paste",
            Self::DropBlock(_) => "DropBlock",
            Self::DeleteBlock(_) => "DeleteBlock",
            Self::Duplicate(_) => "Duplicate",
            Self::ReplaceWithChildren(_) => "ReplaceWithChildren",
            Self::AddParagraphAtBottom(_) => "AddParagraphAtBottom"
        };

        let data = match self {
            Self::AddBlock(step) => step.to_json()?,
            Self::AddMarkStep(step) => step.to_json()?,
            Self::RemoveMarkStep(step) => step.to_json()?,
            Self::ReplaceStep(step) => step.to_json()?,
            Self::SplitStep(step) => step.to_json()?,
            Self::TurnToChild(step) => step.to_json()?,
            Self::TurnToParent(step) => step.to_json()?,
            Self::TurnInto(step) => step.to_json()?,
            Self::ToggleCompleted(block_id) => json!({ "block_id": block_id }),
            Self::Copy(from, to) => json!({ "from": from.to_json()?, "to": to.to_json()? }),
            Self::Paste(from, to) => json!({ "from": from.to_json()?, "to": to.to_json()? }),
            Self::DropBlock(event) => event.to_json()?,
            Self::DeleteBlock(block_id) => json!({ "block_id": block_id }),
            Self::Duplicate(block_id) => json!({ "block_id": block_id }),
            Self::ReplaceWithChildren(event) => event.to_json()?,
            Self::AddParagraphAtBottom(root_block_id) => json!({ "root_block_id": root_block_id })
        };
        
        js_sys::Reflect::set(&obj, &JsValue::from_str("_type"), &JsValue::from(_type)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("data"), &JsValue::from(data.to_string())).unwrap();
        return Ok(JsValue::from(obj))
    }
}
