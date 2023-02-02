#[cfg(test)]
mod tests {
    use std::{vec};

    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap},
        steps_generator::{event::{Event, ReplaceWithChildrenEvent},
        selection::{SubSelection, Selection}, generate_steps}, steps_actualisor::actualise_steps, custom_copy::CustomCopy
    };
    use serde_json::json;

    #[test]
    fn can_replace_block_with_children() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id1 = "1".to_string();
        let paragraph_block_id2 = "2".to_string();
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
            "parent": paragraph_block_id1.clone()
        });
        let paragraph_block1 = json!({
            "_id": paragraph_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [paragraph_block_id2.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        });
        let paragraph_block2 = json!({
            "_id": paragraph_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": paragraph_block_id1.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::ReplaceWithChildren(ReplaceWithChildrenEvent { block_id: paragraph_block_id1.clone()});

        // selection does not matter for this event
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![paragraph_block_id2.clone()]);
    }
}