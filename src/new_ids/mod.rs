use crate::steps_generator::StepError;


pub struct NewIds(pub Vec<String>);

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
        let hardcoded_ids = vec![
            "636b20fc2c8fcc320d5efb8c".to_string(), "636b20fc2c8fcc320d5efb8d".to_string(), "636b20fc2c8fcc320d5efb8e".to_string(), "636b20fc2c8fcc320d5efb8f".to_string(),
        ];
        return Self(hardcoded_ids.to_vec().into_iter().map(|id| id.to_string()).collect())
    }
}

// "636b20fc2c8fcc320d5efb90", "636b20fc2c8fcc320d5efb91", "636b20fc2c8fcc320d5efb92", "636b20fc2c8fcc320d5efb93",
// "636b20fc2c8fcc320d5efb94", "636b20fc2c8fcc320d5efb95", "636b20fc2c8fcc320d5efb96", "636b20fc2c8fcc320d5efb97",
// "636b20fc2c8fcc320d5efb98", "636b20fc2c8fcc320d5efb99", "636b20fc2c8fcc320d5efb9a", "636b20fc2c8fcc320d5efb9b",
// "636b20fc2c8fcc320d5efb9c", "636b20fc2c8fcc320d5efb9d", "636b20fc2c8fcc320d5efb9e", "636b20fc2c8fcc320d5efb9f",
// "636b20fc2c8fcc320d5efba0", "636b20fc2c8fcc320d5efba1", "636b20fc2c8fcc320d5efba2", "636b20fc2c8fcc320d5efba3",
// "636b20fc2c8fcc320d5efba4", "636b20fc2c8fcc320d5efba5", "636b20fc2c8fcc320d5efba6", "636b20fc2c8fcc320d5efba7"