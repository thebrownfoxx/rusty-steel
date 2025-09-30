use super::prior_work_penalty;
use super::InnateCost;
use crate::item::Item;

pub struct TargetPriorWorkPenalty(pub Box<dyn InnateCost>);

impl InnateCost for TargetPriorWorkPenalty {
    fn innate_cost(&self, target: &Item, sacrifice: &Item) -> u8 {
        self.0.innate_cost(target, sacrifice) + prior_work_penalty(target.anvil_use_count)
    }
}

pub trait AddTargetPriorWorkPenalty {
    fn add_target_prior_work_penalty(self) -> TargetPriorWorkPenalty;
}

impl<T> AddTargetPriorWorkPenalty for T
where
    T: InnateCost + 'static,
{
    fn add_target_prior_work_penalty(self) -> TargetPriorWorkPenalty {
        TargetPriorWorkPenalty(Box::new(self))
    }
}
