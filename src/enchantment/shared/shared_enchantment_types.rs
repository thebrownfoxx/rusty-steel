use crate::edition::Edition;
use crate::enchantment::enchantment_type::{
    CostMultiplier, EnchantmentType,
};
use crate::enchantment::shared::shared_enchantment_type::SharedEnchantmentType;

pub trait SharedEnchantmentTypes {
    fn for_edition(&self, edition: Edition) -> Vec<EnchantmentType>;
}

impl<F> SharedEnchantmentTypes for Vec<SharedEnchantmentType<F>>
where
    F: Fn(Edition) -> Option<CostMultiplier>,
{
    fn for_edition(&self, edition: Edition) -> Vec<EnchantmentType> {
        self.iter()
            .filter_map(|shared| shared.for_edition(edition))
            .collect()
    }
}
