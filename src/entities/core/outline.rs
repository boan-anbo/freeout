use std::collections::HashMap;

use eyre::Report;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::entities::core::block::{Block, Blocks};

#[derive(Serialize, Deserialize, Clone)]
pub struct Outline {
    pub items: Vec<OutlineItem>,
}

impl Outline {
    pub fn build_outline(blocks: &HashMap<usize, Block>) -> Result<Outline, Report> {
        let mut root_outline_items: Vec<OutlineItem> = Vec::new();

        // collect root blocks whose parents are None
        root_outline_items = blocks
            .values()
            .filter(|block| block.parent_id.is_none())
            .map(|block| OutlineItem::from_block(block, blocks))
            .sorted_by(|a, b| a.block.id.cmp(&b.block.id))
            .collect();
        Ok(Outline {
            items: root_outline_items,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OutlineItem {
    pub block: Block,
    pub subitems: Vec<OutlineItem>,
}

impl OutlineItem {
    fn from_block(block: &Block, blocks: &Blocks) -> OutlineItem {
        let mut children = Vec::new();

        for &child_id in &block.children_ids {
            if let Some(child_block) = blocks.get(&child_id) {
                let child_item = OutlineItem::from_block(child_block, blocks);
                children.push(child_item);
            }
        }

        OutlineItem {
            block: block.clone(),
            subitems: children
                .into_iter()
                .sorted_by(|a, b| a.block.id.cmp(&b.block.id))
                .collect(),
        }
    }
}
