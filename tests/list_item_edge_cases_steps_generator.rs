#[cfg(test)]
mod tests {
    use rust_mirror::blocks::standard_blocks::StandardBlockType;
    use rust_mirror::blocks::standard_blocks::content_block::ContentBlock;
    use rust_mirror::step::TurnInto;
    use rust_mirror::steps_generator::StepError;
    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap}, steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps}, step::Step};
    use serde_json::json;

    #[test]
    fn enter_in_empty_list_block_should_turn_into_paragraph() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let list_block_id = new_ids.get_id().unwrap();
        let inline_block_id = new_ids.get_id().unwrap();

        let inline_block = json!({
            "_id": inline_block_id.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": list_block_id.clone()
        });
        let list_block = json!({
            "_id": list_block_id.clone(),
            "kind": "standard",
            "_type": "to-do list",
            "content": {
                "inline_blocks": [inline_block_id.clone()],
                "completed": false
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![list_block_id.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block.to_string(), list_block.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Enter, None));
        let sub_selection = SubSelection::from(inline_block_id.clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::TurnInto(TurnInto { block_id, new_block_type  }) => {
                assert_eq!(block_id, &list_block_id);
                assert_eq!(new_block_type, &StandardBlockType::Paragraph(ContentBlock::new(vec![])));
            },
            s => return Err(StepError(format!("Expected turn into step. Got: {:#?}", s)))
        };
        return Ok(())
    }
    #[test]
    fn backspace_in_empty_list_block_should_turn_into_paragraph() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let list_block_id = new_ids.get_id().unwrap();
        let inline_block_id = new_ids.get_id().unwrap();

        let inline_block = json!({
            "_id": inline_block_id.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": ""
            },
            "marks": [],
            "parent": list_block_id.clone()
        });
        let list_block = json!({
            "_id": list_block_id.clone(),
            "kind": "standard",
            "_type": "to-do list",
            "content": {
                "inline_blocks": [inline_block_id.clone()],
                "completed": false
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![list_block_id.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block.to_string(), list_block.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let sub_selection = SubSelection::from(inline_block_id.clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::TurnInto(TurnInto { block_id, new_block_type  }) => {
                assert_eq!(block_id, &list_block_id);
                assert_eq!(new_block_type, &StandardBlockType::Paragraph(ContentBlock::new(vec![])));
            },
            s => return Err(StepError(format!("Expected turn into step. Got: {:#?}", s)))
        };
        return Ok(())
    }
}