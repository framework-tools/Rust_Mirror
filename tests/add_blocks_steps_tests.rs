#[cfg(test)]
mod tests {
    use rust_mirror::{new_ids::NewIds, blocks::{RootBlock, BlockMap, standard_blocks::{StandardBlockType, content_block::ContentBlock}}, steps_generator::{event::{Event, SlashScrimEvent}, selection::{SubSelection, Selection}, generate_steps}, step::{Step, ReplaceSlice}};
    use serde_json::json;


    #[test]
    fn can_handle_slash_scrim_add_block() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id = new_ids.get_id().unwrap();
        let paragraph_block_id = new_ids.get_id().unwrap();
        let root_block_id = new_ids.get_id().unwrap();

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
        let event = Event::SlashScrim(SlashScrimEvent { block_type: "paragraph".to_string() });
        let sub_selection = SubSelection::from(inline_block_id.clone().clone(), 17, None);
        let selection = Selection::from(sub_selection.clone(), sub_selection.clone());

        let steps = generate_steps(&event, &block_map, selection).unwrap();
        assert_eq!(steps.len(), 2);
        match &steps[0] {
            Step::ReplaceStep(replace_step) => {
                assert_eq!(replace_step.block_id, paragraph_block_id.clone());
                assert_eq!(replace_step.from, SubSelection { block_id: inline_block_id.clone(), offset: 12, subselection: None });
                assert_eq!(replace_step.to, SubSelection { block_id: inline_block_id.clone(), offset: 17, subselection: None });
                assert_eq!(replace_step.slice, ReplaceSlice::String("".to_string()));
            },
            _ => panic!("Expected replace step")
        };
        match &steps[1] {
            Step::AddBlock(add_block_step) => {
                assert_eq!(add_block_step.block_id, root_block_id.clone());
                assert_eq!(add_block_step.child_offset, 1);
                assert_eq!(add_block_step.block_type, StandardBlockType::Paragraph(ContentBlock::new(vec![])));
            },
            _ => panic!("Expected replace step")
        };
    }
}