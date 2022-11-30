#[cfg(test)]
mod tests {
    use rust_mirror::{steps_generator::{StepError, event::{Event, FormatBarEvent}, selection::{SubSelection, Selection}, generate_steps}, blocks::{RootBlock, BlockMap}, steps_executor::execute_steps, mark::Mark, new_ids::NewIds};

    use serde_json::json;

    #[test]
    fn can_execute_apply_mark_with_simple_selection_within_one_inline() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let paragraph_block_id = new_ids.get_id()?;
        let inline_block_id = new_ids.get_id()?;
        let inline_block = json!({
            "_id": inline_block_id.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
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
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![inline_block.to_string(), block.to_string(), root_block.to_string()]).unwrap();
        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection::from(inline_block_id.clone(), 1, None);
        let sub_selection_to = SubSelection::from(inline_block_id.clone(), 5, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let updated_standard_block = updated_state.block_map.get_standard_block(&paragraph_block_id).unwrap();
        let content_block = updated_standard_block.content_block().unwrap();
        assert_eq!(content_block.inline_blocks.len(), 3);

        let inline_block1 = updated_state.block_map.get_inline_block(&content_block.inline_blocks[0]).unwrap();
        assert_eq!(inline_block1.text().unwrap().clone().to_string().as_str(), "H");
        assert_eq!(inline_block1.marks.len(), 0);

        let inline_block2 = updated_state.block_map.get_inline_block(&content_block.inline_blocks[1]).unwrap();
        assert_eq!(inline_block2.text().unwrap().clone().to_string().as_str(), "ello");
        assert_eq!(inline_block2.marks.len(), 1);
        assert_eq!(inline_block2.marks[0], Mark::Bold);

        let inline_block3 = updated_state.block_map.get_inline_block(&content_block.inline_blocks[2]).unwrap();
        assert_eq!(inline_block3.text().unwrap().clone().to_string().as_str(), " World");
        assert_eq!(inline_block3.marks.len(), 0);

        let expected_selection = Selection {
            anchor: SubSelection {
                block_id: inline_block2.id(),
                offset: 0,
                subselection: None
            },
            head: SubSelection {
                block_id: inline_block2.id(),
                offset: 4,
                subselection: None
            },
        };

        assert_eq!(updated_state.selection, Some(expected_selection));
        return Ok(())
    }

    #[test]
    fn can_execute_remove_mark_selection_across_multiple_inline_blocks() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let paragraph_block_id = new_ids.get_id()?;
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block1 = json!({
            "_id": inline_block_id1.clone().to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": ["italic"],
            "parent": paragraph_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone().to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World"
            },
            "marks": ["bold", "italic"],
            "parent": paragraph_block_id.clone()
        });
        let block = json!({
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone().to_string(), inline_block_id2.clone().to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::FormatBar(FormatBarEvent::Italic);
        let sub_selection_from = SubSelection::from(inline_block_id1.clone(), 2, None);
        let sub_selection_to = SubSelection::from(inline_block_id2.clone(), 3, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();
        let updated_standard_block = updated_state.block_map.get_standard_block(&paragraph_block_id).unwrap();
        let content_block = updated_standard_block.content_block().unwrap();
        assert_eq!(content_block.inline_blocks.len(), 4);
        let mut i = 0;
        for id in content_block.inline_blocks.iter() {
            let inline_block = updated_state.block_map.get_inline_block(id).unwrap();
            if i == 0 {
                assert_eq!(inline_block.text().unwrap().clone().to_string().as_str(), "He");
                assert_eq!(inline_block.marks.len(), 1);
                assert_eq!(inline_block.marks[0], Mark::Italic);
            } else if i == 1 {
                assert_eq!(inline_block.text().unwrap().clone().to_string().as_str(), "llo ");
                assert_eq!(inline_block.marks.len(), 0);
            } else if i == 2 {
                assert_eq!(inline_block.marks[0], Mark::Bold);
                assert_eq!(inline_block.text().unwrap().clone().to_string().as_str(), "Wor");
                assert_eq!(inline_block.marks.len(), 1);
            } else if i == 3 {
                assert_eq!(inline_block.text().unwrap().clone().to_string().as_str(), "ld");
                assert_eq!(inline_block.marks.len(), 2);
                assert_eq!(inline_block.marks.contains(&Mark::Bold), true);
                assert_eq!(inline_block.marks.contains(&Mark::Italic), true);
            }
            i += 1;
        }

        let expected_selection = Selection {
            anchor: SubSelection { block_id: content_block.inline_blocks[1].clone(), offset: 0, subselection: None },
            head: SubSelection { block_id: content_block.inline_blocks[2].clone(), offset: 3, subselection: None },
        };

        assert_eq!(updated_state.selection, Some(expected_selection));

        Ok(())
    }

    #[test]
    fn can_execute_apply_mark_selection_across_multiple_inline_blocks_and_execute_merge() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let inline_block1 = json!({
            "_id": inline_block_id1.clone().to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone().to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "brave new "
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone().to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        });
        let block = json!({
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [
                    inline_block_id1.clone().to_string(), inline_block_id2.clone().to_string(), inline_block_id3.clone().to_string()
                ]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);
        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection::from(inline_block_id1.clone(), 0, None);
        let sub_selection_to = SubSelection::from(inline_block_id3.clone(), 3, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();
        let updated_standard_block = updated_state.block_map.get_standard_block(&paragraph_block_id).unwrap();
        let content_block = updated_standard_block.content_block().unwrap();
        assert_eq!(content_block.inline_blocks.len(), 2);
        let mut i = 0;
        let mut first_inline_block_id_after_update = "".to_string();
        for id in content_block.inline_blocks.iter() {
            let inline_block = updated_state.block_map.get_inline_block(id).unwrap();
            if i == 0 {
                assert_eq!(inline_block.text().unwrap().clone().to_string().as_str(), "Hello brave new Wor");
                assert_eq!(inline_block.marks.len(), 1);
                assert_eq!(inline_block.marks[0], Mark::Bold);
                first_inline_block_id_after_update = inline_block.id();

            } else if i == 1 {
                assert_eq!(inline_block.text().unwrap().clone().to_string().as_str(), "ld!");
                assert_eq!(inline_block.marks.len(), 0);
            }
            i += 1;
        }

        let expected_selection = Selection {
            anchor: SubSelection { block_id: first_inline_block_id_after_update.clone(), offset: 0, subselection: None },
            head: SubSelection { block_id: first_inline_block_id_after_update.clone(), offset: 19, subselection: None }
        };
        assert_eq!(updated_state.selection, Some(expected_selection));
    }

    #[test]
    fn can_execute_apply_mark_with_selection_across_standard_blocks() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let paragraph_block_id1 = new_ids.get_id()?;
        let paragraph_block_id2 = new_ids.get_id()?;
        let paragraph_block_id3 = new_ids.get_id()?;
        let root_block_id = new_ids.get_id()?;

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World"
            },
            "marks": ["underline"],
            "parent": paragraph_block_id1.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye World"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello again!"
            },
            "marks": [],
            "parent": paragraph_block_id3.clone()
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
            "parent": root_block_id.to_string()
        });
        let paragraph_block2 = json!({
            "_id": paragraph_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.to_string()]
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
                "inline_blocks": [inline_block_id4.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(
            root_block_id.clone(),
        vec![paragraph_block_id1.clone(), paragraph_block_id2.clone(), paragraph_block_id3.clone()
        ]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), inline_block4.to_string(),
            paragraph_block1.to_string(), paragraph_block2.to_string(), paragraph_block3.to_string(), root_block.to_string()
        ]).unwrap();

        let event = Event::FormatBar(FormatBarEvent::Underline);
        let sub_selection_from = SubSelection::from(paragraph_block_id1.clone(), 0, Some(Box::new(
            SubSelection::from(
            inline_block_id1,
            2,
            None
        ))));
        let sub_selection_to = SubSelection::from(paragraph_block_id3.clone(), 0, Some(Box::new(
            SubSelection::from(
            inline_block_id4,
            1,
            None
        ))));
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection)?;
        let updated_state = execute_steps(steps, block_map, &mut new_ids)?;

        let updated_paragraph_block_1 = updated_state.block_map.get_standard_block(&paragraph_block_id1)?;
        assert_eq!(updated_paragraph_block_1.content_block()?.inline_blocks.len(), 2);

        let updated_first_inline_block = updated_state.block_map.get_inline_block(&updated_paragraph_block_1.content_block()?.inline_blocks[0])?;
        assert_eq!(updated_first_inline_block.text()?.clone().to_string(), "He".to_string());
        assert_eq!(updated_first_inline_block.marks, vec![]);
        let updated_second_inline_block = updated_state.block_map.get_inline_block(&updated_paragraph_block_1.content_block()?.inline_blocks[1])?;
        assert_eq!(updated_second_inline_block.text()?.clone().to_string(), "llo World".to_string());
        assert_eq!(updated_second_inline_block.marks, vec![Mark::Underline]);

        let updated_paragraph_block_2 = updated_state.block_map.get_standard_block(&paragraph_block_id2)?;
        assert_eq!(updated_paragraph_block_2.content_block()?.inline_blocks.len(), 1);

        let updated_third_inline_block = updated_state.block_map.get_inline_block(&updated_paragraph_block_2.content_block()?.inline_blocks[0])?;
        assert_eq!(updated_third_inline_block.text()?.clone().to_string(), "Goodbye World".to_string());
        assert_eq!(updated_third_inline_block.marks, vec![Mark::Underline]);

        let updated_paragraph_block_3 = updated_state.block_map.get_standard_block(&paragraph_block_id3)?;
        assert_eq!(updated_paragraph_block_3.content_block()?.inline_blocks.len(), 2);

        let updated_fourth_inline_block = updated_state.block_map.get_inline_block(&updated_paragraph_block_3.content_block()?.inline_blocks[0])?;
        assert_eq!(updated_fourth_inline_block.text()?.clone().to_string(), "H".to_string());
        assert_eq!(updated_fourth_inline_block.marks, vec![Mark::Underline]);
        let updated_fifth_inline_block = updated_state.block_map.get_inline_block(&updated_paragraph_block_3.content_block()?.inline_blocks[1])?;
        assert_eq!(updated_fifth_inline_block.text()?.clone().to_string(), "ello again!".to_string());
        assert_eq!(updated_fifth_inline_block.marks, vec![]);


        return Ok(())
    }

    /// NOT ALL BOLD => SHOULD ADD BOLD
    /// <1>A</1> *start of selection*
    ///     <2>B</2>
    ///         <3>C</3>
    ///     <4>D</4>
    /// <5>E</5>
    /// <6>F</6>
    ///     <7>G</7>
    ///         <8>H</8>
    ///     <9>I</9>
    ///         <10>J</10> *end of selection*
    #[test]
    fn can_add_mark_over_many_different_layers_where_from_block_is_shallower_than_to_block() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let p_id7 = "7".to_string();
        let p_id8 = "8".to_string();
        let p_id9 = "9".to_string();
        let p_id10 = "10".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let inline_block_id7 = new_ids.get_id()?;
        let inline_block_id8 = new_ids.get_id()?;
        let inline_block_id9 = new_ids.get_id()?;
        let inline_block_id10 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id4.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": ["bold"],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": ["bold"],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": ["bold"],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": ["bold"],
            "parent": p_id5.clone()
        });
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [p_id7.clone(), p_id9.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "F"
            },
            "marks": ["bold"],
            "parent": p_id6.clone()
        });
        let p7 = json!({
            "_id": p_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [p_id8.clone()],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "G"
            },
            "marks": ["bold"],
            "parent": p_id7.clone()
        });
        let p8 = json!({
            "_id": p_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id8.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id7.clone()
        });
        let inline_block8 = json!({
            "_id": inline_block_id8.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "H"
            },
            "marks": [],
            "parent": p_id8.clone()
        });
        let p9 = json!({
            "_id": p_id9.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id9.clone()]
            },
            "children": [p_id10.clone()],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block9 = json!({
            "_id": inline_block_id9.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "I"
            },
            "marks": ["bold"],
            "parent": p_id9.clone()
        });
        let p10 = json!({
            "_id": p_id10.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id10.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id9.clone()
        });
        let inline_block10 = json!({
            "_id": inline_block_id10.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "J"
            },
            "marks": ["bold"],
            "parent": p_id10.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone(), p_id5.clone(), p_id6.clone()]
        );

        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id1.clone(), 0, None)))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id6.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id9.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: p_id9.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: p_id10.clone(),
                        offset: 0,
                        subselection: Some(Box::new(SubSelection::from(inline_block_id10, 1, None)))
        }))}))}))};

        let selection = Selection::from(sub_selection_to.clone(), sub_selection_from.clone());

        let block_map = BlockMap::from(vec![
            root_block.to_string(),
            p1.to_string(), inline_block1.to_string(),
            p2.to_string(), inline_block2.to_string(),
            p3.to_string(), inline_block3.to_string(),
            p4.to_string(), inline_block4.to_string(),
            p5.to_string(), inline_block5.to_string(),
            p6.to_string(), inline_block6.to_string(),
            p7.to_string(), inline_block7.to_string(),
            p8.to_string(), inline_block8.to_string(),
            p9.to_string(), inline_block9.to_string(),
            p10.to_string(), inline_block10.to_string()
        ])?;

        let steps = generate_steps(&event, &block_map, selection)?;
        let updated_state = execute_steps(steps, block_map, &mut new_ids)?;

        let mut i = 1 as usize;
        while i < 11 {
            let updated_p = updated_state.block_map.get_standard_block(&i.to_string())?;
            let updated_inline_block = updated_state.block_map.get_inline_block(&updated_p.content_block()?.inline_blocks[0])?;
            assert_eq!(updated_inline_block.marks, vec![Mark::Bold]);
            i += 1;
        }
        return Ok(())
    }

    /// <1>
    ///     <2> *selection starts here*
    ///         <3>
    ///     <4>
    ///         <5>
    ///             <6> *selection ends here*
    ///                 <7>
    #[test]
    fn can_add_mark_within_same_root_std_block_selection_in_different_layers() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let p_id7 = "7".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id2b = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let inline_block_id6b = new_ids.get_id()?;
        let inline_block_id7 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id4.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": ["bold"],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone(), inline_block_id2b.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": ["bold"],
            "parent": p_id2.clone()
        });
        let inline_block2b = json!({
            "_id": inline_block_id2b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Bvdsdsdsvsdvdsvdv"
            },
            "marks": [],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone()]
            },
            "children": [p_id5.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "D"
            },
            "marks": [],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [p_id6.clone()],
            "marks": [],
            "parent": p_id4.clone()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": ["bold"],
            "parent": p_id5.clone()
        });
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone(), inline_block_id6b.clone()]
            },
            "children": [p_id7.clone()],
            "marks": [],
            "parent": p_id5.clone()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Fdsfdsdsdsds"
            },
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block6b = json!({
            "_id": inline_block_id6b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "F"
            },
            "marks": ["underline"],
            "parent": p_id6.clone()
        });
        let p7 = json!({
            "_id": p_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "G"
            },
            "marks": [],
            "parent": p_id7.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone()]
        );

        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(
                    SubSelection {
                        block_id: inline_block_id2b.clone(),
                        offset: 3,
                        subselection: None
                    }
                ))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id4.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id5.clone(),
                offset: 0,
                subselection: Some(Box::new(
                    SubSelection {
                        block_id: p_id6.clone(),
                        offset: 0,
                        subselection: Some(Box::new(
                            SubSelection {
                                block_id: inline_block_id6.clone(),
                                offset: 3,
                                subselection: None
                            }
                        ))
                    }
                ))
            }))
        };

        let selection = Selection::from(sub_selection_to.clone(), sub_selection_from.clone());
        let block_map = BlockMap::from(vec![
            root_block.to_string(),
            p1.to_string(), inline_block1.to_string(),
            p2.to_string(), inline_block2.to_string(), inline_block2b.to_string(),
            p3.to_string(), inline_block3.to_string(),
            p4.to_string(), inline_block4.to_string(),
            p5.to_string(), inline_block5.to_string(),
            p6.to_string(), inline_block6.to_string(), inline_block6b.to_string(),
            p7.to_string(), inline_block7.to_string()
        ])?;
        let steps = generate_steps(&event, &block_map, selection)?;
        let updated_state = execute_steps(steps, block_map, &mut new_ids)?;

        let mut i = 2 as usize;
        while i < 8 {
            let updated_p = updated_state.block_map.get_standard_block(&i.to_string())?;
            let updated_inline_block = updated_state.block_map.get_inline_block(&updated_p.content_block()?.inline_blocks[0])?;
            if i == 2 {
                let updated_inline_block_b = updated_state.block_map.get_inline_block(&updated_p.content_block()?.inline_blocks[1])?;
                assert_eq!(updated_inline_block_b.text()?.clone().to_string(), "Bvd".to_string());
                assert_eq!(updated_inline_block_b.marks, vec![]);
                let updated_inline_block_c = updated_state.block_map.get_inline_block(&updated_p.content_block()?.inline_blocks[2])?;
                assert_eq!(updated_inline_block_c.marks, vec![Mark::Bold]);
                assert_eq!(updated_inline_block_c.text()?.clone().to_string(), "sdsdsvsdvdsvdv".to_string());
            } else if i == 6 {
                assert_eq!(updated_inline_block.marks, vec![Mark::Bold]);
                assert_eq!(updated_inline_block.text()?.clone().to_string(), "Fds".to_string());
                let updated_inline_block_b = updated_state.block_map.get_inline_block(&updated_p.content_block()?.inline_blocks[1])?;
                assert_eq!(updated_inline_block_b.marks, vec![]);
            } else if i != 7 {
                assert_eq!(updated_inline_block.marks, vec![Mark::Bold]);
            } else {
                assert_eq!(updated_inline_block.marks, vec![]);
            }
            i += 1;
        }
        return Ok(())
    }

     /// ALL BOLD => SHOULD REMOVE BOLD
    /// <1>A</1>
    ///     <2>B</2>
    ///         <3>C</3> *start of selection*
    ///     <4>D</4>
    /// <5>E</5>
    /// <6>F</6> *end of selection*
    ///     <7>G</7>
    ///         <8>H</8>
    ///     <9>I</9>
    ///         <10>J</10>
    #[test]
    fn can_remove_mark_over_many_different_layers_where_from_block_is_deeper_than_to_block() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let p_id7 = "7".to_string();
        let p_id8 = "8".to_string();
        let p_id9 = "9".to_string();
        let p_id10 = "10".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id3b = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let inline_block_id6b = new_ids.get_id()?;
        let inline_block_id7 = new_ids.get_id()?;
        let inline_block_id8 = new_ids.get_id()?;
        let inline_block_id9 = new_ids.get_id()?;
        let inline_block_id10 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id4.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": [],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": [],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone(), inline_block_id3b.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": [],
            "parent": p_id3.clone()
        });
        let inline_block3b = json!({
            "_id": inline_block_id3b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "fdsasfdfsdfds"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": ["bold"],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": ["bold"],
            "parent": p_id5.clone()
        });
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone(), inline_block_id6b.clone()]
            },
            "children": [p_id7.clone(), p_id9.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Fafdsfsd"
            },
            "marks": ["bold"],
            "parent": p_id6.clone()
        });
        let inline_block6b = json!({
            "_id": inline_block_id6b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Fb"
            },
            "marks": [],
            "parent": p_id6.clone()
        });
        let p7 = json!({
            "_id": p_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [p_id8.clone()],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "G"
            },
            "marks": [],
            "parent": p_id7.clone()
        });
        let p8 = json!({
            "_id": p_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id8.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id7.clone()
        });
        let inline_block8 = json!({
            "_id": inline_block_id8.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "H"
            },
            "marks": [],
            "parent": p_id8.clone()
        });
        let p9 = json!({
            "_id": p_id9.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id9.clone()]
            },
            "children": [p_id10.clone()],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block9 = json!({
            "_id": inline_block_id9.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "I"
            },
            "marks": [],
            "parent": p_id9.clone()
        });
        let p10 = json!({
            "_id": p_id10.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id10.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id9.clone()
        });
        let inline_block10 = json!({
            "_id": inline_block_id10.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "J"
            },
            "marks": [],
            "parent": p_id10.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone(), p_id5.clone(), p_id6.clone()]
        );

        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: p_id3.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection::from(inline_block_id3b.clone(), 4, None)))
                }))
            }))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id6.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id6.clone(), 4, None)))
        };

        let selection = Selection::from(sub_selection_to.clone(), sub_selection_from.clone());

        let block_map = BlockMap::from(vec![
            root_block.to_string(),
            p1.to_string(), inline_block1.to_string(),
            p2.to_string(), inline_block2.to_string(),
            p3.to_string(), inline_block3.to_string(),
            p4.to_string(), inline_block4.to_string(),
            p5.to_string(), inline_block5.to_string(),
            p6.to_string(), inline_block6.to_string(),
            p7.to_string(), inline_block7.to_string(),
            p8.to_string(), inline_block8.to_string(),
            p9.to_string(), inline_block9.to_string(),
            p10.to_string(), inline_block10.to_string(),
            inline_block3b.to_string(), inline_block6b.to_string()
        ])?;

        let steps = generate_steps(&event, &block_map, selection.clone()).unwrap();
        let updated_state = execute_steps(steps, block_map, &mut new_ids).unwrap();

        let mut i = 3 as usize;
        while i < 7 {
            let p = updated_state.block_map.get_standard_block(&i.to_string()).unwrap();
            let updated_inline_block = match i {
                3 => updated_state.block_map.get_inline_block(&p.content_block()?.inline_blocks[2])?,
                4 => updated_state.block_map.get_inline_block(&inline_block_id4.clone())?,
                5 => updated_state.block_map.get_inline_block(&inline_block_id5.clone())?,
                6 => updated_state.block_map.get_inline_block(&p.content_block()?.inline_blocks[0])?,
                _ => panic!("Should not happen")
            };
            assert_eq!(updated_inline_block.marks, vec![]);
            if i == 3 {
                assert_eq!(updated_inline_block.text()?.clone().to_string(), "sfdfsdfds");
            }
            if i == 6 {
                assert_eq!(updated_inline_block.text()?.clone().to_string(), "Fafd");
            }

            i += 1;
        }

        return Ok(());
    }

    /// <1>
    ///     <2>
    ///         <3> *selection starts here*
    ///     <4> *selection ends here*
    ///         <5>
    ///             <6>
    ///                 <7>
    #[test]
    fn can_remove_mark_with_selection_across_inline_blocks_in_different_standard_blocks() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let p_id7 = "7".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id3b = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id4b = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let inline_block_id7 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id4.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": [],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": [],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone(), inline_block_id3b.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": [],
            "parent": p_id3.clone()
        });
        let inline_block3b = json!({
            "_id": inline_block_id3b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "lololololol"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone(), inline_block_id4b.clone()]
            },
            "children": [p_id5.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "hehehehehe"
            },
            "marks": ["bold"],
            "parent": p_id4.clone()
        });
        let inline_block4b = json!({
            "_id": inline_block_id4b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "abc"
            },
            "marks": [],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [p_id6.clone()],
            "marks": [],
            "parent": p_id4.clone()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": [],
            "parent": p_id5.clone()
        });
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [p_id7.clone()],
            "marks": [],
            "parent": p_id5.clone()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "F"
            },
            "marks": [],
            "parent": p_id6.clone()
        });
        let p7 = json!({
            "_id": p_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "G"
            },
            "marks": [],
            "parent": p_id7.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone()]
        );

        let block_map = BlockMap::from(
            vec![
                root_block.to_string(),
                p1.to_string(), inline_block1.to_string(),
                p2.to_string(), inline_block2.to_string(),
                p3.to_string(), inline_block3.to_string(), inline_block3b.to_string(),
                p4.to_string(), inline_block4.to_string(), inline_block4b.to_string(),
                p5.to_string(), inline_block5.to_string(),
                p6.to_string(), inline_block6.to_string(),
                p7.to_string(), inline_block7.to_string()
            ]
        ).unwrap();

        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(
                    SubSelection {
                        block_id: p_id3.clone(),
                        offset: 0,
                        subselection: Some(Box::new(SubSelection::from(inline_block_id3b.clone(), 4, None)))
                    }
                ))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id4.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id4.clone(), 4, None)))
        };

        let selection = Selection {
            anchor: sub_selection_from.clone(),
            head: sub_selection_to.clone()
        };
        let steps = generate_steps(&event, &block_map, selection)?;
        let updated_state = execute_steps(steps, block_map, &mut new_ids)?;

        let mut i = 3 as usize;
        while i < 5 {
            let updated_inline_block = match i {
                3 => updated_state.block_map.get_inline_block(&inline_block_id3b.clone())?,
                4 => updated_state.block_map.get_inline_block(&inline_block_id4.clone())?,
                _ => panic!("Should not happen")
            };
            assert_eq!(updated_inline_block.marks, vec![]);

            i += 1;
        }

        return Ok(());
    }

    /// <1> *selection starts here*
    ///     <2> *selection ends here*
    /// <3>
    #[test]
    fn can_add_mark_with_selection_across_inline_blocks_in_different_standard_blocks_2() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": [],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": [],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": [],
            "parent": p_id3.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone(), p_id3.clone()]
        );

        let block_map = BlockMap::from(
            vec![
                root_block.to_string(),
                p1.to_string(), inline_block1.to_string(),
                p2.to_string(), inline_block2.to_string(),
                p3.to_string(), inline_block3.to_string(),
            ]
        ).unwrap();

        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id1.clone(), 0, None)))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection::from(inline_block_id2.clone(), 1, None)))}))
        };

        let selection = Selection {
            anchor: sub_selection_from.clone(),
            head: sub_selection_to.clone()
        };
        let steps = generate_steps(&event, &block_map, selection)?;
        let updated_state = execute_steps(steps, block_map, &mut new_ids)?;

        let updated_p1 = updated_state.block_map.get_standard_block(&p_id1)?;
        assert_eq!(updated_state.block_map.get_inline_block(&updated_p1.content_block()?.inline_blocks[0])?.marks, vec![Mark::Bold]);
        let updated_p2 = updated_state.block_map.get_standard_block(&p_id2)?;
        assert_eq!(updated_state.block_map.get_inline_block(&updated_p2.content_block()?.inline_blocks[0])?.marks, vec![Mark::Bold]);
        return Ok(())
    }
}