use crate::edition::Edition;
use crate::enchantment::enchantment_type::{CostMultiplier, EnchantmentType, EnchantmentTypeId};

pub struct SharedEnchantmentType<F: Fn(Edition) -> Option<CostMultiplier>> {
    pub id: EnchantmentTypeId,
    pub name: String,
    pub cost_multiplier: F,
}

impl<F: Fn(Edition) -> Option<CostMultiplier>> SharedEnchantmentType<F> {
    pub fn for_edition(&self, edition: Edition) -> Option<EnchantmentType> {
        let cost_multiplier = (self.cost_multiplier)(edition)?;
        Some(EnchantmentType {
            id: self.id.clone(),
            name: self.name.clone(),
            cost_multiplier,
        })
    }
}
