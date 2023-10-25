use crate::entities::core::block::Block;

pub trait Visitor {
    fn visit_block(&mut self, block: &Block);
}