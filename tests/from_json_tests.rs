#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_mirror::steps_generator::selection::{Selection, SubSelection};
    use serde_json::json;

    #[test]
    fn can_parse_selection_from_json() {
        let json = r#"{
            "from": {
                "block_id": "6367242bd94bdaae59511ccd",
                "offset": 0,
                "subselection": {
                    "block_id": "6367242bd94bdaae59511ccd",
                    "offset": 0,
                    "subselection": null
                }
            },
            "to": {
                "block_id": "6367242bd94bdaae59511ccd",
                "offset": 0,
                "subselection": null
            }
        }"#;

        let selection: Selection = serde_json::from_str(json).unwrap();
        let id = "6367242bd94bdaae59511ccd".to_string();
        assert_eq!(
            selection,
            Selection {
                from: SubSelection {
                    block_id: id.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: id.clone(),
                        offset: 0,
                        subselection: None
                    }))
                },
                to: SubSelection {
                    block_id: id,
                    offset: 0,
                    subselection: None
                }
            }
        )
    }
}