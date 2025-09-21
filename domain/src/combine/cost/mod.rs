use crate::item::Item;

pub trait CombiningCost {
    fn combining_cost(&self, target: &Item, sacrifice: &Item);
}
