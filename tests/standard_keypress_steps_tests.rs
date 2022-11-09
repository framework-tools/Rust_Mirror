#[cfg(test)]
mod tests {

    use rust_mirror::{steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps},
        blocks::{Block, standard_blocks::{StandardBlockType, content_block::ContentBlock}, inline_blocks::{text_block::TextBlock, InlineBlockType}, RootBlock,
        BlockMap
    }, step::{Step}, mark::Mark};
    use mongodb::bson::oid::ObjectId;
    use serde_json::json;

    #[test]
    fn can_generate_steps_for_standard_keypress() {
        let root_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let inline_block_id = ObjectId::new();
        let inline_block = json!({
            "_id": inline_block_id.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![inline_block, block, root_block]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Standard('a'), None));
        let sub_selection = SubSelection::from(inline_block_id, 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(paragraph_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(paragraph_block_id, 1, None));
                assert_eq!(replace_step.slice.len(), 1);
                let first_block_id = &replace_step.slice[0];
                assert_eq!(replace_step.blocks_to_update.len(), 1);
                match &replace_step.blocks_to_update[0] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(&inline_block._id, first_block_id);
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "a".to_string() } ));
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                }
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    #[test]
    fn can_generate_steps_for_standard_keypress_with_different_mark() {
        let root_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let inline_block_id = ObjectId::new();
        let inline_block = json!({
            "_id": inline_block_id.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "dsfkjhl"
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.to_string()
        });

        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![
            inline_block, block, root_block
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('9'), None));
        let sub_selection = SubSelection::from(inline_block_id, 2, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);

        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(paragraph_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(paragraph_block_id, 1, None));
                assert_eq!(replace_step.slice.len(), 1);
                let first_block_id = &replace_step.slice[0];
                assert_eq!(replace_step.blocks_to_update.len(), 1);
                match &replace_step.blocks_to_update[0] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(&inline_block._id, first_block_id);
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "ds9fkjhl".to_string() } ));
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![Mark::Bold]);
                    },
                    _ => panic!("Expected inline block"),
                }
            },
            _ => panic!("Expected ReplaceStep")
        }
    }
    #[test]
    fn can_generate_steps_for_standard_keypress_with_selection_across_single_block() {
        let root_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let inline_block_id = ObjectId::new();
        let inline_block = json!({
            "_id": inline_block_id.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "some text"
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });

        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![
            inline_block, block, root_block
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('k'), None));
        let anchor_sub_selection = SubSelection::from(inline_block_id, 2, None);
        let head_sub_selection = SubSelection::from(inline_block_id, 4, None);
        let selection = Selection::from(anchor_sub_selection.clone(), head_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);

        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(paragraph_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(paragraph_block_id, 1, None));
                assert_eq!(replace_step.slice.len(), 1);
                let first_block_id = &replace_step.slice[0];
                assert_eq!(replace_step.blocks_to_update.len(), 1);
                match &replace_step.blocks_to_update[0] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(&inline_block._id, first_block_id);
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "sok text".to_string() } ));
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                }
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    // //<p> <TB>Hello </TB><TB>World</TB> </p>
    #[test]
    fn can_handle_across_2_inline_blocks() {
        let root_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let inline_block_id_1 = ObjectId::new();
        let inline_block_id_2 = ObjectId::new();
        let inline_block1 = json!({
            "_id": inline_block_id_1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id_2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!( {
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id_1.to_string(), inline_block_id_2.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, block, root_block
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('a'), None));
        let anchor = SubSelection::from(inline_block_id_1, 2, None);
        let head = SubSelection::from(inline_block_id_2, 2, None);
        let selection = Selection::from(anchor, head);

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(paragraph_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(paragraph_block_id, 2, None));
                assert_eq!(replace_step.slice.len(), 2);
                assert_eq!(replace_step.slice[0], inline_block_id_1);
                assert_eq!(replace_step.slice[1], inline_block_id_2);
                assert_eq!(replace_step.blocks_to_update.len(), 2);
                match &replace_step.blocks_to_update[0] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "Hea".to_string() }));
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![]);
                        assert_eq!(inline_block._id, inline_block_id_1);
                    },
                    _ => panic!("Expected Some Inline Block"),
                };
                match &replace_step.blocks_to_update[1] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "rld!".to_string() }));
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![Mark::Bold]);
                        assert_eq!(inline_block._id, inline_block_id_2);
                    },
                    _ => panic!("Expected Some Inline Block"),
                };
            },
            _ => panic!("Expected ReplaceStep")
        };
    }

    // // <p>|Hello |brave new|| world!!!|</p>
    #[test]
    fn can_handle_across_3_inline_blocks() {
        let root_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let inline_block_id_1 = ObjectId::new();
        let inline_block_id_2 = ObjectId::new();
        let inline_block_id_3 = ObjectId::new();
        let inline_block1 = json!({
            "_id": inline_block_id_1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id_2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "brave new"
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block3 = json!({
            "_id": inline_block_id_3.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " world!!!"
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id_1.to_string(), inline_block_id_2.to_string(), inline_block_id_3.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, inline_block3, block, root_block
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard(' '), None));
        let selection = Selection {
            anchor: SubSelection {
                block_id: inline_block_id_1,
                offset: 4,
                subselection: None,
            },
            head: SubSelection {
                block_id: inline_block_id_3,
                offset: 2,
                subselection: None,
            },
        };

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);

        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(paragraph_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(paragraph_block_id, 3, None));

                assert_eq!(replace_step.slice.len(), 2);
                assert_eq!(replace_step.slice[0], inline_block_id_1);
                assert_eq!(replace_step.slice[1], inline_block_id_3);
                assert_eq!(replace_step.blocks_to_update.len(), 2);
                match &replace_step.blocks_to_update[0] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "Hell ".to_string() }));
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![]);
                        assert_eq!(inline_block._id, inline_block_id_1);
                    },
                    _ => panic!("Expected Some Inline Block"),
                };
                match &replace_step.blocks_to_update[1] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "orld!!!".to_string() }));
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![]);
                        assert_eq!(inline_block._id, inline_block_id_3);
                    },
                    _ => panic!("Expected Some Inline Block"),
                };
            },
            _ => panic!("Expected ReplaceStep")
        };
    }

    /// Input:
    /// <1>H|ello world</1>
    ///     <4/>
    /// <3>Goo|dbye world</3>
    ///     <2/>
    ///        | | |
    ///        | | |
    ///        V V V
    /// Output:
    /// <1>Hadbye world</1>
    ///    <2/>
    #[test]
    fn can_handle_across_2_standard_blocks() {
        let root_block_id = ObjectId::new();
        let std_block_id1 = ObjectId::new();
        let inline_block_id1 = ObjectId::new();
        let std_block_id2 = ObjectId::new();
        let inline_block_id2 = ObjectId::new();
        let std_block_id3 = ObjectId::new();
        let std_block_id4 = ObjectId::new();

        let inline_block1 = json!({
            "_id": inline_block_id1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello world!"
            },
            "marks": [],
            "parent": std_block_id1.to_string()
        });

        let std_block1 = json!({
            "_id": std_block_id1.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.to_string()]
            },
            "children": [std_block_id4.to_string()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let std_block_4 = Block::new_std_block_json(std_block_id4, std_block_id1);

        let inline_block2 = json!({
            "_id": inline_block_id2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye world!"
            },
            "marks": [],
            "parent": std_block_id3.to_string()
        });

        let std_block2 = Block::new_std_block_json(std_block_id2, std_block_id3);
        let std_block3 = json!({
            "_id": std_block_id3.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.to_string()]
            },
            "children": [std_block_id2.to_string()],
            "marks": [],
            "parent": root_block_id.to_string()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![std_block_id1, std_block_id3]);

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, std_block1, std_block2, root_block, std_block3, std_block_4
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('a'), None));

        let selection = Selection {
            anchor: SubSelection {
                block_id: std_block_id1,
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id1,
                    offset: 1,
                    subselection: None,
                }))
            },
            head: SubSelection {
                block_id: std_block_id3,
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id2.clone(),
                    offset: 3,
                    subselection: None,
                }))
            },
        };

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);

        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.block_id, root_block_id);
                assert_eq!(replace_step.from, SubSelection::from(root_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(root_block_id, 2, None));

                assert_eq!(replace_step.slice.len(), 1);
                assert_eq!(replace_step.slice[0], std_block_id1);

                assert_eq!(replace_step.blocks_to_update.len(), 3);
                match &replace_step.blocks_to_update[0] {
                    Block::StandardBlock(standard_block) => {
                        assert_eq!(standard_block.content, StandardBlockType::Paragraph(ContentBlock {
                            inline_blocks: vec![inline_block_id1, inline_block_id2]
                        }));
                        assert_eq!(standard_block.children, vec![std_block_id2]);
                        assert_eq!(standard_block.marks, vec![]);
                        assert_eq!(standard_block._id, std_block_id1);
                    },
                    _ => panic!("Expected Some Standard Block"),
                };
                match &replace_step.blocks_to_update[1] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "Ha".to_string() }));
                        assert_eq!(inline_block.parent, std_block_id1);
                        assert_eq!(inline_block.marks, vec![]);
                        assert_eq!(inline_block._id, inline_block_id1);
                    },
                    _ => panic!("Expected Some Inline Block"),
                };
                match &replace_step.blocks_to_update[2] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block.content, InlineBlockType::TextBlock(TextBlock{ text: "dbye world!".to_string() }));
                        assert_eq!(inline_block.parent, std_block_id1);
                        assert_eq!(inline_block.marks, vec![]);
                        assert_eq!(inline_block._id, inline_block_id2);
                    },
                    _ => panic!("Expected Some Inline Block"),
                };
            },
            _ => panic!("Expected ReplaceStep")
        };
    }
}