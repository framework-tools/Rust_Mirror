#[cfg(test)]
mod tests {
    use std::alloc::Layout;

    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap, standard_blocks::StandardBlockType}, steps_generator::{event::{Event, DropBlockEvent, Side}, selection::{SubSelection, Selection}, generate_steps}, steps_actualisor::actualise_steps, custom_copy::CustomCopy};
    use serde_json::json;


    #[test]
    fn can_drop_block_below() {
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
            "children": [],
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
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::DropBlock(DropBlockEvent {
            drag_block_id: paragraph_block_id1.clone(),
            drop_block_id: paragraph_block_id2.clone(),
            side_dropped: Side::Bottom
        });
        // selection does not matter for this event
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children, vec![paragraph_block_id2.clone(), paragraph_block_id1.clone()]);
    }

    #[test]
    fn can_drop_block_on_side() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = "root_block_id".to_string();
        let paragraph_block_id1 = "1".to_string();
        let paragraph_block_id2 = "2".to_string();
        let inline_block_id1 = "inline1".to_string();
        let inline_block_id2 = "inline2".to_string();

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
            "children": [],
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
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string()
        ]).unwrap();
        let event = Event::DropBlock(DropBlockEvent {
            drag_block_id: paragraph_block_id1.clone(),
            drop_block_id: paragraph_block_id2.clone(),
            side_dropped: Side::Left
        });
        // selection does not matter for this event
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children.len(), 1);
        let layout_block = updated_state.block_map.get_standard_block(&updated_root_block.children[0]).unwrap();
        match layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.blocks, vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);
                assert_eq!(layout_block.horizontal, true);
            },
            _ => panic!("Expected layout block")
        };

    }
}
