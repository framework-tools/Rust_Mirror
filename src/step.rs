
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

#[derive(Debug, PartialEq, Clone)]
pub enum ReplaceSlice {
    Blocks(Vec<String>), // Vec<Id>
    String(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkStep {
    pub block_id: String,
    pub from: SubSelection,
    pub to: SubSelection,
    pub mark: Mark,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SplitStep {
    pub subselection: SubSelection
}

#[derive(Debug, PartialEq, Clone)]
pub struct TurnToChild {
    pub block_id: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct TurnToParent {
    pub block_id: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddBlockStep {
    pub block_id: String,
    pub child_offset: usize,
    pub block_type: StandardBlockType,
    pub focus_block_below: bool
}

#[derive(Debug, PartialEq, Clone)]
pub struct TurnInto {
    pub block_id: String,
    pub new_block_type: StandardBlockType
}

impl Step {
    pub fn to_js_obj(self) -> Result<JsValue, StepError> {
        let obj = js_sys::Object::new();

        let _type = match self {
            Self::AddBlock(_) =>
        }

        js_sys::Reflect::set(&obj, &JsValue::from_str("anchor"), &JsValue::from(self.anchor.to_js_obj()?)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("head"), &JsValue::from(self.head.to_js_obj()?)).unwrap();
        return Ok(obj)
    }
}