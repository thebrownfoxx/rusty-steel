use crate::enchantment::enchantment_type::{EnchantmentType, EnchantmentTypeId};

trait EnchantmentTypeSource {
    fn get(&self, id: EnchantmentTypeId) -> Option<EnchantmentType>;
}