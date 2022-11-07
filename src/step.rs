use mongodb::bson::oid::ObjectId;
use crate::{steps_generator::selection::SubSelection, mark::Mark, blocks::Block};


#[derive(Debug, PartialEq)]
pub enum Step {
    ReplaceStep(ReplaceStep),
    AddMarkStep(MarkStep),
    RemoveMarkStep(MarkStep),
    //ReplaceAroundStep
}

#[derive(Debug, PartialEq)]
pub struct ReplaceStep {
    pub block_id: ObjectId,
    pub from: SubSelection,
    pub to: SubSelection,
    pub slice: Vec<ObjectId>,
    pub blocks_to_update: Vec<Block>,
}

#[derive(Debug, PartialEq)]
pub struct MarkStep {
    pub block_id: ObjectId,
    pub from: SubSelection,
    pub to: SubSelection,
    pub mark: Mark,
}