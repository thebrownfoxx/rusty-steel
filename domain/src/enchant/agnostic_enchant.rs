use super::Enchant;
use super::Result;
use crate::enchantment::Enchantment;
use crate::item::Item;

pub struct AgnosticEnchant;

impl Enchant for AgnosticEnchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        item.enchantments.push(enchantment);
        Ok(())
    }
}
