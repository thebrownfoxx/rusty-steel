use super::prior_work_penalty;
use super::InnateCost;
use crate::item::Item;

pub struct SacrificePriorWorkPenalty(pub Box<dyn InnateCost>);

impl InnateCost for SacrificePriorWorkPenalty {
    fn innate_cost(&self, target: &Item, sacrifice: &Item) -> u8 {
        self.0.innate_cost(target, sacrifice) + prior_work_penalty(sacrifice.anvil_use_count)
    }
}

pub trait AddSacrificePriorWorkPenalty {
    fn add_sacrifice_prior_work_penalty(self) -> SacrificePriorWorkPenalty;
}

impl<T> AddSacrificePriorWorkPenalty for T
where
    T: InnateCost + 'static,
{
    fn add_sacrifice_prior_work_penalty(self) -> SacrificePriorWorkPenalty {
        SacrificePriorWorkPenalty(Box::new(self))
    }
}
