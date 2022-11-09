
#[cfg(test)]
mod tests {
    use rust_mirror::{blocks::{Block, standard_blocks::{StandardBlock, StandardBlockType, content_block::ContentBlock}, RootBlock, inline_blocks::{InlineBlock, InlineBlockType, text_block::TextBlock}}, mark::{Color, Mark}};
    use mongodb::bson::oid::ObjectId;

    #[test]
    fn can_turn_standard_block_to_json() {
        let block_id = ObjectId::new();
        let child_block_id = ObjectId::new();
        let parent_block_id = ObjectId::new();
        let inline_block_id = ObjectId::new();
        let block = Block::StandardBlock(StandardBlock {
            _id: block_id,
            content: StandardBlockType::Paragraph(ContentBlock {
                inline_blocks: vec![inline_block_id]
            }),
            children: vec![child_block_id],
            parent: parent_block_id,
            marks: vec![Mark::Bold, Mark::ForeColor(Color(255, 0, 0, 1))]
        });

        let json = block.to_json().unwrap();
        let expected = serde_json::json!({
            "_id": block_id.to_string(),
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
        let child_block_id = ObjectId::new();
        let root_block_id = ObjectId::new();
        let block = Block::Root(RootBlock {
            _id: root_block_id,
            children: vec![child_block_id]
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
        let inline_block_id = ObjectId::new();
        let parent_block_id = ObjectId::new();
        let block = Block::InlineBlock(InlineBlock {
            _id: inline_block_id,
            content: InlineBlockType::TextBlock(TextBlock {
                text: "Hello World".to_string()
            }),
            marks: vec![Mark::Italic, Mark::BackColor(Color(255, 0, 0, 1))],
            parent: parent_block_id
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