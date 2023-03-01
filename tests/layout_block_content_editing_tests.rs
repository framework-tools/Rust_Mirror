#[cfg(test)]
mod tests {
    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap}, steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps}, steps_actualisor::actualise_steps, custom_copy::CustomCopy};
    use serde_json::json;

    /// <LayoutBlock> <Column><p1/><p2/></Column> <Column><p1/><p2/></Column> </LayoutBlock>
    #[test]
    fn can_handle_slash_scrim_add_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1 = "Inline1".to_string();
        let inline_block_id2 = "Inline2".to_string();
        let inline_block_id3 = "Inline3".to_string();
        let inline_block_id4 = "Inline4".to_string();
        let paragraph_block_id1 = "Paragraph1".to_string();
        let paragraph_block_id2 = "Paragraph2".to_string();
        let paragraph_block_id3 = "Paragraph3".to_string();
        let paragraph_block_id4 = "Paragraph4".to_string();
        let layout_block_id = "layout".to_string();
        let layout_column_id1 = "column1".to_string();
        let layout_column_id2 = "column2".to_string();
        let root_block_id = "Root".to_string();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "hello"
            },
            "marks": [],
            "parent": paragraph_block_id1.clone()
        }).to_string();
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "b"
            },
            "marks": [],
            "parent": paragraph_block_id2.clone()
        }).to_string();
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id3.clone()
        }).to_string();
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id4.clone()
        }).to_string();
        let paragraph1 = json!({
            "_id": paragraph_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id1.clone()
        }).to_string();
        let paragraph2 = json!({
            "_id": paragraph_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id1.clone()
        }).to_string();
        let paragraph3 = json!({
            "_id": paragraph_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id2.clone()
        }).to_string();
        let paragraph4 = json!({
            "_id": paragraph_block_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id2.clone()
        }).to_string();
        let layout_block = json!({
            "_id": layout_block_id,
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "children": [layout_column_id1.clone(), layout_column_id2.clone()],
            "marks": [],
            "parent": root_block_id.clone().to_string()
        }).to_string();
        let layout_column1 = json!({
            "_id": layout_column_id1.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "children": [paragraph_block_id1.clone(), paragraph_block_id2.clone()],
            "marks": [],
            "parent": layout_block_id.clone()
        }).to_string();
        let layout_column2 = json!({
            "_id": layout_column_id2.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "children": [paragraph_block_id3.clone(), paragraph_block_id4.clone()],
            "marks": [],
            "parent": layout_block_id.clone()
        }).to_string();

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![layout_block_id.clone()]).to_string();

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, inline_block3, inline_block4, paragraph1, paragraph2, paragraph3, paragraph4,
            layout_block, layout_column1, layout_column2, root_block
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let anchor = SubSelection {
            block_id: layout_column_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: paragraph_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection { block_id: inline_block_id1.clone(), offset: 2, subselection: None })),
            }))
        };
        let head = SubSelection {
            block_id: layout_column_id2.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: paragraph_block_id3.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection { block_id: inline_block_id3.clone(), offset: 2, subselection: None })),
            }))
        };

        let selection = Selection::from(anchor, head);

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_column1 = updated_state.block_map.get_standard_block(&layout_column_id1).unwrap();
        assert_eq!(updated_column1.children, vec![paragraph_block_id1.clone(), paragraph_block_id4.clone()]);

        let updated_column2 = updated_state.block_map.get_standard_block(&layout_column_id2).unwrap();
        assert_eq!(updated_column2.children.len(), 0);

        let updated_inline1 = updated_state.block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline1.text().unwrap().clone().to_string(), "Heodbye".to_string());
    }
}