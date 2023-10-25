use crate::entities::core::position::Position;

pub struct TextUtils {}

impl TextUtils {
    /// Get the end position of the text.
    ///
    /// This method uses byte-based operations for performance reasons.
    /// It assumes the input text is valid UTF-8, as is the requirement for Rust's `str` type.
    ///
    /// Caveats:
    /// 1. The `column` value will be in bytes, not characters. This means that if the last line has multi-byte
    ///    characters (like many non-ASCII characters), the `column` value will be larger than the actual character count.
    ///    If you need an accurate character count for the `column`, you will need to count the characters in the substring
    ///    from the last newline position to the end, which will impact performance.
    /// 2. The `offset` value is the byte length of the entire string, which in most practical purposes works, but
    ///    remember it's not the character count. If you need character count, use `text.chars().count()` instead.
    ///
    pub fn get_end_position(text: &str) -> Position {
        // Counting the number of newline characters to determine the line count.
        let line_count = text.as_bytes().iter().filter(|&&b| b == b'\n').count();

        // Find the start of the last line.
        let last_newline_position = text.rfind('\n').map_or(0, |pos| pos + 1);

        // Calculate the column count as the byte length of the last line.
        let column_count = text[last_newline_position..].len();

        Position {
            line: line_count,
            column: column_count,
            // This is byte length; for character count, replace with `text.chars().count()`
            offset: text.len(),
        }
    }
}
