use std::fmt::Debug;
use std::hash::Hash;

pub trait CostMultiplier: Eq + Copy + Hash + Debug {
    fn for_book(&self) -> u8;
    fn for_item(&self) -> u8;
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct OwnedCostMultiplier {
    pub for_book: u8,
    pub for_item: u8,
}

impl OwnedCostMultiplier {
    fn new(for_book: u8, for_item: u8) -> Self {
        OwnedCostMultiplier { for_item, for_book }
    }
}

impl CostMultiplier for OwnedCostMultiplier {
    fn for_book(&self) -> u8 {
        self.for_book
    }

    fn for_item(&self) -> u8 {
        self.for_item
    }
}
