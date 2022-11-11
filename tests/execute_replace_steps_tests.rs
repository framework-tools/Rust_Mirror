#[cfg(test)]
mod tests {
    use serde_json::json;

    use rust_mirror::{steps_generator::{StepError, event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps}, new_ids::NewIds, blocks::{RootBlock, BlockMap, Block}, steps_executor::execute_steps, mark::Mark};

    #[test]
    fn can_execute_steps_for_standard_keypress() -> Result<(), StepError> {
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

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids)?;
        let updated_state = execute_steps(steps, block_map, &mut new_ids)?;

        let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id)?;
        assert_eq!(updated_inline_block.text()?, "a");
        let expected_subselection = SubSelection { block_id: inline_block_id, offset: 1, subselection: None };
        assert_eq!(updated_state.selection, Selection { from: expected_subselection.clone(), to: expected_subselection });
        Ok(())
    }

    // #[test]
    // pub fn can_execute_backspace_with_caret_selection_in_middle_of_two_inline_blocks() {
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

    //     let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();
    //     let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

    //     let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id)?;
    //     assert_eq!(updated_inline_block.text()?, "a");
    //     let expected_subselection = SubSelection { block_id: inline_block_id, offset: 1, subselection: None };
    //     assert_eq!(updated_state.selection, Selection { from: expected_subselection.clone(), to: expected_subselection });
    //     Ok(())
    // }

    #[test]
    fn can_execute_steps_for_standard_keypress_with_selection_across_single_block() {
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

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id).unwrap();
        assert_eq!(updated_inline_block.text().unwrap(), "sok text");
        assert_eq!(updated_inline_block.marks, vec![Mark::Bold]);
    }

    #[test]
    fn can_execute_for_selection_across_multiple_inline_blocks() {
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

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let updated_inline_block = updated_state.block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline_block.text().unwrap(), "He orld");
        let updated_paragraph_block = updated_state.block_map.get_standard_block(&paragraph_block_id).unwrap();
        let updated_content_block = updated_paragraph_block.content_block().unwrap();
        assert_eq!(updated_content_block.inline_blocks.len(), 1);
        assert_eq!(updated_content_block.inline_blocks.contains(&inline_block_id1), true);
    }

//     /// Input:
//     /// <1>H|ello world</1>
//     ///     <4/>
//     /// <3>Goo|dbye world</3>
//     ///     <2/>
//     ///        | | |
//     ///        | | |
//     ///        V V V
//     /// Output:
//     /// <1>H dbye world</1>
//     ///    <2/>
    #[test]
    fn can_handle_keypress_execution_across_2_standard_blocks() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let std_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let std_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let std_block_id3 = new_ids.get_id().unwrap();
        let std_block_id4 = new_ids.get_id().unwrap();

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

        let std_block2 = Block::new_std_block_json(std_block_id2.clone(), std_block_id3.clone());
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

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            std_block_id1.clone(), std_block_id3.clone()
            ]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), std_block1.to_string(), std_block2.to_string(),
            root_block.to_string(), std_block3.to_string(), std_block_4.to_string(), inline_block3.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard(' '), None));

        let selection = Selection {
            from: SubSelection {
                block_id: std_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id1.clone(),
                    offset: 1,
                    subselection: None,
                }))
            },
            to: SubSelection {
                block_id: std_block_id3.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id3.clone(),
                    offset: 3,
                    subselection: None,
                }))
            },
        };

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();
        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![std_block_id1.clone()]);
        let updated_std_block1 = updated_state.block_map.get_standard_block(&std_block_id1).unwrap();
        assert_eq!(updated_std_block1.children, vec![std_block_id2]);
        assert_eq!(
            updated_std_block1.content_block().unwrap().inline_blocks,
            vec![inline_block_id1.clone(), inline_block_id3.clone()]
        );

        let updated_inline_block1 = updated_state.block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline_block1.text().unwrap(), "H ");
        let updated_inline_block3 = updated_state.block_map.get_inline_block(&inline_block_id3).unwrap();
        assert_eq!(updated_inline_block3.text().unwrap(), "ld!");


        let expected_subselection = SubSelection {
            block_id: inline_block_id1,
            offset: 2,
            subselection: None,
        };
        assert_eq!(updated_state.selection, Selection { from: expected_subselection.clone(), to: expected_subselection })
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
//         let updated_block_map = execute_steps(steps, block_map, &mut new_ids).unwrap();
//         let updated_standard_block = updated_block_map.get_standard_block(&std_block_id1).unwrap();
//         let content_block = updated_standard_block.content_block().unwrap();
//         assert_eq!(content_block.inline_blocks, vec![inline_block_id1.clone()]);
//         let updated_inline_block1 = updated_block_map.get_inline_block(&inline_block_id1).unwrap();
//         assert_eq!(updated_inline_block1.text().unwrap(), "Hello World!");
//     }
}