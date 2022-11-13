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
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![inline_block.to_string(), block.to_string(), root_block.to_string()]).unwrap();
        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_from = SubSelection::from(inline_block_id.clone(), 6, None);
        let sub_selection_to = SubSelection::from(inline_block_id.clone(), 11, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::AddMarkStep(add_mark_step) => {
                assert_eq!(add_mark_step.block_id, inline_block_id);
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
            step => return Err(StepError(format!("Expected RemoveMarkStep. Got: {:?}", step)))
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
            "parent": paragraph_block_id2.clone()
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
            "parent": paragraph_block_id2.clone()
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
            step => return Err(StepError(format!("Expected RemoveMarkStep. Got: {:?}", step)))
        };
        return Ok(())
    }

}