use crate::steps_generator::StepError;


pub struct NewIds(Vec<String>);

impl NewIds {
    pub fn get_id(&mut self) -> Result<String, StepError> {
        match self {
            NewIds(ids) => {
                let new_id = ids.pop();
                return match new_id {
                    Some(id) => Ok(id),
                    None => Err(StepError("New ids vec is empty!".to_string()))
                }
            }
        }
    }

    pub fn hardcoded_new_ids_for_tests() -> Self  {
        return Self(HARDCODED_IDS.to_vec().into_iter().map(|id| id.to_string()).collect())
    }
}

const HARDCODED_IDS: [&str; 100] = [
    "636b20fc2c8fcc320d5efb8c", "636b20fc2c8fcc320d5efb8d", "636b20fc2c8fcc320d5efb8e", "636b20fc2c8fcc320d5efb8f",
    "636b20fc2c8fcc320d5efb90", "636b20fc2c8fcc320d5efb91", "636b20fc2c8fcc320d5efb92", "636b20fc2c8fcc320d5efb93",
    "636b20fc2c8fcc320d5efb94", "636b20fc2c8fcc320d5efb95", "636b20fc2c8fcc320d5efb96", "636b20fc2c8fcc320d5efb97",
    "636b20fc2c8fcc320d5efb98", "636b20fc2c8fcc320d5efb99", "636b20fc2c8fcc320d5efb9a", "636b20fc2c8fcc320d5efb9b",
    "636b20fc2c8fcc320d5efb9c", "636b20fc2c8fcc320d5efb9d", "636b20fc2c8fcc320d5efb9e", "636b20fc2c8fcc320d5efb9f",
    "636b20fc2c8fcc320d5efba0", "636b20fc2c8fcc320d5efba1", "636b20fc2c8fcc320d5efba2", "636b20fc2c8fcc320d5efba3",
    "636b20fc2c8fcc320d5efba4", "636b20fc2c8fcc320d5efba5", "636b20fc2c8fcc320d5efba6", "636b20fc2c8fcc320d5efba7",
    "636b20fc2c8fcc320d5efba8", "636b20fc2c8fcc320d5efba9", "636b20fc2c8fcc320d5efbaa", "636b20fc2c8fcc320d5efbab",
    "636b20fc2c8fcc320d5efbac", "636b20fc2c8fcc320d5efbad", "636b20fc2c8fcc320d5efbae", "636b20fc2c8fcc320d5efbaf",
    "636b20fc2c8fcc320d5efbb0", "636b20fc2c8fcc320d5efbb1", "636b20fc2c8fcc320d5efbb2", "636b20fc2c8fcc320d5efbb3",
    "636b20fc2c8fcc320d5efbb4", "636b20fc2c8fcc320d5efbb5", "636b20fc2c8fcc320d5efbb6", "636b20fc2c8fcc320d5efbb7",
    "636b20fc2c8fcc320d5efbb8", "636b20fc2c8fcc320d5efbb9", "636b20fc2c8fcc320d5efbba", "636b20fc2c8fcc320d5efbbb",
    "636b20fc2c8fcc320d5efbbc", "636b20fc2c8fcc320d5efbbd", "636b20fc2c8fcc320d5efbbe", "636b20fc2c8fcc320d5efbbf",
    "636b20fc2c8fcc320d5efbc0", "636b20fc2c8fcc320d5efbc1", "636b20fc2c8fcc320d5efbc2", "636b20fc2c8fcc320d5efbc3",
    "636b20fc2c8fcc320d5efbc4", "636b20fc2c8fcc320d5efbc5", "636b20fc2c8fcc320d5efbc6", "636b20fc2c8fcc320d5efbc7",
    "636b20fc2c8fcc320d5efbc8", "636b20fc2c8fcc320d5efbc9", "636b20fc2c8fcc320d5efbca", "636b20fc2c8fcc320d5efbcb",
    "636b20fc2c8fcc320d5efbcc", "636b20fc2c8fcc320d5efbcd", "636b20fc2c8fcc320d5efbce", "636b20fc2c8fcc320d5efbcf",
    "636b20fc2c8fcc320d5efbd0", "636b20fc2c8fcc320d5efbd1", "636b20fc2c8fcc320d5efbd2", "636b20fc2c8fcc320d5efbd3",
    "636b20fc2c8fcc320d5efbd4", "636b20fc2c8fcc320d5efbd5", "636b20fc2c8fcc320d5efbd6", "636b20fc2c8fcc320d5efbd7",
    "636b20fc2c8fcc320d5efbd8", "636b20fc2c8fcc320d5efbd9", "636b20fc2c8fcc320d5efbda", "636b20fc2c8fcc320d5efbdb",
    "636b20fc2c8fcc320d5efbdc", "636b20fc2c8fcc320d5efbdd", "636b20fc2c8fcc320d5efbde", "636b20fc2c8fcc320d5efbdf",
    "636b20fc2c8fcc320d5efbe0", "636b20fc2c8fcc320d5efbe1", "636b20fc2c8fcc320d5efbe2", "636b20fc2c8fcc320d5efbe3",
    "636b20fc2c8fcc320d5efbe4", "636b20fc2c8fcc320d5efbe5", "636b20fc2c8fcc320d5efbe6", "636b20fc2c8fcc320d5efbe7",
    "636b20fc2c8fcc320d5efbe8", "636b20fc2c8fcc320d5efbe9", "636b20fc2c8fcc320d5efbea", "636b20fc2c8fcc320d5efbeb",
    "636b20fc2c8fcc320d5efbec", "636b20fc2c8fcc320d5efbed", "636b20fc2c8fcc320d5efbee", "636b20fc2c8fcc320d5efbef"
];