use unicode_segmentation::UnicodeSegmentation;
use crate::entities::core::block_range::BlockRange;
use crate::entities::core::position::Position;

const BACKTRACK_BYTES: usize = 12;
pub struct PositionUtils {

}

impl PositionUtils {

    /// Merge two block ranges into one.
    ///
    /// # Use case
    ///
    /// `let block_range = BlockRange::merge(&block1.header_range, &block2.block_range);`
    pub fn merge_range(start_range: &BlockRange, end_range: &BlockRange) -> BlockRange {
        BlockRange {
            start: start_range.start.clone(),
            end: end_range.end.clone(),
        }
    }

    /// Get the immediate prior position before the current position.
    ///
    /// # Example
    ///
    /// ```markdown
    ///
    /// # Header 1
    ///
    /// Content
    ///  // <-- prior position = "\n"
    /// # Header 2 // <-- current position = "#"
    ///
    /// ```
    pub fn get_immediate_prior_position(
        text: &str,
        current_position: &Position,
    ) -> Position {
        // We will backtrack a safe, arbitrary byte count from the current offset.
        // This should typically cover the last grapheme.
        let sub_text = &text[..current_position.offset];
        // Use the graphemes iterator to get the grapheme just before the current offset.
        let prior_grapheme_offset = sub_text
            .graphemes(true)
            .next_back()
            .map(|g| g.len())
            .unwrap_or(0);

        // Adjust offset by the length of the grapheme in bytes
        let new_offset = current_position
            .offset
            .saturating_sub(prior_grapheme_offset);

        // Here, we assume that each grapheme cluster corresponds to a single column, which might not always be the case, but for simplicity, it works. If you want to handle more complex scenarios like tab characters or full-width characters, you'd need additional logic.
        if current_position.column > 0 {
            Position {
                line: current_position.line,
                column: current_position.column - 1,
                offset: new_offset,
            }
        } else if current_position.line > 0 {
            // To get the exact column of the prior line, you'd need to iterate over graphemes of that line. For simplicity, we just set it to 0 here.
            Position {
                line: current_position.line - 1,
                column: 0,
                offset: new_offset,
            }
        } else {
            // If it's the very first position in the text, return it as is.
            current_position.clone()
        }
    }

    pub fn get_text_by_range<'a>(text: &'a str, range: &'a BlockRange) -> &'a str {
        &text[range.start.offset..range.end.offset]
    }
}
#[cfg(test)]
mod tests {
    use crate::entities::core::block_range::BlockRange;
    use crate::entities::core::position::Position;
    use crate::utils::block_range_utils::PositionUtils;

    #[test]
    fn test_get_text_by_range() {
        let text = "# Header 1\n\nContent\n# Header 2";
        let block_range = BlockRange {
            start: Position {
                line: 0,
                column: 0,
                offset: 0,
            },
            end: Position {
                line: 1,
                column: 1,
                offset: 1,
            },
        };
        let text_by_range = PositionUtils::get_text_by_range(text, &block_range);

        assert_eq!(text_by_range, "#");

        let block_range = BlockRange {
            start: Position {
                line: 1,
                column: 9,
                offset: 9,
            },
            end: Position {
                line: 3,
                column: 1,
                offset: 13,
            },
        };
        let text_by_range = PositionUtils::get_text_by_range(text, &block_range);

        assert_eq!(text_by_range, "1\n\nC");
        assert_eq!(text_by_range.len(), 4);
    }

    #[test]
    fn test_get_immediate_prior_position_eng() {
        let text = "1\n2";
        let block_range = BlockRange {
            start: Position {
                line: 0,
                column: 0,
                offset: 0,
            },
            end: Position {
                line: 1,
                column: 1,
                offset: 3,
            },
        };

        let prior_position = PositionUtils::get_immediate_prior_position(text, &block_range.start);

        assert_eq!(prior_position.line, 0);
        assert_eq!(prior_position.column, 0);
        assert_eq!(prior_position.offset, 0);

        let prior_position = PositionUtils::get_immediate_prior_position(text, &block_range.end);

        assert_eq!(prior_position.line, 1);
        assert_eq!(prior_position.column, 0);
        assert_eq!(prior_position.offset, 2);

    }
}
