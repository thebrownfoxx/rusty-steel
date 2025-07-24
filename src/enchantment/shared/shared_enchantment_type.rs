use crate::edition::Edition;
use crate::enchantment::cost_multiplier::CostMultiplier;
use crate::enchantment::enchantment_type::EnchantmentType;
use crate::enchantment::enchantment_type_id::EnchantmentTypeId;

pub struct SharedEnchantmentType<F>
where
    F: Fn(Edition) -> Option<CostMultiplier>,
{
    pub id: EnchantmentTypeId,
    pub name: String,
    pub max_level: i8,
    pub cost_multiplier: F,
}

impl<F> SharedEnchantmentType<F>
where
    F: Fn(Edition) -> Option<CostMultiplier>,
{
    pub fn new(
        id: impl Into<EnchantmentTypeId>,
        name: impl Into<String>,
        max_level: i8,
        cost_multiplier: F,
    ) -> SharedEnchantmentType<F> {
        SharedEnchantmentType {
            id: id.into(),
            name: name.into(),
            max_level,
            cost_multiplier,
        }
    }

    pub fn for_edition(&self, edition: Edition) -> Option<EnchantmentType> {
        let cost_multiplier = (self.cost_multiplier)(edition)?;
        Some(EnchantmentType {
            id: self.id.clone(),
            name: self.name.clone(),
            max_level: self.max_level,
            cost_multiplier,
        })
    }
}
