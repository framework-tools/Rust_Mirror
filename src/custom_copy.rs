use std::collections::HashMap;

use serde::__private::de;
use wasm_bindgen::JsValue;

use crate::{utilities::Tree, steps_generator::StepError, blocks::BlockMap, frontend_interface::get_js_field};

#[derive(Debug, Clone)]
pub enum CustomCopy {
    Rust(Tree),
    Js(js_sys::Object)
}

impl CustomCopy {
    pub fn new() -> Self {
        return CustomCopy::Rust(Tree { top_blocks: vec![], block_map: BlockMap::Rust(HashMap::new()) })
    }

    pub fn from(tree: Tree) -> Self {
        return CustomCopy::Rust(tree)
    }

    pub fn update(self, tree: Tree) -> Result<Self, StepError> {
        return match self {
            Self::Rust(_) => Ok(Self::Rust(tree)),
            Self::Js(js_tree) => {
                let top_blocks = js_sys::Array::new();
                for block in tree.top_blocks {
                    top_blocks.push(&JsValue::from_str(&block._id));
                }
                js_sys::Reflect::set(
                    &js_tree,
                    &JsValue::from_str("top_blocks"),
                    &JsValue::from(top_blocks)
                ).unwrap();
                let block_map = tree.block_map.to_js_map()?;
                js_sys::Reflect::set(
                    &js_tree,
                    &JsValue::from_str("block_map"),
                    &JsValue::from(block_map)
                ).unwrap();
                Ok(Self::Js(js_tree))
            }
        }
    }

    pub fn to_tree(self) -> Result<Tree, StepError> {
        return match self {
            Self::Rust(tree) => Ok(tree),
            Self::Js(js_tree) => {
                let block_map = get_js_field(&js_tree, "block_map")?;
                let block_map = BlockMap::Js(js_sys::Map::from(block_map));

                let top_blocks_ids = get_js_field(&js_tree, "top_blocks")?;
                let top_blocks_ids = js_sys::Array::from(&top_blocks_ids).to_vec();
                let mut top_blocks = Vec::new();
                for id in top_blocks_ids {
                    let id = id.as_string().unwrap();
                    let block = block_map.get_standard_block(&id)?;
                    top_blocks.push(block);
                }

                return Ok(Tree {
                    top_blocks,
                    block_map,
                })
            }
        }
    }
}