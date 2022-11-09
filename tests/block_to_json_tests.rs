
#[cfg(test)]
mod tests {
    use rust_mirror::{blocks::{Block, standard_blocks::{StandardBlock, StandardBlockType, content_block::ContentBlock}, RootBlock, inline_blocks::{InlineBlock, InlineBlockType, text_block::TextBlock}}, mark::{Color, Mark}, new_ids::NewIds};


    #[test]
    fn can_turn_standard_block_to_json() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let block_id = new_ids.get_id().unwrap();
        let child_block_id = new_ids.get_id().unwrap();
        let parent_block_id = new_ids.get_id().unwrap();
        let inline_block_id = new_ids.get_id().unwrap();
        let block = Block::StandardBlock(StandardBlock {
            _id: block_id.clone(),
            content: StandardBlockType::Paragraph(ContentBlock {
                inline_blocks: vec![inline_block_id.clone()]
            }),
            children: vec![child_block_id.clone()],
            parent: parent_block_id.clone(),
            marks: vec![Mark::Bold, Mark::ForeColor(Color(255, 0, 0, 1))]
        });

        let json = block.to_json().unwrap();
        let expected = serde_json::json!({
            "_id": block_id,
            "kind": "standard",
            "_type": "paragraph",
            "content": {
                "inline_blocks": [inline_block_id.to_string()]
            },
            "children": [child_block_id.to_string()],
            "parent": parent_block_id.to_string(),
            "marks": ["bold", "fore_color(255, 0, 0, 1)"],
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn can_turn_root_block_to_json() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let child_block_id = new_ids.get_id().unwrap();
        let root_block_id = new_ids.get_id().unwrap();
        let block = Block::Root(RootBlock {
            _id: root_block_id.clone(),
            children: vec![child_block_id.clone()]
        });
        let json = block.to_json().unwrap();
        let expected = serde_json::json!({
            "_id": root_block_id.to_string(),
            "kind": "root",
            "children": [child_block_id.to_string()]
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn can_turn_inline_block_to_json() {
        let mut new_ids = NewIds::hardcoded_new_ids_for_tests();

        let inline_block_id = new_ids.get_id().unwrap();
        let parent_block_id = new_ids.get_id().unwrap();
        let block = Block::InlineBlock(InlineBlock {
            _id: inline_block_id.clone(),
            content: InlineBlockType::TextBlock(TextBlock {
                text: "Hello World".to_string()
            }),
            marks: vec![Mark::Italic, Mark::BackColor(Color(255, 0, 0, 1))],
            parent: parent_block_id.clone()
        });
        let json = block.to_json().unwrap();
        let expected = serde_json::json!({
            "_id": inline_block_id.to_string(),
            "kind": "inline",
            "_type": "text",
            "content": {
                "text": "Hello World"
            },
            "marks": ["italic", "back_color(255, 0, 0, 1)"],
            "parent": parent_block_id.to_string()
        });
        assert_eq!(json, expected);
    }
}