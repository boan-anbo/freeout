use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::entities::core::position::Position;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BlockRange {
    pub start: Position,
    pub end: Position,
}

impl BlockRange {

}

