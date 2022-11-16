
#[cfg(test)]
mod tests {
    use rust_mirror::{steps_generator::selection::{Selection, SubSelection}, new_ids::NewIds, blocks::BlockMap};
    use serde_json::json;

    #[test]
    fn can_get_from_and_to_same_inline_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello"
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let paragraph_block1 = json!({
            "_id": paragraph_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), paragraph_block1.to_string()
        ]).unwrap();

        let selection = Selection {
            anchor: SubSelection {
                block_id: inline_block_id1.clone(),
                offset: 2,
                subselection: None
            },
            head: SubSelection {
                block_id: inline_block_id1.clone(),
                offset: 0,
                subselection: None
            }
        };

        let (from, to) = selection.clone().get_from_to(&block_map).unwrap();
        assert_eq!(from, selection.head);
        assert_eq!(to, selection.anchor);
    }

    #[test]
    fn can_get_from_and_to_different_blocks_at_top_layer() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello"
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " World"
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let paragraph_block1 = json!({
            "_id": paragraph_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone(), inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string()
        ]).unwrap();

        let selection = Selection {
            anchor: SubSelection {
                block_id: inline_block_id2.clone(),
                offset: 2,
                subselection: None
            },
            head: SubSelection {
                block_id: inline_block_id1.clone(),
                offset: 1,
                subselection: None
            }
        };

        let (from, to) = selection.clone().get_from_to(&block_map).unwrap();
        assert_eq!(from, selection.head);
        assert_eq!(to, selection.anchor);
    }

    #[test]
    fn can_get_from_and_to_different_amount_of_layers_same_top_level_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();

        let paragraph_block1 = json!({
            "_id": paragraph_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [paragraph_block_id2.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello"
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let paragraph_block2 = json!({
            "_id": paragraph_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " World"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        });

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string()
        ]).unwrap();

        let selection = Selection {
            anchor: SubSelection {
                block_id: paragraph_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: paragraph_block_id2.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: inline_block_id2.clone(),
                        offset: 2,
                        subselection: None
                    }))
                }))
            },
            head: SubSelection {
                block_id: paragraph_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id1.clone(),
                    offset: 2,
                    subselection: None
                }))
            }
        };

        let (from, to) = selection.clone().get_from_to(&block_map).unwrap();
        assert_eq!(from, selection.head);
        assert_eq!(to, selection.anchor);
    }

    #[test]
    fn can_get_deepest_two_layers() {
        let subselection = SubSelection {
            block_id: "1".to_string(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: "2".to_string(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: "3".to_string(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: "4".to_string(),
                        offset: 5,
                        subselection: None
                    }))
        }))}))};

        let two_deepest_layers = subselection.get_two_deepest_layers().unwrap();
        let expected = SubSelection {
            block_id: "3".to_string(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: "4".to_string(),
                offset: 5,
                subselection: None
            }))
        };
        assert_eq!(two_deepest_layers, expected);
    }
}