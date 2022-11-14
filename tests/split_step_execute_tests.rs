
#[cfg(test)]
mod tests {
    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap}, steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps, StepError}, steps_executor::execute_steps, mark::Mark};
    use serde_json::json;


    #[test]
    fn can_execute_enter_with_no_text_and_no_selection() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let root_block_id = new_ids.get_id().unwrap();

        let inline_block = json!({
            "_id": inline_block_id.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        }).to_string();
        let block = json!({
            "_id": paragraph_block_id,
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone().to_string()
        }).to_string();
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id.clone()]).to_string();

        let block_map = BlockMap::from(vec![inline_block, block, root_block]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let sub_selection = SubSelection::from(inline_block_id.clone().clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let original_paragraph_block = updated_state.block_map.get_standard_block(&paragraph_block_id).unwrap();
        let inline_blocks = &original_paragraph_block.content_block().unwrap().inline_blocks;
        assert_eq!(inline_blocks.len(), 1);
        assert_eq!(inline_blocks[0], inline_block_id.clone());

        let newly_added_standard_blocks = updated_state.block_map.get_newly_added_standard_blocks(vec![
            inline_block_id,
            paragraph_block_id,
            root_block_id
        ]).unwrap();
        assert_eq!(newly_added_standard_blocks.len(), 1);
        let new_std_block = &newly_added_standard_blocks[0];
        let inline_blocks = &new_std_block.content_block().unwrap().inline_blocks;
        assert_eq!(inline_blocks.len(), 1);
        let new_inline_block = updated_state.block_map.get_inline_block(&inline_blocks[0]).unwrap();
        assert_eq!(new_inline_block.text().unwrap(), &"".to_string());
        assert_eq!(new_inline_block.parent, new_std_block.id());
    }

    #[test]
    fn can_execute_enter_in_middle_of_text_with_caret_selection() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1= new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let root_block_id = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.clone()
        }).to_string();
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        }).to_string();
        let block = json!({
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone(), inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone().to_string()
        }).to_string();
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id.clone()]).to_string();

        let block_map = BlockMap::from(vec![inline_block1, inline_block2, block, root_block]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let sub_selection = SubSelection::from(inline_block_id1.clone(), 3, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let original_paragraph_block = updated_state.block_map.get_standard_block(&paragraph_block_id).unwrap();
        let inline_blocks = &original_paragraph_block.content_block().unwrap().inline_blocks;
        assert_eq!(inline_blocks.len(), 1);
        assert_eq!(inline_blocks[0], inline_block_id1.clone());

        let newly_added_standard_blocks = updated_state.block_map.get_newly_added_standard_blocks(vec![
            inline_block_id1,
            inline_block_id2.clone(),
            paragraph_block_id,
            root_block_id.clone()
        ]).unwrap();
        assert_eq!(newly_added_standard_blocks.len(), 1);
        let new_std_block = &newly_added_standard_blocks[0];
        let inline_blocks = &new_std_block.content_block().unwrap().inline_blocks;
        assert_eq!(inline_blocks.len(), 2);
        let new_inline_block = updated_state.block_map.get_inline_block(&inline_blocks[0]).unwrap();
        assert_eq!(new_inline_block.text().unwrap(), &"lo ".to_string());
        assert_eq!(new_inline_block.parent, new_std_block.id());
        assert_eq!(new_inline_block.marks, vec![Mark::Bold]);
        assert_eq!(inline_blocks[1], inline_block_id2);
        let inline_block_2 = updated_state.block_map.get_inline_block(&inline_block_id2).unwrap();
        assert_eq!(inline_block_2.text().unwrap(), "World!");

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children[1], new_std_block.id());

        match updated_state.selection {
            Some(selection) => {
                assert_eq!(selection.from, selection.to);
                assert_eq!(selection.from.block_id, new_inline_block.id());
                assert_eq!(selection.from.offset, 0);
            },
            None => panic!("Should be some selection")
        }
    }

    #[test]
    fn can_handle_with_selection_across_inline_blocks() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let root_block_id = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello"
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        }).to_string();
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "brave new "
            },
            "marks": ["italic"],
            "parent": paragraph_block_id.clone()
        }).to_string();
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        }).to_string();
        let block = json!({
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [
                    inline_block_id1.clone(), inline_block_id2.clone(), inline_block_id3.clone()
                ]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        }).to_string();
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id.clone()]).to_string();

        let block_map = BlockMap::from(vec![inline_block1, inline_block2, inline_block3, block, root_block]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let from_sub_selection = SubSelection::from(inline_block_id1.clone(), 4, None);
        let to_sub_selection = SubSelection::from(inline_block_id2.clone(), 1, None);
        let selection = Selection::from(from_sub_selection.clone(), to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let original_paragraph_block = updated_state.block_map.get_standard_block(&paragraph_block_id).unwrap();
        let inline_blocks = &original_paragraph_block.content_block().unwrap().inline_blocks;
        assert_eq!(inline_blocks.len(), 1);
        assert_eq!(inline_blocks[0], inline_block_id1.clone());

        let newly_added_standard_blocks = updated_state.block_map.get_newly_added_standard_blocks(vec![
            inline_block_id1,
            inline_block_id2.clone(),
            paragraph_block_id,
            root_block_id.clone()
        ]).unwrap();
        assert_eq!(newly_added_standard_blocks.len(), 1);
        let new_std_block = &newly_added_standard_blocks[0];
        let inline_blocks = &new_std_block.content_block().unwrap().inline_blocks;

        assert_eq!(inline_blocks.len(), 2);
        let new_inline_block = updated_state.block_map.get_inline_block(&inline_blocks[0]).unwrap();
        assert_eq!(new_inline_block.text().unwrap(), &"rave new ".to_string());
        assert_eq!(new_inline_block.parent, new_std_block.id());
        assert_eq!(new_inline_block.marks, vec![Mark::Italic]);
        assert_eq!(inline_blocks[1], inline_block_id3);
        let updated_inline_block_2 = updated_state.block_map.get_inline_block(&inline_block_id2).unwrap();
        assert_eq!(updated_inline_block_2.parent, new_std_block.id());

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children[1], new_std_block.id());

        match updated_state.selection {
            Some(selection) => {
                assert_eq!(selection.from, selection.to);
                assert_eq!(selection.from.block_id, new_inline_block.id());
                assert_eq!(selection.from.offset, 0);
            },
            None => panic!("Should be some selection")
        }
    }

    #[test]
    fn can_handle_enter_over_standard_blocks() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let paragraph_block_id1 = new_ids.get_id()?;
        let paragraph_block_id2 = new_ids.get_id()?;
        let paragraph_block_id3 = new_ids.get_id()?;
        let root_block_id = new_ids.get_id()?;

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello again!"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye World"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
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
        let paragraph_block2 = json!({
            "_id": paragraph_block_id2.to_string(),
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
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            paragraph_block_id1.clone(), paragraph_block_id2.clone(), paragraph_block_id3.clone()
        ]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(),
            paragraph_block1.to_string(), paragraph_block2.to_string(), paragraph_block3.to_string(), root_block.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let from_sub_selection = SubSelection::from(paragraph_block_id1.clone(), 0, Some(Box::new(
            SubSelection::from(inline_block_id1.clone(), 1, None)
        )));
        let to_sub_selection = SubSelection::from(paragraph_block_id3.clone(), 0, Some(Box::new(
            SubSelection::from(inline_block_id3.clone(), 3, None)
        )));
        let selection = Selection::from(from_sub_selection.clone(), to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let original_paragraph_block = updated_state.block_map.get_standard_block(&paragraph_block_id1).unwrap();
        let inline_blocks = &original_paragraph_block.content_block().unwrap().inline_blocks;
        assert_eq!(inline_blocks.len(), 1);
        assert_eq!(inline_blocks[0], inline_block_id1.clone());

        let newly_added_standard_blocks = updated_state.block_map.get_newly_added_standard_blocks(vec![
            inline_block_id1.to_string(), inline_block_id2.to_string(), inline_block_id3.to_string(),
            paragraph_block_id1.to_string(), paragraph_block_id2.to_string(), paragraph_block_id3.to_string(), root_block.to_string()
        ]).unwrap();

        assert_eq!(newly_added_standard_blocks.len(), 1);
        let new_std_block = &newly_added_standard_blocks[0];
        let inline_blocks = &new_std_block.content_block().unwrap().inline_blocks;

        assert_eq!(inline_blocks.len(), 2);
        let new_inline_block = updated_state.block_map.get_inline_block(&inline_blocks[0]).unwrap();
        assert_eq!(new_inline_block.text().unwrap(), &"rave new ".to_string());
        assert_eq!(new_inline_block.parent, new_std_block.id());
        assert_eq!(new_inline_block.marks, vec![Mark::Italic]);
        assert_eq!(inline_blocks[1], inline_block_id3);
        let updated_inline_block_2 = updated_state.block_map.get_inline_block(&inline_block_id2).unwrap();
        assert_eq!(updated_inline_block_2.parent, new_std_block.id());

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children[1], new_std_block.id());

        match updated_state.selection {
            Some(selection) => {
                assert_eq!(selection.from, selection.to);
                assert_eq!(selection.from.block_id, new_inline_block.id());
                assert_eq!(selection.from.offset, 0);
            },
            None => panic!("Should be some selection")
        }

        return Ok(())
    }
}