use super::{Combine, Result};
use crate::item::Item;

struct BedrockAnvil;

impl Combine for BedrockAnvil {
    fn combine(&self, target: Item, sacrifice: Item) -> Result<Item> {
        todo!()
    }
}