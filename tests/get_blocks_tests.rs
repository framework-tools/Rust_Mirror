#[cfg(test)]
mod tests {
    use rust_mirror::{utilities::{BlockStructure, BlocksBetween, get_blocks_between, Tree}, new_ids::NewIds, blocks::{Block, RootBlock, BlockMap}, steps_generator::{selection::{Selection, SubSelection}, StepError}};
    use serde_json::json;


    /// Input:
    /// <1>H|ello world!</1>
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
                    block_id: inline_block_id2.clone(),
                    offset: 3,
                    subselection: None,
                }))
            },
        };

        let blocks_as_tree = get_blocks_between(
            BlockStructure::Tree,
            &selection.anchor,
            &selection.head,
            &block_map,
            &mut new_ids
        ).unwrap();
        match blocks_as_tree {
            BlocksBetween::Tree(Tree { top_blocks, block_map }) => {
                assert_eq!(top_blocks[0].id(), std_block_id1);
                assert_eq!(top_blocks[1].id(), std_block_id5);
                assert_eq!(top_blocks[2].id(), std_block_id3);
                assert_eq!(top_blocks.len(), 3);

                let first_blocks_inline_blocks = &top_blocks[0].content_block().unwrap().inline_blocks;
                assert_eq!(first_blocks_inline_blocks.len(), 1);
                let first_block_inline_block = block_map.get_inline_block(&first_blocks_inline_blocks[0]).unwrap();
                assert_eq!(first_block_inline_block.text().unwrap().clone().to_string(), "ello world!".to_string());

                block_map.get_inline_block(&inline_block_id2).unwrap();

                let last_block = &top_blocks[2];
                let expected_children: Vec<String> = vec![];
                assert_eq!(last_block.children.clone(), expected_children);

                let last_blocks_inline_blocks = &last_block.content_block().unwrap().inline_blocks;
                assert_eq!(last_blocks_inline_blocks.len(), 1);
                let last_block_inline_block = block_map.get_inline_block(&last_blocks_inline_blocks[0]).unwrap();
                assert_eq!(last_block_inline_block.text().unwrap().clone().to_string(), "Goo".to_string());
            },
            _ => panic!("Expected tree"),
        }
    }
    /// Input:
    /// <1>H|ello world</1>
    ///     <4/>
    /// <5></5>
    /// <3>Goo|dbye world</3>
    ///     <2/>
    #[test]
    fn get_blocks_test1_flat() {
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

        let blocks = get_blocks_between(
            BlockStructure::Flat,
            &selection.anchor,
            &selection.head,
            &block_map,
            &mut new_ids
        ).unwrap();
        match blocks {
            BlocksBetween::Flat(blocks) => {
                assert_eq!(blocks[0].id(), std_block_id1);
                assert_eq!(blocks[1].id(), std_block_id4);
                assert_eq!(blocks[2].id(), std_block_id5);
                assert_eq!(blocks[3].id(), std_block_id3);
                assert_eq!(blocks.len(), 4);
            },
            _ => panic!("Expected tree"),
        }
    }

    /// <1>A</1> *start of selection*
    ///     <2>B</2>
    ///         <3>C</3>
    ///     <4>D</4>
    /// <5>E</5>
    /// <6>F</6>
    ///     <7>G</7>
    ///         <8>H</8>
    ///     <9>I</9>
    ///         <10>J</10> *end of selection*
    #[test]
    fn get_blocks_test2() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let p_id7 = "7".to_string();
        let p_id8 = "8".to_string();
        let p_id9 = "9".to_string();
        let p_id10 = "10".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let inline_block_id7 = new_ids.get_id()?;
        let inline_block_id8 = new_ids.get_id()?;
        let inline_block_id9 = new_ids.get_id()?;
        let inline_block_id10 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id4.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": ["bold"],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": ["bold"],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": ["bold"],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": ["bold"],
            "parent": p_id5.clone()
        });
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [p_id7.clone(), p_id9.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "F"
            },
            "marks": ["bold"],
            "parent": p_id6.clone()
        });
        let p7 = json!({
            "_id": p_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [p_id8.clone()],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "G"
            },
            "marks": ["bold"],
            "parent": p_id7.clone()
        });
        let p8 = json!({
            "_id": p_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id8.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id7.clone()
        });
        let inline_block8 = json!({
            "_id": inline_block_id8.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "H"
            },
            "marks": ["bold"],
            "parent": p_id8.clone()
        });
        let p9 = json!({
            "_id": p_id9.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id9.clone()]
            },
            "children": [p_id10.clone()],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block9 = json!({
            "_id": inline_block_id9.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "I"
            },
            "marks": ["bold"],
            "parent": p_id9.clone()
        });
        let p10 = json!({
            "_id": p_id10.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id10.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id9.clone()
        });
        let inline_block10 = json!({
            "_id": inline_block_id10.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "J"
            },
            "marks": ["bold"],
            "parent": p_id10.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone(), p_id5.clone(), p_id6.clone()]
        );

        let sub_selection_from = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id1.clone(), 0, None)))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id6.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id9.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: p_id9.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection {
                        block_id: p_id10.clone(),
                        offset: 1,
                        subselection: Some(Box::new(SubSelection::from(inline_block_id10.clone(), 1, None)))
        }))}))}))};

        let block_map = BlockMap::from(vec![
            root_block.to_string(),
            inline_block1.to_string(), p1.to_string(),
            inline_block2.to_string(), p2.to_string(),
            inline_block3.to_string(), p3.to_string(),
            inline_block4.to_string(), p4.to_string(),
            inline_block5.to_string(), p5.to_string(),
            inline_block6.to_string(), p6.to_string(),
            inline_block7.to_string(), p7.to_string(),
            inline_block8.to_string(), p8.to_string(),
            inline_block9.to_string(), p9.to_string(),
            inline_block10.to_string(), p10.to_string(),
        ]).unwrap();
        let blocks_as_tree = get_blocks_between(
            BlockStructure::Tree,
            &sub_selection_from,
            &sub_selection_to,
            &block_map,
            &mut new_ids
        ).unwrap();
        match blocks_as_tree {
            BlocksBetween::Tree(Tree { top_blocks, block_map }) => {
                assert_eq!(top_blocks[0].id(), p_id1);
                assert_eq!(top_blocks[1].id(), p_id5);
                assert_eq!(top_blocks[2].id(), p_id6);
                assert_eq!(top_blocks.len(), 3);

                block_map.get_standard_block(&p_id1).unwrap();
                block_map.get_standard_block(&p_id2).unwrap();
                block_map.get_standard_block(&p_id3).unwrap();
                block_map.get_standard_block(&p_id4).unwrap();
                block_map.get_standard_block(&p_id5).unwrap();
                block_map.get_standard_block(&p_id6).unwrap();
                block_map.get_standard_block(&p_id7).unwrap();
                block_map.get_standard_block(&p_id8).unwrap();
                block_map.get_standard_block(&p_id9).unwrap();
                block_map.get_standard_block(&p_id10).unwrap();

                block_map.get_inline_block(&inline_block_id2).unwrap();
                block_map.get_inline_block(&inline_block_id3).unwrap();
                block_map.get_inline_block(&inline_block_id4).unwrap();
                block_map.get_inline_block(&inline_block_id5).unwrap();
                block_map.get_inline_block(&inline_block_id6).unwrap();
                block_map.get_inline_block(&inline_block_id7).unwrap();
                block_map.get_inline_block(&inline_block_id8).unwrap();
                block_map.get_inline_block(&inline_block_id9).unwrap();

                return Ok(())
            },
            _ => panic!("Expected tree"),
        }
    }

    /// <1>
    ///     <2> *selection starts here*
    ///         <3>
    ///     <4>
    ///         <5>
    ///             <6> *selection ends here*
    ///                 <7>
    #[test]
    fn get_blocks_test3() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let p_id7 = "7".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let inline_block_id7 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id4.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": ["bold"],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": ["bold"],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone()]
            },
            "children": [p_id5.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "D"
            },
            "marks": ["bold"],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [p_id6.clone()],
            "marks": [],
            "parent": p_id4.clone()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": ["bold"],
            "parent": p_id5.clone()
        });
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [p_id7.clone()],
            "marks": [],
            "parent": p_id5.clone()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "F"
            },
            "marks": ["bold"],
            "parent": p_id6.clone()
        });
        let p7 = json!({
            "_id": p_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "G"
            },
            "marks": ["bold"],
            "parent": p_id7.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone()]
        );

        let sub_selection_from = SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(
                    SubSelection {
                        block_id: inline_block_id2.clone(),
                        offset: 0,
                        subselection: None
                    }
                ))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id4.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id5.clone(),
                offset: 0,
                subselection: Some(Box::new(
                    SubSelection {
                        block_id: p_id6.clone(),
                        offset: 0,
                        subselection: Some(Box::new(
                            SubSelection {
                                block_id: inline_block_id6.clone(),
                                offset: 1,
                                subselection: None
                            }
                        ))
                    }
                ))
            }))
        };

        let block_map = BlockMap::from(vec![
            root_block.to_string(),
            inline_block1.to_string(), p1.to_string(),
            inline_block2.to_string(), p2.to_string(),
            inline_block3.to_string(), p3.to_string(),
            inline_block4.to_string(), p4.to_string(),
            inline_block5.to_string(), p5.to_string(),
            inline_block6.to_string(), p6.to_string(),
            inline_block7.to_string(), p7.to_string(),
        ]).unwrap();
        let blocks_as_tree = get_blocks_between(
            BlockStructure::Tree,
            &sub_selection_from,
            &sub_selection_to,
            &block_map,
            &mut new_ids
        ).unwrap();
        match blocks_as_tree {
            BlocksBetween::Tree(Tree { top_blocks, block_map }) => {
                assert_eq!(top_blocks[0].id(), p_id2);
                assert_eq!(top_blocks[1].id(), p_id4);
                assert_eq!(top_blocks.len(), 2);

                block_map.get_standard_block(&p_id2).unwrap();
                block_map.get_standard_block(&p_id3).unwrap();
                block_map.get_standard_block(&p_id4).unwrap();
                block_map.get_standard_block(&p_id5).unwrap();
                let last_block = block_map.get_standard_block(&p_id6).unwrap();
                assert_eq!(last_block.children.len(), 0);

                return Ok(())
            },
            _ => panic!("Expected tree"),
        }
    }

    /// <1>A</1>
    ///     <2>B</2>
    ///         <3>C</3> *start of selection*
    ///     <4>D</4>
    /// <5>E</5>
    /// <6>F</6> *end of selection*
    ///     <7>G</7>
    ///         <8>H</8>
    ///     <9>I</9>
    ///         <10>J</10>
    #[test]
    fn get_blocks_test4() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let p_id7 = "7".to_string();
        let p_id8 = "8".to_string();
        let p_id9 = "9".to_string();
        let p_id10 = "10".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id3b = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let inline_block_id6b = new_ids.get_id()?;
        let inline_block_id7 = new_ids.get_id()?;
        let inline_block_id8 = new_ids.get_id()?;
        let inline_block_id9 = new_ids.get_id()?;
        let inline_block_id10 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id4.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": [],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": [],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone(), inline_block_id3b.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": [],
            "parent": p_id3.clone()
        });
        let inline_block3b = json!({
            "_id": inline_block_id3b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "fdsasfdfsdfds"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": ["bold"],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": ["bold"],
            "parent": p_id5.clone()
        });
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone(), inline_block_id6b.clone()]
            },
            "children": [p_id7.clone(), p_id9.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Fafdsfsd"
            },
            "marks": ["bold"],
            "parent": p_id6.clone()
        });
        let inline_block6b = json!({
            "_id": inline_block_id6b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Fb"
            },
            "marks": [],
            "parent": p_id6.clone()
        });
        let p7 = json!({
            "_id": p_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [p_id8.clone()],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "G"
            },
            "marks": [],
            "parent": p_id7.clone()
        });
        let p8 = json!({
            "_id": p_id8.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id8.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id7.clone()
        });
        let inline_block8 = json!({
            "_id": inline_block_id8.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "H"
            },
            "marks": [],
            "parent": p_id8.clone()
        });
        let p9 = json!({
            "_id": p_id9.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id9.clone()]
            },
            "children": [p_id10.clone()],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block9 = json!({
            "_id": inline_block_id9.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "I"
            },
            "marks": [],
            "parent": p_id9.clone()
        });
        let p10 = json!({
            "_id": p_id10.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id10.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id9.clone()
        });
        let inline_block10 = json!({
            "_id": inline_block_id10.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "J"
            },
            "marks": [],
            "parent": p_id10.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone(), p_id5.clone(), p_id6.clone()]
        );

        let sub_selection_from = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection {
                    block_id: p_id3.clone(),
                    offset: 0,
                    subselection: Some(Box::new(SubSelection::from(inline_block_id3b.clone(), 4, None)))
                }))
            }))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id6.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id6.clone(), 4, None)))
        };

        let block_map = BlockMap::from(vec![
            root_block.to_string(),
            p1.to_string(), inline_block1.to_string(),
            p2.to_string(), inline_block2.to_string(),
            p3.to_string(), inline_block3.to_string(),
            p4.to_string(), inline_block4.to_string(),
            p5.to_string(), inline_block5.to_string(),
            p6.to_string(), inline_block6.to_string(),
            p7.to_string(), inline_block7.to_string(),
            p8.to_string(), inline_block8.to_string(),
            p9.to_string(), inline_block9.to_string(),
            p10.to_string(), inline_block10.to_string(),
            inline_block3b.to_string(), inline_block6b.to_string()
        ])?;

        let blocks_as_tree = get_blocks_between(
            BlockStructure::Tree,
            &sub_selection_from,
            &sub_selection_to,
            &block_map,
            &mut new_ids
        ).unwrap();
        match blocks_as_tree {
            BlocksBetween::Tree(Tree { top_blocks, block_map }) => {
                assert_eq!(top_blocks[0].id(), p_id3);
                assert_eq!(top_blocks[1].id(), p_id4);
                assert_eq!(top_blocks[2].id(), p_id5);
                assert_eq!(top_blocks[3].id(), p_id6);
                assert_eq!(top_blocks.len(), 4);

                block_map.get_standard_block(&p_id3).unwrap();
                block_map.get_standard_block(&p_id4).unwrap();
                block_map.get_standard_block(&p_id5).unwrap();
                block_map.get_standard_block(&p_id6).unwrap();
                let last_block = block_map.get_standard_block(&p_id6).unwrap();
                assert_eq!(last_block.children.len(), 0);

                return Ok(())
            },
            _ => panic!("Expected tree"),
        }
    }

    /// <1>
    ///     <2>
    ///         <3> *selection starts here*
    ///     <4> *selection ends here*
    ///         <5>
    ///             <6>
    ///                 <7>
    #[test]
    fn get_blocks_test5() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let p_id7 = "7".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id3b = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id4b = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let inline_block_id7 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id4.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": [],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": [],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone(), inline_block_id3b.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Ccc"
            },
            "marks": ["italic"],
            "parent": p_id3.clone()
        });
        let inline_block3b = json!({
            "_id": inline_block_id3b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "lololololol"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone(), inline_block_id4b.clone()]
            },
            "children": [p_id5.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "hehehehehe"
            },
            "marks": ["bold"],
            "parent": p_id4.clone()
        });
        let inline_block4b = json!({
            "_id": inline_block_id4b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "abc"
            },
            "marks": [],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [p_id6.clone()],
            "marks": [],
            "parent": p_id4.clone()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": [],
            "parent": p_id5.clone()
        });
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [p_id7.clone()],
            "marks": [],
            "parent": p_id5.clone()
        });
        let inline_block6 = json!({
            "_id": inline_block_id6.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "F"
            },
            "marks": [],
            "parent": p_id6.clone()
        });
        let p7 = json!({
            "_id": p_id7.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id7.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id6.clone()
        });
        let inline_block7 = json!({
            "_id": inline_block_id7.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "G"
            },
            "marks": [],
            "parent": p_id7.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone()]
        );

        let block_map = BlockMap::from(
            vec![
                root_block.to_string(),
                p1.to_string(), inline_block1.to_string(),
                p2.to_string(), inline_block2.to_string(),
                p3.to_string(), inline_block3.to_string(), inline_block3b.to_string(),
                p4.to_string(), inline_block4.to_string(), inline_block4b.to_string(),
                p5.to_string(), inline_block5.to_string(),
                p6.to_string(), inline_block6.to_string(),
                p7.to_string(), inline_block7.to_string()
            ]
        ).unwrap();

        let sub_selection_from = SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(
                    SubSelection {
                        block_id: p_id3.clone(),
                        offset: 0,
                        subselection: Some(Box::new(SubSelection::from(inline_block_id3.clone(), 3, None)))
                    }
                ))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id4.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id4b.clone(), 0, None)))
        };

        let blocks_as_tree = get_blocks_between(
            BlockStructure::Tree,
            &sub_selection_from,
            &sub_selection_to,
            &block_map,
            &mut new_ids
        ).unwrap();
        match blocks_as_tree {
            BlocksBetween::Tree(Tree { top_blocks, block_map }) => {
                assert_eq!(top_blocks[0].id(), p_id3);
                assert_eq!(top_blocks[1].id(), p_id4);
                assert_eq!(top_blocks.len(), 2);

                block_map.get_standard_block(&p_id3).unwrap();
                block_map.get_standard_block(&p_id4).unwrap();
                let last_block = block_map.get_standard_block(&p_id4).unwrap();
                assert_eq!(last_block.children.len(), 0);

                return Ok(())
            },
            _ => panic!("Expected tree"),
        }
    }

    /// <1> *selection starts here*
    ///     <2> *selection ends here*
    /// <3>
    #[test]
    fn get_blocks_test6() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": [],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": [],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "C"
            },
            "marks": [],
            "parent": p_id3.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone(), p_id3.clone()]
        );

        let block_map = BlockMap::from(
            vec![
                root_block.to_string(),
                p1.to_string(), inline_block1.to_string(),
                p2.to_string(), inline_block2.to_string(),
                p3.to_string(), inline_block3.to_string(),
            ]
        ).unwrap();

        let sub_selection_from = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id1.clone(), 0, None)))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id2.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection::from(inline_block_id2.clone(), 1, None)))}))
        };

        let blocks_as_tree = get_blocks_between(
            BlockStructure::Tree,
            &sub_selection_from,
            &sub_selection_to,
            &block_map,
            &mut new_ids
        ).unwrap();
        match blocks_as_tree {
            BlocksBetween::Tree(Tree { top_blocks, block_map }) => {
                assert_eq!(top_blocks[0].id(), p_id1);
                assert_eq!(top_blocks.len(), 1);

                block_map.get_standard_block(&p_id1).unwrap();
                block_map.get_standard_block(&p_id2).unwrap();
                let last_block = block_map.get_standard_block(&p_id2).unwrap();
                assert_eq!(last_block.children.len(), 0);

                return Ok(())
            },
            _ => panic!("Expected tree"),
        }
    }

    /// <1> Hell|o </1><2> brave </2><3> ne|w </3><4> world </4>
    #[test]
    fn get_blocks_test_inline() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_id1 = new_ids.get_id()?;
        let inline_id2 = new_ids.get_id()?;
        let inline_id3 = new_ids.get_id()?;
        let inline_id4 = new_ids.get_id()?;
        let p_id = new_ids.get_id()?;
        let root_block_id = new_ids.get_id()?;

        let p = json!({
            "_id": p_id.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_id1.clone(), inline_id2.clone(), inline_id3.clone(), inline_id4.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " Hello "
            },
            "marks": [],
            "parent": p_id.clone()
        });
        let inline_block2 = json!({
            "_id": inline_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " brave "
            },
            "marks": ["bold"],
            "parent": p_id.clone()
        });
        let inline_block3 = json!({
            "_id": inline_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " new "
            },
            "marks": [],
            "parent": p_id.clone()
        });
        let inline_block4 = json!({
            "_id": inline_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": " world "
            },
            "marks": ["bold"],
            "parent": p_id.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id.clone()]
        );

        let block_map = BlockMap::from(
            vec![
                root_block.to_string(), p.to_string(), inline_block1.to_string(),
                inline_block2.to_string(), inline_block3.to_string(), inline_block4.to_string()
            ]
        ).unwrap();

        let sub_selection_from = SubSelection::from(inline_id1.clone(), 5, None);
        let sub_selection_to = SubSelection::from(inline_id3.clone(), 3, None);

        let blocks_as_tree = get_blocks_between(
            BlockStructure::Tree,
            &sub_selection_from,
            &sub_selection_to,
            &block_map,
            &mut new_ids
        ).unwrap();
        match blocks_as_tree {
            BlocksBetween::Tree(Tree { top_blocks, block_map }) => {
                let std_block = top_blocks[0].clone();
                assert_eq!(std_block.id(), p_id);
                assert_eq!(top_blocks.len(), 1);

                let inline_blocks = &std_block.content_block()?.inline_blocks;
                assert_eq!(inline_blocks.len(), 3);

                let mut i = 0;
                for id in inline_blocks {
                    let inline_block = block_map.get_inline_block(id)?;
                    if i == 0 {
                        assert_eq!(inline_block.text()?.clone().to_string(), "o ".to_string());
                    }
                    if i == 1 {
                        assert_eq!(inline_block.text()?.clone().to_string(), " brave ".to_string());
                    }
                    if i == 2 {
                        assert_eq!(inline_block.text()?.clone().to_string(), " ne".to_string());
                    }

                    i += 1;
                }


                return Ok(())
            },
            _ => panic!("Expected tree"),
        }
    }

    /// <1> *selection starts here*
    ///     <2>
    ///         <3>
    ///             <4>
    /// <5> *selection ends here*
    #[test]
    fn get_blocks_test7() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id3b = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id4b = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone()],
            "marks": [],
            "parent": root_block_id.to_string()
        });
        let inline_block1 = json!({
            "_id": inline_block_id1.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "A"
            },
            "marks": [],
            "parent": p_id1.clone()
        });
        let p2 = json!({
            "_id": p_id2.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id2.clone()]
            },
            "children": [p_id3.clone()],
            "marks": [],
            "parent": p_id1.clone()
        });
        let inline_block2 = json!({
            "_id": inline_block_id2.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "B"
            },
            "marks": [],
            "parent": p_id2.clone()
        });
        let p3 = json!({
            "_id": p_id3.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id3.clone(), inline_block_id3b.clone()]
            },
            "children": [p_id4.clone()],
            "marks": [],
            "parent": p_id2.clone()
        });
        let inline_block3 = json!({
            "_id": inline_block_id3.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Ccc"
            },
            "marks": ["italic"],
            "parent": p_id3.clone()
        });
        let inline_block3b = json!({
            "_id": inline_block_id3b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "lololololol"
            },
            "marks": ["bold"],
            "parent": p_id3.clone()
        });
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone(), inline_block_id4b.clone()]
            },
            "children": [],
            "marks": [],
            "parent": p_id3.clone()
        });
        let inline_block4 = json!({
            "_id": inline_block_id4.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "hehehehehe"
            },
            "marks": ["bold"],
            "parent": p_id4.clone()
        });
        let inline_block4b = json!({
            "_id": inline_block_id4b.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "abc"
            },
            "marks": [],
            "parent": p_id4.clone()
        });
        let p5 = json!({
            "_id": p_id5.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id5.clone()]
            },
            "children": [],
            "marks": [],
            "parent": root_block_id.clone()
        });
        let inline_block5 = json!({
            "_id": inline_block_id5.clone(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "E"
            },
            "marks": [],
            "parent": p_id5.clone()
        });

        let root_block = RootBlock::json_from(
            root_block_id.clone(),
            vec![p_id1.clone(), p_id5.clone()]
        );

        let block_map = BlockMap::from(
            vec![
                root_block.to_string(),
                p1.to_string(), inline_block1.to_string(),
                p2.to_string(), inline_block2.to_string(),
                p3.to_string(), inline_block3.to_string(), inline_block3b.to_string(),
                p4.to_string(), inline_block4.to_string(), inline_block4b.to_string(),
                p5.to_string(), inline_block5.to_string(),
            ]
        ).unwrap();

        let sub_selection_from = SubSelection {
                block_id: p_id1.clone(),
                offset: 0,
                subselection: Some(Box::new(SubSelection::from(inline_block_id1.clone(), 0, None)))
        };
        let sub_selection_to = SubSelection {
            block_id: p_id5.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id5.clone(), 0, None)))
        };

        get_blocks_between(
            BlockStructure::Tree,
            &sub_selection_from,
            &sub_selection_to,
            &block_map,
            &mut new_ids
        ).unwrap();
        return Ok(())
    }

}