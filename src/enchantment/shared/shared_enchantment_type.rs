use crate::edition::Edition;
use crate::enchantment::enchantment_type::{CostMultiplier, EnchantmentType, EnchantmentTypeId};
use std::borrow::Cow;

pub struct SharedEnchantmentType<'a, T>
where
    T: Fn(Edition) -> Option<CostMultiplier>,
{
    pub id: &'a EnchantmentTypeId,
    pub name: String,
    pub cost_multiplier: T,
}

impl<'a, T> SharedEnchantmentType<'a, T>
where
    T: Fn(Edition) -> Option<CostMultiplier>,
{
    pub fn for_edition(&self, edition: Edition) -> Option<EnchantmentType> {
        Some(EnchantmentType::new(
            self.id,
            Cow::Borrowed(&self.name),
            (self.cost_multiplier)(edition)?,
        ))
    }
}
