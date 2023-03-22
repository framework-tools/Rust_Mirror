#[cfg(test)]
mod tests {
    use core::panic;

    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap, standard_blocks::{StandardBlockType, content_block::ContentBlock, list_block::ListBlock}},
    steps_generator::{event::{Event, SlashScrimEvent}, selection::{SubSelection, Selection}, generate_steps}, step::{Step, ReplaceSlice}, steps_actualisor::actualise_steps, custom_copy::CustomCopy};
    use serde_json::json;

    #[test]
        fn can_duplicate() {
            let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

            let inline_block_id = "Inline".to_string();
            let paragraph_block_id = "Paragraph".to_string();
            let root_block_id = "Root".to_string();

            let inline_block = json!({
                "_id": inline_block_id.clone(),
                "kind": "inline",
                "_type": "text",
                "content": {
                    "text": "Hello world /para fdsafds"
                },
                "marks": [],
                "parent": paragraph_block_id.clone()
            }).to_string();
            let block = json!({
                "_id": paragraph_block_id,
                "kind": "standard",
                "_type": "paragraph",
                "content": {
                    "inline_blocks": [inline_block_id.clone()]
                },
                "children": [],
                "marks": [],
                "parent": root_block_id.clone().to_string()
            }).to_string();
            let root_block = RootBlock::json_from(root_block_id.clone(), vec![paragraph_block_id.clone()]).to_string();

            let block_map = BlockMap::from(vec![inline_block, block, root_block]).unwrap();
            let event = Event::Duplicate(paragraph_block_id.clone());
            let sub_selection = SubSelection::from(inline_block_id.clone().clone(), 17, None);
            let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

            let steps = generate_steps(&event, &block_map, selection).unwrap();
            let updated_state = actualise_steps(steps, block_map, &mut new_ids, CustomCopy::new()).unwrap();
            let updated_root_block = updated_state.block_map.get_root_block(&root_block_id).unwrap();
            assert_eq!(updated_root_block.children.len(), 2);
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
    }