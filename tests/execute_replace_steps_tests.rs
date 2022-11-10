#[cfg(test)]
mod tests {
    use rust_mirror::{steps_generator::{StepError, event::{Event, KeyPress, Key, FormatBarEvent},
    selection::{SubSelection, Selection}, generate_steps}, blocks::{RootBlock, BlockMap, Block},
    steps_executor::execute_steps, mark::Mark, step::{Step, ReplaceStep}, new_ids::NewIds};

    use serde_json::json;

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

        let block_map = BlockMap::from(vec![inline_block, block, root_block]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Standard('a'), None));
        let sub_selection = SubSelection::from(inline_block_id.clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids)?;
        let updated_block_map = execute_steps(steps, block_map, &mut new_ids)?;

        let updated_inline_block = updated_block_map.get_inline_block(&inline_block_id)?;
        assert_eq!(updated_inline_block.text()?, "a");
        Ok(())
    }

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
            inline_block, block, root_block
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('k'), None));
        let anchor_sub_selection = SubSelection::from(inline_block_id.clone(), 2, None);
        let head_sub_selection = SubSelection::from(inline_block_id.clone(), 4, None);
        let selection = Selection::from(anchor_sub_selection.clone(), head_sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();
        let updated_block_map = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let updated_inline_block = updated_block_map.get_inline_block(&inline_block_id).unwrap();
        assert_eq!(updated_inline_block.text().unwrap(), "sok text");
        assert_eq!(updated_inline_block.marks, vec![Mark::Bold]);
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
    fn can_handle_keypress_execution_across_2_standard_blocks() {
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
            "children": [std_block_id2.to_string()],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            std_block_id1.clone(), std_block_id3.clone()
            ]);

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, std_block1, std_block2, root_block, std_block3, std_block_4
        ]).unwrap();

        let event = Event::KeyPress(KeyPress::new(Key::Standard('a'), None));

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
                    block_id: inline_block_id2.clone(),
                    offset: 3,
                    subselection: None,
                }))
            },
        };

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();
        let updated_block_map = execute_steps(steps, block_map, &mut new_ids).unwrap();
        let updated_root_block = updated_block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![std_block_id1.clone()]);
        let updated_std_block1 = updated_block_map.get_standard_block(&std_block_id1).unwrap();
        assert_eq!(updated_std_block1.children, vec![std_block_id2]);
        assert_eq!(
            updated_std_block1.content_block().unwrap().inline_blocks,
            vec![inline_block_id1.clone(), inline_block_id2.clone()]
        );

        let updated_inline_block1 = updated_block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline_block1.text().unwrap(), "Ha");
        let updated_inline_block2 = updated_block_map.get_inline_block(&inline_block_id2).unwrap();
        assert_eq!(updated_inline_block2.text().unwrap(), "dbye world!");

    }

    #[test]
    pub fn can_merge_2_inline_blocks_that_should_be() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let std_block_id1 = new_ids.get_id().unwrap();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello"
            },
            "marks": ["bold"],
            "parent": std_block_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " World!"
            },
            "marks": ["bold"],
            "parent": std_block_id1.clone()
        });

        let std_block1 = json!({
            "_id": std_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id, vec![std_block_id1.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, std_block1, root_block
        ]).unwrap();

        let steps = vec![
            Step::ReplaceStep(ReplaceStep {
                block_id: std_block_id1.clone(),
                from: SubSelection { block_id: std_block_id1.clone(), offset: 1, subselection: None },
                to: SubSelection { block_id: std_block_id1.clone(), offset: 1, subselection: None },
                slice: vec![inline_block_id2],
                blocks_to_update: vec![]
            })
        ];
        let updated_block_map = execute_steps(steps, block_map, &mut new_ids).unwrap();
        let updated_standard_block = updated_block_map.get_standard_block(&std_block_id1).unwrap();
        let content_block = updated_standard_block.content_block().unwrap();
        assert_eq!(content_block.inline_blocks, vec![inline_block_id1.clone()]);
        let updated_inline_block1 = updated_block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline_block1.text().unwrap(), "Hello World!");
    }
}