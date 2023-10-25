use crate::entities::core::block::Blocks;
use crate::entities::core::freeout::Freeout;
use crate::entities::core::words_statistics::{WordsStatus, WordsTarget};
use crate::utils::block_utils::BlockUtils;

impl Freeout {
    /// # Process Content of the block tree
    ///
    /// This function is called after the block tree is generated from the reader.
    ///
    /// It generates the following information:
    /// - Populate `block_range`
    /// - Hash of the content of each block
    /// - Word count of each block
    ///
    /// For any of these fields that have been set by the Reader, the function will not overwrite them.
    pub(crate) fn process_content(&mut self) {
        // populate range
        BlockUtils::populate_block_ranges(&mut self.blocks, &self.text);
        // after all blocks are generated, traverse again to hash all content and count words
        for (_, block) in self.blocks.iter_mut() {
            if block.content.is_none() {
                continue;
            }
            if block.hash.is_none() && self.opt.include_content {
                block.hash();
            }
            if block.self_stats.count.words == 0 {
                block.self_stats.count(block.content.as_ref().unwrap());
            }
        }


    }
}

