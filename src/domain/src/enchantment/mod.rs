use crate::enchantment::enchantment_type::EnchantmentType;

pub mod cost_multiplier;
pub mod enchantment_type;
pub mod enchantment_type_provider;

pub trait Enchantment {
    fn get_type(&self) -> &impl EnchantmentType;
    fn level(&self) -> u8;
}
