#[cfg(test)]
mod tests {
    use rust_mirror::{blocks::{standard_blocks::{StandardBlockType, content_block::ContentBlock}, RootBlock, BlockMap}, new_ids::NewIds, steps_generator::{StepError, event::{Event, FormatBarEvent}, selection::{SubSelection, Selection}, generate_steps}, step::Step, steps_actualisor::actualise_steps};
    use serde_json::json;

    #[test]
    fn can_handle_turn_into_steps() -> Result<(), StepError> {
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
        let event = Event::FormatBar(FormatBarEvent::TurnInto(StandardBlockType::H1(ContentBlock::new(vec![]))));
        let sub_selection_from = SubSelection::from(inline_block_id.clone(), 6, None);
        let sub_selection_to = SubSelection::from(inline_block_id.clone(), 11, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::TurnInto(turn_into_step) => {
                assert_eq!(turn_into_step.block_id, paragraph_block_id);
                assert_eq!(turn_into_step.new_block_type, StandardBlockType::H1(ContentBlock::new(vec![])));
            },
            step => return Err(StepError(format!("Expected Turn Into Step. Got: {:?}", step)))
        };

        return Ok(())
    }
    #[test]
    fn can_actualise_turn_into_steps() -> Result<(), StepError> {
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
        let event = Event::FormatBar(FormatBarEvent::TurnInto(StandardBlockType::H1(ContentBlock::new(vec![]))));
        let sub_selection_from = SubSelection::from(inline_block_id.clone(), 6, None);
        let sub_selection_to = SubSelection::from(inline_block_id.clone(), 11, None);
        let selection = Selection::from(sub_selection_from.clone(), sub_selection_to.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids)?;

        let updated_block = updated_state.block_map.get_standard_block(&paragraph_block_id)?;
        assert_eq!(updated_block.content, StandardBlockType::H1(ContentBlock { inline_blocks: vec![inline_block_id] }));

        return Ok(())
    }
}