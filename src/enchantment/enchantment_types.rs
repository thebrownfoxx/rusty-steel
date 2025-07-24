use crate::enchantment::enchantment_type::EnchantmentType;
use crate::enchantment::enchantment_type_id::EnchantmentTypeId;

pub trait EnchantmentTypes {
    fn all(&self) -> Vec<&EnchantmentTypeId>;
    fn get(&self, id: &EnchantmentTypeId) -> Option<&EnchantmentType>;
}

impl EnchantmentTypes for Vec<EnchantmentType> {
    fn all(&self) -> Vec<&EnchantmentTypeId> {
        self.iter()
            .map(|enchantment_type| &enchantment_type.id)
            .collect()
    }

    fn get(&self, id: &EnchantmentTypeId) -> Option<&EnchantmentType> {
        self.iter()
            .find(|enchantment_type| enchantment_type.id == *id)
    }
}
