#[cfg(test)]
mod tests {
    use rust_mirror::{steps_generator::{StepError, event::{Event, FormatBarEvent}, selection::{SubSelection, Selection}, generate_steps}, blocks::{RootBlock, BlockMap}, steps_executor::execute_steps, mark::Mark};
    use mongodb::bson::oid::ObjectId;
    use serde_json::json;

    #[test]
    fn can_execute_apply_mark_with_simple_selection_within_one_inline() -> Result<(), StepError> {
        let root_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let inline_block_id = ObjectId::new();
        let inline_block = json!({
            "_id": inline_block_id.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![inline_block, block, root_block]).unwrap();
        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_anchor = SubSelection::from(inline_block_id, 1, None);
        let sub_selection_head = SubSelection::from(inline_block_id, 5, None);
        let selection = Selection::from(sub_selection_anchor.clone(), sub_selection_head.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_block_map = execute_steps(steps, block_map).unwrap();

        let updated_standard_block = updated_block_map.get_standard_block(&paragraph_block_id).unwrap();
        let content_block = updated_standard_block.content_block().unwrap();
        assert_eq!(content_block.inline_blocks.len(), 3);

        let inline_block1 = updated_block_map.get_inline_block(&content_block.inline_blocks[0]).unwrap();
        assert_eq!(inline_block1.text().unwrap(), "H");
        assert_eq!(inline_block1.marks.len(), 0);

        let inline_block2 = updated_block_map.get_inline_block(&content_block.inline_blocks[1]).unwrap();
        assert_eq!(inline_block2.text().unwrap(), "ello");
        assert_eq!(inline_block2.marks.len(), 1);
        assert_eq!(inline_block2.marks[0], Mark::Bold);

        let inline_block3 = updated_block_map.get_inline_block(&content_block.inline_blocks[2]).unwrap();
        assert_eq!(inline_block3.text().unwrap(), " World");
        assert_eq!(inline_block3.marks.len(), 0);
        return Ok(())
    }

    #[test]
    fn can_execute_remove_mark_selection_across_multiple_inline_blocks() -> Result<(), StepError> {
        let root_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let inline_block_id1 = ObjectId::new();
        let inline_block_id2 = ObjectId::new();
        let inline_block1 = json!({
            "_id": inline_block_id1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": ["italic"],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World"
            },
            "marks": ["bold", "italic"],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.to_string(), inline_block_id2.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);

        let block_map = BlockMap::from(vec![inline_block1, inline_block2, block, root_block]).unwrap();
        let event = Event::FormatBar(FormatBarEvent::Italic);
        let sub_selection_anchor = SubSelection::from(inline_block_id1, 2, None);
        let sub_selection_head = SubSelection::from(inline_block_id2, 3, None);
        let selection = Selection::from(sub_selection_anchor.clone(), sub_selection_head.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_block_map = execute_steps(steps, block_map).unwrap();
        let updated_standard_block = updated_block_map.get_standard_block(&paragraph_block_id).unwrap();
        let content_block = updated_standard_block.content_block().unwrap();
        assert_eq!(content_block.inline_blocks.len(), 4);
        let mut i = 0;
        for id in content_block.inline_blocks.iter() {
            let inline_block = updated_block_map.get_inline_block(id).unwrap();
            if i == 0 {
                assert_eq!(inline_block.text().unwrap(), "He");
                assert_eq!(inline_block.marks.len(), 1);
                assert_eq!(inline_block.marks[0], Mark::Italic);
            } else if i == 1 {
                assert_eq!(inline_block.text().unwrap(), "llo ");
                assert_eq!(inline_block.marks.len(), 0);
            } else if i == 2 {
                assert_eq!(inline_block.text().unwrap(), "Wor");
                assert_eq!(inline_block.marks.len(), 1);
                assert_eq!(inline_block.marks[0], Mark::Bold);
            } else if i == 3 {
                assert_eq!(inline_block.text().unwrap(), "ld");
                assert_eq!(inline_block.marks.len(), 2);
                assert_eq!(inline_block.marks.contains(&Mark::Bold), true);
                assert_eq!(inline_block.marks.contains(&Mark::Italic), true);
            }
            i += 1;
        }

        Ok(())
    }

    #[test]
    fn can_execute_apply_mark_selection_across_multiple_inline_blocks_and_execute_merge() {
        let root_block_id = ObjectId::new();
        let paragraph_block_id = ObjectId::new();
        let inline_block_id1 = ObjectId::new();
        let inline_block_id2 = ObjectId::new();
        let inline_block_id3 = ObjectId::new();
        let inline_block1 = json!({
            "_id": inline_block_id1.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello "
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "brave new "
            },
            "marks": ["bold"],
            "parent": paragraph_block_id.to_string()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "World!"
            },
            "marks": [],
            "parent": paragraph_block_id.to_string()
        });
        let block = json!({
            "_id": paragraph_block_id.to_string(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.to_string(), inline_block_id2.to_string(), inline_block_id3.to_string()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let root_block = RootBlock::json_from(root_block_id, vec![paragraph_block_id]);
        let event = Event::FormatBar(FormatBarEvent::Bold);
        let sub_selection_anchor = SubSelection::from(inline_block_id1, 0, None);
        let sub_selection_head = SubSelection::from(inline_block_id3, 3, None);
        let selection = Selection::from(sub_selection_anchor.clone(), sub_selection_head.clone());

        let block_map = BlockMap::from(vec![inline_block1, inline_block2, inline_block3, block, root_block]).unwrap();

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        let updated_block_map = execute_steps(steps, block_map).unwrap();
        let updated_standard_block = updated_block_map.get_standard_block(&paragraph_block_id).unwrap();
        let content_block = updated_standard_block.content_block().unwrap();
        assert_eq!(content_block.inline_blocks.len(), 2);
        let mut i = 0;
        for id in content_block.inline_blocks.iter() {
            let inline_block = updated_block_map.get_inline_block(id).unwrap();
            if i == 0 {
                assert_eq!(inline_block.text().unwrap(), "Hello brave new Wor");
                assert_eq!(inline_block.marks.len(), 1);
                assert_eq!(inline_block.marks[0], Mark::Bold);
            } else if i == 1 {
                assert_eq!(inline_block.text().unwrap(), "ld!");
                assert_eq!(inline_block.marks.len(), 0);
            }
            i += 1;
        }
    }
}