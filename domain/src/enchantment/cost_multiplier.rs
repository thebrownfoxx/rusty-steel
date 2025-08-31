use std::fmt::Debug;
use std::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct CostMultiplier {
    pub for_book: u8,
    pub for_item: u8,
}
