mod zero_combining_cost;
mod target_prior_work_penalty;
mod prior_work_penalty;
mod sacrifice_prior_work_penalty;

use crate::item::Item;

// TODO: Revert to innate and individual enchantments.
//  This will cause too many unnecessary iterations.
pub trait CombiningCost {
    fn combining_cost(&self, target: &Item, sacrifice: &Item) -> u8;
}
