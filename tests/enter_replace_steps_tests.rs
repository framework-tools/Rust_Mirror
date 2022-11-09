#[cfg(test)]
mod tests {
    use rust_mirror::{blocks::{BlockMap, Block, standard_blocks::StandardBlockType,
        inline_blocks::{InlineBlockType, text_block::TextBlock}, RootBlock},
        steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps, StepError},
        step::Step, mark::Mark};
    use mongodb::bson::oid::ObjectId;
    use serde_json::json;

    #[test]
    fn can_handle_enter_with_no_text_and_no_selection() {
        let inline_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let root_block_id = ObjectId::new();

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
        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let sub_selection = SubSelection::from(inline_block_id, 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(root_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(root_block_id, 1, None));
                assert_eq!(replace_step.slice.len(), 2);
                assert_eq!(replace_step.slice[0], paragraph_block_id);
                assert_eq!(replace_step.blocks_to_update.len(), 4);
                match &replace_step.blocks_to_update[0] {
                    Block::StandardBlock(standard_block) => {
                        assert_eq!(standard_block._id, paragraph_block_id);
                        match &standard_block.content {
                            StandardBlockType::Paragraph(content_block) => {
                                assert_eq!(content_block.inline_blocks.len(), 1);
                                assert_eq!(content_block.inline_blocks[0], inline_block_id);
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(standard_block.parent, root_block_id);
                        assert_eq!(standard_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[1] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block._id, inline_block_id);
                        match &inline_block.content {
                            InlineBlockType::TextBlock(TextBlock { text }) => {
                                assert_eq!(text, &"".to_string());
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[2] {
                    Block::StandardBlock(standard_block) => {
                        match &standard_block.content {
                            StandardBlockType::Paragraph(content_block) => {
                                assert_eq!(content_block.inline_blocks.len(), 1);
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(standard_block.parent, root_block_id);
                        assert_eq!(standard_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[3] {
                    Block::InlineBlock(inline_block) => {
                        match &inline_block.content {
                            InlineBlockType::TextBlock(TextBlock { text }) => {
                                assert_eq!(text, &"".to_string());
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
                        assert_eq!(inline_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

    #[test]
    fn can_handle_in_middle_of_text_with_no_selection() {
        let inline_block_id1 = ObjectId::new();
        let inline_block_id2 = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let root_block_id = ObjectId::new();

        let inline_block1 = json!({
            "_id": inline_block_id1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.to_string(), inline_block_id2.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![inline_block1, inline_block2, block, root_block]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let sub_selection = SubSelection::from(inline_block_id1, 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(root_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(root_block_id, 1, None));
                assert_eq!(replace_step.slice.len(), 2);
                assert_eq!(replace_step.slice[0], paragraph_block_id);
                assert_eq!(replace_step.blocks_to_update.len(), 4);
                match &replace_step.blocks_to_update[0] {
                    Block::StandardBlock(standard_block) => {
                        assert_eq!(standard_block._id, paragraph_block_id);
                        match &standard_block.content {
                            StandardBlockType::Paragraph(content_block) => {
                                assert_eq!(content_block.inline_blocks.len(), 1);
                                assert_eq!(content_block.inline_blocks[0], inline_block_id1);
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(standard_block.parent, root_block_id);
                        assert_eq!(standard_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[1] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block._id, inline_block_id1);
                        match &inline_block.content {
                            InlineBlockType::TextBlock(TextBlock { text }) => {
                                assert_eq!(text, &"Hell".to_string());
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![Mark::Bold]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[2] {
                    Block::StandardBlock(standard_block) => {
                        match &standard_block.content {
                            StandardBlockType::Paragraph(content_block) => {
                                assert_eq!(content_block.inline_blocks.len(), 2);
                                assert_eq!(content_block.inline_blocks[0], replace_step.blocks_to_update[3].id());
                                assert_eq!(content_block.inline_blocks[1], inline_block_id2);
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(standard_block.parent, root_block_id);
                        assert_eq!(standard_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[3] {
                    Block::InlineBlock(inline_block) => {
                        match &inline_block.content {
                            InlineBlockType::TextBlock(TextBlock { text }) => {
                                assert_eq!(text, &"o ".to_string());
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
                        assert_eq!(inline_block.marks, vec![Mark::Bold]);
                    },
                    _ => panic!("Expected inline block"),
                }
            },
            _ => panic!("Expected ReplaceStep")
        };
    }
    #[test]
    fn can_handle_in_middle_of_inline_block_with_some_selection() {
        let inline_block_id1 = ObjectId::new();
        let inline_block_id2 = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let root_block_id = ObjectId::new();

        let inline_block1 = json!({
            "_id": inline_block_id1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.to_string(), inline_block_id2.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![inline_block1, inline_block2, block, root_block]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let sub_selection = SubSelection::from(inline_block_id1, 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(root_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(root_block_id, 1, None));
                assert_eq!(replace_step.slice.len(), 2);
                assert_eq!(replace_step.slice[0], paragraph_block_id);
                assert_eq!(replace_step.blocks_to_update.len(), 4);
                match &replace_step.blocks_to_update[0] {
                    Block::StandardBlock(standard_block) => {
                        assert_eq!(standard_block._id, paragraph_block_id);
                        match &standard_block.content {
                            StandardBlockType::Paragraph(content_block) => {
                                assert_eq!(content_block.inline_blocks.len(), 1);
                                assert_eq!(content_block.inline_blocks[0], inline_block_id1);
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(standard_block.parent, root_block_id);
                        assert_eq!(standard_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[1] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block._id, inline_block_id1);
                        match &inline_block.content {
                            InlineBlockType::TextBlock(TextBlock { text }) => {
                                assert_eq!(text, &"Hell".to_string());
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![Mark::Bold]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[2] {
                    Block::StandardBlock(standard_block) => {
                        match &standard_block.content {
                            StandardBlockType::Paragraph(content_block) => {
                                assert_eq!(content_block.inline_blocks.len(), 2);
                                assert_eq!(content_block.inline_blocks[0], replace_step.blocks_to_update[3].id());
                                assert_eq!(content_block.inline_blocks[1], inline_block_id2);
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(standard_block.parent, root_block_id);
                        assert_eq!(standard_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[3] {
                    Block::InlineBlock(inline_block) => {
                        match &inline_block.content {
                            InlineBlockType::TextBlock(TextBlock { text }) => {
                                assert_eq!(text, &"o ".to_string());
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
                        assert_eq!(inline_block.marks, vec![Mark::Bold]);
                    },
                    _ => panic!("Expected inline block"),
                }
            },
            _ => panic!("Expected ReplaceStep")
        };
    }

    #[test]
    fn can_handle_with_selection_across_inline_blocks() {
        let inline_block_id1 = ObjectId::new();
        let inline_block_id2 = ObjectId::new();
        let inline_block_id3 = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let root_block_id = ObjectId::new();

        let inline_block1 = json!({
            "_id": inline_block_id1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello"
            },
            "marks": ["italic"],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "brave new "
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.to_string(), inline_block_id2.to_string(), inline_block_id3.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![inline_block1, inline_block2, inline_block3, block, root_block]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let anchor_sub_selection = SubSelection::from(inline_block_id1, 4, None);
        let head_sub_selection = SubSelection::from(inline_block_id3, 1, None);
        let selection = Selection::from(anchor_sub_selection, head_sub_selection);

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(root_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(root_block_id, 1, None));
                assert_eq!(replace_step.slice.len(), 2);
                assert_eq!(replace_step.slice[0], paragraph_block_id);
                assert_eq!(replace_step.blocks_to_update.len(), 4);
                match &replace_step.blocks_to_update[0] {
                    Block::StandardBlock(standard_block) => {
                        assert_eq!(standard_block._id, paragraph_block_id);
                        match &standard_block.content {
                            StandardBlockType::Paragraph(content_block) => {
                                assert_eq!(content_block.inline_blocks.len(), 1);
                                assert_eq!(content_block.inline_blocks[0], inline_block_id1);
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(standard_block.parent, root_block_id);
                        assert_eq!(standard_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[1] {
                    Block::InlineBlock(inline_block) => {
                        assert_eq!(inline_block._id, inline_block_id1);
                        match &inline_block.content {
                            InlineBlockType::TextBlock(TextBlock { text }) => {
                                assert_eq!(text, &"Hell".to_string());
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(inline_block.parent, paragraph_block_id);
                        assert_eq!(inline_block.marks, vec![Mark::Bold]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[2] {
                    Block::StandardBlock(standard_block) => {
                        match &standard_block.content {
                            StandardBlockType::Paragraph(content_block) => {
                                assert_eq!(content_block.inline_blocks.len(), 1);
                                assert_eq!(content_block.inline_blocks[0], replace_step.blocks_to_update[3].id());
                                assert_eq!(content_block.inline_blocks[1], inline_block_id3);
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(standard_block.parent, root_block_id);
                        assert_eq!(standard_block.marks, vec![]);
                    },
                    _ => panic!("Expected inline block"),
                };
                match &replace_step.blocks_to_update[3] {
                    Block::InlineBlock(inline_block) => {
                        match &inline_block.content {
                            InlineBlockType::TextBlock(TextBlock { text }) => {
                                assert_eq!(text, &"orld!".to_string());
                            },
                            _ => panic!("Expected paragraph block"),
                        };
                        assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
                        assert_eq!(inline_block.marks, vec![Mark::Bold]);
                    },
                    _ => panic!("Expected inline block"),
                }
            },
            _ => panic!("Expected ReplaceStep")
        };
    }

    #[test]
    fn can_handle_enter_over_standard_blocks() -> Result<(), StepError> {
        let inline_block_id1 = ObjectId::new();
        let inline_block_id2 = ObjectId::new();
        let inline_block_id3 = ObjectId::new();
        let paragraph_block_id1 = ObjectId::new();
        let paragraph_block_id2 = ObjectId::new();
        let paragraph_block_id3 = ObjectId::new();
        let root_block_id = ObjectId::new();

        let inline_block1 = json!({
            "_id": inline_block_id1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": [],
            "parent": paragraph_block_id1.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello again!"
            },
            "marks": [],
            "parent": paragraph_block_id2.to_string()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye World"
            },
            "marks": [],
            "parent": paragraph_block_id2.to_string()
        });
        let paragraph_block1 = json!({
            "_id": paragraph_block_id1.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let paragraph_block2 = json!({
            "_id": paragraph_block_id2.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id1, paragraph_block_id2, paragraph_block_id3]);

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, inline_block3,
            paragraph_block1, paragraph_block2, paragraph_block3, root_block
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let anchor_sub_selection = SubSelection::from(paragraph_block_id1, 0, Some(Box::new(
            SubSelection::from(inline_block_id1, 1, None)
        )));
        let head_sub_selection = SubSelection::from(paragraph_block_id3, 1, Some(Box::new(
            SubSelection::from(inline_block_id3, 3, None)
        )));
        let selection = Selection::from(anchor_sub_selection, head_sub_selection);

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.from, SubSelection::from(root_block_id, 0, None));
                assert_eq!(replace_step.to, SubSelection::from(root_block_id, 3, None));
                assert_eq!(replace_step.slice.len(), 3);
                assert_eq!(replace_step.slice[0], paragraph_block_id1);
                assert_eq!(replace_step.slice[2], paragraph_block_id3);
                assert_eq!(replace_step.blocks_to_update.len(), 4);
                // need to finish
                // match &replace_step.blocks_to_update[0] {
                //     Block::StandardBlock(standard_block) => {
                //         assert_eq!(standard_block._id, paragraph_block_id);
                //         match &standard_block.content {
                //             StandardBlockType::Paragraph(content_block) => {
                //                 assert_eq!(content_block.inline_blocks.len(), 1);
                //                 assert_eq!(content_block.inline_blocks[0], inline_block_id1);
                //             },
                //             _ => panic!("Expected paragraph block"),
                //         };
                //         assert_eq!(standard_block.parent, root_block_id);
                //         assert_eq!(standard_block.marks, vec![]);
                //     },
                //     _ => panic!("Expected inline block"),
                // };
                // match &replace_step.blocks_to_update[1] {
                //     Block::InlineBlock(inline_block) => {
                //         assert_eq!(inline_block._id, inline_block_id1);
                //         match &inline_block.content {
                //             InlineBlockType::TextBlock(TextBlock { text }) => {
                //                 assert_eq!(text, &"Hell".to_string());
                //             },
                //             _ => panic!("Expected paragraph block"),
                //         };
                //         assert_eq!(inline_block.parent, paragraph_block_id);
                //         assert_eq!(inline_block.marks, vec![Mark::Bold]);
                //     },
                //     _ => panic!("Expected inline block"),
                // };
                // match &replace_step.blocks_to_update[2] {
                //     Block::StandardBlock(standard_block) => {
                //         match &standard_block.content {
                //             StandardBlockType::Paragraph(content_block) => {
                //                 assert_eq!(content_block.inline_blocks.len(), 1);
                //                 assert_eq!(content_block.inline_blocks[0], replace_step.blocks_to_update[3].id());
                //                 assert_eq!(content_block.inline_blocks[1], inline_block_id3);
                //             },
                //             _ => panic!("Expected paragraph block"),
                //         };
                //         assert_eq!(standard_block.parent, root_block_id);
                //         assert_eq!(standard_block.marks, vec![]);
                //     },
                //     _ => panic!("Expected inline block"),
                // };
                // match &replace_step.blocks_to_update[3] {
                //     Block::InlineBlock(inline_block) => {
                //         match &inline_block.content {
                //             InlineBlockType::TextBlock(TextBlock { text }) => {
                //                 assert_eq!(text, &"orld!".to_string());
                //             },
                //             _ => panic!("Expected paragraph block"),
                //         };
                //         assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
                //         assert_eq!(inline_block.marks, vec![Mark::Bold]);
                //     },
                //     _ => panic!("Expected inline block"),
                // }
            },
            _ => panic!("Expected ReplaceStep")
        };
        Ok(())
    }
}