

#[cfg(test)]
mod tests {
    use rust_mirror::steps_executor::execute_steps;
    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap}, steps_generator::{event::{Event, KeyPress, Key, KeyPressMetadata}, selection::{SubSelection, Selection}, generate_steps}, step::Step};
    use serde_json::json;

    #[test]
    fn can_execute_turn_to_child_step_simple() {
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

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Tab, None));
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let updated_paragraph_block1 = updated_state.block_map.get_standard_block(&paragraph_block_id1.clone()).unwrap();
        assert_eq!(updated_paragraph_block1.children.len(), 1);
        assert_eq!(updated_paragraph_block1.children[0], paragraph_block_id2.clone());

        let updated_paragraph_block2 = updated_state.block_map.get_standard_block(&paragraph_block_id2.clone()).unwrap();
        assert_eq!(updated_paragraph_block2.parent, paragraph_block_id1.clone());

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id.clone()).unwrap();
        assert_eq!(updated_root_block.children.len(), 1);
        assert_eq!(updated_root_block.children[0], paragraph_block_id1);
    }

    #[test]
    fn can_turn_to_parent() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
        let paragraph_block_id3 = new_ids.get_id().unwrap();
        let paragraph_block_id4 = new_ids.get_id().unwrap();
        let paragraph_block_id5 = new_ids.get_id().unwrap();
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
            "children": [paragraph_block_id5.clone()],
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
        let paragraph_block5 = json!({
            "_id": paragraph_block_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id3.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(),
            paragraph_block3.to_string(), paragraph_block4.to_string(), paragraph_block5.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress { key: Key::Tab, metadata: KeyPressMetadata { shift_down: true, meta_down: false, ctrl_down: false, alt_down: false } });
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        assert_eq!(updated_state.selection, None);

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![paragraph_block_id1.clone(), paragraph_block_id3.clone()]);

        let new_parent = updated_state.block_map.get_standard_block(&paragraph_block_id3).unwrap();
        assert_eq!(new_parent.parent, root_block_id.clone());
        assert_eq!(new_parent.children, vec![paragraph_block_id5, paragraph_block_id4]);

        let previous_parent = updated_state.block_map.get_standard_block(&paragraph_block_id1).unwrap();
        assert_eq!(previous_parent.children, vec![paragraph_block_id2]);
    }
}