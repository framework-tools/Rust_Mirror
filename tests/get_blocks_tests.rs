#[cfg(test)]
mod tests {
    use rust_mirror::{utilities::{BlockStructure, get_std_blocks_between}, new_ids::NewIds, blocks::{Block, RootBlock, BlockMap}, steps_generator::selection::{Selection, SubSelection}};
    use serde_json::json;


    /// Input:
    /// <1>H|ello world</1>
    ///     <4/>
    /// <5></5>
    /// <3>Goo|dbye world</3>
    ///     <2/>
    #[test]
    fn get_blocks_test1() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id().unwrap();
        let std_block_id1 = "1".to_string();
        let inline_block_id1 = new_ids.get_id().unwrap();
        let std_block_id2 = "2".to_string();
        let inline_block_id2 = new_ids.get_id().unwrap();
        let inline_block_id3 = new_ids.get_id().unwrap();
        let std_block_id3 = "3".to_string();
        let std_block_id4 = "4".to_string();
        let std_block_id5 = "5".to_string();

        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello world!"
            },
            "marks": [],
            "parent": std_block_id1.clone()
        });

        let std_block1 = json!({
            "_id": std_block_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [std_block_id4.clone()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let std_block_4 = Block::new_std_block_json(std_block_id4.clone(), std_block_id1.clone());

        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Goodbye "
            },
            "marks": [],
            "parent": std_block_id3.clone()
        });

        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "world!"
            },
            "marks": ["bold"],
            "parent": std_block_id3.clone()
        });

        let std_block2 = json!({
            "_id": std_block_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": std_block_id3.clone()
        });
        let std_block3 = json!({
            "_id": std_block_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone(), inline_block_id3.clone()]
            },
            "children": [std_block_id2.to_string()],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let std_block5 = json!({
            "_id": std_block_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": []
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });

        let root_block = RootBlock::json_from(root_block_id.clone(), vec![
            std_block_id1.clone(), std_block_id5.clone(), std_block_id3.clone()
            ]);

        let block_map = BlockMap::from(vec![
            inline_block1.to_string(), inline_block2.to_string(), std_block1.to_string(), std_block2.to_string(),
            root_block.to_string(), std_block3.to_string(), std_block_4.to_string(), inline_block3.to_string(), std_block5.to_string()
        ]).unwrap();

        let selection = Selection {
            anchor: SubSelection {
                block_id: std_block_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id1.clone(),
                    offset: 1,
                    subselection: None,
                }))
            },
            head: SubSelection {
                block_id: std_block_id3.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: inline_block_id3.clone(),
                    offset: 3,
                    subselection: None,
                }))
            },
        };

        let blocks_as_tree = get_std_blocks_between(
            &selection.anchor,
            &selection.head,
            BlockStructure::Tree,
            &block_map
        ).unwrap();
        assert_eq!(blocks_as_tree[0].id(), std_block_id1);
        assert_eq!(blocks_as_tree[1].id(), std_block_id5);
        assert_eq!(blocks_as_tree[2].id(), std_block_id3);
        assert_eq!(blocks_as_tree.len(), 3);
    }
}