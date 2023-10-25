use crate::entities::core::block::{Block, Blocks};
use crate::entities::core::freeout::FreeoutOptions;
use eyre::Report;
use std::collections::HashMap;

pub trait ReaderTrait {
    fn read(&self, source: &str, options: &FreeoutOptions) -> Result<Blocks, Report>;

    /// # Get the name of the reader
    ///
    /// Override this function to return the name of the reader for logging purposes.
    fn name(&self) -> &'static str {
        "Unknown Reader"
    }
}
