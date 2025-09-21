mod error;
mod combination;
mod bedrock_anvil;
mod cost;

pub use error::{Error, Result};

use crate::item::Item;

pub trait Combine {
    fn combine(&self, target: Item, sacrifice: Item) -> Result<Item>;
}
