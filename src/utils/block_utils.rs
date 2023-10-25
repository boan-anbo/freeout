use crate::entities::core::block::{Block, Blocks};
use crate::entities::core::block_range::BlockRange;
use crate::entities::core::position::Position;
use crate::utils::block_range_utils::PositionUtils;
use crate::utils::text_utils::TextUtils;
use itertools::Itertools;
use std::collections::HashMap;

pub struct BlockUtils {}

impl BlockUtils {
    /// # Populate the block range
    ///
    /// This operation finds the target block (parent), and its last recursive children (e.g. grandchild) before the next sibling or uncle or the parent,and merge their ranges.
    ///
    /// # Example
    ///
    /// ```markdown
    ///
    /// # Header 1 // <-- "#" = start of `block_range` after merge. "1" = end of `header_ranger` before merge.
    ///
    /// Content 1
    ///
    /// ## Header 2
    ///
    /// Content 2
    ///
    /// // <-- end of block_range after merge.
    /// # Header 3
    ///
    /// ```
    pub fn populate_block_ranges(blocks: &mut Blocks, text: &str) {
        let mut populated_block_range: HashMap<usize, BlockRange> = HashMap::new();
        for (block_id, block) in blocks.iter() {
            let mut block_range_end_position: Option<Position> = None;
            let next_sibling_or_uncle_id =
                Self::get_next_sibling_or_uncle_id(blocks, block_id, &block.depth);

            if next_sibling_or_uncle_id.is_none() {
                // if there is no next sibling or uncle, then the end position is the end of the text
                block_range_end_position = Some(TextUtils::get_end_position(text));
            } else {
                let next_sibling_or_uncle = blocks.get(&next_sibling_or_uncle_id.unwrap()).unwrap();
                // use the prior position of the start of the next sibling or uncle's header_range
                block_range_end_position = Some(next_sibling_or_uncle.header_range.start.clone());
            }

            let new_block_range = BlockRange {
                start: block.header_range.start.clone(),
                end: block_range_end_position.unwrap(),
            };

            populated_block_range.insert(*block_id, new_block_range);
        }

        // update the block range
        for (block_id, range) in populated_block_range.into_iter() {
            blocks.get_mut(&block_id).unwrap().block_range = Some(range);
        }
    }
    pub fn get_immediate_parent(
        blocks: &Blocks,
        current_block_id: &usize,
        current_block_depth: &usize,
    ) -> Option<usize> {
        blocks
            .iter()
            .filter(|(block_id, block)| {
                // iterate over the depth in reverse
                (block.depth < *current_block_depth) && *block_id < current_block_id
            })
            // get the one with highest id, because parent might not necessarily be only 1 depth above but must be the closest one
            .sorted_by(|(_, block_a), (_, block_b)| block_b.id.cmp(&block_a.id))
            .map(|(block_id, _)| *block_id)
            .next()
    }

    pub fn get_root_parent(
        blocks: &Blocks,
        current_block_id: &usize,
        current_block_depth: &usize,
    ) -> Option<usize> {
        let mut current_block_id = *current_block_id;
        let mut current_block_depth = *current_block_depth;
        let mut root_parent_id = None;
        while current_block_depth > 1 {
            root_parent_id =
                Self::get_immediate_parent(blocks, &current_block_id, &current_block_depth);
            if let Some(root_parent_id) = root_parent_id {
                current_block_id = root_parent_id;
                current_block_depth -= 1;
            } else {
                break;
            }
        }
        root_parent_id
    }
    pub fn get_block_ids_by_title(blocks: &Blocks, title: &str) -> Vec<usize> {
        blocks
            .iter()
            .filter(|(_, block)| block.title == title)
            .map(|(block_id, _)| *block_id)
            .collect()
    }
    /// # check if the block id exists
    pub fn has_block_id(blocks: &Blocks, block_id: &usize) -> bool {
        blocks.contains_key(block_id)
    }

    /// # Get the previous block id by current block id
    ///
    /// This retrieve the previous block id whose order is one less than the current block id.
    pub fn get_previous_block_id(blocks: &Blocks, current_block_id: &str) -> Option<usize> {
        let current_block_id = current_block_id.parse::<usize>().unwrap();
        if current_block_id == 0 {
            return None;
        }
        let previous_block_id = current_block_id - 1;
        // check the existence of the previous block
        if !Self::has_block_id(blocks, &previous_block_id) {
            return None;
        }

        Some(previous_block_id)
    }

    /// # Get the next sibling block id by current block id and depth
    pub fn get_next_sibling_id(
        blocks: &Blocks,
        current_block_id: &usize,
        current_block_depth: &usize,
    ) -> Option<usize> {
        blocks
            .iter()
            .filter(|(block_id, _)| *block_id > current_block_id)
            .filter(|(block_id, block)| block.depth == *current_block_depth)
            .map(|(block_id, _)| *block_id)
            .next()
    }

    /// # Get the next sibling or uncle block id by current block id and depth
    pub fn get_next_sibling_or_uncle_id(
        blocks: &Blocks,
        current_block_id: &usize,
        current_block_depth: &usize,
    ) -> Option<usize> {
        blocks
            .iter()
            // id greater than current block id
            .filter(|(block_id, _)| *block_id > current_block_id)
            .filter(|(block_id, block)| {
                // the next sibling or uncle block must have a depth greater than or equal to the current block
                block.depth <= *current_block_depth
            })
            // get the one with lowest id
            .sorted_by(|(_, block_a), (_, block_b)| block_a.id.cmp(&block_b.id))
            .map(|(block_id, _)| *block_id)
            .next()
    }

    /// # Get the last recursive children of the block
    ///
    /// Looks for the next block whose depth is great or equal to the current block (meaning it's either a sibling or a parent), and then returns the previous block.
    pub fn get_last_recursive_children_id_and_depth(
        blocks: &Blocks,
        block_id: &usize,
        block_depth: &usize,
    ) -> Option<usize> {
        Self::get_next_sibling_or_uncle_id(blocks, block_id, block_depth).and_then(
            |next_sibling_or_uncle_id| {
                Self::get_previous_block_id(blocks, &next_sibling_or_uncle_id.to_string())
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use tracing::{debug, info};

    use tracing_test::traced_test;

    use crate::entities::core::block::Block;
    use crate::entities::core::freeout::Freeout;
    use crate::readers::markdown::MarkdownReader;
    use crate::utils::block_range_utils::PositionUtils;
    use crate::utils::block_utils::BlockUtils;

    fn get_test_blocks() -> HashMap<usize, Block> {
        let mut blocks: HashMap<usize, Block> = HashMap::new();

        let grandma = Block {
            id: 1,
            depth: 1,
            children_ids: vec![2],
            title: "grandma".to_string(),
            ..Default::default()
        };

        let bobo = Block {
            id: 2,
            depth: 2,
            children_ids: vec![3],
            title: "bobo".to_string(),
            ..Default::default()
        };

        let tang_ge = Block {
            id: 3,
            depth: 3,
            children_ids: vec![],
            title: "tang_ge".to_string(),
            ..Default::default()
        };

        let dad = Block {
            id: 4,
            depth: 2,
            children_ids: vec![7, 8, 12],
            title: "dad".to_string(),
            ..Default::default()
        };

        let shushu = Block {
            id: 5,
            depth: 2,
            children_ids: vec![6],
            title: "shushu".to_string(),
            ..Default::default()
        };

        let tang_di = Block {
            id: 6,
            depth: 3,
            children_ids: vec![],
            title: "tang_di".to_string(),
            ..Default::default()
        };

        let my_older_sister = Block {
            id: 7,
            depth: 3,
            children_ids: vec![],
            title: "my_older_sister".to_string(),
            ..Default::default()
        };

        let me = Block {
            id: 8,
            depth: 3,
            children_ids: vec![9, 10],
            title: "me".to_string(),
            ..Default::default()
        };

        let my_first_child = Block {
            id: 9,
            depth: 4,
            children_ids: vec![],
            title: "my_first_child".to_string(),
            ..Default::default()
        };

        let my_second_child = Block {
            id: 10,
            depth: 4,
            children_ids: vec![11],
            title: "my_second_child".to_string(),
            ..Default::default()
        };

        let my_second_child_first_grand_child = Block {
            id: 11,
            depth: 5,
            children_ids: vec![],
            title: "my_second_child_first_grand_child".to_string(),
            ..Default::default()
        };

        let my_younger_sister = Block {
            id: 12,
            depth: 3,
            children_ids: vec![],
            title: "my_younger_sister".to_string(),
            ..Default::default()
        };

        let tests_cases = vec![
            grandma,
            bobo,
            tang_ge,
            dad,
            shushu,
            tang_di,
            my_older_sister,
            me,
            my_first_child,
            my_second_child,
            my_second_child_first_grand_child,
            my_younger_sister,
        ];

        assert_eq!(tests_cases.len(), 12);

        for test_case in tests_cases {
            blocks.insert(test_case.id, test_case);
        }

        blocks
    }
    #[test]
    fn test_get_block_by_relation() {
        let mut freeout = Freeout::new("fake".to_string(), None);

        let blocks = &get_test_blocks();

        let grandma_id = BlockUtils::get_block_ids_by_title(blocks, "grandma")[0];

        let bobo_id = BlockUtils::get_block_ids_by_title(blocks, "bobo")[0];

        let previous_block_to_bobo =
            BlockUtils::get_previous_block_id(blocks, &bobo_id.to_string());

        assert_eq!(previous_block_to_bobo, Some(grandma_id));

        let my_id = BlockUtils::get_block_ids_by_title(blocks, "me")[0];

        let my_younger_sister_id =
            BlockUtils::get_block_ids_by_title(blocks, "my_younger_sister")[0];

        let next_sibling_id = BlockUtils::get_next_sibling_id(blocks, &my_id, &3);

        assert_eq!(next_sibling_id, Some(my_younger_sister_id));

        let my_first_child_id = BlockUtils::get_block_ids_by_title(blocks, "my_first_child")[0];

        let my_second_child_id = BlockUtils::get_block_ids_by_title(blocks, "my_second_child")[0];

        // my second_child's next sibling or uncle should be my sister
        let my_second_child_next_sibling_or_uncle_id =
            BlockUtils::get_next_sibling_or_uncle_id(blocks, &my_second_child_id, &4);

        assert_eq!(
            my_second_child_next_sibling_or_uncle_id,
            Some(my_younger_sister_id)
        );

        let my_second_child_first_grand_child_id =
            BlockUtils::get_block_ids_by_title(blocks, "my_second_child_first_grand_child")[0];

        // this should get the next sibling or uncle of my first grand child, which should be my younger sister.
        let my_first_child_next_sibling_or_uncle_id =
            BlockUtils::get_next_sibling_or_uncle_id(blocks, &my_first_child_id, &4);

        assert_eq!(
            my_first_child_next_sibling_or_uncle_id,
            Some(my_second_child_id)
        );

        let my_last_recursive_child_id =
            BlockUtils::get_last_recursive_children_id_and_depth(blocks, &my_id, &3);

        assert_eq!(
            my_last_recursive_child_id,
            Some(my_second_child_first_grand_child_id)
        );

        let immediate_parent =
            BlockUtils::get_immediate_parent(blocks, &my_second_child_first_grand_child_id, &5);

        assert_eq!(immediate_parent, Some(my_second_child_id));

        let root_parent =
            BlockUtils::get_root_parent(blocks, &my_second_child_first_grand_child_id, &5);

        assert_eq!(root_parent, Some(grandma_id));
    }

    #[traced_test]
    #[test]
    fn test_block_validator() {
        let mut freeout_blocks_in_reverse = Freeout::new("fake".to_string(), None);

        let mut block_in_rev: HashMap<usize, Block> = HashMap::new();

        let block_1 = Block {
            id: 3,
            depth: 2,
            children_ids: vec![],
            title: "block_1".to_string(),
            ..Default::default()
        };

        let block_2 = Block {
            id: 2,
            depth: 1,
            children_ids: vec![3],
            title: "block_2".to_string(),
            ..Default::default()
        };

        block_in_rev.insert(block_1.id, block_1);
        block_in_rev.insert(block_2.id, block_2);

        freeout_blocks_in_reverse.blocks = block_in_rev;

        assert!(freeout_blocks_in_reverse.validate_blocks().is_err());

        // fix by adding a block with id 1 and key 1
        let mut block_1 = freeout_blocks_in_reverse.get_block_mut(&3).unwrap().clone();

        block_1.id = 1;

        // insert the new block id with key 1

        freeout_blocks_in_reverse.blocks.insert(block_1.id, block_1);

        assert!(freeout_blocks_in_reverse.validate_blocks().is_ok());

        let mut block_with_gap: HashMap<usize, Block> = HashMap::new();

        let block_1 = Block {
            id: 1,
            depth: 1,
            children_ids: vec![2],
            title: "block_1".to_string(),
            ..Default::default()
        };

        let block_2 = Block {
            id: 4,
            depth: 2,
            children_ids: vec![],
            title: "block_2".to_string(),
            ..Default::default()
        };

        block_with_gap.insert(block_1.id, block_1);

        block_with_gap.insert(block_2.id, block_2);

        let mut freeout_blocks_with_gap = Freeout::new("fake".to_string(), None);

        freeout_blocks_with_gap.blocks = block_with_gap;

        assert!(freeout_blocks_with_gap.validate_blocks().is_err());

        // fix the block id value and the hashmap key
        let mut block_4 = freeout_blocks_with_gap.get_block_mut(&4).unwrap().clone();

        block_4.id = 2;

        // remove   the old block id with key 4
        freeout_blocks_with_gap.blocks.remove(&4);

        // insert the new block id with key 2
        freeout_blocks_with_gap.blocks.insert(block_4.id, block_4);

        assert!(freeout_blocks_with_gap.validate_blocks().is_ok());
    }

    #[traced_test]
    #[test]
    fn populate_block_ranges() {
        let markdown = "# Header 1\n\n## Header 2\n\n# Header3".to_string();

        let mut freeout = Freeout::new(markdown, None);

        let outline = freeout.outline(&MarkdownReader::default()).unwrap();

        debug!(
            "outline: {:}",
            serde_json::to_string_pretty(&outline).unwrap()
        );

        // let first item
        let first_item = outline.items.get(0).unwrap();

        // let first item range
        let first_item_block_range = first_item.block.block_range.as_ref().unwrap();

        info!(
            "first_item_range: {:?}",
            serde_json::to_string_pretty(&first_item_block_range).unwrap()
        );

        // let last item
        let last_item = outline.items.get(1).unwrap();
        // assert last item title
        assert_eq!(last_item.block.title, "Header3");

        // let last item header range
        let last_item_range_start = last_item.block.header_range.start.clone();

        // the first item block range's end should be the prior position of the last item header range's start
        let prior_step_before_last_item_range_start =
            PositionUtils::get_immediate_prior_position(&freeout.text, &last_item_range_start);

        assert_eq!(
            first_item_block_range.end,
            prior_step_before_last_item_range_start
        );
    }
}
