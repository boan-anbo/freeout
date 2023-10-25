use crate::entities::core::block::{compute_hash, Block};
use crate::entities::core::block_range::BlockRange;
use crate::entities::core::outline::Outline;
use crate::entities::core::words_statistics::WordCount;
use crate::entities::reader::reader_trait::ReaderTrait;
use eyre::Report;
use std::collections::HashMap;
use tracing::debug;

#[derive(Clone)]
pub struct FreeoutOptions {
    pub include_content: bool,
}

impl Default for FreeoutOptions {
    fn default() -> Self {
        Self {
            include_content: true,
        }
    }
}

pub struct Freeout {
    pub(crate) text: String,
    pub blocks: HashMap<usize, Block>, // Using HashMap for quick block lookups by ID.
    pub line_starts: Vec<usize>,
    pub(crate) opt: FreeoutOptions,
}
impl Freeout {
    pub fn new(source: String, opt: Option<FreeoutOptions>) -> Freeout {
        let mut line_starts = vec![0]; // Start of the first line is at index 0
        for (index, _) in source.match_indices('\n') {
            line_starts.push(index + 1); // +1 to start after the newline
        }

        Self {
            opt: opt.unwrap_or_default(),
            text: source,
            blocks: HashMap::new(),
            line_starts,
        }
    }

    pub fn get_line(&self, line: usize) -> Option<&str> {
        if line >= self.line_starts.len() {
            return None; // Line number out of range
        }

        let start = self.line_starts[line];
        let end = if line + 1 < self.line_starts.len() {
            self.line_starts[line + 1] - 1 // -1 to exclude the newline
        } else {
            self.text.len()
        };

        Some(&self.text[start..end])
    }

    // Returns the total number of lines in the text.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }
}

fn compute_current_outline(document: &str) -> Vec<Block> {
    // Implementation of the actual outline computation.
    // This can be the existing logic to generate blocks from a document.
    Vec::new() // Placeholder
}
impl Freeout {
    pub fn outline<R>(&mut self, reader: &R) -> Result<Outline, Report>
    where
        R: ReaderTrait,
    {
        debug!("Running reader: {}", reader.name());
        self.blocks = reader.read(&self.text, &self.opt)?;

        // validate blocks
        debug!("Validating blocks");
        self.validate_blocks()?;

        // process content
        debug!("Processing content");
        self.process_content();

        Outline::build_outline(&self.blocks)
    }

    pub fn get_block_content(&self, block_id: &usize) -> Option<&str> {
        self.blocks.get(block_id).map(|block| {
            self.get_content_by_range(&block.header_range)
                .ok_or_else(|| eyre::eyre!("Failed to get content for block {}", block_id))
                .unwrap()
        })
    }

    pub fn get_content_by_range(&self, range: &BlockRange) -> Option<&str> {
        Some(&self.text[range.start.offset..range.end.offset])
    }

    pub fn compute_block_hash(&mut self, block_id: usize) -> Option<u64> {
        // Fetch the content first
        let content = self.get_block_content(&block_id);

        // Map the content (which is inside an Option) through the compute_hash function
        let hash_val = content.as_ref().map(|s| compute_hash(s));

        if let Some(val) = hash_val {
            if let Some(block) = self.blocks.get_mut(&block_id) {
                block.hash = Some(val);
                return Some(val);
            }
        }

        None
    }
    pub fn generate_incremental_outline(&self, previous_outline: Option<Vec<Block>>) -> Vec<Block> {
        if let Some(prev) = previous_outline {
            todo!()
        } else {
            compute_current_outline(&self.text)
        }
    }
}
