#[cfg(test)]
mod tests {
    use rust_mirror::{blocks::{BlockMap, Block, standard_blocks::StandardBlockType,
        inline_blocks::{InlineBlockType, text_block::TextBlock}, RootBlock},
        steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps, StepError},
        step::{Step, ReplaceStep, ReplaceSlice, SplitStep}, mark::Mark, new_ids::NewIds};

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
                assert_eq!(split_step.block_id, paragraph_block_id.clone());
                assert_eq!(split_step.subselection, sub_selection.clone());
            },
            _ => panic!("Expected ReplaceStep")
        }
    }

//     #[test]
//     fn can_handle_in_middle_of_text_with_no_selection() {
//         let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

//         let inline_block_id1= new_ids.get_id().unwrap();
//         let inline_block_id2 = new_ids.get_id().unwrap();
//         let paragraph_block_id = new_ids.get_id().unwrap();
//         let root_block_id = new_ids.get_id().unwrap();

//         let inline_block1 = json!({
//             "_id": inline_block_id1.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "Hello "
//             },
//             "marks": ["bold"],
//             "parent": paragraph_block_id.clone()
//         }).to_string();
//         let inline_block2 = json!({
//             "_id": inline_block_id2.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "World!"
//             },
//             "marks": [],
//             "parent": paragraph_block_id.clone()
//         }).to_string();
//         let block = json!({
//             "_id": paragraph_block_id.clone(),
//             "kind": "standard",
//             "_type": "paragraph",
//             "content": {
//                 "inline_blocks": [inline_block_id1.clone(), inline_block_id2.clone()]
//             },
//             "children": [],
//             "marks": [],
//             "parent": root_block_id.clone().to_string()
//         }).to_string();
//         let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id.clone()]).to_string();

//         let block_map = BlockMap::from(vec![inline_block1, inline_block2, block, root_block]).unwrap();
//         let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
//         let sub_selection = SubSelection::from(inline_block_id1.clone(), 4, None);
//         let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

//         let steps = generate_steps(&event, &block_map, selection).unwrap();

//         assert_eq!(steps.len(), 1);
//         match &steps[0] {
//             Step::ReplaceStep(replace_step) => {
//                 assert_eq!(replace_step.from, SubSelection::from(root_block_id.clone(), 0, None));
//                 assert_eq!(replace_step.to, SubSelection::from(root_block_id.clone(), 1, None));
//                 assert_eq!(replace_step.slice.len(), 2);
//                 assert_eq!(replace_step.slice[0], paragraph_block_id.clone());
//                 assert_eq!(replace_step.blocks_to_update.len(), 4);
//                 match &replace_step.blocks_to_update[0] {
//                     Block::StandardBlock(standard_block) => {
//                         assert_eq!(standard_block.id(), paragraph_block_id.clone());
//                         match &standard_block.content {
//                             StandardBlockType::Paragraph(content_block) => {
//                                 assert_eq!(content_block.inline_blocks.len(), 1);
//                                 assert_eq!(content_block.inline_blocks[0], inline_block_id1.clone());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(standard_block.parent, root_block_id.clone());
//                         assert_eq!(standard_block.marks, vec![]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[1] {
//                     Block::InlineBlock(inline_block) => {
//                         assert_eq!(inline_block.id(), inline_block_id1.clone());
//                         match &inline_block.content {
//                             InlineBlockType::TextBlock(TextBlock { text }) => {
//                                 assert_eq!(text, &"Hell".to_string());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(inline_block.parent, paragraph_block_id.clone());
//                         assert_eq!(inline_block.marks, vec![Mark::Bold]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[2] {
//                     Block::StandardBlock(standard_block) => {
//                         match &standard_block.content {
//                             StandardBlockType::Paragraph(content_block) => {
//                                 assert_eq!(content_block.inline_blocks.len(), 2);
//                                 assert_eq!(content_block.inline_blocks[0], replace_step.blocks_to_update[3].id());
//                                 assert_eq!(content_block.inline_blocks[1], inline_block_id2.clone());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(standard_block.parent, root_block_id.clone());
//                         assert_eq!(standard_block.marks, vec![]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[3] {
//                     Block::InlineBlock(inline_block) => {
//                         match &inline_block.content {
//                             InlineBlockType::TextBlock(TextBlock { text }) => {
//                                 assert_eq!(text, &"o ".to_string());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
//                         assert_eq!(inline_block.marks, vec![Mark::Bold]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 }
//             },
//             _ => panic!("Expected ReplaceStep")
//         };
//     }
//     #[test]
//     fn can_handle_in_middle_of_inline_block_with_some_selection() {
//         let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

//         let inline_block_id1 = new_ids.get_id().unwrap();
//         let inline_block_id2 = new_ids.get_id().unwrap();
//         let paragraph_block_id = new_ids.get_id().unwrap();
//         let root_block_id = new_ids.get_id().unwrap();

//         let inline_block1 = json!({
//             "_id": inline_block_id1.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "Hello "
//             },
//             "marks": ["bold"],
//             "parent": paragraph_block_id.clone()
//         }).to_string();
//         let inline_block2 = json!({
//             "_id": inline_block_id2.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "World!"
//             },
//             "marks": [],
//             "parent": paragraph_block_id.clone()
//         }).to_string();
//         let block = json!({
//             "_id": paragraph_block_id.clone(),
//             "kind": "standard",
//             "_type": "paragraph",
//             "content": {
//                 "inline_blocks": [inline_block_id1.clone(), inline_block_id2.clone()]
//             },
//             "children": [],
//             "marks": [],
//             "parent": root_block_id.clone().to_string()
//         }).to_string();
//         let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id.clone()]).to_string();

//         let block_map = BlockMap::from(vec![inline_block1, inline_block2, block, root_block]).unwrap();
//         let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
//         let sub_selection = SubSelection::from(inline_block_id1.clone(), 4, None);
//         let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

//         let steps = generate_steps(&event, &block_map, selection).unwrap();

//         assert_eq!(steps.len(), 1);
//         match &steps[0] {
//             Step::ReplaceStep(replace_step) => {
//                 assert_eq!(replace_step.from, SubSelection::from(root_block_id.clone(), 0, None));
//                 assert_eq!(replace_step.to, SubSelection::from(root_block_id.clone(), 1, None));
//                 assert_eq!(replace_step.slice.len(), 2);
//                 assert_eq!(replace_step.slice[0], paragraph_block_id.clone());
//                 assert_eq!(replace_step.blocks_to_update.len(), 4);
//                 match &replace_step.blocks_to_update[0] {
//                     Block::StandardBlock(standard_block) => {
//                         assert_eq!(standard_block.id(), paragraph_block_id.clone());
//                         match &standard_block.content {
//                             StandardBlockType::Paragraph(content_block) => {
//                                 assert_eq!(content_block.inline_blocks.len(), 1);
//                                 assert_eq!(content_block.inline_blocks[0], inline_block_id1.clone());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(standard_block.parent, root_block_id.clone());
//                         assert_eq!(standard_block.marks, vec![]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[1] {
//                     Block::InlineBlock(inline_block) => {
//                         assert_eq!(inline_block.id(), inline_block_id1.clone());
//                         match &inline_block.content {
//                             InlineBlockType::TextBlock(TextBlock { text }) => {
//                                 assert_eq!(text, &"Hell".to_string());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(inline_block.parent, paragraph_block_id.clone());
//                         assert_eq!(inline_block.marks, vec![Mark::Bold]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[2] {
//                     Block::StandardBlock(standard_block) => {
//                         match &standard_block.content {
//                             StandardBlockType::Paragraph(content_block) => {
//                                 assert_eq!(content_block.inline_blocks.len(), 2);
//                                 assert_eq!(content_block.inline_blocks[0], replace_step.blocks_to_update[3].id());
//                                 assert_eq!(content_block.inline_blocks[1], inline_block_id2.clone());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(standard_block.parent, root_block_id.clone());
//                         assert_eq!(standard_block.marks, vec![]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[3] {
//                     Block::InlineBlock(inline_block) => {
//                         match &inline_block.content {
//                             InlineBlockType::TextBlock(TextBlock { text }) => {
//                                 assert_eq!(text, &"o ".to_string());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
//                         assert_eq!(inline_block.marks, vec![Mark::Bold]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 }
//             },
//             _ => panic!("Expected ReplaceStep")
//         };
//     }

//     #[test]
//     fn can_handle_with_selection_across_inline_blocks() {
//         let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

//         let inline_block_id1 = new_ids.get_id().unwrap();
//         let inline_block_id2 = new_ids.get_id().unwrap();
//         let inline_block_id3 = new_ids.get_id().unwrap();
//         let paragraph_block_id = new_ids.get_id().unwrap();
//         let root_block_id = new_ids.get_id().unwrap();

//         let inline_block1 = json!({
//             "_id": inline_block_id1.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "Hello"
//             },
//             "marks": ["italic"],
//             "parent": paragraph_block_id.clone()
//         }).to_string();
//         let inline_block2 = json!({
//             "_id": inline_block_id2.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "brave new "
//             },
//             "marks": [],
//             "parent": paragraph_block_id.clone()
//         }).to_string();
//         let inline_block3 = json!({
//             "_id": inline_block_id3.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "World!"
//             },
//             "marks": [],
//             "parent": paragraph_block_id.clone()
//         }).to_string();
//         let block = json!({
//             "_id": paragraph_block_id.clone(),
//             "kind": "standard",
//             "_type": "paragraph",
//             "content": {
//                 "inline_blocks": [
//                     inline_block_id1.clone(), inline_block_id2.clone(), inline_block_id3.clone()
//                 ]
//             },
//             "children": [],
//             "marks": [],
//             "parent": root_block_id.clone()
//         }).to_string();
//         let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id.clone()]).to_string();

//         let block_map = BlockMap::from(vec![inline_block1, inline_block2, inline_block3, block, root_block]).unwrap();
//         let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
//         let from_sub_selection = SubSelection::from(inline_block_id1.clone(), 4, None);
//         let to_sub_selection = SubSelection::from(inline_block_id3.clone(), 1, None);
//         let selection = Selection::from(from_sub_selection, to_sub_selection);

//         let steps = generate_steps(&event, &block_map, selection).unwrap();

//         assert_eq!(steps.len(), 1);
//         match &steps[0] {
//             Step::ReplaceStep(replace_step) => {
//                 assert_eq!(replace_step.from, SubSelection::from(root_block_id.clone(), 0, None));
//                 assert_eq!(replace_step.to, SubSelection::from(root_block_id.clone(), 1, None));
//                 assert_eq!(replace_step.slice.len(), 2);
//                 assert_eq!(replace_step.slice[0], paragraph_block_id.clone());
//                 assert_eq!(replace_step.blocks_to_update.len(), 4);
//                 match &replace_step.blocks_to_update[0] {
//                     Block::StandardBlock(standard_block) => {
//                         assert_eq!(standard_block.id(), paragraph_block_id.clone());
//                         match &standard_block.content {
//                             StandardBlockType::Paragraph(content_block) => {
//                                 assert_eq!(content_block.inline_blocks.len(), 1);
//                                 assert_eq!(content_block.inline_blocks[0], inline_block_id1.clone());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(standard_block.parent, root_block_id.clone());
//                         assert_eq!(standard_block.marks, vec![]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[1] {
//                     Block::InlineBlock(inline_block) => {
//                         assert_eq!(inline_block.id(), inline_block_id1.clone());
//                         match &inline_block.content {
//                             InlineBlockType::TextBlock(TextBlock { text }) => {
//                                 assert_eq!(text, &"Hell".to_string());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(inline_block.parent, paragraph_block_id.clone());
//                         assert_eq!(inline_block.marks, vec![Mark::Bold]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[2] {
//                     Block::StandardBlock(standard_block) => {
//                         match &standard_block.content {
//                             StandardBlockType::Paragraph(content_block) => {
//                                 assert_eq!(content_block.inline_blocks.len(), 1);
//                                 assert_eq!(content_block.inline_blocks[0], replace_step.blocks_to_update[3].id());
//                                 assert_eq!(content_block.inline_blocks[1], inline_block_id3.clone());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(standard_block.parent, root_block_id.clone());
//                         assert_eq!(standard_block.marks, vec![]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 };
//                 match &replace_step.blocks_to_update[3] {
//                     Block::InlineBlock(inline_block) => {
//                         match &inline_block.content {
//                             InlineBlockType::TextBlock(TextBlock { text }) => {
//                                 assert_eq!(text, &"orld!".to_string());
//                             },
//                             _ => panic!("Expected paragraph block"),
//                         };
//                         assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
//                         assert_eq!(inline_block.marks, vec![Mark::Bold]);
//                     },
//                     _ => panic!("Expected inline block"),
//                 }
//             },
//             _ => panic!("Expected ReplaceStep")
//         };
//     }

//     #[test]
//     fn can_handle_enter_over_standard_blocks() -> Result<(), StepError> {
//         let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

//         let inline_block_id1 = new_ids.get_id()?;
//         let inline_block_id2 = new_ids.get_id()?;
//         let inline_block_id3 = new_ids.get_id()?;
//         let paragraph_block_id1 = new_ids.get_id()?;
//         let paragraph_block_id2 = new_ids.get_id()?;
//         let paragraph_block_id3 = new_ids.get_id()?;
//         let root_block_id = new_ids.get_id()?;

//         let inline_block1 = json!({
//             "_id": inline_block_id1.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "Hello World"
//             },
//             "marks": [],
//             "parent": paragraph_block_id1.clone()
//         });
//         let inline_block2 = json!({
//             "_id": inline_block_id2.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "Hello again!"
//             },
//             "marks": [],
//             "parent": paragraph_block_id2.clone()
//         });
//         let inline_block3 = json!({
//             "_id": inline_block_id3.clone(),
//             "kind": "inline",
//             "_type": "text",
//             "content": {
//                 "text": "Goodbye World"
//             },
//             "marks": [],
//             "parent": paragraph_block_id2.clone()
//         });
//         let paragraph_block1 = json!({
//             "_id": paragraph_block_id1.clone(),
//             "kind": "standard",
//             "_type": "paragraph",
//             "content": {
//                 "inline_blocks": [inline_block_id1.clone()]
//             },
//             "children": [],
//             "marks": [],
//             "parent": root_block_id.clone()
//         });
//         let paragraph_block2 = json!({
//             "_id": paragraph_block_id2.to_string(),
//             "kind": "standard",
//             "_type": "paragraph",
//             "content": {
//                 "inline_blocks": [inline_block_id2.clone()]
//             },
//             "children": [],
//             "marks": [],
//             "parent": root_block_id.clone()
//         });
//         let paragraph_block3 = json!({
//             "_id": paragraph_block_id3.clone(),
//             "kind": "standard",
//             "_type": "paragraph",
//             "content": {
//                 "inline_blocks": [inline_block_id3.clone()]
//             },
//             "children": [],
//             "marks": [],
//             "parent": root_block_id.clone()
//         });
//         let root_block = RootBlock::json_from(root_block_id.clone(), vec![
//             paragraph_block_id1.clone(), paragraph_block_id2.clone(), paragraph_block_id3.clone()
//         ]);

//         let block_map = BlockMap::from(vec![
//             inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(),
//             paragraph_block1.to_string(), paragraph_block2.to_string(), paragraph_block3.to_string(), root_block.to_string()
//         ]).unwrap();

//         let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
//         let from_sub_selection = SubSelection::from(paragraph_block_id1.clone(), 0, Some(Box::new(
//             SubSelection::from(inline_block_id1.clone(), 1, None)
//         )));
//         let to_sub_selection = SubSelection::from(paragraph_block_id3.clone(), 1, Some(Box::new(
//             SubSelection::from(inline_block_id3.clone(), 3, None)
//         )));
//         let selection = Selection::from(from_sub_selection, to_sub_selection);

//         let steps = generate_steps(&event, &block_map, selection).unwrap();

//         assert_eq!(steps.len(), 1);
//         match &steps[0] {
//             Step::ReplaceStep(replace_step) => {
//                 assert_eq!(replace_step.from, SubSelection::from(root_block_id.clone(), 0, None));
//                 assert_eq!(replace_step.to, SubSelection::from(root_block_id.clone(), 3, None));
//                 assert_eq!(replace_step.slice.len(), 3);
//                 assert_eq!(replace_step.slice[0], paragraph_block_id1);
//                 assert_eq!(replace_step.slice[2], paragraph_block_id3);
//                 assert_eq!(replace_step.blocks_to_update.len(), 4);
//                 // need to finish
//                 // match &replace_step.blocks_to_update[0] {
//                 //     Block::StandardBlock(standard_block) => {
//                 //         assert_eq!(standard_block.id(), paragraph_block_id);
//                 //         match &standard_block.content {
//                 //             StandardBlockType::Paragraph(content_block) => {
//                 //                 assert_eq!(content_block.inline_blocks.len(), 1);
//                 //                 assert_eq!(content_block.inline_blocks[0], inline_block_id.clone()1);
//                 //             },
//                 //             _ => panic!("Expected paragraph block"),
//                 //         };
//                 //         assert_eq!(standard_block.parent, root_block_id.clone());
//                 //         assert_eq!(standard_block.marks, vec![]);
//                 //     },
//                 //     _ => panic!("Expected inline block"),
//                 // };
//                 // match &replace_step.blocks_to_update[1] {
//                 //     Block::InlineBlock(inline_block) => {
//                 //         assert_eq!(inline_block.id(), inline_block_id.clone()1);
//                 //         match &inline_block.content {
//                 //             InlineBlockType::TextBlock(TextBlock { text }) => {
//                 //                 assert_eq!(text, &"Hell".to_string());
//                 //             },
//                 //             _ => panic!("Expected paragraph block"),
//                 //         };
//                 //         assert_eq!(inline_block.parent, paragraph_block_id);
//                 //         assert_eq!(inline_block.marks, vec![Mark::Bold]);
//                 //     },
//                 //     _ => panic!("Expected inline block"),
//                 // };
//                 // match &replace_step.blocks_to_update[2] {
//                 //     Block::StandardBlock(standard_block) => {
//                 //         match &standard_block.content {
//                 //             StandardBlockType::Paragraph(content_block) => {
//                 //                 assert_eq!(content_block.inline_blocks.len(), 1);
//                 //                 assert_eq!(content_block.inline_blocks[0], replace_step.blocks_to_update[3].id());
//                 //                 assert_eq!(content_block.inline_blocks[1], inline_block_id.clone()3);
//                 //             },
//                 //             _ => panic!("Expected paragraph block"),
//                 //         };
//                 //         assert_eq!(standard_block.parent, root_block_id.clone());
//                 //         assert_eq!(standard_block.marks, vec![]);
//                 //     },
//                 //     _ => panic!("Expected inline block"),
//                 // };
//                 // match &replace_step.blocks_to_update[3] {
//                 //     Block::InlineBlock(inline_block) => {
//                 //         match &inline_block.content {
//                 //             InlineBlockType::TextBlock(TextBlock { text }) => {
//                 //                 assert_eq!(text, &"orld!".to_string());
//                 //             },
//                 //             _ => panic!("Expected paragraph block"),
//                 //         };
//                 //         assert_eq!(inline_block.parent, replace_step.blocks_to_update[2].id());
//                 //         assert_eq!(inline_block.marks, vec![Mark::Bold]);
//                 //     },
//                 //     _ => panic!("Expected inline block"),
//                 // }
//             },
//             _ => panic!("Expected ReplaceStep")
//         };
//         Ok(())
//     }
}