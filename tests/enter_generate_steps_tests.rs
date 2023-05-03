#[cfg(test)]
mod tests {
    use rust_mirror::{blocks::{BlockMap, standard_blocks::StandardBlockType, RootBlock},
        steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps, StepError},
        step::{Step, ReplaceSlice, SplitStep, AddBlockStep}, new_ids::NewIds};

    use serde_json::json;

    #[test]
    fn can_handle_enter_with_no_text_and_no_selection() {
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
        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::SplitStep(split_step) => {
                assert_eq!(split_step.subselection, sub_selection.clone());
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    #[test]
    fn can_handle_in_middle_of_text_with_caret_selection() {
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
        let sub_selection = SubSelection::from(inline_block_id1.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0], Step::SplitStep(SplitStep { subselection: sub_selection }))
    }

    #[test]
    fn can_handle_in_middle_of_inline_block_with_some_selection() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1 = new_ids.get_id().unwrap();
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
        let from_sub_selection = SubSelection::from(inline_block_id1.clone(), 2, None);
        let to_sub_selection = SubSelection::from(inline_block_id1.clone(), 4, None);
        let selection = Selection::from(from_sub_selection.clone(),to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 2);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, from_sub_selection.clone());
                assert_eq!(replace_step.to, to_sub_selection.clone());
                match &replace_step.slice {
                    ReplaceSlice::String(s) => assert_eq!(s, &"".to_string()),
                    ReplaceSlice::Blocks(_) => panic!("Expected string replace slice")
                }
            },
            _ => panic!("Expected ReplaceStep")
        };
        match &steps[1] {
            Step::SplitStep(split_step) => {
                assert_eq!(split_step.subselection, from_sub_selection.clone());
            },
            _ => panic!("Expected Split step")
        };
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
            "marks": ["italic"],
            "parent": paragraph_block_id.clone()
        }).to_string();
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "brave new "
            },
            "marks": [],
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
        let to_sub_selection = SubSelection::from(inline_block_id3.clone(), 1, None);
        let selection = Selection::from(from_sub_selection.clone(), to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 2);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, from_sub_selection.clone());
                assert_eq!(replace_step.to, to_sub_selection.clone());
                match &replace_step.slice {
                    ReplaceSlice::String(s) => assert_eq!(s, &"".to_string()),
                    ReplaceSlice::Blocks(_) => panic!("Expected string replace slice")
                }
            },
            _ => panic!("Expected ReplaceStep")
        };
        match &steps[1] {
            Step::SplitStep(split_step) => {
                assert_eq!(split_step.subselection, from_sub_selection.clone());
            },
            _ => panic!("Expected Split step")
        };
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
        let to_sub_selection = SubSelection::from(paragraph_block_id3.clone(), 1, Some(Box::new(
            SubSelection::from(inline_block_id3.clone(), 3, None)
        )));
        let selection = Selection::from(from_sub_selection.clone(), to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 2);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, from_sub_selection.clone());
                assert_eq!(replace_step.to, to_sub_selection.clone());
                match &replace_step.slice {
                    ReplaceSlice::String(s) => assert_eq!(s, &"".to_string()),
                    ReplaceSlice::Blocks(_) => panic!("Expected string replace slice")
                }
            },
            _ => panic!("Expected ReplaceStep")
        };
        match &steps[1] {
            Step::SplitStep(split_step) => {
                assert_eq!(split_step.subselection, SubSelection::from(inline_block_id1.clone(), 1, None));
            },
            _ => panic!("Expected Split step")
        };
        return Ok(())
    }

    #[test]
    fn enter_at_start_of_block_edge_case_should_add_paragraph_block_above() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1 = new_ids.get_id()?;
        let paragraph_block_id1 = new_ids.get_id()?;
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
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            paragraph_block_id1.clone(),
        ]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(),
            paragraph_block1.to_string(),
            root_block.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let sub_selection = SubSelection::from(inline_block_id1.clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::AddBlock(AddBlockStep {
                block_id,
                child_offset,
                block_type,
                focus_block_below
            }) => {
                assert_eq!(*block_id, root_block_id.clone());
                assert_eq!(*child_offset, 0);
                assert_eq!(*focus_block_below, true);
                match block_type {
                    StandardBlockType::Paragraph(_) => {},
                    _ => panic!("Expected block type to be paragraph")
                }
            },
            _ => panic!("Expected AddBlock step")
        };
        return Ok(())
    }
}