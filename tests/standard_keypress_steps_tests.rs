#[cfg(test)]
mod tests {
    use rust_mirror::{steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps},
        blocks::{Block, standard_blocks::{StandardBlockType, content_block::ContentBlock}, inline_blocks::{text_block::TextBlock, InlineBlockType}, RootBlock,
        BlockMap
    }, step::{Step, ReplaceSlice}, mark::Mark, new_ids::NewIds};

    use serde_json::json;

    #[test]
    fn can_generate_steps_for_standard_keypress() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let inline_block_id = new_ids.get_id().unwrap();
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
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![inline_block.to_string(), block.to_string(), root_block.to_string()]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Standard('a'), None));
        let sub_selection = SubSelection::from(inline_block_id.clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.block_id, paragraph_block_id);
                assert_eq!(replace_step.from, SubSelection::from(inline_block_id.clone(), 0, None));
                assert_eq!(replace_step.to, SubSelection::from(inline_block_id.clone(), 0, None));
                assert_eq!(replace_step.slice, ReplaceSlice::String("a".to_string()));
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    #[test]
    fn can_generate_steps_for_standard_keypress_with_different_mark() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let inline_block_id = new_ids.get_id().unwrap();
        let inline_block = json!({
            "_id": inline_block_id.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "dsfkjhl"
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

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('9'), None));
        let sub_selection = SubSelection::from(inline_block_id.clone(), 2, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();

        assert_eq!(steps.len(), 1);

        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.block_id, paragraph_block_id);
                assert_eq!(replace_step.from, SubSelection::from(inline_block_id.clone(), 2, None));
                assert_eq!(replace_step.to, SubSelection::from(inline_block_id.clone(), 2, None));
                assert_eq!(replace_step.slice, ReplaceSlice::String("9".to_string()));
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    #[test]
    fn can_generate_steps_for_standard_keypress_with_selection_across_single_block() {
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

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('k'), None));
        let from_sub_selection = SubSelection::from(inline_block_id.clone(), 2, None);
        let to_sub_selection = SubSelection::from(inline_block_id.clone(), 4, None);
        let selection = Selection::from(from_sub_selection.clone(), to_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();

        assert_eq!(steps.len(), 1);

        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.block_id, paragraph_block_id);
                assert_eq!(replace_step.from, SubSelection::from(inline_block_id.clone(), 2, None));
                assert_eq!(replace_step.to, SubSelection::from(inline_block_id.clone(), 4, None));
                assert_eq!(replace_step.slice, ReplaceSlice::String("k".to_string()));
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    ///<p> <TB>Hello </TB><TB>World</TB> </p>
    #[test]
    fn can_handle_across_2_inline_blocks() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let inline_block_id_1 = new_ids.get_id().unwrap();
        let inline_block_id_2 = new_ids.get_id().unwrap();
        let inline_block1 = json!({
            "_id": inline_block_id_1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id_2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.clone()
        });
        let block = json!( {
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id_1.clone(), inline_block_id_2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('a'), None));
        let from = SubSelection::from(inline_block_id_1.clone(), 2, None);
        let to = SubSelection::from(inline_block_id_2.clone(), 2, None);
        let selection = Selection::from(from, to);

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();

        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.block_id, paragraph_block_id);
                assert_eq!(replace_step.from, SubSelection::from(inline_block_id_1.clone(), 2, None));
                assert_eq!(replace_step.to, SubSelection::from(inline_block_id_2.clone(), 2, None));
                assert_eq!(replace_step.slice, ReplaceSlice::String("a".to_string()));
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    // // // <p>|Hello |brave new|| world!!!|</p>
    // #[test]
    // fn can_handle_across_3_inline_blocks() {
    //     let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

    //     let root_block_id = new_ids.get_id().unwrap();
    //     let paragraph_block_id = new_ids.get_id().unwrap();
    //     let inline_block_id_1 = new_ids.get_id().unwrap();
    //     let inline_block_id_2 = new_ids.get_id().unwrap();
    //     let inline_block_id_3 = new_ids.get_id().unwrap();
    //     let inline_block1 = json!({
    //         "_id": inline_block_id_1.clone(),
    //         "kind": "inline",
    //         "_type": "text",
    //         "content": {
    //             "text": "Hello "
    //         },
    //         "marks": [],
    //         "parent": paragraph_block_id.clone()
    //     });
    //     let inline_block2 = json!({
    //         "_id": inline_block_id_2.clone(),
    //         "kind": "inline",
    //         "_type": "text",
    //         "content": {
    //             "text": "brave new"
    //         },
    //         "marks": ["bold"],
    //         "parent": paragraph_block_id.clone()
    //     });
    //     let inline_block3 = json!({
    //         "_id": inline_block_id_3.clone(),
    //         "kind": "inline",
    //         "_type": "text",
    //         "content": {
    //             "text": " world!!!"
    //         },
    //         "marks": [],
    //         "parent": paragraph_block_id.clone()
    //     });
    //     let block = json!({
    //         "_id": paragraph_block_id.clone(),
    //         "kind": "standard",
    //         "_type": "paragraph",
    //         "content": {
    //             "inline_blocks": [inline_block_id_1.clone(), inline_block_id_2.clone(), inline_block_id_3.clone()]
    //         },
    //         "children": [],
    //         "marks": [],
    //         "parent": root_block_id.clone()
    //     });
    //     let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

    //     let block_map = BlockMap::from(vec![
    //         inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), block.to_string(), root_block.to_string()
    //     ]).unwrap();

    //     let event = Event::KeyPress(KeyPress::new(Key::Standard(' '), None));
    //     let selection = Selection {
    //         from: SubSelection {
    //             block_id: inline_block_id_1.clone(),
    //             offset: 4,
    //             subselection: None,
    //         },
    //         to: SubSelection {
    //             block_id: inline_block_id_3.clone(),
    //             offset: 2,
    //             subselection: None,
    //         },
    //     };

    //     let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();

    //     assert_eq!(steps.len(), 1);

    //     match &steps[0] {
    //         Step::ReplaceStep(replace_step) => {
    //             assert_eq!(replace_step.from, SubSelection::from(paragraph_block_id.clone(), 0, None));
    //             assert_eq!(replace_step.to, SubSelection::from(paragraph_block_id.clone(), 3, None));

    //             assert_eq!(replace_step.slice.len(), 2);
    //             assert_eq!(replace_step.slice[0], inline_block_id_1);
    //             assert_eq!(replace_step.slice[1], inline_block_id_3);
    //             assert_eq!(replace_step.blocks_to_update.len(), 2);
    //             match &replace_step.blocks_to_update[0] {
    //                 Block::InlineBlock(inline_block) => {
    //                     assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "Hell ".to_string() }));
    //                     assert_eq!(inline_block.parent, paragraph_block_id);
    //                     assert_eq!(inline_block.marks, vec![]);
    //                     assert_eq!(inline_block.id(), inline_block_id_1);
    //                 },
    //                 _ => panic!("Expected Some Inline Block"),
    //             };
    //             match &replace_step.blocks_to_update[1] {
    //                 Block::InlineBlock(inline_block) => {
    //                     assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "orld!!!".to_string() }));
    //                     assert_eq!(inline_block.parent, paragraph_block_id);
    //                     assert_eq!(inline_block.marks, vec![]);
    //                     assert_eq!(inline_block.id(), inline_block_id_3);
    //                 },
    //                 _ => panic!("Expected Some Inline Block"),
    //             };
    //         },
    //         _ => panic!("Expected ReplaceStep")
    //     };
    // }

    // /// Input:
    // /// <1>H|ello world</1>
    // ///     <4/>
    // /// <3>Goo|dbye world</3>
    // ///     <2/>
    // ///        | | |
    // ///        | | |
    // ///        V V V
    // /// Output:
    // /// <1>H dbye world</1>
    // ///    <2/>
    #[test]
    fn can_handle_across_2_standard_blocks() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let std_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let std_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
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
            "children": [std_block_id4.to_string()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let std_block_4 = Block::new_std_block_json(std_block_id4.clone(), std_block_id1.clone());

        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye world!"
            },
            "marks": [],
            "parent": std_block_id3.clone()
        });

        let std_block2 = Block::new_std_block_json(std_block_id2.clone(), std_block_id3.clone());
        let std_block3 = json!({
            "_id": std_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [std_block_id2.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(),
        vec![std_block_id1.clone(), std_block_id3.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), std_block1.to_string(), std_block2.to_string(),
            root_block.to_string(), std_block3.to_string(), std_block_4.to_string()
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
                block_id: std_block_id3,
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id2.clone(),
                    offset: 3,
                    subselection: None,
                }))
            },
        };

        let steps = generate_steps(&event, &block_map, selection.clone(), &mut new_ids).unwrap();

        assert_eq!(steps.len(), 1);

        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.block_id, root_block_id);
                assert_eq!(replace_step.from, selection.from);
                assert_eq!(replace_step.to, selection.to);
                assert_eq!(replace_step.slice, ReplaceSlice::String(" ".to_string()));
            },
            _ => panic!("Expected ReplaceStep")
        }
    }
}