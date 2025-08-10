use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct CostMultiplier {
    pub for_book: u8,
    pub for_item: u8,
}

impl CostMultiplier {
    pub fn new(for_book: u8, for_item: u8) -> Self {
        CostMultiplier { for_item, for_book }
    }
}
