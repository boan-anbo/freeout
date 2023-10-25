/// # Block
///
/// A block is the building block of a tree.
use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::entities::core::block_range::BlockRange;
use crate::entities::core::words_statistics::WordStatistics;
use crate::entities::visitor::visitor_trait::Visitor;

pub type Blocks = HashMap<usize, Block>;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Block {
    /// 1-indexed
    /// Unique Id
    /// Order in which the block will be arranged in Outline.
    pub id: usize,

    pub depth: usize,
    /// Section marker, e.g. # for markdown, and = for typst.
    pub marker: String,
    /// Section title
    pub title: String,
    /// plain content if the config asked the parser to return plain text of this block
    pub content: Option<String>,
    /// Section note
    pub note: Option<String>,
    /// Parend id
    ///
    /// The id of the parent block.
    ///
    /// None means it's a root level block.
    pub parent_id: Option<usize>,

    /// # The reader is expected to provide this.
    ///
    /// ## Example
    ///
    /// ```markdown
    ///
    /// # Header 1 // <-- the header start = "#" and end = "\n"
    ///
    /// Content
    ///
    /// # Header 2
    /// ```
    pub header_range: BlockRange,

    /// # The range of a block.
    ///
    /// ## Example
    ///
    /// ```markdown
    ///
    /// # Header 1 // <-- block_range.start = index of "#"
    ///
    /// Content
    ///
    /// # Header 2 // <-- block_range.end = index of "#"
    ///
    /// ```
    ///
    /// ## Note
    ///
    /// If this is provided by the reader, Freeout will not touch it, other wise, it will default to the range between the beginning of the header and the last character before the beginning of the next block header.
    pub block_range: Option<BlockRange>,

    /// Statistics for the content of this block itself.
    pub self_stats: WordStatistics,
    /// Combined statistics of this block and all its descendants.
    pub aggregate_stats: WordStatistics,

    /// Whether this block should be excluded from all calculations and basically ignored. This is needed when, for example, I want to mark a section as a note, but I don't want it to be counted in the word count.
    pub exclude: bool,

    pub hash: Option<u64>,

    pub children_ids: Vec<usize>, // Indices of child blocks.
}

// impl Visitor pattern for Block
impl Block {
    pub fn accept(&self, visitor: &mut dyn Visitor, blocks: &HashMap<usize, Block>) {
        visitor.visit_block(self);

        for &child_id in &self.children_ids {
            if let Some(child) = blocks.get(&child_id) {
                child.accept(visitor, blocks);
            }
        }
    }

    pub fn hash(&mut self) -> () {
        self.hash = Some(compute_hash(&self.title));
    }
}

pub fn compute_hash(text: &str) -> u64 {
    let hasher = blake3::hash(text.as_bytes());
    let bytes = hasher.as_bytes();
    // Convert the first 8 bytes of the hash into a u64.
    u64::from_le_bytes(bytes[0..8].try_into().unwrap())
}
