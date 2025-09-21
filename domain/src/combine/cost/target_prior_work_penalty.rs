use crate::combine::cost::CombiningCost;
use crate::combine::cost::prior_work_penalty::prior_work_penalty;
use crate::combine::cost::zero_combining_cost::ZeroCombiningCost;
use crate::item::Item;

pub struct TargetPriorWorkPenalty(Box<dyn CombiningCost>);

impl CombiningCost for TargetPriorWorkPenalty {
    fn combining_cost(&self, target: &Item, sacrifice: &Item) -> u8 {
        self.0.combining_cost(target, sacrifice) + prior_work_penalty(target.anvil_use_count)
    }
}

impl Default for TargetPriorWorkPenalty {
    fn default() -> Self {
        ZeroCombiningCost.add_target_prior_work_penalty()
    }
}

pub trait AddTargetPriorWorkPenalty {
    fn add_target_prior_work_penalty(self) -> TargetPriorWorkPenalty;
}

impl<T> AddTargetPriorWorkPenalty for T
where
    T: CombiningCost + 'static
{
    fn add_target_prior_work_penalty(self) -> TargetPriorWorkPenalty {
        TargetPriorWorkPenalty(Box::new(self))
    }
}
