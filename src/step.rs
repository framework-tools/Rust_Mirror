
use crate::{steps_generator::selection::SubSelection, mark::Mark, blocks::Block};


#[derive(Debug, PartialEq, Clone)]
pub enum Step {
    ReplaceStep(ReplaceStep),
    SplitStep(SplitStep),
    AddMarkStep(MarkStep),
    RemoveMarkStep(MarkStep),
    TurnToChild(TurnToChild),
    TurnToParent(TurnToParent),
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