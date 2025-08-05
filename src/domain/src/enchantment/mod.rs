use crate::enchantment::enchantment_type::EnchantmentType;

mod cost_multiplier;
mod enchantment_type;
mod enchantment_type_provider;

pub trait Enchantment {
    fn get_type(&self) -> &impl EnchantmentType;
    fn level(&self) -> u8;
}
