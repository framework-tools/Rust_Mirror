#[cfg(test)]
mod tests {
    use core::panic;

    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap, standard_blocks::{StandardBlockType, content_block::ContentBlock, list_block::ListBlock}},
    steps_generator::{event::{Event, SlashScrimEvent}, selection::{SubSelection, Selection}, generate_steps}, step::{Step, ReplaceSlice}, steps_actualisor::actualise_steps, custom_copy::CustomCopy};
    use serde_json::json;

    #[test]
        fn can_duplicate_single_std_block() {
            let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

            let inline_block_id1 = "Inline1".to_string();
            let inline_block_id2 = "Inline2".to_string();
            let paragraph_block_id1 = "1".to_string();
            let paragraph_block_id2 = "2".to_string();
            let root_block_id = "Root".to_string();

            let inline_block1 = json!({
                "_id": inline_block_id1.clone(),
                "kind": "inline",
                "_type": "text",
                "content": {
                    "text": "Hello world /para fdsafds"
                },
                "marks": [],
                "parent": paragraph_block_id1.clone()
            }).to_string();
            let block1 = json!({
                "_id": paragraph_block_id1,
                "kind": "standard",
                "_type": "paragraph",
                "content": {
                    "inline_blocks": [inline_block_id1.clone()]
                },
                "children": [],
                "marks": [],
                "parent": root_block_id.clone().to_string()
            }).to_string();
            let inline_block2 = json!({
                "_id": inline_block_id2.clone(),
                "kind": "inline",
                "_type": "text",
                "content": {
                    "text": "Hello world /para fdsafds"
                },
                "marks": [],
                "parent": paragraph_block_id2.clone()
            }).to_string();
            let block2 = json!({
                "_id": paragraph_block_id2,
                "kind": "standard",
                "_type": "paragraph",
                "content": {
                    "inline_blocks": [inline_block_id2.clone()]
                },
                "children": [],
                "marks": [],
                "parent": root_block_id.clone().to_string()
            }).to_string();
            let root_block = RootBlock::json_from(root_block_id.clone(), vec![
                paragraph_block_id1.clone(),
                paragraph_block_id2.clone(),
            ]).to_string();

            let block_map = BlockMap::from(vec![
                inline_block1, block1,
                inline_block2, block2,
                root_block
            ]).unwrap();
            let event = Event::Duplicate(paragraph_block_id1.clone());
            let sub_selection = SubSelection::from(inline_block_id1.clone().clone(), 0, None);
            let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

            let steps = generate_steps(&event, &block_map, selection).unwrap();
            let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();
            let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();

            assert_eq!(updated_root_block.children.len(), 3);
            let new_block = updated_state.block_map.get_standard_block(&updated_root_block.children[1]).unwrap();
            match new_block.content {
                StandardBlockType::Paragraph(content) => {
                    assert_eq!(content.inline_blocks.len(), 1);
                    let new_inline = updated_state.block_map.get_inline_block(&content.inline_blocks[0]).unwrap();
                    assert_eq!(new_inline.text().unwrap().clone().to_string(), "Hello world /para fdsafds".to_string());
                },
                _ => panic!("Should be paragraph")
            }
        }

        #[test]
        fn can_duplicate_parent_block() {
            let mut new_ids = NewIds::hardcoded_new_ids_for_tests();
            let inline_block_id1 = "Inline1".to_string();
            let inline_block_id2 = "Inline2".to_string();
            let inline_block_id3 = "Inline3".to_string();
            let paragraph_block_id1 = "1".to_string();
            let paragraph_block_id2 = "2".to_string();
            let paragraph_block_id3 = "3".to_string();
            let root_block_id = "Root".to_string();

            let inline_block1 = json!({
                "_id": inline_block_id1.clone(),
                "kind": "inline",
                "_type": "text",
                "content": {
                    "text": "Hello world /para fdsafds"
                },
                "marks": [],
                "parent": paragraph_block_id1.clone()
            }).to_string();
            let block1 = json!({
                "_id": paragraph_block_id1,
                "kind": "standard",
                "_type": "paragraph",
                "content": {
                    "inline_blocks": [inline_block_id1.clone()]
                },
                "children": [paragraph_block_id2],
                "marks": [],
                "parent": root_block_id.clone().to_string()
            }).to_string();
            let inline_block2 = json!({
                "_id": inline_block_id2.clone(),
                "kind": "inline",
                "_type": "text",
                "content": {
                    "text": "Hello world /para fdsafds"
                },
                "marks": [],
                "parent": paragraph_block_id2.clone()
            }).to_string();
            let block2 = json!({
                "_id": paragraph_block_id2,
                "kind": "standard",
                "_type": "paragraph",
                "content": {
                    "inline_blocks": [inline_block_id2.clone()]
                },
                "children": [],
                "marks": [],
                "parent": paragraph_block_id1.clone().to_string()
            }).to_string();
            let inline_block3 = json!({
                "_id": inline_block_id3.clone(),
                "kind": "inline",
                "_type": "text",
                "content": {
                    "text": "Hello world /para fdsafds"
                },
                "marks": [],
                "parent": paragraph_block_id3.clone()
            }).to_string();
            let block3 = json!({
                "_id": paragraph_block_id3,
                "kind": "standard",
                "_type": "paragraph",
                "content": {
                    "inline_blocks": [inline_block_id3.clone()]
                },
                "children": [],
                "marks": [],
                "parent": root_block_id.clone().to_string()
            }).to_string();
            let root_block = RootBlock::json_from(root_block_id.clone(), vec![
                paragraph_block_id1.clone(), 
                paragraph_block_id3.clone()
            ]).to_string();

            let block_map = BlockMap::from(vec![
                inline_block1, block1,
                inline_block2, block2,
                inline_block3, block3,
                root_block
            ]).unwrap();
            let event = Event::Duplicate(paragraph_block_id1.clone());
            let sub_selection = SubSelection::from(inline_block_id1.clone().clone(), 0, None);
            let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

            let steps = generate_steps(&event, &block_map, selection).unwrap();
            let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();
            let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();

            assert_eq!(updated_root_block.children.len(), 3);
            let new_block = updated_state.block_map.get_standard_block(&updated_root_block.children[1]).unwrap();
            match new_block.content {
                StandardBlockType::Paragraph(content) => {
                    assert_eq!(content.inline_blocks.len(), 1);
                    let new_inline = updated_state.block_map.get_inline_block(&content.inline_blocks[0]).unwrap();
                    assert_eq!(new_inline.text().unwrap().clone().to_string(), "Hello world /para fdsafds".to_string());
                },
                _ => panic!("Should be paragraph")
            }

            assert_eq!(new_block.children.len(), 1);
            let new_block_child = updated_state.block_map.get_standard_block(&new_block.children[0]).unwrap();
            match new_block_child.content {
                StandardBlockType::Paragraph(content) => {
                    assert_eq!(content.inline_blocks.len(), 1);
                    let new_inline = updated_state.block_map.get_inline_block(&content.inline_blocks[0]).unwrap();
                    assert_eq!(new_inline.text().unwrap().clone().to_string(), "Hello world /para fdsafds".to_string());
                },
                _ => panic!("Should be paragraph")
            }
            println!("{:#?}", updated_state.block_map);
        }

        #[test]
        fn can_duplicate_last_child_block() {
            let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

            let inline_block_id1 = "Inline1".to_string();
            let inline_block_id2 = "Inline2".to_string();
            let paragraph_block_id1 = "1".to_string();
            let paragraph_block_id2 = "2".to_string();
            let root_block_id = "Root".to_string();

            let inline_block1 = json!({
                "_id": inline_block_id1.clone(),
                "kind": "inline",
                "_type": "text",
                "content": {
                    "text": "Hello world /para fdsafds"
                },
                "marks": [],
                "parent": paragraph_block_id1.clone()
            }).to_string();
            let block1 = json!({
                "_id": paragraph_block_id1,
                "kind": "standard",
                "_type": "paragraph",
                "content": {
                    "inline_blocks": [inline_block_id1.clone()]
                },
                "children": [paragraph_block_id2],
                "marks": [],
                "parent": root_block_id.clone().to_string()
            }).to_string();
            let inline_block2 = json!({
                "_id": inline_block_id2.clone(),
                "kind": "inline",
                "_type": "text",
                "content": {
                    "text": "Hello world /para fdsafds"
                },
                "marks": [],
                "parent": paragraph_block_id2.clone()
            }).to_string();
            let block2 = json!({
                "_id": paragraph_block_id2,
                "kind": "standard",
                "_type": "paragraph",
                "content": {
                    "inline_blocks": [inline_block_id2.clone()]
                },
                "children": [],
                "marks": [],
                "parent": paragraph_block_id1.clone().to_string()
            }).to_string();
            let root_block = RootBlock::json_from(root_block_id.clone(), vec![
                paragraph_block_id1.clone()
            ]).to_string();

            let block_map = BlockMap::from(vec![
                inline_block1, block1,
                inline_block2, block2,
                root_block
            ]).unwrap();
            let event = Event::Duplicate(paragraph_block_id2.clone());
            let sub_selection = SubSelection::from(inline_block_id2.clone().clone(), 0, None);
            let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

            let steps = generate_steps(&event, &block_map, selection).unwrap();
            let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();
            let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();

            assert_eq!(updated_root_block.children.len(), 1);
            let new_block = updated_state.block_map.get_standard_block(&updated_root_block.children[0]).unwrap();
            assert_eq!(new_block.children.len(), 2);
            match new_block.content {
                StandardBlockType::Paragraph(content) => {
                    assert_eq!(content.inline_blocks.len(), 1);
                    let new_inline = updated_state.block_map.get_inline_block(&content.inline_blocks[0]).unwrap();
                    assert_eq!(new_inline.text().unwrap().clone().to_string(), "Hello world /para fdsafds".to_string());
                },
                _ => panic!("Should be paragraph")
            }
            
        }
    }
    