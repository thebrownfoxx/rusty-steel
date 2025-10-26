use crate::enchant::{Enchant, EnchantResult};
use crate::enchantment::Enchantment;
use crate::item::Item;

pub struct AgnosticEnchant;

impl Enchant for AgnosticEnchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> EnchantResult {
        item.enchantments.push(enchantment);
        Ok(())
    }
}
