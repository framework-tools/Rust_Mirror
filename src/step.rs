
use crate::{steps_generator::selection::SubSelection, mark::Mark, blocks::{standard_blocks::StandardBlockType}};


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
    pub block_type: StandardBlockType
}

#[derive(Debug, PartialEq, Clone)]
pub struct TurnInto {
    pub block_id: String,
    pub new_block_type: StandardBlockType
}