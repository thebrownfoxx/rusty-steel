use crate::enchantment::enchantment_type::EnchantmentTypeId;

pub struct Enchantment {
    pub type_id: EnchantmentTypeId,
    pub level: i8,
}