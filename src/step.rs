
use std::str::FromStr;

use serde_json::{Value, json};
use wasm_bindgen::JsValue;

use crate::{steps_generator::{selection::{SubSelection}, event::{DropBlockEvent, ReplaceWithChildrenEvent}, StepError},
mark::Mark, blocks::{standard_blocks::StandardBlockType},
utilities::Tree, backend_interface::{get_json_field_as_string, get_json_field_as_int, get_json_field_as_bool}};


#[derive(Debug, Clone)]
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
    Paste(PasteStep), // TODO: NEEDS TO STORE WHAT WAS PASTED INSIDE STEP
    DropBlock(DropBlockEvent),
    DeleteBlock(String), //ID
    Duplicate(DuplicateStep),
    ReplaceWithChildren(ReplaceWithChildrenEvent),
    AddParagraphAtBottom(AddParagraphAtBottomStep) // (Root block id)
    //ReplaceAroundStep
}

impl Step {
    pub fn from_json(_type: &str, json: &str) -> Result<Self, StepError> {
        let json = match serde_json::Value::from_str(json) {
            Ok(json) => json,
            Err(_) => return Err(StepError(format!("Could not parse json block from str: {}", json)))
        };
        return Ok(match _type {
            "AddBlock" => Step::AddBlock(AddBlockStep::from_json(json)?),
            "AddMarkStep" => Step::AddMarkStep(MarkStep::from_json(json)?),
            "RemoveMarkStep" => Step::AddMarkStep(MarkStep::from_json(json)?),
            "ReplaceStep" => Step::ReplaceStep(ReplaceStep::from_json(json)?),
            "SplitStep" => Step::SplitStep(SplitStep::from_json(json)?),
            "TurnToChild" => Step::TurnToChild(TurnToChild::from_json(json)?),
            "TurnToParent" => Step::TurnToParent(TurnToParent::from_json(json)?),
            "TurnInto" => Step::TurnInto(TurnInto::from_json(json)?),
            "ToggleCompleted" => Step::ToggleCompleted(get_json_field_as_string(&json, "block_id")?),
            "Copy" => unreachable!(), // copy should be ignored everywhere except when applied on frontend
            "Paste" => unimplemented!(), // need to add
            "DropBlock" => Step::DropBlock(DropBlockEvent::from_json(json)?),
            "DeleteBlock" => Step::DeleteBlock(get_json_field_as_string(&json, "block_id")?),
            "Duplicate" => Step::Duplicate(DuplicateStep::from_json(json)?),
            "ReplaceWithChildren" => Step::ReplaceWithChildren(ReplaceWithChildrenEvent::from_json(json)?),
            "AddParagraphAtBottom" => Step::AddParagraphAtBottom(AddParagraphAtBottomStep::from_json(json)?),
            _type => Err(StepError(format!("_type: {:?}, is not a valid step type!", _type)))?
        })
    }

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
            Self::Paste(_) => "Paste",
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
            Self::Paste(paste_step) => unimplemented!(),// json!({ "from": from.to_json()?, "to": to.to_json()? }),
            Self::DropBlock(event) => event.to_json()?,
            Self::DeleteBlock(block_id) => json!({ "block_id": block_id }),
            Self::Duplicate(step) => step.to_json()?,
            Self::ReplaceWithChildren(event) => event.to_json()?,
            Self::AddParagraphAtBottom(step) => step.to_json()?
        };

        js_sys::Reflect::set(&obj, &JsValue::from_str("_type"), &JsValue::from(_type)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("data"), &JsValue::from(data.to_string())).unwrap();
        return Ok(JsValue::from(obj))
    }
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
            "slice": self.slice.to_string()?
        }))
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_id = get_json_field_as_string(&json, "block_id")?;
        let from = SubSelection::from_json(json.get("from")
            .ok_or(StepError(format!("Block does not have from field: {}", json)))?.clone())?;
        let to = SubSelection::from_json(json.get("to")
            .ok_or(StepError(format!("Block does not have to field: {}", json)))?.clone())?;
        let slice = ReplaceSlice::String(get_json_field_as_string(&json, "slice")?);
        return Ok(Self { block_id, from, to, slice })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReplaceSlice {
    Blocks(Vec<String>), // Vec<Id>
    String(String)
}

impl ReplaceSlice {
    pub fn to_string(self) -> Result<String, StepError> {
        return Ok(
            match self {
                Self::Blocks(blocks) => unimplemented!("Blocks not implemented yet"),
                Self::String(string) => string
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkStep {
    pub block_id: String,
    pub from: SubSelection,
    pub to: SubSelection,
    pub mark: Mark,
    pub from_new_inline_id: String,
    pub to_new_inline_id: String
}

impl MarkStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id,
            "from": self.from.to_json()?,
            "to": self.to.to_json()?,
            "mark": self.mark.to_string(),
            "from_new_inline_id": self.from_new_inline_id,
            "to_new_inline_id": self.to_new_inline_id
        }))
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let from = SubSelection::from_json(json.get("from")
            .ok_or(StepError(format!("Block does not have from field: {}", json)))?.clone())?;
        let to = SubSelection::from_json(json.get("to")
            .ok_or(StepError(format!("Block does not have to field: {}", json)))?.clone())?;
        let mark = Mark::from_str(&get_json_field_as_string(&json, "mark")?)?;
        return Ok(Self {
            block_id: get_json_field_as_string(&json, "block_id")?,
            from,
            to,
            mark,
            from_new_inline_id: get_json_field_as_string(&json, "from_new_inline_id")?,
            to_new_inline_id: get_json_field_as_string(&json, "to_new_inline_id")?
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SplitStep {
    pub subselection: SubSelection,
    pub new_std_block_id: String,
    pub new_inline_block_id: String,
}

impl SplitStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "subselection": self.subselection.to_json()?,
            "new_std_block_id": self.new_std_block_id,
            "new_inline_block_id": self.new_inline_block_id,
        }))
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let subselection = SubSelection::from_json(json.get("subselection")
            .ok_or(StepError(format!("Step does not have subselection field: {}", json)))?.clone())?;
        return Ok(Self {
            subselection,
            new_std_block_id: get_json_field_as_string(&json, "new_std_block_id")?,
            new_inline_block_id: get_json_field_as_string(&json, "new_inline_block_id")?,
        })
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

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_id = json.get("block_id")
            .ok_or(StepError(format!("Block does not have block_id field: {}", json)))?
            .as_str().ok_or(StepError("block_id field is not a string".to_string()))?;
        return Ok(Self { block_id: block_id.to_string() })
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

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_id = json.get("block_id")
            .ok_or(StepError(format!("Block does not have block_id field: {}", json)))?
            .as_str().ok_or(StepError("block_id field is not a string".to_string()))?;
        return Ok(Self { block_id: block_id.to_string() })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddBlockStep {
    pub block_id: String,
    pub new_std_block_id: String,
    pub new_inline_block_id: String,
    pub child_offset: usize,
    pub block_type: StandardBlockType,
    pub focus_block_below: bool
}

impl AddBlockStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id,
            "new_std_block_id": self.new_std_block_id,
            "new_inline_block_id": self.new_inline_block_id,
            "child_offset": self.child_offset,
            "block_type": self.block_type.to_json(),
            "focus_block_below": self.focus_block_below
        }))
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_type = StandardBlockType::from_json_block(json.get("block_type")
            .ok_or(StepError(format!("Block does not have block_type field: {}", json)))?)?;
        return Ok(Self {
            block_id: get_json_field_as_string(&json, "block_id")?,
            new_std_block_id: get_json_field_as_string(&json, "new_std_block_id")?,
            new_inline_block_id: get_json_field_as_string(&json, "new_inline_block_id")?,
            child_offset: get_json_field_as_int(&json, "child_offset")? as usize,
            block_type,
            focus_block_below: get_json_field_as_bool(&json, "focus_block_below")?
        })
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

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_id = get_json_field_as_string(&json, "block_id")?;
        let new_block_type = StandardBlockType::from_json_block(json.get("new_block_type")
            .ok_or(StepError(format!("Block does not have new_block_type field: {}", json)))?)?;
        return Ok(Self { block_id, new_block_type })
    }
}

#[derive(Debug, Clone)]
pub struct PasteStep {
    pub from: SubSelection,
    pub to: SubSelection,
    pub copy_tree: Tree
}

impl PasteStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        unimplemented!()
        // return Ok(json!({
        //     "block_id": self.block_id,
        //     "new_block_type": self.new_block_type.to_json()
        // }))
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        unimplemented!()
    }

}


#[derive(Debug, Clone)]
pub struct DuplicateStep {
    pub duplicate_block_id: String,
    pub new_block_id: String
}

impl DuplicateStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "duplicate_block_id": self.duplicate_block_id,
            "new_block_id": self.new_block_id
        }))
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        return Ok(Self {
            duplicate_block_id: get_json_field_as_string(&json, "duplicate_block_id")?,
            new_block_id: get_json_field_as_string(&json, "new_block_id")?,
        })
    }

}

#[derive(Debug, Clone)]
pub struct AddParagraphAtBottomStep {
    pub root_block_id: String,
    pub new_block_id: String
}

impl AddParagraphAtBottomStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "root_block_id": self.root_block_id,
            "new_block_id": self.new_block_id
        }))
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        return Ok(Self {
            root_block_id: get_json_field_as_string(&json, "root_block_id")?,
            new_block_id: get_json_field_as_string(&json, "new_block_id")?,
        })
    }

}