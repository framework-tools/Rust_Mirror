#[cfg(test)]
mod tests {
    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap}, steps_generator::{event::{Event, KeyPress, Key}, selection::{SubSelection, Selection}, generate_steps}, steps_actualisor::actualise_steps, custom_copy::CustomCopy};
    use serde_json::json;

    /// * = selection start or end
    /// Input:
    /// <LayoutBlock> <C1></p1*></p2></C1> <C2></p3*></p4></C2> <C3></p5></p6></C3> </LayoutBlock>
    /// Output:
    /// <LayoutBlock> <C1></p1></C1> <C2></p4></C2> <C3></p5></p6></C3> </LayoutBlock>
    #[test]
    fn can_handle_editing_selection_across_2_layout_columns_with_total_3_layout_columns() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1 = "Inline1".to_string();
        let inline_block_id2 = "Inline2".to_string();
        let inline_block_id3 = "Inline3".to_string();
        let inline_block_id4 = "Inline4".to_string();
        let inline_block_id5 = "Inline5".to_string();
        let inline_block_id6 = "Inline6".to_string();
        let paragraph_block_id1 = "Paragraph1".to_string();
        let paragraph_block_id2 = "Paragraph2".to_string();
        let paragraph_block_id3 = "Paragraph3".to_string();
        let paragraph_block_id4 = "Paragraph4".to_string();
        let paragraph_block_id5 = "Paragraph5".to_string();
        let paragraph_block_id6 = "Paragraph6".to_string();
        let layout_block_id = "layout".to_string();
        let layout_column_id1 = "column1".to_string();
        let layout_column_id2 = "column2".to_string();
        let layout_column_id3 = "column3".to_string();
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
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id5.clone()
        }).to_string();
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id6.clone()
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
        let paragraph5 = json!({
            "_id": paragraph_block_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id3.clone()
        }).to_string();
        let paragraph6 = json!({
            "_id": paragraph_block_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id3.clone()
        }).to_string();
        let layout_block = json!({
            "_id": layout_block_id,
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "children": [layout_column_id1.clone(), layout_column_id2.clone(), layout_column_id3.clone()],
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
        let layout_column3 = json!({
            "_id": layout_column_id3.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "children": [paragraph_block_id5.clone(), paragraph_block_id6.clone()],
            "marks": [],
            "parent": layout_block_id.clone()
        }).to_string();

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![layout_block_id.clone()]).to_string();

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, inline_block3, inline_block4, inline_block5, inline_block6,
            paragraph1, paragraph2, paragraph3, paragraph4, paragraph5, paragraph6,
            layout_block, layout_column1, layout_column2, layout_column3, root_block,
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
        assert_eq!(updated_column1.children, vec![paragraph_block_id1.clone()]);

        let updated_inline1 = updated_state.block_map.get_inline_block(&inline_block_id1).unwrap();
        assert_eq!(updated_inline1.text().unwrap().clone().to_string(), "heodbye".to_string());

        let updated_column2 = updated_state.block_map.get_standard_block(&layout_column_id2).unwrap();
        assert_eq!(updated_column2.children, vec![paragraph_block_id4.clone()]);
        let updated_column3 = updated_state.block_map.get_standard_block(&layout_column_id3).unwrap();
        assert_eq!(updated_column3.children, vec![paragraph_block_id5.clone(), paragraph_block_id6.clone()]);
    }

    /// * = selection start or end
    /// Input:
    /// <LayoutBlock> <C1><p1*/><p2/></C1> <C2><p3*/><p4/></C2> </LayoutBlock>
    /// Output:
    /// <LayoutBlock> <C1></p1></C1> <C2></p4></C2> </LayoutBlock>
    #[test]
    fn can_handle_editing_across_2_layout_columns_with_only_2_layout_columns() {
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
        assert_eq!(updated_column1.children, vec![paragraph_block_id1.clone()]);

        let updated_column2 = updated_state.block_map.get_standard_block(&layout_column_id2).unwrap();
        assert_eq!(updated_column2.children, vec![paragraph_block_id4.clone()]);

    }

    /// * = selection start or end
    /// Input:
    /// <LayoutBlock> <C1></p1></p2></C1> <C2></p3*></p4></C2> <C3></p5></p6></C3> <C4></p7*></p8></C4> </LayoutBlock>
    /// Output:
    /// <LayoutBlock> <C1></p1></p2></C1> <C2></p3></C2> <C3></C3> <C4><</p8></C4> </LayoutBlock>
    #[test]
    fn can_handle_editing_across_3_layout_columns_with_4_total_layout_columns() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id1 = "Inline1".to_string();
        let inline_block_id2 = "Inline2".to_string();
        let inline_block_id3 = "Inline3".to_string();
        let inline_block_id4 = "Inline4".to_string();
        let inline_block_id5 = "Inline5".to_string();
        let inline_block_id6 = "Inline6".to_string();
        let inline_block_id7 = "Inline7".to_string();
        let inline_block_id8 = "Inline8".to_string();
        let paragraph_block_id1 = "Paragraph1".to_string();
        let paragraph_block_id2 = "Paragraph2".to_string();
        let paragraph_block_id3 = "Paragraph3".to_string();
        let paragraph_block_id4 = "Paragraph4".to_string();
        let paragraph_block_id5 = "Paragraph5".to_string();
        let paragraph_block_id6 = "Paragraph6".to_string();
        let paragraph_block_id7 = "Paragraph7".to_string();
        let paragraph_block_id8 = "Paragraph8".to_string();
        let layout_block_id = "layout".to_string();
        let layout_column_id1 = "column1".to_string();
        let layout_column_id2 = "column2".to_string();
        let layout_column_id3 = "column3".to_string();
        let layout_column_id4 = "column4".to_string();
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
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id5.clone()
        }).to_string();
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id6.clone()
        }).to_string();
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id7.clone()
        }).to_string();
        let inline_block8 = json!({
            "_id": inline_block_id8.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "goodbye"
            },
            "marks": [],
            "parent": paragraph_block_id8.clone()
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
        let paragraph5 = json!({
            "_id": paragraph_block_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id3.clone()
        }).to_string();
        let paragraph6 = json!({
            "_id": paragraph_block_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id3.clone()
        }).to_string();
        let paragraph7 = json!({
            "_id": paragraph_block_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id4.clone()
        }).to_string();
        let paragraph8 = json!({
            "_id": paragraph_block_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id8.clone()]
            },
            "children": [],
            "marks": [],
            "parent": layout_column_id4.clone()
        }).to_string();
        let layout_block = json!({
            "_id": layout_block_id,
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": true
            },
            "children": [layout_column_id1.clone(), layout_column_id2.clone(), layout_column_id3.clone(), layout_column_id4.clone()],
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
        let layout_column3 = json!({
            "_id": layout_column_id3.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "children": [paragraph_block_id5.clone(), paragraph_block_id6.clone()],
            "marks": [],
            "parent": layout_block_id.clone()
        }).to_string();
        let layout_column4 = json!({
            "_id": layout_column_id4.clone(),
            "kind": "standard",
            "_type": "layout",
            "content": {
                "horizontal": false
            },
            "children": [paragraph_block_id7.clone(), paragraph_block_id8.clone()],
            "marks": [],
            "parent": layout_block_id.clone()
        }).to_string();

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![layout_block_id.clone()]).to_string();

        let block_map = BlockMap::from(vec![
            inline_block1, inline_block2, inline_block3, inline_block4, inline_block5, inline_block6, inline_block7, inline_block8,
            paragraph1, paragraph2, paragraph3, paragraph4, paragraph5, paragraph6, paragraph7, paragraph8,
            layout_block, layout_column1, layout_column2, layout_column3, layout_column4, root_block,
        ]).unwrap();
        let event = Event::KeyPress(KeyPress::new(Key::Backspace, None));
        let anchor = SubSelection {
            block_id: layout_column_id2.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: paragraph_block_id3.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection { block_id: inline_block_id3.clone(), offset: 1, subselection: None })),
            }))
        };
        let head = SubSelection {
            block_id: layout_column_id4.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: paragraph_block_id7.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection { block_id: inline_block_id7.clone(), offset: 2, subselection: None })),
            }))
        };

        let selection = Selection::from(anchor, head);

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();

        let updated_column1 = updated_state.block_map.get_standard_block(&layout_column_id1).unwrap();
        assert_eq!(updated_column1.children, vec![paragraph_block_id1.clone(), paragraph_block_id2.clone()]);


        let updated_column2 = updated_state.block_map.get_standard_block(&layout_column_id2).unwrap();
        assert_eq!(updated_column2.children, vec![paragraph_block_id3.clone()]);

        let updated_column3 = updated_state.block_map.get_standard_block(&layout_column_id3).unwrap();
        assert_eq!(updated_column3.children.len(), 0);

        let updated_column4 = updated_state.block_map.get_standard_block(&layout_column_id4).unwrap();
        assert_eq!(updated_column4.children, vec![paragraph_block_id8]);
    }

    // TODO:
    // layout block entirely selected
    // partial selection with external block above
    // partial selection with external block below
}