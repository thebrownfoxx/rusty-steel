use crate::edition::Edition;
use crate::enchantment::enchantment_type::{CostMultiplier, EnchantmentType, EnchantmentTypeId};
use crate::enchantment::shared::shared_enchantment_type::SharedEnchantmentType;
use crate::enchantment::shared::shared_enchantment_types::SharedEnchantmentTypes;

impl<'a, T> SharedEnchantmentTypes<'a> for Vec<SharedEnchantmentType<'a, T>>
where
    T: Fn(Edition) -> Option<CostMultiplier>,
{
    fn all_for_edition(&self, edition: Edition) -> Vec<&EnchantmentTypeId> {
        self.iter()
            .filter_map(|shared| Some(shared.for_edition(edition)?.id))
            .collect()
    }

    fn get_for_edition(&self, edition: Edition, id: &EnchantmentTypeId) -> Option<EnchantmentType> {
        self.iter()
            .find(|shared| shared.id == id)?
            .for_edition(edition)
    }
}
