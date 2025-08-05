use std::fmt::Debug;
use std::hash::Hash;

pub trait CostMultiplier: Eq + Copy + Hash + Debug {
    fn from_book() -> i8;
    fn from_item() -> u8;
}
