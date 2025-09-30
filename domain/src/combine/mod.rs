mod bedrock_anvil;
mod combination;
mod cost;
mod error;

pub use cost::{
    AddSacrificePriorWorkPenalty, AddTargetPriorWorkPenalty, InnateCost, SacrificePriorWorkPenalty,
    TargetPriorWorkPenalty,
};
pub use error::{Error, Result};

use crate::item::Item;

pub trait Combine {
    fn combine(&self, target: Item, sacrifice: Item) -> Result<Item>;
}
