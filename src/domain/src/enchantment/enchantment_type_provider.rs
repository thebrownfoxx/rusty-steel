use crate::enchantment::enchantment_type::{EnchantmentType, EnchantmentTypeId};

pub trait EnchantmentTypeProvider {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentTypeId>;
    fn get(&self, id: &EnchantmentTypeId) -> Option<&impl EnchantmentType>;
}

pub struct OwnedEnchantmentTypeProvider<T: EnchantmentType> {
    pub enchantment_types: Vec<T>,
}

impl<T: EnchantmentType> EnchantmentTypeProvider for OwnedEnchantmentTypeProvider<T> {
    fn all_ids(&self) -> impl Iterator<Item=&EnchantmentTypeId> {
        self.enchantment_types
            .iter()
            .map(|enchantment_type| enchantment_type.id())
    }

    fn get(&self, id: &EnchantmentTypeId) -> Option<&impl EnchantmentType> {
        self.enchantment_types
            .iter()
            .find(|enchantment_type| enchantment_type.id() == id)
    }
}