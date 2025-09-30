use super::InnateCost;
use crate::item::Item;

pub struct ZeroInnateCost;

impl InnateCost for ZeroInnateCost {
    fn innate_cost(&self, _: &Item, _: &Item) -> u8 {
        0
    }
}
