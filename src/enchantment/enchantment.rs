use crate::enchantment::enchantment_type::EnchantmentType;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Enchantment {
    pub enchantment_type: EnchantmentType,
    pub level: i8,
}