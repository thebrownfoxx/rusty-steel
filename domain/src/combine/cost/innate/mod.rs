mod prior_work_penalty;
mod target_prior_work_penalty;
mod zero_innate_cost;
mod sacrifice_prior_work_penalty;

pub use target_prior_work_penalty::{AddTargetPriorWorkPenalty, TargetPriorWorkPenalty};
pub use sacrifice_prior_work_penalty::{AddSacrificePriorWorkPenalty, SacrificePriorWorkPenalty};

pub(crate) use prior_work_penalty::prior_work_penalty;

use crate::item::Item;

pub trait InnateCost {
    fn innate_cost(&self, target: &Item, sacrifice: &Item) -> u8;
}
