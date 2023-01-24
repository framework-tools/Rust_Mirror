#[cfg(test)]
mod tests {
    use rust_mirror::{new_ids::NewIds, steps_generator::{StepError, event::Event, selection::{SubSelection, Selection}, generate_steps}, blocks::{RootBlock, BlockMap}, steps_actualisor::actualise_steps, custom_copy::CustomCopy};
    use serde_json::json;

    #[test]
    fn can_delete_block() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let paragraph_block_id1 = new_ids.get_id()?;
        let inline_block_id1 = new_ids.get_id()?;
        let paragraph_block_id2 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });
        let block1 = json!({
            "_id": paragraph_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        });
        let block2 = json!({
            "_id": paragraph_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(),
            block1.to_string(),
            inline_block2.to_string(),
            block2.to_string(),
            root_block.to_string()
        ]).unwrap();
        let event = Event::DeleteBlock(paragraph_block_id2);
        // selection does not matter
        let sub_selection_from = SubSelection::from(inline_block_id1.clone(), 6, None);
        let sub_selection_to = SubSelection::from(inline_block_id1.clone(), 11, None);
        let selection = Selection::from(sub_selection_from, sub_selection_to);

        let steps = generate_steps(&event, &block_map, selection.clone()).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new())?;

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id)?;
        assert_eq!(updated_root_block.children, vec![paragraph_block_id1]);

        return Ok(())
    }
}