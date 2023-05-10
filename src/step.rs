
use std::str::FromStr;

use serde_json::{Value, json};
use wasm_bindgen::JsValue;

use crate::{steps_generator::{selection::{SubSelection}, event::{DropBlockEvent, ReplaceWithChildrenEvent}, StepError}, mark::Mark, blocks::{standard_blocks::StandardBlockType, BlockMap}, frontend_interface::{get_js_field_as_string, get_js_field, get_js_field_as_f64, get_js_field_as_bool}, utilities::Tree};


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
    Duplicate(String), //ID
    ReplaceWithChildren(ReplaceWithChildrenEvent),
    AddParagraphAtBottom(String) // (Root block id)
    //ReplaceAroundStep
}

impl Step {
    pub fn from_js_obj(step_js: JsValue) -> Result<Self, StepError> {
        let _type = get_js_field_as_string(&step_js, "_type")?;
        let data = get_js_field(&step_js, "data")?;

        return match _type.as_str() {
            "AddBlock" => Ok(Step::AddBlock(AddBlockStep::from_js_obj(data)?)),
            "AddMarkStep" => Ok(Step::AddMarkStep(MarkStep::from_js_obj(data)?)),
            "RemoveMarkStep" => Ok(Step::AddMarkStep(MarkStep::from_js_obj(data)?)),
            "ReplaceStep" => Ok(Step::ReplaceStep(ReplaceStep::from_js_obj(data)?)),
            "SplitStep" => Ok(Step::SplitStep(SplitStep::from_js_obj(data)?)),
            "TurnToChild" => Ok(Step::TurnToChild(TurnToChild::from_js_obj(data)?)),
            "TurnToParent" => Ok(Step::TurnToParent(TurnToParent::from_js_obj(data)?)),
            "TurnInto" => Ok(Step::TurnInto(TurnInto::from_js_obj(data)?)),
            "ToggleCompleted" => Ok(Step::ToggleCompleted(get_js_field_as_string(&data, "block_id")?)),
            "Copy" => unreachable!(), // copy should be ignored everywhere except when applied on frontend
            "Paste" => unimplemented!(), // need to add
            "DropBlock" => Ok(Step::DropBlock(DropBlockEvent::from_js_obj(js_sys::Object::from(data))?)),
            "DeleteBlock" => Ok(Step::DeleteBlock(data.as_string().unwrap())),
            "Duplicate" => Ok(Step::Duplicate(data.as_string().unwrap())),
            "ReplaceWithChildren" => Ok(Step::ReplaceWithChildren(ReplaceWithChildrenEvent::from_js_obj(js_sys::Object::from(data))?)),
            "AddParagraphAtBottom" => Ok(Step::AddParagraphAtBottom(data.as_string().unwrap())),
            _type => Err(StepError(format!("_type: {:?}, is not a valid step type!", _type)))
        }
    }

    pub fn from_json(_type: &str, json: &str) -> Result<Self, StepError> {
        let json_data = match serde_json::Value::from_str(json) {
            Ok(json) => json,
            Err(_) => return Err(StepError(format!("Could not parse json block from str: {}", json)))
        };
        return match _type {
            "AddBlock" => Ok(Step::AddBlock(AddBlockStep::from_json(json_data)?)),
            "AddMarkStep" => Ok(Step::AddMarkStep(MarkStep::from_json(json_data)?)),
            "RemoveMarkStep" => Ok(Step::AddMarkStep(MarkStep::from_json(json_data)?)),
            "ReplaceStep" => Ok(Step::ReplaceStep(ReplaceStep::from_json(json_data)?)),
            "SplitStep" => Ok(Step::SplitStep(SplitStep::from_json(json_data)?)),
            "TurnToChild" => Ok(Step::TurnToChild(TurnToChild::from_json(json_data)?)),
            "TurnToParent" => Ok(Step::TurnToParent(TurnToParent::from_json(json_data)?)),
            "TurnInto" => Ok(Step::TurnInto(TurnInto::from_json(json_data)?)),
            "ToggleCompleted" => Ok(Step::ToggleCompleted(json!(json).as_str().unwrap().to_string())),
            "Copy" => unreachable!(), // copy should be ignored everywhere except when applied on frontend
            "Paste" => unimplemented!(), // need to add
            "DropBlock" => Ok(Step::DropBlock(DropBlockEvent::from_json(json_data)?)),
            "DeleteBlock" => Ok(Step::DeleteBlock(json!(json).as_str().unwrap().to_string())),
            "Duplicate" => Ok(Step::Duplicate(json!(json).as_str().unwrap().to_string())),
            "ReplaceWithChildren" => Ok(Step::ReplaceWithChildren(ReplaceWithChildrenEvent::from_json(json_data)?)),
            "AddParagraphAtBottom" => Ok(Step::AddParagraphAtBottom(json!(json).as_str().unwrap().to_string())),
            _type => Err(StepError(format!("_type: {:?}, is not a valid step type!", _type)))
        }
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
            Self::Duplicate(block_id) => json!({ "block_id": block_id }),
            Self::ReplaceWithChildren(event) => event.to_json()?,
            Self::AddParagraphAtBottom(root_block_id) => json!({ "root_block_id": root_block_id })
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

    pub fn from_js_obj(data: JsValue) -> Result<Self, StepError> {
        return Ok(Self {
            block_id: get_js_field_as_string(&data, "block_id")?,
            from: SubSelection::from_js_obj(get_js_field(&data, "from")?)?,
            to: SubSelection::from_js_obj(get_js_field(&data, "to")?)?,
            slice: ReplaceSlice::String(get_js_field_as_string(&data, "slice")?)
        })
    }
    
    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_id = json.get("block_id")
            .ok_or(StepError(format!("Block does not have block_id field: {}", json)))?
            .as_str().ok_or(StepError("block_id field is not a string".to_string()))?;
        let from = SubSelection::from_json(json.get("from")
            .ok_or(StepError(format!("Block does not have from field: {}", json)))?.clone())?;
        let to = SubSelection::from_json(json.get("to")
            .ok_or(StepError(format!("Block does not have to field: {}", json)))?.clone())?;
        let slice = ReplaceSlice::String(json.get("slice")
            .ok_or(StepError(format!("Block does not have slice field: {}", json)))?
            .as_str().ok_or(StepError("slice field is not a string".to_string()))?.to_string());
        return Ok(Self { block_id: block_id.to_string(), from, to, slice })
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
}

impl MarkStep {
    pub fn to_json(self) -> Result<Value, StepError> {
        return Ok(json!({
            "block_id": self.block_id,
            "from": self.from.to_json()?,
            "to": self.to.to_json()?,
            "mark": self.mark.to_string()
        }))
    }

    pub fn from_js_obj(data: JsValue) -> Result<Self, StepError> {
        return Ok(Self {
            block_id: get_js_field_as_string(&data, "block_id")?,
            from: SubSelection::from_js_obj(get_js_field(&data, "from")?)?,
            to: SubSelection::from_js_obj(get_js_field(&data, "to")?)?,
            mark: Mark::from_str(&get_js_field_as_string(&data, "mark")?)?
        })
    }
    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_id = json.get("block_id")
            .ok_or(StepError(format!("Block does not have block_id field: {}", json)))?
            .as_str().ok_or(StepError("block_id field is not a string".to_string()))?;
        let from = SubSelection::from_json(json.get("from")
            .ok_or(StepError(format!("Block does not have from field: {}", json)))?.clone())?;
        let to = SubSelection::from_json(json.get("to")
            .ok_or(StepError(format!("Block does not have to field: {}", json)))?.clone())?;
        let mark = Mark::from_str(json.get("mark")
            .ok_or(StepError(format!("Block does not have mark field: {}", json)))?
            .as_str().ok_or(StepError("mark field is not a string".to_string()))?)?;
        return Ok(Self { block_id: block_id.to_string(), from, to, mark })
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

    pub fn from_js_obj(data: JsValue) -> Result<Self, StepError> {
        return Ok(Self { subselection: SubSelection::from_js_obj(get_js_field(&data, "subselection")?)? })
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let subselection = SubSelection::from_json(json.get("subselection")
            .ok_or(StepError(format!("Block does not have subselection field: {}", json)))?.clone())?;
        return Ok(Self { subselection })
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

    pub fn from_js_obj(data: JsValue) -> Result<Self, StepError> {
        return Ok(Self { block_id: get_js_field_as_string(&data, "block_id")? })
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

    pub fn from_js_obj(data: JsValue) -> Result<Self, StepError> {
        return Ok(Self { block_id: get_js_field_as_string(&data, "block_id")? })
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

    pub fn from_js_obj(data: JsValue) -> Result<Self, StepError> {
        let block_id = get_js_field_as_string(&data, "block_id")?;
        let child_offset: usize = get_js_field_as_f64(&data, "child_offset")? as usize;
        let block_type = StandardBlockType::from_js_block(&get_js_field(&data, "block_type")?)?;
        let focus_block_below = get_js_field_as_bool(&data, "focus_block_below")?;
        return Ok(Self { block_id, child_offset, block_type, focus_block_below })
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_id = json.get("block_id")
            .ok_or(StepError(format!("Block does not have block_id field: {}", json)))?
            .as_str().ok_or(StepError("block_id field is not a string".to_string()))?;
        let child_offset = json.get("child_offset")
            .ok_or(StepError(format!("Block does not have child_offset field: {}", json)))?
            .as_u64().ok_or(StepError("child_offset field is not a u64".to_string()))? as usize;
        let block_type = StandardBlockType::from_json_block(json.get("block_type")
            .ok_or(StepError(format!("Block does not have block_type field: {}", json)))?)?;
        let focus_block_below = json.get("focus_block_below")
            .ok_or(StepError(format!("Block does not have focus_block_below field: {}", json)))?
            .as_bool().ok_or(StepError("focus_block_below field is not a bool".to_string()))?;
        return Ok(Self { block_id: block_id.to_string(), child_offset, block_type, focus_block_below })
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

    pub fn from_js_obj(data: JsValue) -> Result<Self, StepError> {
        return Ok(Self {
            block_id: get_js_field_as_string(&data, "block_id")?,
            new_block_type: StandardBlockType::from_js_block(&get_js_field(&data, "new_block_type")?)?
        })
    }

    pub fn from_json(json: Value) -> Result<Self, StepError> {
        let block_id = json.get("block_id")
            .ok_or(StepError(format!("Block does not have block_id field: {}", json)))?
            .as_str().ok_or(StepError("block_id field is not a string".to_string()))?;
        let new_block_type = StandardBlockType::from_json_block(json.get("new_block_type")
            .ok_or(StepError(format!("Block does not have new_block_type field: {}", json)))?)?;
        return Ok(Self { block_id: block_id.to_string(), new_block_type })
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

    pub fn from_js_obj(data: JsValue) -> Result<Self, StepError> {
        unimplemented!()
        // return Ok(Self {
        //     block_id: get_js_field_as_string(&data, "block_id")?,
        //     new_block_type: StandardBlockType::from_js_block(&get_js_field(&data, "block_type")?)?
        // })
    }
}

