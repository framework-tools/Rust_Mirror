#[cfg(test)]
mod tests {
    use rust_mirror::{blocks::{BlockMap, RootBlock}, steps_generator::{event::{Event, FormatBarEvent},
    selection::{SubSelection, Selection}, generate_steps, StepError}, step::Step, mark::{Mark, Color}, new_ids::NewIds};

    use serde_json::json;

    #[test]
    fn can_apply_mark_simple_selection_within_one_inline() -> Result<(), StepError> {
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
        let sub_selection_from = SubSelection::from(inline_block_id.clone(), 6, None);
        let sub_selection_to = SubSelection::from(inline_block_id.clone(), 11, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::AddMarkStep(add_mark_step) => {
                assert_eq!(add_mark_step.block_id, paragraph_block_id);
                assert_eq!(add_mark_step.from, sub_selection_from);
                assert_eq!(add_mark_step.to, sub_selection_to);
                assert_eq!(add_mark_step.mark, Mark::Bold);
            },
            step => return Err(StepError(format!("Expected AddMarkStep. Got: {:?}", step)))
        };
        return Ok(())
    }

    #[test]
    fn can_remove_mark_selection_across_multiple_inline_blocks() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let paragraph_block_id = new_ids.get_id()?;
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": ["italic"],
            "parent": paragraph_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
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
                "inline_blocks": [inline_block_id1.clone(), inline_block_id2.clone()]
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
        let sub_selection_from = SubSelection::from(inline_block_id1, 2, None);
        let sub_selection_to = SubSelection::from(inline_block_id2, 3, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::RemoveMarkStep(remove_mark_step) => {
                assert_eq!(remove_mark_step.block_id, paragraph_block_id);
                assert_eq!(remove_mark_step.from, sub_selection_from);
                assert_eq!(remove_mark_step.to, sub_selection_to);
                assert_eq!(remove_mark_step.mark, Mark::Italic);
            },
            step => return Err(StepError(format!("Expected RemoveMarkStep. Got: {:?}", step)))
        };
        return Ok(())
    }

    #[test]
    fn can_parse_fore_color_mark() {
        let mark = Mark::from_str("fore_color(0, 0, 0, 1)").unwrap();
        assert_eq!(mark, Mark::ForeColor(Color (0, 0, 0, 1)));
    }

    #[test]
    fn can_apply_color_mark_selection_across_multiple_inline_blocks_with_different_color_already_present() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let paragraph_block_id = new_ids.get_id()?;
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": ["fore_color(255, 255, 255, 1)"],
            "parent": paragraph_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World"
            },
            "marks": ["bold", "fore_color(255, 255, 0, 1)"],
            "parent": paragraph_block_id.clone()
        });
        let block = json!({
            "_id": paragraph_block_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone(), inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::FormatBar(FormatBarEvent::ForeColor(Color(255, 255, 0, 1)));
        let sub_selection_from = SubSelection::from(inline_block_id1, 2, None);
        let sub_selection_to = SubSelection::from(inline_block_id2, 3, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::AddMarkStep(add_mark_step) => {
                assert_eq!(add_mark_step.block_id, paragraph_block_id);
                assert_eq!(add_mark_step.from, sub_selection_from);
                assert_eq!(add_mark_step.to, sub_selection_to);
                assert_eq!(add_mark_step.mark, Mark::ForeColor(Color(255, 255, 0, 1)));
            },
            step => return Err(StepError(format!("Expected AddMarkStep. Got: {:?}", step)))
        };
        return Ok(())
    }

    #[test]
    fn can_apply_mark_with_selection_across_inline_blocks_in_different_standard_blocks() -> Result<(), StepError> {
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
            "_id": inline_block_id3.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye World"
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
        let root_block = RootBlock::json_from(root_block_id.clone(),
        vec![paragraph_block_id1.clone(), paragraph_block_id2.clone(), paragraph_block_id3.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), inline_block3.to_string(), inline_block4.to_string(),
            paragraph_block1.to_string(), paragraph_block2.to_string(), paragraph_block3.to_string(), root_block.to_string()
        ]).unwrap();

        let event = Event::FormatBar(FormatBarEvent::Underline);
        let sub_selection_from = SubSelection::from(paragraph_block_id1, 0, Some(Box::new(
            SubSelection::from(
            inline_block_id1,
            2,
            None
        ))));
        let sub_selection_to = SubSelection::from(paragraph_block_id3, 0, Some(Box::new(
            SubSelection::from(
            inline_block_id4,
            1,
            None
        ))));
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection)?;

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::AddMarkStep(add_mark_step) => {
                assert_eq!(add_mark_step.block_id, root_block_id);
                assert_eq!(add_mark_step.from, sub_selection_from);
                assert_eq!(add_mark_step.to, sub_selection_to);
                assert_eq!(add_mark_step.mark, Mark::Underline);
            },
            step => return Err(StepError(format!("Expected AddMarkStep. Got: {:?}", step)))
        };
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
            "marks": ["bold"],
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

        let mut inline_blocks = vec![
            inline_block1,
            inline_block2,
            inline_block3,
            inline_block4,
            inline_block5,
            inline_block6,
            inline_block7,
            inline_block8,
            inline_block9,
            inline_block10,
        ];

        // testing every case
        let mut i = 0;
        while i < 10 {
            inline_blocks[i]["marks"] = json!([]);
            if i != 0 {
                inline_blocks[i - 1]["marks"] = json!(["bold"]);
            }
            let block_map = BlockMap::from(vec![
                root_block.to_string(),
                inline_blocks[0].to_string(), p1.to_string(),
                inline_blocks[1].to_string(), p2.to_string(),
                inline_blocks[2].to_string(), p3.to_string(),
                inline_blocks[3].to_string(), p4.to_string(),
                inline_blocks[4].to_string(), p5.to_string(),
                inline_blocks[5].to_string(), p6.to_string(),
                inline_blocks[6].to_string(), p7.to_string(),
                inline_blocks[7].to_string(), p8.to_string(),
                inline_blocks[8].to_string(), p9.to_string(),
                inline_blocks[9].to_string(), p10.to_string(),
            ]).unwrap();
            
            let steps = generate_steps(&event, &block_map, selection.clone()).unwrap();
            assert_eq!(steps.len(), 1);
            match &steps[0] {
                Step::AddMarkStep(add_mark_step) => {
                    assert_eq!(add_mark_step.block_id, root_block_id);
                    assert_eq!(add_mark_step.from, sub_selection_from);
                    assert_eq!(add_mark_step.to, sub_selection_to);
                    assert_eq!(add_mark_step.mark, Mark::Bold);
                },
                step => return Err(StepError(format!("Expected AddMarkStep. Got: {:?}", step)))
            };

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
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
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
            "marks": [],
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

        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(
                    SubSelection {
                        block_id: inline_block_id2.clone(),
                        offset: 0,
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
                                offset: 1,
                                subselection: None
                            }
                        ))
                    }
                ))
            }))
        };

        let selection = Selection::from(sub_selection_to.clone(), sub_selection_from.clone());

        let mut inline_blocks = vec![
            inline_block1,
            inline_block2,
            inline_block3,
            inline_block4,
            inline_block5,
            inline_block6,
            inline_block7,
        ];

        // testing every case
        let mut i = 0;
        while i < 7 {
            inline_blocks[i]["marks"] = json!([]);
            if i != 0 {
                inline_blocks[i - 1]["marks"] = json!(["bold"]);
            }
            let block_map = BlockMap::from(vec![
                root_block.to_string(),
                inline_blocks[0].to_string(), p1.to_string(),
                inline_blocks[1].to_string(), p2.to_string(),
                inline_blocks[2].to_string(), p3.to_string(),
                inline_blocks[3].to_string(), p4.to_string(),
                inline_blocks[4].to_string(), p5.to_string(),
                inline_blocks[5].to_string(), p6.to_string(),
                inline_blocks[6].to_string(), p7.to_string(),
            ]).unwrap();
            
            let steps = generate_steps(&event, &block_map, selection.clone()).unwrap();
            assert_eq!(steps.len(), 1);
            match &steps[0] {
                Step::AddMarkStep(add_mark_step) => {
                    assert_eq!(add_mark_step.block_id, p_id1);
                    assert_eq!(add_mark_step.from, sub_selection_from);
                    assert_eq!(add_mark_step.to, sub_selection_to);
                    assert_eq!(add_mark_step.mark, Mark::Bold);
                },
                step => return Err(StepError(format!("Expected AddMarkStep. Got: {:?}", step)))
            };

            i += 1;
        }
        return Ok(())
    }
}