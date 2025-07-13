use crate::edition::Edition;
use crate::enchantment::enchantment_type::{EnchantmentType, EnchantmentTypeId, EnchantmentTypes};

pub trait SharedEnchantmentTypes<'a> {
    fn all_for_edition(&self, edition: Edition) -> Vec<&EnchantmentTypeId>;
    fn get_for_edition(&self, edition: Edition, id: &EnchantmentTypeId) -> Option<EnchantmentType>;
}

pub struct EnchantmentTypesFromShared<'a> {
    pub edition: Edition,
    pub shared_enchantment_types: dyn SharedEnchantmentTypes<'a>, // TODO: Maybe make this generic
}

impl<'a> EnchantmentTypes<'a> for EnchantmentTypesFromShared<'a> {
    fn all(&self) -> Vec<&EnchantmentTypeId> {
        self.shared_enchantment_types.all_for_edition(self.edition)
    }

    fn get(&self, id: &EnchantmentTypeId) -> Option<EnchantmentType> {
        self.shared_enchantment_types.get_for_edition(self.edition, id)
    }
}
