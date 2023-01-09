use crate::steps_generator::StepError;


pub enum NewIds {
    Rust(Vec<String>),
    Js(js_sys::Array)
}

impl NewIds {
    pub fn new() -> Self {
        return NewIds::Rust(Vec::new())
    }

    pub fn get_id(&mut self) -> Result<String, StepError> {
        match self {
            Self::Rust(ids) => {
                let new_id = ids.pop();
                return match new_id {
                    Some(id) => Ok(id),
                    None => Err(StepError("New ids vec is empty!".to_string()))
                }
            },
            Self::Js(ids) => {
                let new_id = ids.pop();
                return match new_id.as_string() {
                    Some(id) => Ok(id),
                    None => Err(StepError("New ids vec is empty!".to_string()))
                }
            },
        }
    }

    pub fn hardcoded_new_ids_for_tests() -> Self  {
        let hardcoded_ids = vec![
            "B", "A", "z", "y",
            "x", "w", "v", "u",
            "t", "s", "r", "q",
            "p", "o", "n", "m",
            "l", "k", "j", "i",
            "h", "g", "f", "e",
            "d", "c", "b", "a"
        ];
        return Self::Rust(hardcoded_ids.to_vec().into_iter().map(|id| id.to_string()).collect())
    }
}






