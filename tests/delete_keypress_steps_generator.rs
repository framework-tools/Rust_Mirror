#[cfg(test)]
mod tests {
    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap}, steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps}, step::{Step, ReplaceSlice}};
    use serde_json::json;

    #[test]
    pub fn backspace_with_caret_selection_in_middle_of_two_inline_blocks() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello"
            },
            "marks": [],
            "parent": paragraph_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " World"
            },
            "marks": [],
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
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), block.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 0, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection, &mut new_ids).unwrap();

        assert_eq!(steps.len(), 1);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.block_id, paragraph_block_id);
                assert_eq!(replace_step.from, SubSelection::from(inline_block_id1.clone(), 4, None));
                assert_eq!(replace_step.to, SubSelection::from(inline_block_id1.clone(), 5, None));
                assert_eq!(replace_step.slice, ReplaceSlice::String("".to_string()));
            },
            _ => panic!("Expected ReplaceStep")
        }
    }
}