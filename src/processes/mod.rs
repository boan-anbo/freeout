//! # Block Processing Pipeline
//!
//! This pipeline describes the sequence of steps to process and compute statistics for blocks.
//!
//! ## 1. Block Content Extraction
//! * **Input**: Raw text (from a given `Reader`).
//! * **Output**: Extracted blocks with basic properties.
//! * **Description**: This step leverages a reader to parse the raw text and create `Block` structures
//!   without most statistics. At this stage, only properties like `depth`, `order`, `marker`, `title`,
//!   `content_range`, `note`, `parent_id`, and `position` are filled.
//! * **Content**: This process also gather content under each block to be used in other steps for calculation.
//! * **Note**: We utilize custom readers that implement a `ReaderTrait` to allow flexibility in the source
//!   of our raw text (e.g., from a file, a web source, etc.).
//!
//! ## 2. Self Statistics and Hash Calculation
//! * **Input**: Blocks from the previous step.
//! * **Output**: Each block's `self_stats` filled with word statistics for its own content and `hash` filled.
//! * **Description**: For each block, we slice the original raw text based on the block's `content_range`
//!   and compute the statistics for that slice, filling out the `self_stats` field. Immediately, the hash
//!   of the content is also computed and stored.
//!
//! ## 3. Aggregate Statistics Calculation
//! * **Input**: Blocks with populated `self_stats`.
//! * **Output**: Blocks with `aggregate_stats` filled out.
//! * **Description**: Starting from leaf nodes (blocks with no children), compute the aggregate statistics
//!   recursively. Each parent's `aggregate_stats` is the sum of its own `self_stats` and the `aggregate_stats`
//!   of all its children.
//!
//! ## 4. Target Distribution (if applicable)
//! * **Input**: Blocks with `self_stats` and `aggregate_stats`, and potentially some targets specified.
//! * **Output**: `self_stats.target` filled out for all blocks.
//! * **Description**: Distribute the root's target (if given) among its children based on a specified logic
//!   (e.g., proportionally or equally). This is propagated down the hierarchy to all children.
//!
//! ## 5. Status Calculation
//! * **Input**: Blocks with populated `self_stats` and `aggregate_stats`.
//! * **Output**: `self_stats.status` and `aggregate_stats.status` filled out.
//! * **Description**: With the counts and targets in place, compute the `status` for each block. This
//!   represents how each block's word count compares with its target.
//!
//! ## 6. Post-Processing (if necessary)
//! * **Input**: Fully populated blocks.
//! * **Output**: Final set of blocks after any additional processing.
//! * **Description**: Perform any additional operations, such as filtering out excluded blocks,
//!   re-indexing, or other necessary transformations.
//!
//! These steps are modular and encapsulated, allowing each to be understood, tested, and potentially
//! parallelized independently.
pub mod content_processor;
