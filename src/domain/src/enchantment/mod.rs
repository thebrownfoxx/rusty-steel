use crate::enchantment::enchantment_type::EnchantmentType;

pub mod cost_multiplier;
pub mod enchantment_type;
pub mod enchantment_type_provider;

pub trait Enchantment {
    fn get_type(&self) -> &impl EnchantmentType;
    fn level(&self) -> u8;
}

pub struct OwnedEnchantment<T: EnchantmentType> {
    pub enchantment_type: T,
    level: u8,
}

impl<T: EnchantmentType> OwnedEnchantment<T> {
    fn new(enchantment_type: T, level: u8) -> Option<Self> {
        if level > enchantment_type.max_level() {
            return None;
        }
        Some(OwnedEnchantment {
            enchantment_type,
            level,
        })
    }
}

impl<T: EnchantmentType> Enchantment for OwnedEnchantment<T> {
    fn get_type(&self) -> &impl EnchantmentType {
        &self.enchantment_type
    }

    fn level(&self) -> u8 {
        self.level
    }
}
