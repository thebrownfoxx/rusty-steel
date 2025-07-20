use crate::edition::Edition;
use crate::enchantment::enchantment_type::{CostMultiplier, EnchantmentType, EnchantmentTypeId, EnchantmentTypes};
use crate::enchantment::shared::shared_enchantment_type::SharedEnchantmentType;

pub trait SharedEnchantmentTypes {
    fn all_for_edition(&self, edition: Edition) -> Vec<EnchantmentTypeId>;
    fn get_for_edition(&self, edition: Edition, id: &EnchantmentTypeId) -> Option<EnchantmentType>;
}

pub struct EnchantmentTypesFromShared<'a, S: SharedEnchantmentTypes> {
    pub edition: Edition,
    pub shared_enchantment_types: &'a S,
}

impl<'a, S: SharedEnchantmentTypes> EnchantmentTypes for EnchantmentTypesFromShared<'a, S> {
    fn all(&self) -> Vec<EnchantmentTypeId> {
        self.shared_enchantment_types.all_for_edition(self.edition)
    }

    fn get(&self, id: &EnchantmentTypeId) -> Option<EnchantmentType> {
        self.shared_enchantment_types.get_for_edition(self.edition, id)
    }
}

impl<F: Fn(Edition) -> Option<CostMultiplier>> SharedEnchantmentTypes
for Vec<SharedEnchantmentType<F>>
{
    fn all_for_edition(&self, edition: Edition) -> Vec<EnchantmentTypeId> {
        self.iter()
            .filter_map(|shared| Some(shared.for_edition(edition)?.id))
            .collect()
    }

    fn get_for_edition(&self, edition: Edition, id: &EnchantmentTypeId) -> Option<EnchantmentType> {
        self.iter()
            .find(|shared| shared.id == *id)?
            .for_edition(edition)
    }
}

