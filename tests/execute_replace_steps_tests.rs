#[cfg(test)]
mod tests {
    use serde_json::json;

    use rust_mirror::{steps_generator::{StepError, event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps},
    new_ids::NewIds, blocks::{RootBlock, BlockMap, Block}, steps_actualisor::actualise_steps,
    mark::Mark, custom_copy::CustomCopy};

    #[test]
    fn can_actualise_steps_for_standard_keypress() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let paragraph_block_id = new_ids.get_id()?;
        let inline_block_id = new_ids.get_id()?;
        let inline_block = json!({
            "_id": inline_block_id.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        });
        let block = json!({
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![inline_block.to_string(), block.to_string(), root_block.to_string()]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Standard('a'), None));
        let sub_selection = SubSelection::from(inline_block_id.clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection)?;
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new())?;

        let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id)?;
        assert_eq!(updated_inline_block.text()?.clone().to_string().as_str(), "a");
        let expected_subselection = SubSelection { block_id: inline_block_id, offset: 1, subselection: None };
        assert_eq!(updated_state.selection, Some(Selection { anchor: expected_subselection.clone(), head: expected_subselection }));
        Ok(())
    }

    // #[test]
    // pub fn can_actualise_backspace_with_caret_selection_in_middle_of_two_inline_blocks() {
    //     let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

    //     let root_block_id = new_ids.get_id().unwrap();
    //     let paragraph_block_id = new_ids.get_id().unwrap();
    //     let inline_block_id1 = new_ids.get_id().unwrap();
    //     let inline_block_id2 = new_ids.get_id().unwrap();
    //     let inline_block1 = json!({
    //         "_id": inline_block_id1.clone(),
    //         "kind": "inline",
    //         "_type": "text",
    //         "content": {
    //             "text": "Hello"
    //         },
    //         "marks": [],
    //         "parent": paragraph_block_id.clone()
    //     });
    //     let inline_block2 = json!({
    //         "_id": inline_block_id2.clone(),
    //         "kind": "inline",
    //         "_type": "text",
    //         "content": {
    //             "text": " World"
    //         },
    //         "marks": [],
    //         "parent": paragraph_block_id.clone()
    //     });
    //     let block = json!({
    //         "_id": paragraph_block_id.clone(),
    //         "kind": "standard",
    //         "_type": "paragraph",
    //         "content": {
    //             "inline_blocks": [inline_block_id1.clone(), inline_block_id2.clone()]
    //         },
    //         "children": [],
    //         "marks": [],
    //         "parent": root_block_id.clone()
    //     });
    //     let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);
    //     let block_map = BlockMap::from(vec![
    //         inline_block1.to_string(), inline_block2.to_string(), block.to_string(), root_block.to_string()
    //     ]).unwrap();
    //     let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
    //     let sub_selection = SubSelection::from(inline_block_id2.clone(), 0, None);
    //     let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

    //     let steps = generate_steps(&event, &block_map, selection).unwrap();
    //     let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

    //     let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id)?;
    //     assert_eq!(updated_inline_block.text()?, "a");
    //     let expected_subselection = SubSelection { block_id: inline_block_id, offset: 1, subselection: None };
    //     assert_eq!(updated_state.selection, Selection { from: expected_subselection.clone(), to: expected_subselection });
    //     Ok(())
    // }

    #[test]
    fn can_actualise_steps_for_standard_keypress_with_selection_across_single_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let inline_block_id = new_ids.get_id().unwrap();
        let inline_block = json!({
            "_id": inline_block_id.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "some text"
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.clone()
        });

        let block = json!({
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![
            inline_block.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('k'), None));
        let from_sub_selection = SubSelection::from(inline_block_id.clone(), 2, None);
        let to_sub_selection = SubSelection::from(inline_block_id.clone(), 4, None);
        let selection = Selection::from(from_sub_selection.clone(), to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id).unwrap();
        assert_eq!(updated_inline_block.text().unwrap().clone().to_string().as_str(), "sok text");
        assert_eq!(updated_inline_block.marks, vec![Mark::Bold]);
    }

    #[test]
    fn can_actualise_for_selection_across_multiple_inline_blocks() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3  = new_ids.get_id().unwrap();
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello"
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " new"
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " World"
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        });

        let block = json!({
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone(), inline_block_id2.clone(), inline_block_id3.clone(), ]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard(' '), None));
        let from_sub_selection = SubSelection::from(inline_block_id1.clone(), 2, None);
        let to_sub_selection = SubSelection::from(inline_block_id3.clone(), 2, None);
        let selection = Selection::from(from_sub_selection.clone(), to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline_block.text().unwrap().clone().to_string().as_str(), "He orld");
        let updated_paragraph_block = updated_state.block_map.get_standard_block(&paragraph_block_id).unwrap();
        let updated_content_block = updated_paragraph_block.content_block().unwrap();
        assert_eq!(updated_content_block.inline_blocks.len(), 1);
        assert_eq!(updated_content_block.inline_blocks.contains(&inline_block_id1), true);
    }

    /// Input:
    /// <1>H|ello world</1>
    ///     <4/>
    /// <5></5>
    /// <3>Goo|dbye world</3>
    ///     <2/>
    ///        | | |
    ///        | | |
    ///        V V V
    /// Output:
    /// <1>H dbye world</1>
    ///    <2/>
    #[test]
    fn can_handle_keypress_execution_across_3_standard_blocks() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let std_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let std_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let std_block_id3 = new_ids.get_id().unwrap();
        let std_block_id4 = new_ids.get_id().unwrap();
        let std_block_id5 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello world!"
            },
            "marks": [],
            "parent": std_block_id1.clone()
        });

        let std_block1 = json!({
            "_id": std_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [std_block_id4.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let std_block_4 = Block::new_std_block_json(std_block_id4.clone(), std_block_id1.clone());

        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye "
            },
            "marks": [],
            "parent": std_block_id3.clone()
        });

        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "world!"
            },
            "marks": ["bold"],
            "parent": std_block_id3.clone()
        });

        let std_block2 = json!({
            "_id": std_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": std_block_id3.clone()
        });
        let std_block3 = json!({
            "_id": std_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone(), inline_block_id3.clone()]
            },
            "children": [std_block_id2.to_string()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let std_block5 = json!({
            "_id": std_block_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            std_block_id1.clone(), std_block_id5.clone(), std_block_id3.clone()
            ]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), std_block1.to_string(), std_block2.to_string(),
            root_block.to_string(), std_block3.to_string(), std_block_4.to_string(), inline_block3.to_string(), std_block5.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard(' '), None));

        let selection = Selection {
            anchor: SubSelection {
                block_id: std_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id1.clone(),
                    offset: 1,
                    subselection: None,
                }))
            },
            head: SubSelection {
                block_id: std_block_id3.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id3.clone(),
                    offset: 3,
                    subselection: None,
                }))
            },
        };

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();
        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![std_block_id1.clone()]);
        let updated_std_block1 = updated_state.block_map.get_standard_block(&std_block_id1).unwrap();
        assert_eq!(updated_std_block1.children, vec![std_block_id2.clone()]);
        assert_eq!(
            updated_std_block1.content_block().unwrap().inline_blocks,
            vec![inline_block_id1.clone(), inline_block_id3.clone()]
        );

        let updated_inline_block1 = updated_state.block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline_block1.text().unwrap().clone().to_string().as_str(), "H ");
        let updated_inline_block3 = updated_state.block_map.get_inline_block(&inline_block_id3).unwrap();
        assert_eq!(updated_inline_block3.text().unwrap().clone().to_string().as_str(), "ld!");
        assert_eq!(updated_inline_block3.parent, updated_std_block1.id());

        let updated_paragraph_block2 = updated_state.block_map.get_standard_block(&std_block_id2).unwrap();
        assert_eq!(updated_paragraph_block2.parent, std_block_id1);

        let expected_subselection = SubSelection {
            block_id: inline_block_id1,
            offset: 2,
            subselection: None,
        };
        assert_eq!(updated_state.selection, Some(Selection { anchor: expected_subselection.clone(), head: expected_subselection }))
    }

    /// Input:
    /// <1>Hello world</1>
    ///     <2/>
    ///     <3/>
    ///         <4/>
    ///         <5>a b c</5>
    /// <6>||Goodbye world</6>
    ///     <7/>
    ///     <8/>
    ///        | | |
    ///        | | |
    ///        V V V
    /// Output:
    /// <1>Hello world</1>
    ///     <2/>
    ///     <3/>
    ///         <4/>
    ///         <5>a b cGoodbye world</5>
    ///             <7/>
    ///             <8/>
    #[test]
    fn can_actualise_backspace_at_start_caret_on_top_layer_creates_replace_step_with_last_and_youngest_child_above() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let std_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let std_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let std_block_id3 = new_ids.get_id().unwrap();
        let std_block_id4 = new_ids.get_id().unwrap();
        let std_block_id5 = new_ids.get_id().unwrap();
        let std_block_id6 = new_ids.get_id().unwrap();
        let std_block_id7 = new_ids.get_id().unwrap();
        let std_block_id8 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello world!"
            },
            "marks": [],
            "parent": std_block_id1.clone()
        });

        let std_block1 = json!({
            "_id": std_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [std_block_id2.clone(), std_block_id3.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let std_block2 = Block::new_std_block_json(std_block_id2.clone(), std_block_id1.clone());
        let std_block3 = json!({
            "_id": std_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [std_block_id4.clone(), std_block_id5.clone()],
            "marks": [],
            "parent": std_block_id1.clone()
        });
        let std_block4 = json!({
            "_id": std_block_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": std_block_id3.clone()
        });
        let std_block5 = json!({
            "_id": std_block_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": std_block_id3.clone()
        });

        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "a b c"
            },
            "marks": [],
            "parent": std_block_id5.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye World"
            },
            "marks": [],
            "parent": std_block_id6.clone()
        });

        let std_block6 = json!({
            "_id": std_block_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [std_block_id7.clone(), std_block_id8.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let std_block7 = json!({
            "_id": std_block_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": std_block_id6.clone()
        });
        let std_block8 = json!({
            "_id": std_block_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": std_block_id6.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(),
        vec![std_block_id1.clone(), std_block_id6.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), std_block1.to_string(), std_block2.to_string(),
            root_block.to_string(), std_block3.to_string(), std_block4.to_string(), std_block5.to_string(), std_block6.to_string(), std_block7.to_string(), std_block8.to_string(),
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));

        let subselection = SubSelection {
            block_id: inline_block_id3.clone(),
            offset: 0,
            subselection: None,
        };
        let selection = Selection {
            anchor: subselection.clone(),
            head: subselection.clone()
        };

        let steps = generate_steps(&event, &block_map, selection.clone()).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_inline_block_2 = updated_state.block_map.get_inline_block(&inline_block_id2).unwrap();
        assert_eq!(updated_inline_block_2.text().unwrap().clone().to_string().as_str(), &"a b cGoodbye World".to_string());

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![std_block_id1.clone()]);

        let updated_block5 = updated_state.block_map.get_standard_block(&std_block_id5).unwrap();
        assert_eq!(updated_block5.children, vec![std_block_id7.clone(), std_block_id8.clone()]);

        let updated_block7 = updated_state.block_map.get_standard_block(&std_block_id7).unwrap();
        assert_eq!(updated_block7.parent, std_block_id5.clone());
        let updated_block8 = updated_state.block_map.get_standard_block(&std_block_id8).unwrap();
        assert_eq!(updated_block8.parent, std_block_id5.clone());
    }
    /// Input:
    /// <1>Hello |world</1>
    ///     <2/>
    ///     <3/>
    ///
    /// <4/>
    /// <5>a b c</5>
    ///     <6>Good|bye world</6>
    ///         <7/>
    ///     <8/>
    ///        | | |
    ///        | | |
    ///        V V V
    /// Output:
    /// <1>Hello bye world</1>
    ///     <7/>
    /// <8/>
    #[test]
    fn can_actualise_replace_where_from_is_shallower_than_to() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let std_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let std_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let std_block_id3 = new_ids.get_id().unwrap();
        let std_block_id4 = new_ids.get_id().unwrap();
        let std_block_id5 = new_ids.get_id().unwrap();
        let std_block_id6 = new_ids.get_id().unwrap();
        let std_block_id7 = new_ids.get_id().unwrap();
        let std_block_id8 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello world!"
            },
            "marks": [],
            "parent": std_block_id1.clone()
        });

        let std_block1 = json!({
            "_id": std_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [std_block_id2.clone(), std_block_id3.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let std_block2 = Block::new_std_block_json(std_block_id2.clone(), std_block_id1.clone());
        let std_block3 = json!({
            "_id": std_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": std_block_id1.clone()
        });
        let std_block4 = json!({
            "_id": std_block_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let std_block5 = json!({
            "_id": std_block_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [std_block_id6.clone(), std_block_id8.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "a b c"
            },
            "marks": [],
            "parent": std_block_id5.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye World"
            },
            "marks": [],
            "parent": std_block_id6.clone()
        });

        let std_block6 = json!({
            "_id": std_block_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [std_block_id7.clone()],
            "marks": [],
            "parent": std_block_id5.clone()
        });
        let std_block7 = json!({
            "_id": std_block_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": std_block_id6.clone()
        });
        let std_block8 = json!({
            "_id": std_block_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": std_block_id5.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(),
        vec![std_block_id1.clone(), std_block_id4.clone(), std_block_id5.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), std_block1.to_string(), std_block2.to_string(),
            root_block.to_string(), std_block3.to_string(), std_block4.to_string(), std_block5.to_string(), std_block6.to_string(), std_block7.to_string(), std_block8.to_string(),
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));

        let from_subselection = SubSelection {
            block_id: std_block_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: inline_block_id1.clone(),
                offset: 6,
                subselection: None
            }))
        };
        let to_subselection = SubSelection {
            block_id: std_block_id5.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: std_block_id6.clone().clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id3.clone(),
                    offset: 4,
                    subselection: None
                }))
            }))
        };

        let selection = Selection {
            anchor: from_subselection.clone(),
            head: to_subselection.clone()
        };

        let steps = generate_steps(&event, &block_map, selection.clone()).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_inline_block_1 = updated_state.block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline_block_1.text().unwrap().clone().to_string().as_str(), &"Hello bye World".to_string());

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![std_block_id1.clone(), std_block_id8.clone()]);

        let updated_block1 = updated_state.block_map.get_standard_block(&std_block_id1).unwrap();
        assert_eq!(updated_block1.children, vec![std_block_id7.clone()]);

        let updated_block7 = updated_state.block_map.get_standard_block(&std_block_id7).unwrap();
        assert_eq!(updated_block7.parent, std_block_id1.clone());

        let updated_block8 = updated_state.block_map.get_standard_block(&std_block_id8).unwrap();
        assert_eq!(updated_block8.parent, root_block_id.clone());
    }

    /// Input:
    /// <1>H|ello world</1>
    ///     <2|/>
    /// <3></3>
    ///        | | |
    ///        | | |
    ///        V V V
    /// Output:
    /// <1>H</1>
    /// <3></3>
    #[test]
    fn can_handle_keypress_execution_across_standard_blocks_with_different_layer_depths() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let std_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let std_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let std_block_id3 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello world!"
            },
            "marks": [],
            "parent": std_block_id1.clone()
        });
        let std_block1 = json!({
            "_id": std_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [std_block_id2.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "2"
            },
            "marks": [],
            "parent": std_block_id2.clone()
        });
        let std_block2 = json!({
            "_id": std_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": std_block_id1.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "3"
            },
            "marks": [],
            "parent": std_block_id3.clone()
        });
        let std_block3 = json!({
            "_id": std_block_id3.clone(),
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
            std_block_id1.clone(),  std_block_id3.clone()
            ]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), std_block1.to_string(), std_block2.to_string(),
            root_block.to_string(), std_block3.to_string(),
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard(' '), None));

        let selection = Selection {
            anchor: SubSelection {
                block_id: std_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id1.clone(),
                    offset: 1,
                    subselection: None,
                }))
            },
            head: SubSelection {
            block_id: std_block_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: std_block_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id2.clone(),
                    offset: 1,
                    subselection: None,
                }))
            }))},
        };

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();
        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children.len(), 2);
    }

    #[test]
    pub fn backspace_with_caret_at_start_of_empty_paragraph_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
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
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "new "
            },
            "marks": ["bold"],
            "parent": paragraph_block_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " World"
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
            "_id": paragraph_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone(), inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![paragraph_block_id1.clone()]);

        let updated_paragraph_block1 = updated_state.block_map.get_standard_block(&paragraph_block_id1).unwrap();
        assert_eq!(updated_paragraph_block1.content_block().unwrap().inline_blocks, vec![inline_block_id1.clone(), inline_block_id2, inline_block_id3]);

        assert_eq!(updated_state.selection, Some(Selection {
            anchor: SubSelection { block_id: inline_block_id1.clone(), offset: 5, subselection: None },
            head: SubSelection { block_id: inline_block_id1.clone(), offset: 5, subselection: None }
        }))
    }

    #[test]
    pub fn backspace_with_selection_across_all_of_a_paragraphs_text() {
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
            "marks": ["bold"],
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
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let from_sub_selection = SubSelection::from(inline_block_id1.clone(), 0, None);
        let to_sub_selection = SubSelection::from(inline_block_id2.clone(), 6, None);
        let selection = Selection::from(from_sub_selection, to_sub_selection);

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_paragraph_block = updated_state.block_map.get_standard_block(&paragraph_block_id1).unwrap();
        assert_eq!(updated_paragraph_block.content_block().unwrap().inline_blocks, vec![inline_block_id1.clone()]);

        let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline_block.text().unwrap().clone().to_string().as_str(), &"".to_string());

        assert_eq!(updated_state.selection, Some(Selection {
            anchor: SubSelection { block_id: inline_block_id1.clone(), offset: 0, subselection: None },
            head: SubSelection { block_id: inline_block_id1.clone(), offset: 0, subselection: None }
        }))
    }

//     #[test]
//     pub fn can_merge_2_inline_blocks_that_should_be() {
//         let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

//         let root_block_id = new_ids.get_id().unwrap();
//         let inline_block_id1 = new_ids.get_id().unwrap();
//         let inline_block_id2 = new_ids.get_id().unwrap();
//         let std_block_id1 = new_ids.get_id().unwrap();

//         let inline_block1 = json!({
//             "_id": inline_block_id1.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "Hello"
//             },
//             "marks": ["bold"],
//             "parent": std_block_id1.clone()
//         });
//         let inline_block2 = json!({
//             "_id": inline_block_id2.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": " World!"
//             },
//             "marks": ["bold"],
//             "parent": std_block_id1.clone()
//         });

//         let std_block1 = json!({
//             "_id": std_block_id1.clone(),
//             "kind": "standard",
//             "_type": "paragraph",
//             "content": {
//                 "inline_blocks": [inline_block_id1.clone()]
//             },
//             "children": [],
//             "marks": [],
//             "parent": root_block_id.clone()
//         });

//         let root_block = RootBlock::json_from(root_block_id, vec![std_block_id1.clone()]);

//         let block_map = BlockMap::from(vec![
//             inline_block1.to_string(), inline_block2.to_string(), std_block1.to_string(), root_block.to_string()
//         ]).unwrap();

//         let steps = vec![
//             Step::ReplaceStep(ReplaceStep {
//                 block_id: std_block_id1.clone(),
//                 from: SubSelection { block_id: std_block_id1.clone(), offset: 1, subselection: None },
//                 to: SubSelection { block_id: std_block_id1.clone(), offset: 1, subselection: None },
//                 slice: vec![inline_block_id2],
//                 blocks_to_update: vec![]
//             })
//         ];
//         let updated_block_map = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();
//         let updated_standard_block = updated_block_map.get_standard_block(&std_block_id1).unwrap();
//         let content_block = updated_standard_block.content_block().unwrap();
//         assert_eq!(content_block.inline_blocks, vec![inline_block_id1.clone()]);
//         let updated_inline_block1 = updated_block_map.get_inline_block(&inline_block_id1).unwrap();
//         assert_eq!(updated_inline_block1.text().unwrap(), "Hello World!");
//     }

    /// Input:
    /// <1></1>
    ///     <2/>
    ///     <3>Hello| World<3/>
    ///     <4/>
    /// <5/>
    ///     <6/>
    ///     <7>Good|bye<7/>
    ///     <8/>
    /// 
    /// Output:
    /// <1></1>
    ///    <2/>
    ///    <3>Hellobye<3/>
    ///    <8/>
    #[test]
    pub fn testing_from_child_to_child_parents_on_root_layer() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();
    
        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
        let paragraph_block_id3 = new_ids.get_id().unwrap();
        let paragraph_block_id4 = new_ids.get_id().unwrap();
        let paragraph_block_id5 = new_ids.get_id().unwrap();
        let paragraph_block_id6 = new_ids.get_id().unwrap();
        let paragraph_block_id7 = new_ids.get_id().unwrap();
        let paragraph_block_id8 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let inline_block_id4 = new_ids.get_id().unwrap();
        let inline_block_id5 = new_ids.get_id().unwrap();
        let inline_block_id6 = new_ids.get_id().unwrap();
        let inline_block_id7 = new_ids.get_id().unwrap();
        let inline_block_id8 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": [],
            "parent": paragraph_block_id3.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id4.clone()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id5.clone()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id7.clone()
        });
        let inline_block8 = json!({
            "_id": inline_block_id8.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id8.clone()
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
        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
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
                "inline_blocks": [inline_block_id4.clone()]
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
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [paragraph_block_id6.clone(), paragraph_block_id7.clone(), paragraph_block_id8.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let paragraph_block6 = json!({
            "_id": paragraph_block_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id5.clone()
        });
        let paragraph_block7 = json!({
            "_id": paragraph_block_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id5.clone()
        });
        let paragraph_block8 = json!({
            "_id": paragraph_block_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id8.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id5.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone(), paragraph_block_id5.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), inline_block4.to_string(),
            inline_block5.to_string(), inline_block6.to_string(), inline_block7.to_string(), inline_block8.to_string(),
            paragraph_block1.to_string(), paragraph_block2.to_string(), paragraph_block3.to_string(), 
            paragraph_block4.to_string(),paragraph_block5.to_string(), paragraph_block6.to_string(), 
            paragraph_block7.to_string(), paragraph_block8.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let selection = Selection {
            anchor: SubSelection {
                block_id: paragraph_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: paragraph_block_id3.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: inline_block_id3.clone(),
                        offset: 5,
                        subselection: None,
                    }))
                }))},
            head: SubSelection {
            block_id: paragraph_block_id5.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: paragraph_block_id7.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id7.clone(),
                    offset: 4,
                    subselection: None,
                }))
            }))},
        };

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![paragraph_block_id1.clone()]);

        let updated_paragraph_block1 = updated_state.block_map.get_standard_block(&paragraph_block_id1).unwrap();
        assert_eq!(updated_paragraph_block1.children, vec![paragraph_block_id2.clone(), paragraph_block_id3.clone(), paragraph_block_id8.clone()]);

    }

    /// Input:
    /// <1>Hello |world</1>
    ///     <2/>
    ///     <3>Hello| World<3/>
    ///     <4/>
    /// 
    /// Output:
    /// <1>Hello World</1>
    ///    <4/>

    
    #[test]
    fn test() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
        let paragraph_block_id3 = new_ids.get_id().unwrap();
        let paragraph_block_id4 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let inline_block_id4 = new_ids.get_id().unwrap();

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
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": [],
            "parent": paragraph_block_id3.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id4.clone()
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
        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
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
                "inline_blocks": [inline_block_id4.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), inline_block4.to_string(),
            paragraph_block1.to_string(), paragraph_block2.to_string(), paragraph_block3.to_string(), 
            paragraph_block4.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let selection = Selection {
            anchor: SubSelection {
                block_id: paragraph_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id1.clone(),
                    offset: 6,
                    subselection: None,
                }))},
            head: SubSelection {
            block_id: paragraph_block_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: paragraph_block_id3.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id3.clone(),
                    offset: 4,
                    subselection: None,
                }))
            }))},
        };

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_paragraph_block1 = updated_state.block_map.get_standard_block(&paragraph_block_id1).unwrap();
        assert_eq!(updated_paragraph_block1.children, vec![paragraph_block_id4.clone()]);

    }
    /// Input:
    /// <1></1>
    ///    <2/> selection starts inside here
    ///      <3/>
    ///    <4/>
    /// <5/>
    ///     <6/>
    ///       <7/>
    ///         <8/> selection ends inside here
    ///    <9/>
    ///    <10/>
    /// 
    /// Output:
    /// <1></1>
    ///     <2/>
    /// <9/>
    /// <10/>

    #[test]
    fn test2() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();
    
        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = new_ids.get_id().unwrap();
        let paragraph_block_id2 = new_ids.get_id().unwrap();
        let paragraph_block_id3 = new_ids.get_id().unwrap();
        let paragraph_block_id4 = new_ids.get_id().unwrap();
        let paragraph_block_id5 = new_ids.get_id().unwrap();
        let paragraph_block_id6 = new_ids.get_id().unwrap();
        let paragraph_block_id7 = new_ids.get_id().unwrap();
        let paragraph_block_id8 = new_ids.get_id().unwrap();
        let paragraph_block_id9 = new_ids.get_id().unwrap();
        let paragraph_block_id10 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let inline_block_id4 = new_ids.get_id().unwrap();
        let inline_block_id5 = new_ids.get_id().unwrap();
        let inline_block_id6 = new_ids.get_id().unwrap();
        let inline_block_id7 = new_ids.get_id().unwrap();
        let inline_block_id8 = new_ids.get_id().unwrap();
        let inline_block_id9 = new_ids.get_id().unwrap();
        let inline_block_id10 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id3.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id4.clone()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id5.clone()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id7.clone()
        });
        let inline_block8 = json!({
            "_id": inline_block_id8.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello Worlds"
            },
            "marks": [],
            "parent": paragraph_block_id8.clone()
        });
        let inline_block9 = json!({
            "_id": inline_block_id9.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id9.clone()
        });
        let inline_block10 = json!({
            "_id": inline_block_id10.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id10.clone()
        });
        let paragraph_block1 = json!({
            "_id": paragraph_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [paragraph_block_id2.clone(), paragraph_block_id4.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let paragraph_block2 = json!({
            "_id": paragraph_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [paragraph_block_id3.clone()],
            "marks": [],
            "parent": paragraph_block_id1.clone()
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
            "parent": paragraph_block_id2.clone()
        });
        let paragraph_block4 = json!({
            "_id": paragraph_block_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
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
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [paragraph_block_id6.clone(), paragraph_block_id9.clone(), paragraph_block_id10.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let paragraph_block6 = json!({
            "_id": paragraph_block_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [paragraph_block_id7.clone()],
            "marks": [],
            "parent": paragraph_block_id5.clone()
        });
        let paragraph_block7 = json!({
            "_id": paragraph_block_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [paragraph_block_id8.clone()],
            "marks": [],
            "parent": paragraph_block_id6.clone()
        });
        let paragraph_block8 = json!({
            "_id": paragraph_block_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id8.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id7.clone()
        });
        let paragraph_block9 = json!({
            "_id": paragraph_block_id9.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id9.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id5.clone()
        });
        let paragraph_block10 = json!({
            "_id": paragraph_block_id10.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id10.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id5.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone(), paragraph_block_id5.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), inline_block4.to_string(),
            inline_block5.to_string(), inline_block6.to_string(), inline_block7.to_string(), inline_block8.to_string(),
            inline_block9.to_string(), inline_block10.to_string(),
            paragraph_block1.to_string(), paragraph_block2.to_string(), paragraph_block3.to_string(), 
            paragraph_block4.to_string(),paragraph_block5.to_string(), paragraph_block6.to_string(), 
            paragraph_block7.to_string(), paragraph_block8.to_string(), paragraph_block9.to_string(), 
            paragraph_block10.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let selection = Selection {
            anchor: SubSelection {
                block_id: paragraph_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: paragraph_block_id2.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: inline_block_id2.clone(),
                        offset: 5,
                        subselection: None,
                    }))
                }))},
            head: SubSelection {
            block_id: paragraph_block_id5.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: paragraph_block_id6.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: paragraph_block_id7.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: paragraph_block_id8.clone(),
                        offset: 0,
                        subselection: Some(Box::new(SubSelection {
                            block_id: inline_block_id8.clone(),
                            offset: 4,
                            subselection: None,
                        })),
                    })),
                }))
            }))},
        };

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![paragraph_block_id1.clone(), paragraph_block_id9.clone(), paragraph_block_id10.clone()]);

        let updated_paragraph_block1 = updated_state.block_map.get_standard_block(&paragraph_block_id1).unwrap();
        assert_eq!(updated_paragraph_block1.children, vec![paragraph_block_id2.clone()]);

    }
}

