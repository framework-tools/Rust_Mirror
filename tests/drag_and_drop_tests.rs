#[cfg(test)]
mod tests {
    use std::{vec};

    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap, standard_blocks::{StandardBlockType, layout_block}}, steps_generator::{event::{Event, DropBlockEvent, Side}, selection::{SubSelection, Selection}, generate_steps}, steps_actualisor::actualise_steps, custom_copy::CustomCopy};
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
    fn can_drop_block_on_side_with_no_layout_block() {
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
        match &layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, true);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(layout_block.children.len(), 2);

        let vertical_layout_block1 = updated_state.block_map.get_standard_block(&layout_block.children[0]).unwrap();
        match &vertical_layout_block1.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, false);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(vertical_layout_block1.children, vec![paragraph_block_id1]);

        let vertical_layout_block2 = updated_state.block_map.get_standard_block(&layout_block.children[1]).unwrap();
        match &vertical_layout_block2.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, false);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(vertical_layout_block2.children, vec![paragraph_block_id2]);
    }

    #[test]
    fn can_drop_block_on_left_side_of_horizontal_layout_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = "root_block_id".to_string();
        let paragraph_block_id1 = "1".to_string();
        let paragraph_block_id2 = "2".to_string();
        let paragraph_block_id3 = "3".to_string();
        let inline_block_id1 = "inline1".to_string();
        let inline_block_id2 = "inline2".to_string();
        let layout_block_id1 = "layout1".to_string();
        let layout_block_id2 = "layout2".to_string();
        let layout_block_id3 = "layout3".to_string();

        let horizontal_layout_block = json!({
            "_id": layout_block_id1.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "marks": [],
            "children": [layout_block_id2.clone(), layout_block_id3.clone()],
            "parent": root_block_id.clone()
        });
        let vertical_layout_block1 = json!({
            "_id": layout_block_id2.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id1.clone()],
            "parent": layout_block_id1.clone()
        });
        let vertical_layout_block2 = json!({
            "_id": layout_block_id3.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id2.clone()],
            "parent": layout_block_id1.clone()
        });

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
            "parent": layout_block_id2.clone()
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
            "parent": layout_block_id3.clone()
        });

        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            layout_block_id1.clone(), paragraph_block_id3.clone()
        ]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(),
            horizontal_layout_block.to_string(), paragraph_block3.to_string(), vertical_layout_block1.to_string(), vertical_layout_block2.to_string()
        ]).unwrap();

        let event = Event::DropBlock(DropBlockEvent {
            drag_block_id: paragraph_block_id3.clone(),
            drop_block_id: layout_block_id1.clone(),
            side_dropped: Side::Left
        });
        // selection does not matter for this event
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children.len(), 1);
        let horizontal_layout_block = updated_state.block_map.get_standard_block(&updated_root_block.children[0]).unwrap();
        match horizontal_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, true);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(horizontal_layout_block.children.len(), 3);

        let new_vertical_layout_block = updated_state.block_map.
            get_standard_block(&horizontal_layout_block.children[0]).unwrap();
        match new_vertical_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, false);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(new_vertical_layout_block.children, vec![paragraph_block_id3]);
    }

    #[test]
    fn can_drop_block_on_right_side_of_horizontal_layout_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = "root_block_id".to_string();
        let paragraph_block_id1 = "1".to_string();
        let paragraph_block_id2 = "2".to_string();
        let paragraph_block_id3 = "3".to_string();
        let inline_block_id1 = "inline1".to_string();
        let inline_block_id2 = "inline2".to_string();
        let layout_block_id1 = "layout1".to_string();
        let layout_block_id2 = "layout2".to_string();
        let layout_block_id3 = "layout3".to_string();

        let horizontal_layout_block = json!({
            "_id": layout_block_id1.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "marks": [],
            "children": [layout_block_id2.clone(), layout_block_id3.clone()],
            "parent": root_block_id.clone()
        });
        let vertical_layout_block1 = json!({
            "_id": layout_block_id2.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id1.clone()],
            "parent": layout_block_id1.clone()
        });
        let vertical_layout_block2 = json!({
            "_id": layout_block_id3.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id2.clone()],
            "parent": layout_block_id1.clone()
        });

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
            "parent": layout_block_id2.clone()
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
            "parent": layout_block_id3.clone()
        });

        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            layout_block_id1.clone(), paragraph_block_id3.clone()
        ]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(),
            horizontal_layout_block.to_string(), paragraph_block3.to_string(), vertical_layout_block1.to_string(), vertical_layout_block2.to_string()
        ]).unwrap();

        let event = Event::DropBlock(DropBlockEvent {
            drag_block_id: paragraph_block_id3.clone(),
            drop_block_id: layout_block_id1.clone(),
            side_dropped: Side::Right
        });
        // selection does not matter for this event
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children.len(), 1);
        let horizontal_layout_block = updated_state.block_map.get_standard_block(&updated_root_block.children[0]).unwrap();
        match horizontal_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, true);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(horizontal_layout_block.children.len(), 3);

        let new_vertical_layout_block = updated_state.block_map.
            get_standard_block(&horizontal_layout_block.children[2]).unwrap();
        match new_vertical_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, false);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(new_vertical_layout_block.children, vec![paragraph_block_id3]);
    }

    #[test]
    fn can_drop_block_on_left_side_of_vertical_layout_column_inside_horizontal_layout_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = "root_block_id".to_string();
        let paragraph_block_id1 = "1".to_string();
        let paragraph_block_id2 = "2".to_string();
        let paragraph_block_id3 = "3".to_string();
        let inline_block_id1 = "inline1".to_string();
        let inline_block_id2 = "inline2".to_string();
        let layout_block_id1 = "layout1".to_string();
        let layout_block_id2 = "layout2".to_string();
        let layout_block_id3 = "layout3".to_string();

        let horizontal_layout_block = json!({
            "_id": layout_block_id1.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "marks": [],
            "children": [layout_block_id2.clone(), layout_block_id3.clone()],
            "parent": root_block_id.clone()
        });
        let vertical_layout_block1 = json!({
            "_id": layout_block_id2.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id1.clone()],
            "parent": layout_block_id1.clone()
        });
        let vertical_layout_block2 = json!({
            "_id": layout_block_id3.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id2.clone()],
            "parent": layout_block_id1.clone()
        });

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
            "parent": layout_block_id2.clone()
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
            "parent": layout_block_id3.clone()
        });

        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            layout_block_id1.clone(), paragraph_block_id3.clone()
        ]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(),
            horizontal_layout_block.to_string(), paragraph_block3.to_string(), vertical_layout_block1.to_string(), vertical_layout_block2.to_string()
        ]).unwrap();

        let event = Event::DropBlock(DropBlockEvent {
            drag_block_id: paragraph_block_id3.clone(),
            drop_block_id: layout_block_id2.clone(),
            side_dropped: Side::Left
        });
        // selection does not matter for this event
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children.len(), 1);
        let horizontal_layout_block = updated_state.block_map.get_standard_block(&updated_root_block.children[0]).unwrap();
        match horizontal_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, true);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(horizontal_layout_block.children.len(), 3);

        let new_vertical_layout_block = updated_state.block_map.
            get_standard_block(&horizontal_layout_block.children[0]).unwrap();
        match new_vertical_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, false);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(new_vertical_layout_block.children, vec![paragraph_block_id3]);
    }
    #[test]
    fn can_drop_block_on_right_side_of_vertical_layout_column_inside_horizontal_layout_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = "root_block_id".to_string();
        let paragraph_block_id1 = "1".to_string();
        let paragraph_block_id2 = "2".to_string();
        let paragraph_block_id3 = "3".to_string();
        let inline_block_id1 = "inline1".to_string();
        let inline_block_id2 = "inline2".to_string();
        let layout_block_id1 = "layout1".to_string();
        let layout_block_id2 = "layout2".to_string();
        let layout_block_id3 = "layout3".to_string();

        let horizontal_layout_block = json!({
            "_id": layout_block_id1.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "marks": [],
            "children": [layout_block_id2.clone(), layout_block_id3.clone()],
            "parent": root_block_id.clone()
        });
        let vertical_layout_block1 = json!({
            "_id": layout_block_id2.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id1.clone()],
            "parent": layout_block_id1.clone()
        });
        let vertical_layout_block2 = json!({
            "_id": layout_block_id3.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id2.clone()],
            "parent": layout_block_id1.clone()
        });

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
            "parent": layout_block_id2.clone()
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
            "parent": layout_block_id3.clone()
        });

        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            layout_block_id1.clone(), paragraph_block_id3.clone()
        ]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(),
            horizontal_layout_block.to_string(), paragraph_block3.to_string(), vertical_layout_block1.to_string(), vertical_layout_block2.to_string()
        ]).unwrap();

        let event = Event::DropBlock(DropBlockEvent {
            drag_block_id: paragraph_block_id3.clone(),
            drop_block_id: layout_block_id2.clone(),
            side_dropped: Side::Right
        });
        // selection does not matter for this event
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children.len(), 1);
        let horizontal_layout_block = updated_state.block_map.get_standard_block(&updated_root_block.children[0]).unwrap();
        match horizontal_layout_block.content {
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, true);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(horizontal_layout_block.children.len(), 3);

        let new_vertical_layout_block = updated_state.block_map.
            get_standard_block(&horizontal_layout_block.children[1]).unwrap();
        match new_vertical_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, false);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(new_vertical_layout_block.children, vec![paragraph_block_id3]);
    }
    #[test]
    fn can_drop_block_on_left_side_of_std_block_inside_layout_column() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = "root_block_id".to_string();
        let paragraph_block_id1 = "1".to_string();
        let paragraph_block_id2 = "2".to_string();
        let paragraph_block_id3 = "3".to_string();
        let inline_block_id1 = "inline1".to_string();
        let inline_block_id2 = "inline2".to_string();
        let layout_block_id1 = "layout1".to_string();
        let layout_block_id2 = "layout2".to_string();
        let layout_block_id3 = "layout3".to_string();

        let horizontal_layout_block = json!({
            "_id": layout_block_id1.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "marks": [],
            "children": [layout_block_id2.clone(), layout_block_id3.clone()],
            "parent": root_block_id.clone()
        });
        let vertical_layout_block1 = json!({
            "_id": layout_block_id2.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id1.clone()],
            "parent": layout_block_id1.clone()
        });
        let vertical_layout_block2 = json!({
            "_id": layout_block_id3.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id2.clone()],
            "parent": layout_block_id1.clone()
        });

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
            "parent": layout_block_id2.clone()
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
            "parent": layout_block_id3.clone()
        });

        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            layout_block_id1.clone(), paragraph_block_id3.clone()
        ]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(),
            horizontal_layout_block.to_string(), paragraph_block3.to_string(), vertical_layout_block1.to_string(), vertical_layout_block2.to_string()
        ]).unwrap();

        let event = Event::DropBlock(DropBlockEvent {
            drag_block_id: paragraph_block_id3.clone(),
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
        let horizontal_layout_block = updated_state.block_map.get_standard_block(&updated_root_block.children[0]).unwrap();
        match horizontal_layout_block.content {
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, true);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(horizontal_layout_block.children.len(), 3);

        let new_vertical_layout_block = updated_state.block_map.
            get_standard_block(&horizontal_layout_block.children[1]).unwrap();
        match new_vertical_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, false);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(new_vertical_layout_block.children, vec![paragraph_block_id3]);
    }
    #[test]
    fn can_drop_block_on_right_side_of_std_block_inside_layout_column() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = "root_block_id".to_string();
        let paragraph_block_id1 = "1".to_string();
        let paragraph_block_id2 = "2".to_string();
        let paragraph_block_id3 = "3".to_string();
        let inline_block_id1 = "inline1".to_string();
        let inline_block_id2 = "inline2".to_string();
        let layout_block_id1 = "layout1".to_string();
        let layout_block_id2 = "layout2".to_string();
        let layout_block_id3 = "layout3".to_string();

        let horizontal_layout_block = json!({
            "_id": layout_block_id1.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "marks": [],
            "children": [layout_block_id2.clone(), layout_block_id3.clone()],
            "parent": root_block_id.clone()
        });
        let vertical_layout_block1 = json!({
            "_id": layout_block_id2.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id1.clone()],
            "parent": layout_block_id1.clone()
        });
        let vertical_layout_block2 = json!({
            "_id": layout_block_id3.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "marks": [],
            "children": [paragraph_block_id2.clone()],
            "parent": layout_block_id1.clone()
        });

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
            "parent": layout_block_id2.clone()
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
            "parent": layout_block_id3.clone()
        });

        let paragraph_block3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            layout_block_id1.clone(), paragraph_block_id3.clone()
        ]);
        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), paragraph_block1.to_string(), paragraph_block2.to_string(), root_block.to_string(),
            horizontal_layout_block.to_string(), paragraph_block3.to_string(), vertical_layout_block1.to_string(), vertical_layout_block2.to_string()
        ]).unwrap();

        let event = Event::DropBlock(DropBlockEvent {
            drag_block_id: paragraph_block_id3.clone(),
            drop_block_id: paragraph_block_id2.clone(),
            side_dropped: Side::Right
        });
        // selection does not matter for this event
        let sub_selection = SubSelection::from(inline_block_id2.clone(), 4, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
        assert_eq!(updated_root_block.children.len(), 1);
        let horizontal_layout_block = updated_state.block_map.get_standard_block(&updated_root_block.children[0]).unwrap();
        match horizontal_layout_block.content {
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, true);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(horizontal_layout_block.children.len(), 3);

        let new_vertical_layout_block = updated_state.block_map.
            get_standard_block(&horizontal_layout_block.children[2]).unwrap();
        match new_vertical_layout_block.content{
            StandardBlockType::Layout(layout_block) => {
                assert_eq!(layout_block.horizontal, false);
            },
            _ => panic!("Expected layout block")
        };
        assert_eq!(new_vertical_layout_block.children, vec![paragraph_block_id3]);
    }
}
