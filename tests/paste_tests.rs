
#[cfg(test)]
mod tests {
    use rust_mirror::{steps_actualisor::{actualise_shortcuts::actualise_copy, actualise_steps}, custom_copy::CustomCopy, steps_generator::{StepError, selection::SubSelection}, new_ids::NewIds, blocks::{BlockMap, RootBlock}, step::Step};
    use serde_json::{json, to_string};

    /// Copy: /// <1> Hell|o </1><2> brave </2><3> ne|w </3><4> world </4>
    /// Paste: <1> Hello </1><2> brave </2><3> new </3><4> ||world </4>
    #[test]
    fn can_paste_inline_blocks() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_id1 = "inline1".to_string();
        let inline_id2 = "inline2".to_string();
        let inline_id3 = "inline3".to_string();
        let inline_id4 = "inline4".to_string();
        let p_id = "p".to_string();
        let root_block_id = "root".to_string();

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

        let from = SubSelection::from(inline_id1.clone(), 5, None);
        let to = SubSelection::from(inline_id3.clone(), 3, None);

        let updated_state = actualise_copy(CustomCopy::new(), from, to, block_map, &mut new_ids, Vec::new())?;

        let paste_subselection = SubSelection::from(inline_id4.clone(), 1, None);
        let updated_state = actualise_steps(
            vec![Step::Paste(paste_subselection.clone(), paste_subselection.clone())],
            updated_state.block_map,
            &mut new_ids,
            updated_state.copy.unwrap()
        )?;

        let updated_p = updated_state.block_map.get_standard_block(&p_id)?;
        let inline_blocks = &updated_p.content_block()?.inline_blocks;
        assert_eq!(inline_blocks.len(), 8);
        let inline4 = updated_state.block_map.get_inline_block(&inline_blocks[3])?;
        assert_eq!(inline4.text()?.clone().to_string(), " ".to_string());
        assert_eq!(inline4.parent, p_id.clone());
        let inline5 = updated_state.block_map.get_inline_block(&inline_blocks[4])?;
        assert_eq!(inline5.text()?.clone().to_string(), "o ".to_string());
        assert_eq!(inline5.parent, p_id.clone());
        let inline6 = updated_state.block_map.get_inline_block(&inline_blocks[5])?;
        assert_eq!(inline6.text()?.clone().to_string(), " brave ".to_string());
        assert_eq!(inline6.parent, p_id.clone());
        let inline7 = updated_state.block_map.get_inline_block(&inline_blocks[6])?;
        assert_eq!(inline7.text()?.clone().to_string(), " ne".to_string());
        assert_eq!(inline7.parent, p_id.clone());
        let inline8 = updated_state.block_map.get_inline_block(&inline_blocks[7])?;
        assert_eq!(inline8.text()?.clone().to_string(), "world ".to_string());
        assert_eq!(inline8.parent, p_id.clone());

        return Ok(())
    }

    /// <1> *selection starts here*
    ///     <2>
    ///     <3>
    ///         <4> *selection ends here*
    /// <5> *paste in this block*
    ///     <6>
    #[test]
    fn can_paste_first_blocks_children_correctly() -> Result<(), StepError> {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let root_block_id = new_ids.get_id()?;
        let p_id1 = "1".to_string();
        let p_id2 = "2".to_string();
        let p_id3 = "3".to_string();
        let p_id4 = "4".to_string();
        let p_id5 = "5".to_string();
        let p_id6 = "6".to_string();
        let inline_block_id1 = new_ids.get_id()?;
        let inline_block_id2 = new_ids.get_id()?;
        let inline_block_id3 = new_ids.get_id()?;
        let inline_block_id4 = new_ids.get_id()?;
        let inline_block_id5 = new_ids.get_id()?;
        let inline_block_id6 = new_ids.get_id()?;
        let p1 = json!({
            "_id": p_id1.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id1.clone()]
            },
            "children": [p_id2.clone(), p_id3.clone()],
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
            "children": [p_id4.clone()],
            "marks": [],
            "parent": p_id1.clone()
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
        let p4 = json!({
            "_id": p_id4.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id4.clone()]
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
                "text": "D"
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
        let p6 = json!({
            "_id": p_id6.clone(),
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id6.clone()]
            },
            "children": [],
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
                p4.to_string(), inline_block4.to_string(),
                p5.to_string(), inline_block5.to_string(),
                p6.to_string(), inline_block6.to_string(),
            ]
        ).unwrap();

        let from = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection::from(inline_block_id1.clone(), 0, None)))
        };
        let to = SubSelection {
            block_id: p_id1.clone(),
            offset: 0,
            subselection: Some(Box::new(SubSelection {
                block_id: p_id3.clone(),
                offset: 0,
                subselection: Some(Box::new( SubSelection {
                    block_id: p_id4.clone(),
                    offset: 0,
                    subselection:Some(Box::new(SubSelection::from(inline_block_id4.clone(), 1, None)))
        }))}))};

        let updated_state = actualise_copy(CustomCopy::new(), from, to, block_map, &mut new_ids, Vec::new())?;

        let paste_subselection = SubSelection::from(inline_block_id5.clone(), 1, None);
        let updated_state = actualise_steps(
            vec![Step::Paste(paste_subselection.clone(), paste_subselection.clone())],
            updated_state.block_map,
            &mut new_ids,
            updated_state.copy.unwrap()
        )?;

        let updated_p = updated_state.block_map.get_standard_block(&p_id5)?;
        assert_eq!(updated_p.children.len(), 3);

        let first_child = updated_state.block_map.get_standard_block(&updated_p.children[0])?;
        assert_eq!(
            updated_state.block_map.get_inline_block(&first_child.content_block()?.inline_blocks[0])?.text()?.clone().to_string(),
            "B".to_string()
        );
        let second_child = updated_state.block_map.get_standard_block(&updated_p.children[1])?;
        assert_eq!(
            updated_state.block_map.get_inline_block(&second_child.content_block()?.inline_blocks[0])?.text()?.clone().to_string(),
            "C".to_string()
        );
        let second_childs_child = updated_state.block_map.get_standard_block(&second_child.children[0])?;
        assert_eq!(
            updated_state.block_map.get_inline_block(&second_childs_child.content_block()?.inline_blocks[0])?.text()?.clone().to_string(),
            "D".to_string()
        );
        let third_child = updated_state.block_map.get_standard_block(&updated_p.children[2])?;
        assert_eq!(
            updated_state.block_map.get_inline_block(&third_child.content_block()?.inline_blocks[0])?.text()?.clone().to_string(),
            "F".to_string()
        );


        return Ok(())
    }
}