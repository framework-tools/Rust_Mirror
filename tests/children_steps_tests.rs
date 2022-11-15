#[cfg(test)]
mod tests {
    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap}, steps_generator::{event::{Event, KeyPress, Key, KeyPressMetadata}, selection::{SubSelection, Selection}, generate_steps}, step::Step};
    use serde_json::json;

    #[test]
    fn can_create_child() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
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
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
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
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Tab, None));
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::TurnToChild(turn_to_child_step) => {
                assert_eq!(turn_to_child_step.block_id, paragraph_block_id2);
            },
            _ => panic!("Expected ReplaceStep")
        }
    }
    #[test]
    fn should_not_create_child_if_standard_block_is_first_index() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
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
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
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
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Tab, None));
        let sub_selection = SubSelection::from(inline_block_id1.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 0);
    }
    #[test]
    fn should_not_create_parent_if_parent_is_root_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
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
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
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
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress { key: Key::Tab, metadata: KeyPressMetadata { shift_down: true, meta_down: false, ctrl_down: false, alt_down: false } });
        let sub_selection = SubSelection::from(inline_block_id1.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 0);
    }

    #[test]
    fn shift_tab_on_inline_should_generate_turn_to_parent_step() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
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
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
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

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id1.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress { key: Key::Tab, metadata: KeyPressMetadata { shift_down: true, meta_down: false, ctrl_down: false, alt_down: false } });
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::TurnToParent(turn_to_parent_step) => {
                assert_eq!(turn_to_parent_step.block_id, paragraph_block_id2);
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    #[test]
    fn tab_with_selection_across_standard_blocks() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
        let paragraph_block_id3 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();

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
            "children": [paragraph_block_id2.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Good"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
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
            "parent": root_block_id.clone()
        });
        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "bye"
            },
            "marks": [],
            "parent": paragraph_block_id3.clone()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id1.clone(), paragraph_block_id2.clone(), paragraph_block_id3.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(), paragraph_block3.to_string(),
            inline_block3.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress { key: Key::Tab, metadata: KeyPressMetadata { shift_down: false, meta_down: false, ctrl_down: false, alt_down: false } });
        let from_sub_selection = SubSelection {
            block_id: paragraph_block_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: inline_block_id1.clone(),
                offset: 1,
                subselection: None
            }))
        };
        let to_sub_selection = SubSelection {
            block_id: paragraph_block_id3.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: inline_block_id3.clone(),
                offset: 1,
                subselection: None
            }))
        };
        let selection = Selection::from(from_sub_selection.clone(), to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 2);
        match &steps[0] {
            Step::TurnToChild(turn_to_child_step) => {
                assert_eq!(turn_to_child_step.block_id, paragraph_block_id2);
            },
            _ => panic!("Expected TurnToChild")
        };
        match &steps[1] {
            Step::TurnToChild(turn_to_child_step) => {
                assert_eq!(turn_to_child_step.block_id, paragraph_block_id3);
            },
            _ => panic!("Expected TurnToChild")
        }
    }

    #[test]
    fn shift_tab_in_a_child() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
        let paragraph_block_id3 = new_ids.get_id().unwrap();
        let paragraph_block_id4 = new_ids.get_id().unwrap();
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
        let paragraph_block1 = json!({
            "_id": paragraph_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [paragraph_block_id2.clone(), paragraph_block_id3.clone(), paragraph_block_id4.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id3.clone()
        });
        let paragraph_block2 = json!({
            "_id": paragraph_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let paragraph_block4 = json!({
            "_id": paragraph_block_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(),
            paragraph_block3.to_string(), paragraph_block4.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress { key: Key::Tab, metadata: KeyPressMetadata { shift_down: true, meta_down: false, ctrl_down: false, alt_down: false } });
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::TurnToParent(turn_to_parent_step) => {
                assert_eq!(turn_to_parent_step.block_id, paragraph_block_id3);
            },
            _ => panic!("Expected turn to parent step")
        };
    }
}