#[cfg(test)]
mod tests {
    use rust_mirror::{steps_generator::{StepError, event::{Event, FormatBarEvent, ContextMenuEvent}, selection::{SubSelection, Selection}, generate_steps}, new_ids::NewIds, blocks::{RootBlock, BlockMap, standard_blocks::{StandardBlockType, content_block::ContentBlock}}, step::Step, steps_actualisor::actualise_steps, custom_copy::CustomCopy};
    use serde_json::json;

    #[test]
    fn can_generate_copy_steps() -> Result<(), StepError> {
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
        let event = Event::ContextMenu(ContextMenuEvent::Copy);
        let sub_selection_from = SubSelection::from(inline_block_id.clone(), 6, None);
        let sub_selection_to = SubSelection::from(inline_block_id.clone(), 11, None);
        let selection = Selection::from(sub_selection_from, sub_selection_to);

        let steps = generate_steps(&event, &block_map, selection.clone()).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::Copy(from, to) => {
                assert_eq!(from, &selection.anchor);
                assert_eq!(to, &selection.head);
            },
            step => return Err(StepError(format!("Expected Copy Step. Got: {:?}", step)))
        };

        return Ok(())
    }
    #[test]
    fn can_actualise_copy_in_single_inline_block() -> Result<(), StepError> {
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
        let event = Event::ContextMenu(ContextMenuEvent::Copy);
        let sub_selection_from = SubSelection::from(inline_block_id.clone(), 6, None);
        let sub_selection_to = SubSelection::from(inline_block_id.clone(), 11, None);
        let selection = Selection::from(sub_selection_from, sub_selection_to);

        let steps = generate_steps(&event, &block_map, selection.clone()).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new())?;

        return Ok(())
    }
}