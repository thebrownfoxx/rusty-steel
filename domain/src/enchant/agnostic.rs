use crate::enchant::{Enchant, EnchantResult};
use crate::enchantment::Enchantment;
use crate::item::Item;

#[derive(Debug)]
pub struct AgnosticEnchanter;

impl Enchant for AgnosticEnchanter {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> EnchantResult {
        item.enchantment_levels.push(enchantment);
        Ok(())
    }
}
