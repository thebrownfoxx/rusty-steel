use crate::combine::cost::enchantment::EnchantmentCost;
use crate::enchantment::Enchantment;
use crate::item::Item;

pub struct ZeroEnchantmentCost;

impl EnchantmentCost for ZeroEnchantmentCost {
    fn enchantment_cost(&self, _: &Item, _: &Enchantment) -> u8 {
        0
    }
}