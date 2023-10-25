use eyre::Report;
use itertools::Itertools;

use crate::entities::core::block::{Block, Blocks};
use crate::entities::core::freeout::Freeout;

impl Freeout {
    /// Validates the continuity and starting point of block IDs in the block tree.
    ///
    /// This will be called immediately after the Reader provided the blocks.
    ///
    /// This ensures that the block tree provided by the reader is valid by checking:
    /// - Block IDs start from 1 (i.e., it's 1-indexed).
    /// - Block IDs are incremental and continuous.
    /// This continuity indicates a linear order of the blocks, which matches the order in which they will be rendered.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The first block ID isn't 1.
    /// - Any block ID is found to not be continuous with the previous block ID.
    pub fn validate_blocks(&self) -> Result<(), Report> {
        // Expected ID starts from 1 since it's 1-indexed.
        let mut expected_block_id = 1;

        for (block_id, _) in self
            .blocks
            .iter()
            .sorted_by(|(block_id_a, _), (block_id_b, _)| block_id_a.cmp(block_id_b))
        {
            if *block_id != expected_block_id {
                return Err(eyre::eyre!(
                    "Expected block id {} but found {}. Block IDs should be 1-indexed (starting from 1) and continuous (without any gaps in between).",
                    expected_block_id,
                    block_id
                ));
            }
            expected_block_id += 1;
        }

        Ok(())
    }


    pub fn get_block_mut(&mut self, block_id: &usize) -> Option<&mut Block> {
        self.blocks.get_mut(block_id)
    }
}