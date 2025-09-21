use crate::combine::cost::CombiningCost;
use crate::combine::cost::prior_work_penalty::prior_work_penalty;
use crate::combine::cost::zero_combining_cost::ZeroCombiningCost;
use crate::item::Item;

pub struct SacrificePriorWorkPenalty(Box<dyn CombiningCost>);

impl CombiningCost for SacrificePriorWorkPenalty {
    fn combining_cost(&self, target: &Item, sacrifice: &Item) -> u8 {
        self.0.combining_cost(target, sacrifice) + prior_work_penalty(sacrifice.anvil_use_count)
    }
}

impl Default for SacrificePriorWorkPenalty {
    fn default() -> Self {
        ZeroCombiningCost.add_sacrifice_prior_work_penalty()
    }
}

pub trait AddSacrificePriorWorkPenalty {
    fn add_sacrifice_prior_work_penalty(self) -> SacrificePriorWorkPenalty;
}

impl<T> AddSacrificePriorWorkPenalty for T
where 
    T: CombiningCost + 'static
{
    fn add_sacrifice_prior_work_penalty(self) -> SacrificePriorWorkPenalty {
        SacrificePriorWorkPenalty(Box::new(self))
    }
}