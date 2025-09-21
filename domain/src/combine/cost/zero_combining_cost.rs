use crate::combine::cost::CombiningCost;
use crate::item::Item;

pub struct ZeroCombiningCost;

impl CombiningCost for ZeroCombiningCost {
    fn combining_cost(&self, _: &Item, _: &Item) -> u8 {
        0
    }
}
