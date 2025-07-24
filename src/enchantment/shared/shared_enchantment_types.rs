use crate::edition::Edition;
use crate::enchantment::enchantment_type::EnchantmentType;
use crate::enchantment::shared::shared_enchantment_type::SharedEnchantmentType;

pub trait SharedEnchantmentTypes {
    fn for_edition(&self, edition: Edition) -> Vec<EnchantmentType>;
}

impl SharedEnchantmentTypes for Vec<SharedEnchantmentType> {
    fn for_edition(&self, edition: Edition) -> Vec<EnchantmentType> {
        self.iter()
            .filter_map(|shared| shared.for_edition(edition))
            .collect()
    }
}
