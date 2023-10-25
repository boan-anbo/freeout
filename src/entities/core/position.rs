use serde::{Deserialize, Serialize};
/// # Position
///
/// A position is a point in text.
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Position {
    /// 0-indexed integer representing a line in a source file.
    pub line: usize,
    /// 0-indexed integer representing a column in a source file.
    pub column: usize,
    /// 0-indexed integer representing a character in a source file.
    pub offset: usize,
}