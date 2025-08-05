use crate::enchantment::enchantment_type::{EnchantmentType, EnchantmentTypeId};

pub trait EnchantmentTypeProvider {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentTypeId>;
    fn get(&self, id: &EnchantmentTypeId) -> &impl EnchantmentType;
}